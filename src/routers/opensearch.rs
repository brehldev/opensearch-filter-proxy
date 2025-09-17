use crate::handlers::opensearch::{handle_cluster_health, handle_msearch, handle_search};
use crate::{config::Config, state::OpenSearchRouterState};
use axum::{
    Router,
    routing::{get, post},
};

pub fn create_router(config: &Config) -> Router {
    Router::new()
        .route("/_cluster/health", get(handle_cluster_health))
        .route("/{index}/_search", post(handle_search))
        .route("/{index}/_msearch", post(handle_msearch))
        .with_state(OpenSearchRouterState::new(config))
}
