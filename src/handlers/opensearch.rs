use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::Value;

use crate::state::OpenSearchRouterState;

pub async fn handle_search(
    State(state): State<OpenSearchRouterState>,
    Path(index): Path<String>,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    // For demonstration, we use a fake filter. In a real application,
    // this would be another api call or derived from user context.
    let fake_filter = state.filter_repository.get_filter();

    let query_with_security_filter = state.security_filter_service.apply(payload, fake_filter.0);

    match state
        .opensearch_repo
        .search(&index, query_with_security_filter)
        .await
    {
        Ok(result) => Json(result).into_response(),
        Err(e) => {
            eprintln!("Search error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": e.to_string()})),
            )
                .into_response()
        }
    }
}

pub async fn handle_msearch(
    State(state): State<OpenSearchRouterState>,
    Path(index): Path<String>,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    match state.opensearch_repo.msearch(&index, payload).await {
        Ok(result) => Json(result).into_response(),
        Err(e) => {
            eprintln!("MSearch error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": e.to_string()})),
            )
                .into_response()
        }
    }
}

pub async fn handle_cluster_health(
    State(state): State<OpenSearchRouterState>,
) -> impl IntoResponse {
    match state.opensearch_repo.cluster_health().await {
        Ok(result) => Json(result).into_response(),
        Err(e) => {
            eprintln!("Cluster health error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": e.to_string()})),
            )
                .into_response()
        }
    }
}
