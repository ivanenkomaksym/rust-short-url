use crate::{services::hashservice, services::hashfunction, models::{linkinfo::LinkInfo, queryparams::QueryParams}, configuration};
use futures_util::TryStreamExt;
use mongodb::{ bson::doc, options::{ ClientOptions, ServerApi, ServerApiVersion }, Client, Collection };

use async_trait::async_trait;

use super::hashserviceerror::HashServiceError;

pub struct MongoHashService {
    database_config: configuration::settings::Database,
    collection: Option<Collection<LinkInfo>>
}

impl MongoHashService {
    pub fn new(config: &configuration::settings::Database) -> impl hashservice::HashService {
        MongoHashService {
            database_config: config.clone(),
            collection: None
        }
    }
}

#[async_trait]
impl hashservice::HashService for MongoHashService {
    async fn insert(&mut self, value: &str) -> Result<String, HashServiceError> {
        let hash_value = hashfunction::hash(value);

        let find_result = self.find(&hash_value).await?;
        if find_result.is_some() {
            return Ok(hash_value);
        }
        
        let new_link = LinkInfo{
            short_url: hash_value.clone(),
            long_url: String::from(value),
            clicks: 0
        };

        self.collection.as_mut().unwrap().insert_one(new_link, None).await?;
        Ok(hash_value)
    }

    async fn find(&mut self, key: &str) -> Result<Option<LinkInfo>, HashServiceError> {
        let find_result = self.collection.as_mut().unwrap().find_one(
            doc! { "short_url": key }, None
        ).await?;
        
        let mut unwrapped_result = match find_result {
            Some(value) => value,
            None => return Ok(None),
        };
        
        unwrapped_result.clicks += 1;
        return Ok(Some(unwrapped_result))
    }

    async fn init(&mut self) -> Result<(), HashServiceError> {
        let mut client_options = ClientOptions::parse(&self.database_config.connection_string).await?;
        // Set the server_api field of the client_options object to Stable API version 1
        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);
        // Create a new client and connect to the server
        let client = Client::with_options(client_options)?;
        // Send a ping to confirm a successful connection
        client.database("admin").run_command(doc! { "ping": 1 }, None).await?;
        log::debug!("Pinged your deployment. You successfully connected to MongoDB!");

        self.collection = Some(client.database(&self.database_config.database_name).collection::<LinkInfo>(&self.database_config.collection_name));

        Ok(())
    }

    async fn get_links(&mut self, query_params: Option<QueryParams>) -> Result<Vec<LinkInfo>, HashServiceError>
    {
        let coll = match &self.collection {
            Some(value) => value,
            None => return Ok([].to_vec())
        };
        
        let cursor = coll.find(
            doc! {}, None
        ).await?;
        
        let urls: Vec<LinkInfo> = cursor.try_collect().await.expect("");

        let query_params = match query_params {
            Some(value) => value,
            None => return Ok(urls)
        };

        let top = query_params.top.unwrap_or(urls.len());
        let skip = query_params.skip.unwrap_or(0);
        
        Ok(urls.into_iter().skip(skip).take(top).collect())
    }
}