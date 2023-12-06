use sha2::{Sha256, Digest};
use std::collections::HashMap;

use crate::{services::hashservice, models::linkinfo::LinkInfo};

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

impl hashservice::HashService for InMemoryHashService {
    fn insert(&mut self, value: &str) -> String {
        let hash_value = hash(value);
        self.urls.entry(hash_value.clone()).or_insert(LinkInfo { long_url: value.to_string(), clicks: 0 });

        return hash_value
    }

    fn find(&mut self, key: &str) -> Option<&LinkInfo> {
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

        Some(result)
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