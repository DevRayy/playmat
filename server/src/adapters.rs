use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct MongoConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub db: String,
}

impl MongoConfig {
    fn as_conn_str(&self) -> String {
        format!(
            "mongodb://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.db
        )
    }
}

pub(crate) async fn mongo(
    config: &MongoConfig,
) -> Result<mongodb::Client, mongodb::error::Error> {
    let client = mongodb::Client::with_uri_str(config.as_conn_str())
        .await?;

    Ok(client)
}

#[derive(Serialize, Deserialize)]
pub(crate) struct EnvLoggerConfig {
    pub level: String,
}

pub(crate) fn init_env_logger(config: &EnvLoggerConfig) {
    use std::str::FromStr;
    env_logger::Builder::new().filter_level(log::LevelFilter::from_str(config.level.as_str()).unwrap()).init();
}
