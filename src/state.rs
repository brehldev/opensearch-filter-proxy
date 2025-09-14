use crate::handlers::security_filter::SecurityFilterService;
use crate::{
    config::Config,
    repositories::{
        filter::FilterRepository, opensearch::OpenSearchRepository, proxy::ProxyRepository,
    },
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
}

impl OpenSearchRouterState {
    pub fn new(config: &Config) -> Self {
        let opensearch_repo = OpenSearchRepository::new(config);
        Self {
            opensearch_repo,
            security_filter_service: SecurityFilterService::new(),
            filter_repository: FilterRepository::new(config),
        }
    }
}

/// Shared state for proxy-related routes.
///
/// This struct contains the repository instance that handlers can use
/// to forward HTTP requests to a target URL.
#[derive(Clone)]
pub struct ProxyRouterState {
    /// The repository responsible for proxying HTTP requests.
    pub(crate) proxy_repo: ProxyRepository,
    pub(crate) config: Config,
}

impl ProxyRouterState {
    pub fn new(config: &Config) -> Self {
        let proxy_repo = ProxyRepository::new(config);
        Self {
            config: config.clone(),
            proxy_repo,
        }
    }
}
