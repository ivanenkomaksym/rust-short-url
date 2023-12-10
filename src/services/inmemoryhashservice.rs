use std::collections::HashMap;

use crate::{services::hashservice, services::hashfunction, models::linkinfo::LinkInfo};

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
    async fn insert(&mut self, value: &str) -> String {
        let hash_value = hashfunction::hash(value);
        self.urls.entry(hash_value.clone()).or_insert(LinkInfo { long_url:value.to_string(),clicks:0, short_url: String::from(value) });

        return hash_value
    }

    async fn find(&mut self, key: &str) -> Option<LinkInfo> {
        #[cfg(debug_assertions)]
        // Print the content of the HashMap
        for (key, value) in &self.urls {
            println!("Key: {}, Value: {:?}", key, value);
        }

        let result: &mut LinkInfo = match self.urls.get_mut(key) {
            None => return None,
            Some(value) => {
                value.clicks += 1;
                value // Directly return the mutable reference to value
            }
        };

        Some(result.clone())
    }

    async fn init(&mut self) -> Result<(), HashServiceError> {
        Ok(())
    }
}