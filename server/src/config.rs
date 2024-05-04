use config::{Config, File};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Debug;
use std::net::SocketAddr;

#[derive(Serialize, Deserialize)]
pub struct Cfg {
    pub log: Log,
    pub grpc: Grpc,
    pub db: Db,
}

#[derive(Serialize, Deserialize)]
pub struct Log {
    pub level: String,
}

#[derive(Serialize, Deserialize)]
pub struct Grpc {
    pub address: SocketAddr,
}

#[derive(Serialize, Deserialize)]
pub struct Db {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub db: String,
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
