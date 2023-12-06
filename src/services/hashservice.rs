use crate::models::linkinfo::LinkInfo;

pub trait HashService: Send + Sync {
    fn insert(&mut self, value: &str) -> String;

    fn find(&mut self, key: &str) -> Option<&LinkInfo>;
}