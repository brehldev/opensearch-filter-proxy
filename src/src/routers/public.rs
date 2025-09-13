use axum::{Json, Router, routing::get};

use crate::models::health::HealthCheckResponse;

pub fn create_router() -> Router {
    Router::new().route("/public/health", get(health_check))
}

async fn health_check() -> Json<HealthCheckResponse> {
    let response = HealthCheckResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };

    Json(response)
}
