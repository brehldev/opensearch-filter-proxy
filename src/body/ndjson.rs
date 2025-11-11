//! NDJSON (Newline Delimited JSON) validation utilities.
//!
//! This module provides functionality to validate NDJSON format data,
//! which is used by OpenSearch's `_msearch` and `_bulk` APIs.

use axum::{
    Json,
    extract::{FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use bytes::Bytes;
use std::fmt;

/// Error type for NDJSON validation failures.
#[derive(Debug, Clone)]
pub struct NdjsonValidationError {
    /// The line number where the error occurred (1-based indexing)
    pub line_number: usize,
    /// Description of the validation error
    pub message: String,
}

impl fmt::Display for NdjsonValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Invalid NDJSON at line {}: {}",
            self.line_number, self.message
        )
    }
}

impl std::error::Error for NdjsonValidationError {}

/// Validates NDJSON format from raw bytes.
///
/// # Arguments
///
/// * `bytes` - The raw bytes to validate as NDJSON
///
/// # Returns
///
/// * `Ok(usize)` - The number of valid JSON lines parsed
/// * `Err(NdjsonValidationError)` - Details about the validation failure
///
pub fn validate_ndjson_lines(bytes: &[u8]) -> Result<usize, NdjsonValidationError> {
    if bytes.is_empty() {
        return Ok(0);
    }

    let lines: Vec<&[u8]> = bytes.split(|b| *b == b'\n').collect();
    let mut valid_line_count = 0;

    for (index, line) in lines.iter().enumerate() {
        let line_number = index + 1; // 1-based line numbering for user-facing errors

        // Skip empty lines, but only if they're at the end
        if line.is_empty() {
            // Check if this is a trailing empty line
            let remaining_lines = &lines[index..];
            let all_remaining_empty = remaining_lines.iter().all(|l| l.is_empty());

            if all_remaining_empty {
                // All remaining lines are empty, this is acceptable
                break;
            } else {
                return Err(NdjsonValidationError {
                    line_number,
                    message: "Empty line found in middle of NDJSON content".to_string(),
                });
            }
        }

        // Validate that the line is valid JSON
        if let Err(e) = serde_json::from_slice::<serde_json::Value>(line) {
            return Err(NdjsonValidationError {
                line_number,
                message: format!("Invalid JSON: {}", e),
            });
        }

        valid_line_count += 1;
    }

    Ok(valid_line_count)
}

/// A custom extractor for NDJSON request bodies.
///
/// This type validates the request body as NDJSON before making it available
/// to handlers. If validation fails, it returns a 400 BAD REQUEST response
/// with details about the error.
#[derive(Debug, Clone)]
pub struct NdjsonBody(pub Bytes);

/// Error response for NDJSON validation failures.
///
/// This type implements `IntoResponse` to convert validation errors into
/// proper HTTP 400 responses with JSON error details.
#[derive(Debug)]
pub struct NdjsonError(pub NdjsonValidationError);

impl IntoResponse for NdjsonError {
    fn into_response(self) -> Response {
        let error_message = self.0.to_string();
        tracing::warn!("NDJSON validation failed: {}", error_message);

        (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": {
                    "type": "invalid_ndjson",
                    "reason": error_message,
                    "line": self.0.line_number,
                }
            })),
        )
            .into_response()
    }
}

