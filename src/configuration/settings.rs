use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::{env, fmt};
use clap::Parser;

use crate::constants::{DEFAULT_CAPACITY, DEFAULT_FILL_RATE};

#[derive(clap::ValueEnum, Default, Clone, Debug, Deserialize)]
pub enum Mode {
    #[default]
    InMemory,
    Mongo,
    Coordinator,
    Redis,
    Firestore
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug, Deserialize)]
#[allow(unused)]
pub struct ApiServer {
    pub application_url: String,
    pub hostname: String
}

#[derive(Clone, Debug, Deserialize)]
#[allow(unused)]
pub struct MongoConfig {
    pub connection_string: String,
    pub database_name: String,
    pub collection_name: String
}

#[derive(Clone, Debug, Deserialize)]
#[allow(unused)]
pub struct RedisConfig {
    pub connection_string: String
}

#[derive(Clone, Debug, Deserialize)]
#[allow(unused)]
pub struct FirestoreConfig {
    pub project_id: String
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[allow(unused)]
pub struct RateLimit {
    pub capacity: usize,
    pub fill_rate: usize
}

#[derive(Clone, Debug, Deserialize)]
#[allow(unused)]
pub struct Coordinator {
    pub hostnames: Vec<String>
}

pub const DEFAULT_RATE_LIMIT: RateLimit = RateLimit{ capacity: DEFAULT_CAPACITY, fill_rate: DEFAULT_FILL_RATE };

#[derive(Clone, Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub debug: bool,
    pub mode: Mode,
    pub apiserver: ApiServer,
    pub mongo_config: Option<MongoConfig>,
    pub redis_config: Option<RedisConfig>,
    pub firestore_config: Option<FirestoreConfig>,
    pub ratelimit: Option<RateLimit>,
    pub coordinator: Option<Coordinator>
}

#[derive(Parser)]
struct Args {
    /// Mode this service will be running in
    #[clap(short, long, value_enum)]
    mode: Option<Mode>,
    /// Address this service will be running on
    #[arg(short, long)]
    application_url: Option<String>,
    /// List of host names separated by space to coordinate requests between
    #[arg(long)]
    hostnames: Option<String>
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let mut config_builder = Box::new(Config::builder()
            // Start off by merging in the "default" configuration file
            //.add_source(File::with_name("config/default").required(false))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            .add_source(
                File::with_name(&format!("src/configuration/{}.toml", run_mode))
                    .required(true),
            ));

        let args = Args::parse();
        if let Some(value) = args.mode {
            config_builder = Box::new(config_builder.clone()
                .set_override("mode", value.to_string())?)
        }

        if let Some(value) = args.application_url {
            config_builder = Box::new(config_builder.clone()
                .set_override("apiserver.application_url", value.clone())?
                .set_override("apiserver.hostname", value)?);
        }
        
        if let Some(value) = args.hostnames {
            let hostnames = (value as String).split(' ').map(|x| x.to_string()).collect::<Vec<String>>();

            config_builder = Box::new(config_builder.clone()
                .set_override("coordinator.hostnames", hostnames)?);
        }

        let config = config_builder
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            //.add_source(File::with_name("config/local").required(false))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            //.add_source(Environment::with_prefix("app"))
            // You may also programmatically change settings
            // .set_override("database.url", "postgres://")?
            .build()?;

        // You can deserialize (and thus freeze) the entire configuration as
        config.try_deserialize()
    }
}