use crate::{
    config::Config,
    repositories::{
        filter::FilterRepository, opensearch::OpenSearchRepository, proxy::ProxyRepository,
    },
    handlers::security_filter::SecurityFilterService
};

/// Shared state for OpenSearch-related routes.
///
/// Contains the repository instance that handlers can access
/// to perform OpenSearch operations.
#[derive(Clone)]
pub struct OpenSearchRouterState {
    pub(crate) opensearch_repo: OpenSearchRepository,
    pub(crate) security_filter_service: SecurityFilterService,
    pub(crate) filter_repository: FilterRepository,
    pub(crate) proxy_repo: ProxyRepository,
}

impl OpenSearchRouterState {
    pub fn new(config: &Config) -> Self {
        Self {
            opensearch_repo: OpenSearchRepository::new(config),
            security_filter_service: SecurityFilterService::new(),
            filter_repository: FilterRepository::new(config),
            proxy_repo: ProxyRepository::new(config),
        }
    }
}
