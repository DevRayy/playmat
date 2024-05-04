mod build;
mod config;
mod repository;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let cfg = config::Cfg::new()?;

    println!("Config: {:?}", cfg);

    let mongo = build::mongo(cfg.db).await?;
    let users_repo = repository::users::Users::new(mongo);

    let auth_service = services::auth::Service::new(users_repo);

    tonic::transport::Server::builder()
        .add_service(auth_service.into_server())
        .serve(cfg.grpc.address)
        .await?;

    Ok(())
}
