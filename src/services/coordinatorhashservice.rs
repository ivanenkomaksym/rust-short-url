use crate::{services::hashservice, models::{linkinfo::LinkInfo, queryparams::QueryParams}, configuration};

use async_trait::async_trait;

use super::hashserviceerror::HashServiceError;

pub struct CoordinatorHashService {
    coordinator_config: configuration::settings::Coordinator,
}

impl CoordinatorHashService {
    pub fn new(config: &configuration::settings::Coordinator) -> Self {
        CoordinatorHashService {
            coordinator_config: config.clone()
        }
    }
}

#[async_trait]
impl hashservice::HashService for CoordinatorHashService {
    async fn init(&mut self) -> Result<(), HashServiceError> {            
        let _host_port_pairs: Vec<(String, usize)> = self.coordinator_config.hostnames.iter().map(|x| {
            let key_value = x.split(':').map(|y| y.to_string()).collect::<Vec<String>>();
            return (key_value[0].clone(), key_value[1].parse::<usize>().unwrap())
        }).collect::<Vec<(String, usize)>>();

        todo!("Implement it");
    }

    async fn get_links(&self, _query_info: Option<QueryParams>) -> Vec<LinkInfo> {
        todo!("Implement it");
    }

    async fn insert(&mut self, _value: &str) -> String {
        todo!("Implement it");
    }

    async fn find(&mut self, _key: &str) -> Option<LinkInfo> {
        todo!("Implement it");
    }
}