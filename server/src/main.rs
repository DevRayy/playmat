mod build;
mod config;
mod features;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let cfg = config::Cfg::new()?;

    println!("Config: {:?}", cfg);

    let mongo = build::mongo(cfg.db).await?;

    features::dbinit::Feature::new(mongo.clone()).run().await?;

    let feat = features::auth_register_user::Feature::new(mongo).await;

    let auth_service = services::auth::Service::new(feat);

    tonic::transport::Server::builder()
        .add_service(auth_service.into_server())
        .serve(cfg.grpc.address)
        .await?;

    Ok(())
}
