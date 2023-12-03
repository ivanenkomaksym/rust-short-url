use std::{collections::HashMap, io};

use crate::services::hashservice;

pub struct InMemoryHashService {
    urls: HashMap<String, String>,
}

impl hashservice::HashService for InMemoryHashService{
    
    fn insert(value: &str) -> String{
        todo!()
    }

    fn find(key: &str) -> Result<String, io::Error> {
        todo!()
    }
}