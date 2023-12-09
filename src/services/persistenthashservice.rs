use crate::{services::hashservice, models::linkinfo::LinkInfo, configuration};

use async_trait::async_trait;

pub struct PersistentHashService {
}

impl PersistentHashService {
    pub fn new(_database_config: &configuration::settings::Database) -> impl hashservice::HashService {
        PersistentHashService {
        }
    }
}

#[async_trait]
impl hashservice::HashService for PersistentHashService {
    fn insert(&mut self, _value: &str) -> String {
        todo!("Implement it")
    }

    fn find(&mut self, _key: &str) -> Option<&LinkInfo> {
        todo!("Implement it")
    }

    async fn init(&mut self) {
        todo!()
    }
}
