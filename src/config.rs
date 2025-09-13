use serde::Deserialize;

/// Application configuration loaded from environment variables.
///
/// # Fields
/// - `opensearch_url` - OpenSearch instance URL (OPENSEARCH_URL)
/// - `reverse_proxy_target_url` - Reverse proxy target URL (REVERSE_PROXY_TARGET_URL)
/// - `reverse_proxy_prefix` - Reverse proxy prefix (REVERSE_PROXY_PREFIX)
/// - `reverse_proxy_banned_query_params` - Optional banned query parameters (REVERSE_PROXY_BANNED_QUERY_PARAMS)
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub opensearch_url: String,           // OPENSEARCH_URL
    pub reverse_proxy_target_url: String, // REVERSE_PROXY_TARGET_URL
    pub reverse_proxy_prefix: String,     // REVERSE_PROXY_PREFIX
    pub reverse_proxy_banned_query_params: Option<Vec<String>>, // REVERSE_PROXY_BANNED_QUERY_PARAMS
}

impl Config {
    /// Loads the configuration from environment variables using the `envy`
    /// crate.
    ///
    /// # Returns
    /// - `Ok(Config)` if the environment variables are successfully parsed into
    ///   a `Config` struct.
    /// - `Err(envy::Error)` if there is an error during parsing.
    ///
    /// # Panics
    /// This function will panic if the configuration cannot be loaded, printing
    /// the error details. ```
    pub fn from_env() -> Result<Self, envy::Error> {
        match envy::from_env::<Config>() {
            Ok(config) => {
                println!("Configuration loaded: {:#?}", config);
                Ok(config)
            }
            Err(error) => panic!("{:#?}", error),
        }
    }
}
