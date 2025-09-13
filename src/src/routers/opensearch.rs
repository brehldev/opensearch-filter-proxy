use axum::{Router, routing::post};

use crate::{config::Config, handlers, state::OpenSearchRouterState};

pub fn create_router(config: &Config) -> Router {
    let state = OpenSearchRouterState::new(config);

    Router::new()
        .route(
            "/{index}/_search",
            post(handlers::opensearch::handle_search),
        )
        .route(
            "/{index}/_msearch",
            post(handlers::opensearch::handle_msearch),
        )
        .with_state(state)
}