impl<S> FromRequest<S> for NdjsonBody
where
    S: Send + Sync,
{
    type Rejection = NdjsonError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // Extract the body bytes
        let bytes = Bytes::from_request(req, state).await.map_err(|_| {
            NdjsonError(NdjsonValidationError {
                line_number: 0,
                message: "Failed to read request body".to_string(),
            })
        })?;

        // Validate NDJSON format
        let line_count = validate_ndjson_lines(&bytes).map_err(NdjsonError)?;

        tracing::debug!(
            "Successfully validated NDJSON body with {} lines ({} bytes)",
            line_count,
            bytes.len()
        );

        Ok(NdjsonBody(bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ndjson() {
        let input = b"{\"index\":{}}\n{\"query\":{\"match_all\":{}}}\n";
        let result = validate_ndjson_lines(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2);
    }

    #[test]
    fn test_valid_ndjson_with_trailing_newline() {
        let input = b"{\"index\":{}}\n{\"query\":{\"match_all\":{}}}\n";
        let result = validate_ndjson_lines(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2);
    }

    #[test]
    fn test_valid_ndjson_without_trailing_newline() {
        let input = b"{\"index\":{}}\n{\"query\":{\"match_all\":{}}}";
        let result = validate_ndjson_lines(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2);
    }

    #[test]
    fn test_invalid_json_first_line() {
        let input = b"{invalid}\n{\"query\":{\"match_all\":{}}}\n";
        let result = validate_ndjson_lines(input);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.line_number, 1);
        assert!(err.message.contains("Invalid JSON"));
    }

    #[test]
    fn test_invalid_json_second_line() {
        let input = b"{\"index\":{}}\n{invalid json}\n";
        let result = validate_ndjson_lines(input);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.line_number, 2);
        assert!(err.message.contains("Invalid JSON"));
    }

    #[test]
    fn test_empty_input() {
        let input = b"";
        let result = validate_ndjson_lines(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn test_empty_line_in_middle() {
        let input = b"{\"index\":{}}\n\n{\"query\":{\"match_all\":{}}}\n";
        let result = validate_ndjson_lines(input);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.line_number, 2);
        assert!(err.message.contains("Empty line"));
    }

    #[test]
    fn test_multiple_trailing_empty_lines() {
        let input = b"{\"index\":{}}\n{\"query\":{\"match_all\":{}}}\n\n\n";
        let result = validate_ndjson_lines(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2);
    }

    #[test]
    fn test_complex_ndjson() {
        let input = b"{\"index\":{\"_index\":\"test\"}}\n{\"field\":\"value\",\"nested\":{\"key\":123}}\n{\"delete\":{\"_id\":\"1\"}}\n";
        let result = validate_ndjson_lines(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 3);
    }

    #[tokio::test]
    async fn test_ndjson_body_extractor_valid() {
        use axum::body::Body;
        use axum::http::Request;

        let valid_ndjson = b"{\"index\":{}}\n{\"query\":{\"match_all\":{}}}\n";
        let req = Request::builder()
            .method("POST")
            .uri("/test/_msearch")
            .body(Body::from(&valid_ndjson[..]))
            .unwrap();

        let result = NdjsonBody::from_request(req, &()).await;
        assert!(result.is_ok());
        let NdjsonBody(bytes) = result.unwrap();
        assert_eq!(bytes.as_ref(), valid_ndjson);
    }

    #[tokio::test]
    async fn test_ndjson_body_extractor_invalid() {
        use axum::body::Body;
        use axum::http::Request;

        let invalid_ndjson = b"{\"index\":{}}\n{invalid json}\n";
        let req = Request::builder()
            .method("POST")
            .uri("/test/_msearch")
            .body(Body::from(&invalid_ndjson[..]))
            .unwrap();

        let result = NdjsonBody::from_request(req, &()).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.0.line_number, 2);
    }

    #[tokio::test]
    async fn test_ndjson_body_extractor_empty() {
        use axum::body::Body;
        use axum::http::Request;

        let empty_ndjson = b"";
        let req = Request::builder()
            .method("POST")
            .uri("/test/_msearch")
            .body(Body::from(&empty_ndjson[..]))
            .unwrap();

        let result = NdjsonBody::from_request(req, &()).await;
        assert!(result.is_ok());
        let NdjsonBody(bytes) = result.unwrap();
        assert_eq!(bytes.len(), 0);
    }

    #[tokio::test]
    async fn test_ndjson_body_extractor_msearch_format() {
        use axum::body::Body;
        use axum::http::Request;

        // Typical msearch format: alternating metadata and query lines
        let msearch_ndjson = b"{\"index\":\"test-index\"}\n{\"query\":{\"match\":{\"field\":\"value\"}}}\n{\"index\":\"another-index\"}\n{\"query\":{\"match_all\":{}}}\n";
        let req = Request::builder()
            .method("POST")
            .uri("/test/_msearch")
            .body(Body::from(&msearch_ndjson[..]))
            .unwrap();

        let result = NdjsonBody::from_request(req, &()).await;
        assert!(result.is_ok());
        let NdjsonBody(bytes) = result.unwrap();
        assert_eq!(bytes.as_ref(), msearch_ndjson);
    }

    #[tokio::test]
    async fn test_ndjson_error_into_response() {
        use axum::response::IntoResponse;

        let error = NdjsonError(NdjsonValidationError {
            line_number: 5,
            message: "Invalid JSON syntax".to_string(),
        });

        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
