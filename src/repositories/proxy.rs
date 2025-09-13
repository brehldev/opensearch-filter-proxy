use reqwest::Client;
use serde_json::Value;

use crate::config::Config;

#[derive(Clone)]
pub struct ProxyRepository {
    client: Client,
    target_url: String,
}

impl ProxyRepository {
    pub fn new(config: &Config) -> Self {
        let client = Client::new();

        Self {
            client,
            target_url: config.reverse_proxy_target_url.clone(),
        }
    }

    pub async fn proxy_get_request(&self, rest: &str) -> Result<Value, reqwest::Error> {
        let url = format!("{}/{}", self.target_url, rest);
        let response = self.client.get(&url).send().await?;
        let body = response.json().await?;
        Ok(body)
    }
}
