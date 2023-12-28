use std::fmt;
use std::sync::Arc;

use actix_web::web::Buf;
use futures_util::lock::Mutex;
use http_body_util::{Empty, BodyExt};
use hyper::{Request, client::conn::http1::SendRequest};
use hyper::body::Bytes;
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;

use crate::{services::hashservice, models::{linkinfo::LinkInfo, queryparams::QueryParams}, configuration};

use hash_ring::HashRing;
use async_trait::async_trait;

use super::hashserviceerror::HashServiceError;

#[derive(Clone, Debug)]
pub struct Node {
    pub host: String,
    pub port: u16,
    pub hostname: String,
    pub sender: Arc<Mutex<SendRequest<Empty<Bytes>>>>
}

impl fmt::Display for Node {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}:{}", self.host, self.port)
    }
}

pub struct CoordinatorHashService {
    coordinator_config: configuration::settings::Coordinator,
    host_port_pairs: Vec<(String, usize)>,
    hash_ring: Option<HashRing<Node>>
}

impl CoordinatorHashService {
    pub fn new(config: &configuration::settings::Coordinator) -> Self {
        CoordinatorHashService {
            coordinator_config: config.clone(),
            host_port_pairs: Vec::new(),
            hash_ring: None,
        }
    }
}

#[async_trait]
impl hashservice::HashService for CoordinatorHashService {
    async fn init(&mut self) -> Result<(), HashServiceError> {            
        self.host_port_pairs = self.coordinator_config.hostnames.iter().map(|x| {
            let key_value = x.split(':').map(|y| y.to_string()).collect::<Vec<String>>();
            println!("{}, {}", key_value[0], key_value[1]);
            return (key_value[0].clone(), key_value[1].parse::<usize>().unwrap())
        }).collect::<Vec<(String, usize)>>();
        
        let mut nodes: Vec<Node> = Vec::new();
        for host_port in &self.host_port_pairs {
            let host = host_port.0.clone();
            let port = host_port.1;

            let mut sender = setup_connection(&host, port).await?;
            
            test_connection(&host, port, &mut sender).await?;
        
            let sender = Arc::new(Mutex::new(sender));

            nodes.push(Node {
                host: host.clone(),
                port: port as u16,
                hostname: format!("{}:{}", &host, port),
                sender
            });
        }

        self.hash_ring = Some(HashRing::new(nodes, 10));

        Ok(())
    }

    async fn get_links(&self, _query_info: Option<QueryParams>) -> Vec<LinkInfo> {
        todo!("Implement it");
    }

    async fn insert(&mut self, _value: &str) -> String {
        todo!("Implement it");
    }

    async fn find(&mut self, key: &str) -> Option<LinkInfo> {
        let hash_ring = match &self.hash_ring {
            Some(value) => value,
            None => return None
        };

        let node = hash_ring.get_node(key.to_string()).unwrap();

        let url = format!("http://{}:{}/{}", node.host, node.port, key.to_string()).parse::<hyper::Uri>().unwrap();

        let authority = url.authority().unwrap().clone();

        // Create an HTTP request with an empty body and a HOST header
        let req: Request<Empty<Bytes>> = Request::builder()
            .uri(url)
            .header(hyper::header::HOST, authority.as_str())
            .body(Empty::<Bytes>::new()).unwrap();

        // Await the response...
        let res = match node.sender.lock().await.send_request(req).await {
            Ok(value) => value,
            Err(_) => return None
        };
        
        println!("Response: {}", res.status());
        println!("Headers: {:#?}\n", res.headers());

        // asynchronously aggregate the chunks of the body
        let body = res.collect().await.unwrap().aggregate();

        // try to parse as json with serde_json
        let linkinfo: Option<LinkInfo> = Some(serde_json::from_reader(body.reader()).unwrap());

        linkinfo
    }
}

pub async fn setup_connection(host: &String, port: usize) -> Result<SendRequest<Empty<Bytes>>, HashServiceError> {
    let address = format!("{}:{}", host, port);
    // Open a TCP connection to the remote host
    let stream = TcpStream::connect(address).await?;

    // Use an adapter to access something implementing `tokio::io` traits as if they implement
    // `hyper::rt` IO traits.
    let io = TokioIo::new(stream);

    // Perform a TCP handshake
    let (sender, conn) = hyper::client::conn::http1::handshake(io).await?;

    // Spawn a task to poll the connection, driving the HTTP state
    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    return Ok(sender);
}

pub async fn test_connection(host: &String, port: usize, sender: &mut SendRequest<Empty<Bytes>>) -> Result<(), HashServiceError> {
    let url = format!("http://{}:{}/hello", host, port).parse::<hyper::Uri>().unwrap();

    let authority = url.authority().unwrap().clone();

    // Create an HTTP request with an empty body and a HOST header
    let req: Request<Empty<Bytes>> = Request::builder()
        .uri(url)
        .header(hyper::header::HOST, authority.as_str())
        .body(Empty::<Bytes>::new()).unwrap();

    // Await the response...
    let res = sender.send_request(req).await?;
    
    println!("Response: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());

    Ok(())
}