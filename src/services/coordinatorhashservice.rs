use std::collections::HashMap;
use std::fmt;

use crate::{services::hashservice, models::{linkinfo::LinkInfo, queryparams::QueryParams}, configuration};

use hash_ring::HashRing;
use async_trait::async_trait;

use super::hashserviceerror::HashServiceError;

#[derive(Clone, Debug)]
pub struct Node {
    pub host: String,
    pub port: u16,
    pub hostname: String
}

impl fmt::Display for Node {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}:{}", self.host, self.port)
    }
}

pub struct CoordinatorHashService {
    coordinator_config: configuration::settings::Coordinator,
    host_port_pairs: Vec<(String, usize)>,
    nodes: Vec<Node>,
    hash_ring: Option<HashRing<Node>>
}

impl CoordinatorHashService {
    pub fn new(config: &configuration::settings::Coordinator) -> Self {
        CoordinatorHashService {
            coordinator_config: config.clone(),
            host_port_pairs: Vec::new(),
            nodes: Vec::new(),
            hash_ring: None,
        }
    }
}

#[async_trait]
impl hashservice::HashService for CoordinatorHashService {
    async fn init(&mut self) -> Result<(), HashServiceError> {            
        self.host_port_pairs = self.coordinator_config.hostnames.iter().map(|x| {
            let key_value = x.split(':').map(|y| y.to_string()).collect::<Vec<String>>();
            return (key_value[0].clone(), key_value[1].parse::<usize>().unwrap())
        }).collect::<Vec<(String, usize)>>();
        
        let mut nodes: Vec<Node> = Vec::new();
        for host_port in &self.host_port_pairs {
            let host = host_port.0.clone();
            let port = host_port.1;
            
            test_connection(&host, port).await?;

            nodes.push(Node {
                host: host.clone(),
                port: port as u16,
                hostname: format!("{}:{}", &host, port)
            });
        }

        self.nodes = nodes.clone();
        self.hash_ring = Some(HashRing::new(nodes, 10));

        Ok(())
    }

    async fn get_links(&mut self, query_info: Option<QueryParams>) -> Result<Vec<LinkInfo>, HashServiceError> {
        let mut result: Vec<LinkInfo> = Vec::<LinkInfo>::new();

        for node in &self.nodes {
            let node_result = get_links_impl(&node.host, node.port.into(), query_info.clone()).await?;

            if result.len() > 0 {
                // TODO: Error handling in case values are different, so there is an inconsistency between replicas
            } else {
                result = node_result;
            }
        }

        Ok(result)
    }

    async fn insert(&mut self, value: &str) -> Result<String, HashServiceError> {
        let mut result: String = String::from("");

        for node in &self.nodes {
            let node_result = insert_impl(&node.host, node.port.into(), value).await?;
            
            if result.len() > 0 {
                // TODO: Error handling in case values are different, so there is an inconsistency between replicas
            } else {
                result = node_result;
            }
        }

        Ok(result)
    }

    async fn find(&mut self, key: &str) -> Result<Option<LinkInfo>, HashServiceError> {
        let mut result: Option<LinkInfo> = None;

        for node in &self.nodes {
            let node_result = match find_impl(&node.host, node.port.into(), key).await {
                Ok(value) => value,
                Err(e) => panic!("{}", e)
            };
            
            if result.is_some() {
                // TODO: Error handling in case values are different, so there is an inconsistency between replicas
            } else {
                result = node_result;
            }
        }

        Ok(result)
    }

    async fn delete(&mut self, _: &str) -> Result<bool, HashServiceError> {
        todo!()
    }
}

pub async fn test_connection(host: &String, port: usize) -> Result<(), HashServiceError> {
    let _resp = reqwest::get(format!("http://{}:{}/hello", host, port))
        .await?
            .json::<HashMap<String, String>>()
            .await?;

    Ok(())
}

pub async fn get_links_impl(host: &str, port: usize, _query_info: Option<QueryParams>) -> Result<Vec<LinkInfo>, HashServiceError> {
    // TODO: Implement QueryParams

    let urls = reqwest::get(format!("http://{}:{}/urls", host, port))
        .await?
            .json::<Vec<LinkInfo>>()
            .await?;
    
    Ok(urls)
}

pub async fn find_impl(host: &str, port: usize, key: &str) -> Result<Option<LinkInfo>, HashServiceError> {
    let response = match reqwest::get(format!("http://{}:{}/{}/summary", host, port, key))
        .await?
        .json::<LinkInfo>()
        .await {
            Ok(value) => value,
            Err(_) => return Ok(None)
        };

    Ok(Some(response))
}

pub async fn insert_impl(host: &str, port: usize, value: &str) -> Result<String, HashServiceError> {
    let short_url = reqwest::get(format!("http://{}:{}/shorten?long_url={}", host, port, value))
        .await?
            .text()
            .await?;

    let key_value = short_url.split('/').map(|y| y).collect::<Vec<&str>>();

    Ok(key_value[1].to_string())
}