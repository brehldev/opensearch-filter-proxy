use serde::Deserialize;

/// Application configuration loaded from environment variables.
///
/// # Fields
/// - `opensearch_url` - OpenSearch instance URL (OPENSEARCH_URL)
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub opensearch_url: String, // OPENSEARCH_URL
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
