use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;

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
            target_url: config.opensearch_url.clone(),
        }
    }

    pub async fn get(
        &self,
        rest: &str,
        query_params: &HashMap<String, String>,
    ) -> Result<Value, reqwest::Error> {
        let url = format!("{}{}", self.target_url, rest);
        let response = self.client.get(&url).query(&query_params).send().await?;
        let body = response.json().await?;
        Ok(body)
    }
}
