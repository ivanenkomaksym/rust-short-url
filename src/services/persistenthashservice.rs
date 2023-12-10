use crate::{services::hashservice, models::linkinfo::LinkInfo, configuration};
use mongodb::{ bson::doc, options::{ ClientOptions, ServerApi, ServerApiVersion }, Client, Collection };

use async_trait::async_trait;
use sha2::{Sha256, Digest};

use super::hashserviceerror::HashServiceError;

pub struct PersistentHashService {
    database_config: configuration::settings::Database,
    collection: Option<Collection<LinkInfo>>
}

impl PersistentHashService {
    pub fn new(config: &configuration::settings::Database) -> impl hashservice::HashService {
        PersistentHashService {
            database_config: config.clone(),
            collection: None
        }
    }
}

#[async_trait]
impl hashservice::HashService for PersistentHashService {
    async fn insert(&mut self, value: &str) -> String {
        let hash_value = hash(value);

        let find_result = self.find(&hash_value).await;
        if find_result.is_some() {
            return hash_value;
        }
        
        let new_link = LinkInfo{
            short_url: hash_value.clone(),
            long_url: String::from(value),
            clicks: 0
        };

        let insert_result = self.collection.as_mut().unwrap().insert_one(new_link, None).await;
        match  insert_result {
            Ok(_) => hash_value,
            Err(e) => panic!("Problem inserting document: {:?}", e)
        }
    }

    async fn find(&mut self, key: &str) -> Option<LinkInfo> {
        let find_result = self.collection.as_mut().unwrap().find_one(
            doc! { "short_url": key }, None
        ).await;

        match find_result {
            Ok(result) => result,
            Err(_) => None
        }
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
        println!("Pinged your deployment. You successfully connected to MongoDB!");

        self.collection = Some(client.database(&self.database_config.database_name).collection::<LinkInfo>(&self.database_config.collection_name));

        Ok(())
    }
}

fn hash(value_to_hash: &str) -> String {
    let mut sha256 = Sha256::new();
    sha256.update(value_to_hash);    
    let hash_result = sha256.finalize();

    // Take the first 4 bytes (32 bits) of the hash and convert them to u32
    let hash_value = u32::from_be_bytes([hash_result[0], hash_result[1], hash_result[2], hash_result[3]]);

    // Format the u32 as an 8-digit string
    return format!("{:X}", hash_value)
}