use std::collections::HashMap;

use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde_json::Value;

use crate::{
    state::ProxyRouterState,
    utils::{is_banned_path, is_banned_query_param},
};

pub async fn proxy_handler(
    State(state): State<ProxyRouterState>,
    Path(path): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Json<Value> {
    if is_banned_path(&path) {
        return Json(serde_json::json!({"error": "Access to this path is banned"}));
    }

    if is_banned_query_param(&params, &state.config.reverse_proxy_banned_query_params) {
        return Json(serde_json::json!({"error": "Access to this query parameter is banned"}));
    }

    match state.proxy_repo.proxy_get_request(&path).await {
        Ok(response) => Json(response),
        Err(e) => {
            eprintln!("Proxy error: {}", e);
            Json(serde_json::json!({"error": e.to_string()}))
        }
    }
}
