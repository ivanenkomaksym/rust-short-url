use crate::{services::hashservice, models::{linkinfo::LinkInfo, queryparams::QueryParams}, configuration};

use async_trait::async_trait;
use redis::{Commands, JsonCommands};

use super::{hashfunction, hashserviceerror::HashServiceError};

pub struct RedisHashService {
    redis_config: configuration::settings::RedisConfig,
    connection: Option<redis::Connection>,
}

impl RedisHashService {
    pub fn new(config: &configuration::settings::RedisConfig) -> impl hashservice::HashService {
        RedisHashService {
            redis_config: config.clone(),
            connection: None
        }
    }
}

#[async_trait]
impl hashservice::HashService for RedisHashService {
    async fn init(&mut self) -> Result<(), HashServiceError> {
        let client = redis::Client::open(self.redis_config.connection_string.clone()).unwrap();
        self.connection = Some(client.get_connection().unwrap());

        Ok(())
    }

    async fn insert(&mut self, value: &str) -> Result<LinkInfo, HashServiceError> {
        let hash_value = hashfunction::hash(value);

        let new_link = LinkInfo{
            short_url: hash_value.clone(),
            long_url: String::from(value),
            clicks: 0
        };
        
        self.connection.as_mut().unwrap().json_set::<_, _, _, LinkInfo>(&hash_value, "$", &new_link)?;

        Ok(new_link)
    }

    async fn update(&mut self, key: &str, value: &LinkInfo) -> Result<bool, HashServiceError> {
        self.connection.as_mut().unwrap().json_set::<_, _, _, LinkInfo>(key, "$", value)?;

        Ok(true)
    }

    async fn find(&mut self, key: &str) -> Result<Option<LinkInfo>, HashServiceError> {
        let result = self.connection.as_mut().unwrap().json_get::<&str, &str, String>(key, "$")?;
        
        let found_links: Vec<LinkInfo> = match serde_json::from_str(result.as_str()) {
            Ok(v) => v,
            Err(err) => panic!("{}", err)
        };

        if found_links.is_empty() {
            return Ok(None);
        }

        let mut found_link = found_links.first().unwrap().clone();
        found_link.clicks += 1;

        self.connection.as_mut().unwrap().json_set::<_, _, _, LinkInfo>(key.to_string(), "$", &found_link)?;

        return Ok(Some(found_link));
    }

    async fn delete(&mut self, key: &str) -> Result<bool, HashServiceError> {
        let result = self.connection.as_mut().unwrap().del(key)?;

        Ok(result)
    }

    async fn get_links(&mut self, query_params: Option<QueryParams>) -> Result<Vec<LinkInfo>, HashServiceError> {
        // Get all keys
        let keys: Vec<String> = self.connection.as_mut().unwrap().keys("*")?;

        let mut links: Vec<LinkInfo> = vec![];
        
        // Iterate over keys and get their values
        for key in keys {
            let result = self.connection.as_mut().unwrap().json_get::<&str, &str, String>(key.as_str(), "$")?;
            
            let found_links: Vec<LinkInfo> = match serde_json::from_str(result.as_str()) {
                Ok(v) => v,
                Err(err) => panic!("{}", err)
            };

            links.push(found_links.first().unwrap().clone());
        }

        // Filter
        let query_params = match query_params {
            Some(value) => value,
            None => return Ok(links)
        };

        let top = query_params.top.unwrap_or(links.len());
        let skip = query_params.skip.unwrap_or(0);
        
        Ok(links.into_iter().skip(skip).take(top).collect())
    }
}