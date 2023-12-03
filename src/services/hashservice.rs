pub trait HashService: Send + Sync {
    fn insert(& mut self, value: &str) -> String;

    fn find(&self, key: &str) -> Option<&String>;
}