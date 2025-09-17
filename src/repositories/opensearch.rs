use std::error::Error;

use opensearch::cluster::ClusterHealthParts;
use opensearch::{
    MsearchParts, OpenSearch, SearchParts,
    http::{request::JsonBody, transport::Transport},
};
use serde_json::Value;

use crate::config::Config;

#[derive(Clone)]
pub struct OpenSearchRepository {
    client: OpenSearch,
}

impl OpenSearchRepository {
    pub fn new(config: &Config) -> Self {
        let transport =
            Transport::single_node(&config.opensearch_url).expect("Failed to create transport");
        let client = OpenSearch::new(transport);

        Self { client }
    }

    pub async fn cluster_health(&self) -> Result<Value, Box<dyn Error + Send + Sync>> {
        let response = self
            .client
            .cluster()
            .health(ClusterHealthParts::None)
            .send()
            .await?;
        let response_body = response.json::<Value>().await?;
        Ok(response_body)
    }

    pub async fn search(
        &self,
        index: &str,
        payload: Value,
    ) -> Result<Value, Box<dyn Error + Send + Sync>> {
        let response = self
            .client
            .search(SearchParts::Index(&[index]))
            .body(payload)
            .send()
            .await?;

        let response_body = response.json::<Value>().await?;
        Ok(response_body)
    }

    pub async fn msearch(
        &self,
        index: &str,
        payload: Value,
    ) -> Result<Value, Box<dyn Error + Send + Sync>> {
        let body = vec![JsonBody::new(payload)];

        let response = self
            .client
            .msearch(MsearchParts::Index(&[index]))
            .body(body)
            .send()
            .await?;

        let response_body = response.json::<Value>().await?;
        Ok(response_body)
    }
}
