use std::collections::HashMap;

use crate::{services::hashservice, services::hashfunction, models::{linkinfo::LinkInfo, queryparams::QueryParams}};

use async_trait::async_trait;

use super::hashserviceerror::HashServiceError;

pub struct InMemoryHashService {
    pub urls: HashMap<String, LinkInfo>,
}

impl InMemoryHashService {
    pub fn new() -> Self {
        InMemoryHashService {
            urls: HashMap::new()
        }
    }
}

#[async_trait]
impl hashservice::HashService for InMemoryHashService {
    async fn insert(&mut self, value: &str) -> Result<LinkInfo, HashServiceError> {
        let hash_value = hashfunction::hash(value);

        let new_link = LinkInfo {
            long_url: value.to_string(),
            short_url: hash_value.clone(),
            clicks: 0
        };

        self.urls.entry(hash_value.clone()).or_insert(new_link.clone());

        return Ok(new_link)
    }

    async fn get_links(&mut self, query_params: Option<QueryParams>) -> Result<Vec<LinkInfo>, HashServiceError>
    {
        let urls = self.urls.iter().map(|key_value| key_value.1.clone()).collect();
        let query_params = match query_params {
            Some(value) => value,
            None => return Ok(urls)
        };

        let top = query_params.top.unwrap_or(urls.len());
        let skip = query_params.skip.unwrap_or(0);
        
        Ok(urls.into_iter().skip(skip).take(top).collect())
    }

    async fn find(&mut self, key: &str) -> Result<Option<LinkInfo>, HashServiceError> {
        #[cfg(debug_assertions)]
        // Print the content of the HashMap
        for (key, value) in &self.urls {
            log::debug!("Key: {}, Value: {:?}", key, value);
        }

        match self.urls.get_mut(key) {
            None => return Ok(None),
            Some(value) => {
                value.clicks += 1;
                return Ok(Some(value.clone())) // Directly return the mutable reference to value
            }
        }
    }

    async fn delete(&mut self, key: &str) -> Result<bool, HashServiceError> {
        Ok(self.urls.remove(key).is_some())
    }

    async fn init(&mut self) -> Result<(), HashServiceError> {
        Ok(())
    }
}