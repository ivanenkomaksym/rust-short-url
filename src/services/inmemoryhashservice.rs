use std::collections::HashMap;

use crate::{models::{builders::build_link_info, linkinfo::LinkInfo, queryparams::QueryParams}, services::{hashfunction, hashservice}};

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

        let new_link = build_link_info(hash_value.clone(), String::from(value));

        self.urls.entry(hash_value.clone()).or_insert(new_link.clone());

        return Ok(new_link)
    }

    async fn update(&mut self, key: &str, value: &LinkInfo) -> Result<bool, HashServiceError> {
        match self.urls.get_mut(key) {
            None => return Ok(false),
            Some(link) => {
                *link = value.clone();
                return Ok(true)
            }
        }
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