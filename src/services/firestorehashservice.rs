use crate::{services::hashservice, models::{linkinfo::LinkInfo, queryparams::QueryParams}, configuration};

use firestore::*;
use async_trait::async_trait;

use super::{hashfunction, hashserviceerror::HashServiceError};

const COLLECTION_NAME: &'static str = "links";

pub struct FirestoreHashService {
    firestore_config: configuration::settings::FirestoreConfig,
    db: Option<FirestoreDb>,
}

impl FirestoreHashService {
    pub fn new(config: &configuration::settings::FirestoreConfig) -> impl hashservice::HashService {
        FirestoreHashService {
            firestore_config: config.clone(),
            db: None
        }
    }
}

#[async_trait]
impl hashservice::HashService for FirestoreHashService {
    async fn init(&mut self) -> Result<(), HashServiceError> {
        self.db = Some(FirestoreDb::new(self.firestore_config.project_id.clone()).await?);

        Ok(())
    }

    async fn get_links(&mut self, query_params: Option<QueryParams>) -> Result<Vec<LinkInfo>, HashServiceError> {
        let urls : Vec<LinkInfo> = self.db.as_mut().unwrap().fluent()
            .select()
            .from(COLLECTION_NAME)
            .obj()
            .query()
            .await?;

        let query_params = match query_params {
            Some(value) => value,
            None => return Ok(urls)
        };

        let top = query_params.top.unwrap_or(urls.len());
        let skip = query_params.skip.unwrap_or(0);
        
        Ok(urls.into_iter().skip(skip).take(top).collect())
    }

    async fn insert(&mut self, value: &str) -> Result<LinkInfo, HashServiceError> {
        let hash_value = hashfunction::hash(value);

        let find_result = self.find(&hash_value).await?;
        if find_result.is_some() {
            return Ok(find_result.unwrap());
        }
        
        let new_link = LinkInfo{
            short_url: hash_value.clone(),
            long_url: String::from(value),
            clicks: 0
        };

        self.db.as_mut().unwrap().fluent()
            .insert()
            .into(COLLECTION_NAME)
            .document_id(&hash_value)
            .object(&new_link)
            .execute::<()>()
            .await?;
        Ok(new_link)
    }

    async fn find(&mut self, key: &str) -> Result<Option<LinkInfo>, HashServiceError> {
        let find_result: Option<LinkInfo> = self.db.as_mut().unwrap().fluent()
            .select()
            .by_id_in(COLLECTION_NAME)
            .obj()
            .one(&key)
            .await?;

        let mut found_link = match find_result {
            Some(value) => value,
            None => return Ok(None),
        };        
        
        found_link.clicks += 1;

        self.db.as_mut().unwrap().fluent()
            .update()
            .fields(paths!(LinkInfo::clicks)) // Update only specified fields
            .in_col(COLLECTION_NAME)
            .document_id(&key)
            .object(&found_link)
            .execute::<()>()
            .await?;

        return Ok(Some(found_link))
    }

    async fn delete(&mut self, key: &str) -> Result<bool, HashServiceError> {
        self.db.as_mut().unwrap().fluent()
            .delete()
            .from(COLLECTION_NAME)
            .document_id(&key)
            .execute()
            .await?;

        // TODO: Check if the document was deleted
        return Ok(true)
    }
}