mod config;
mod handlers;
mod models;
mod repositories;
mod routers;
mod state;
use axum::Router;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::Config;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum=debug",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env().expect("Failed to load configuration");

    let app = app(&config);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

/// Creates the main application router by combining public and OpenSearch
/// routes.
///
/// Both route groups include HTTP tracing middleware for observability.
fn app(config: &Config) -> Router {
    let public_routes = routers::public::create_router().layer(TraceLayer::new_for_http());
    let opensearch_routes =
        routers::opensearch::create_router(config).layer(TraceLayer::new_for_http());

    Router::new().merge(public_routes).merge(opensearch_routes)
}
