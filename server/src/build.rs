pub(crate) async fn mongo(
    config: crate::config::Db,
) -> Result<mongodb::Client, mongodb::error::Error> {
    let client = mongodb::Client::with_uri_str(&format!(
        "mongodb://{}:{}@{}:{}/{}",
        config.user, config.password, config.host, config.port, config.db
    ))
    .await?;

    Ok(client)
}
