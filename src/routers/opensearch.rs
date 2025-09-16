use crate::{config::Config, handlers, state::OpenSearchRouterState};
use axum::{
    Router,
    routing::{get, post},
};

pub fn create_router(config: &Config) -> Router {
    Router::new()
        .route(
            "/_cluster/health",
            get(handlers::proxy::generic_get_proxy_handler),
        )
        .route(
            "/{index}/_search",
            post(handlers::opensearch::handle_search),
        )
        .route(
            "/{index}/_msearch",
            post(handlers::opensearch::handle_msearch),
        )
        .with_state(OpenSearchRouterState::new(config))
}
