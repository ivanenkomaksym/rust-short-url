use crate::models::{linkinfo::LinkInfo, queryparams::QueryParams};

use async_trait::async_trait;

use super::hashserviceerror::HashServiceError;

#[async_trait]
pub trait HashService: Send + Sync {
    async fn init(&mut self) -> Result<(), HashServiceError>;

    async fn get_links(&mut self, query_info: Option<QueryParams>) -> Result<Vec<LinkInfo>, HashServiceError>;

    async fn insert(&mut self, value: &str) -> Result<String, HashServiceError>;

    async fn find(&mut self, key: &str) -> Result<Option<LinkInfo>, HashServiceError>;

    async fn delete(&mut self, key: &str) -> Result<bool, HashServiceError>;
}