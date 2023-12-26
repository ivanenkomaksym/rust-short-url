use crate::{services::hashservice, models::{linkinfo::LinkInfo, queryparams::QueryParams}, configuration};

use hash_ring::HashRing;
use hash_ring::NodeInfo;
use async_trait::async_trait;

use super::hashserviceerror::HashServiceError;

pub struct CoordinatorHashService {
    coordinator_config: configuration::settings::Coordinator,
    hash_ring: Option<HashRing<NodeInfo>>
}

impl CoordinatorHashService {
    pub fn new(config: &configuration::settings::Coordinator) -> Self {
        CoordinatorHashService {
            coordinator_config: config.clone(),
            hash_ring: None,
        }
    }
}

#[async_trait]
impl hashservice::HashService for CoordinatorHashService {
    async fn init(&mut self) -> Result<(), HashServiceError> {            
        let host_port_pairs: Vec<(String, usize)> = self.coordinator_config.hostnames.iter().map(|x| {
            let key_value = x.split(':').map(|y| y.to_string()).collect::<Vec<String>>();
            return (key_value[0].clone(), key_value[1].parse::<usize>().unwrap())
        }).collect::<Vec<(String, usize)>>();
        
        let mut nodes: Vec<NodeInfo> = Vec::new();
        for host_port in host_port_pairs {
            nodes.push(NodeInfo {
                host: "localhost",
                port: host_port.1 as u16,
            });
        }

        self.hash_ring = Some(HashRing::new(nodes, 10));

        let hashring = self.hash_ring.as_ref().unwrap();

        println!(
            "Key: '{}', Node: {}",
            "hello",
            hashring.get_node(("hello").to_string()).unwrap()
        );
    
        println!(
            "Key: '{}', Node: {}",
            "dude",
            hashring.get_node(("dude").to_string()).unwrap()
        );
    
        println!(
            "Key: '{}', Node: {}",
            "martian",
            hashring.get_node(("martian").to_string()).unwrap()
        );
    
        println!(
            "Key: '{}', Node: {}",
            "tardis",
            hashring.get_node(("tardis").to_string()).unwrap()
        );
    
        println!(
            "Key: '{}', Node: {}",
            "hello",
            hashring.get_node(("hello").to_string()).unwrap()
        );        

        Ok(())
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