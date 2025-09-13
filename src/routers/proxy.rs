use axum::{Router, routing::get};

use crate::{config::Config, handlers::proxy::proxy_handler, state::ProxyRouterState};

pub fn create_router(config: &Config) -> Router {
    let route_path = format!("{}/{{*rest}}", config.reverse_proxy_prefix);

    let state = ProxyRouterState::new(config);

    Router::new()
        .route(&route_path, get(proxy_handler))
        .with_state(state)
}
