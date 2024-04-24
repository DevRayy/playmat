mod config;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let cfg = config::Cfg::new()?;

    println!("Config: {:?}", cfg);

    let auth_service = services::auth::Service::new();

    tonic::transport::Server::builder()
        .add_service(auth_service.into_server())
        .serve(cfg.grpc.address)
        .await?;

    Ok(())
}
