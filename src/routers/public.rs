use crate::handlers::public::health_check;
use axum::{Router, routing::get};

pub fn create_router() -> Router {
    Router::new().route("/public/health", get(health_check))
}
