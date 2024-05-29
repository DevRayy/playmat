use std::fmt;
use std::fmt::Debug;
use std::net::SocketAddr;

use config::{Config, File};
use serde::{Deserialize, Serialize};

use crate::adapters;

#[derive(Serialize, Deserialize)]
pub struct Cfg {
    pub grpc: Grpc,
    pub adapters: Adapters,
}

#[derive(Serialize, Deserialize)]
pub struct Adapters {
    pub env_logger: adapters::EnvLoggerConfig,
    pub mongo: adapters::MongoConfig,
}

#[derive(Serialize, Deserialize)]
pub struct Grpc {
    pub address: SocketAddr,
}

impl Cfg {
    pub fn new() -> Result<Self, config::ConfigError> {
        let builder = Config::builder()
            .add_source(File::with_name("server/config/default"))
            .build()?;

        builder.try_deserialize()
    }
}

impl Debug for Cfg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pretty_string = serde_json::to_string_pretty(&self).unwrap();
        write!(f, "{}", pretty_string)
    }
}
