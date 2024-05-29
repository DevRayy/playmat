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
    config: MongoConfig,
) -> Result<mongodb::Client, mongodb::error::Error> {
    let client = mongodb::Client::with_uri_str(config.as_conn_str())
        .await?;

    Ok(client)
}
