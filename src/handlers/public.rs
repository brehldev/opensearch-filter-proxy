use crate::models::health::HealthCheckResponse;
use axum::Json;

pub async fn health_check() -> Json<HealthCheckResponse> {
    let response = HealthCheckResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };

    Json(response)
}
