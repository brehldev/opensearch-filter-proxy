use axum::{
    Json,
    extract::{Path, State},
};
use serde_json::Value;

use crate::state::OpenSearchRouterState;

pub async fn handle_search(
    State(state): State<OpenSearchRouterState>,
    Path(index): Path<String>,
    Json(payload): Json<Value>,
) -> Json<Value> {
    match state.opensearch_repo.search(&index, payload).await {
        Ok(result) => Json(result),
        Err(e) => {
            eprintln!("Search error: {}", e);
            Json(serde_json::json!({"error": e.to_string()}))
        }
    }
}

pub async fn handle_msearch(
    State(state): State<OpenSearchRouterState>,
    Path(index): Path<String>,
    Json(payload): Json<Value>,
) -> Json<Value> {
    match state.opensearch_repo.msearch(&index, payload).await {
        Ok(result) => Json(result),
        Err(e) => {
            eprintln!("MSearch error: {}", e);
            Json(serde_json::json!({"error": e.to_string()}))
        }
    }
}
