use crate::{services::hashservice, models::linkinfo::LinkInfo, configuration};

pub struct PersistentHashService {
}

impl PersistentHashService {
    pub fn new(_database_config: &configuration::settings::Database) -> impl hashservice::HashService {
        PersistentHashService {
        }
    }
}

impl hashservice::HashService for PersistentHashService {
    fn insert(&mut self, _value: &str) -> String {
        todo!("Implement it")
    }

    fn find(&mut self, _key: &str) -> Option<&LinkInfo> {
        todo!("Implement it")
    }
}
