use std::collections::HashMap;

use axum::{
    Json,
    extract::{Request, State},
    response::IntoResponse,
};

use crate::state::OpenSearchRouterState;

pub async fn generic_get_proxy_handler(
    State(state): State<OpenSearchRouterState>,
    request: Request,
) -> impl IntoResponse {
    let path = request.uri().path();

    match state
        .proxy_repo
        .get(path, &HashMap::new())
        .await
    {
        Ok(response) => Json(response).into_response(),
        Err(e) => {
            eprintln!("Proxy error: {}", e);
            Json(serde_json::json!({"error": e.to_string()})).into_response()
        }
    }
}
