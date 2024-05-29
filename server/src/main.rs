mod adapters;
mod config;
mod features;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = config::Cfg::new()?;

    adapters::init_env_logger(&cfg.adapters.env_logger);
    log::debug!("Starting with config:\n {:?}", cfg);

    let mongo = adapters::mongo(&cfg.adapters.mongo).await?;

    features::dbinit::Feature::new(mongo.clone()).run().await?;

    let feat = features::register_user::Feature::new(mongo).await;

    let auth_service = services::auth::Service::new(feat);

    tonic::transport::Server::builder()
        .add_service(auth_service.into_server())
        .serve(cfg.grpc.address)
        .await?;

    Ok(())
}
