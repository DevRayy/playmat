pub mod grpc {
    tonic::include_proto!("auth");
}

pub struct AuthService {
    client: grpc::auth_client::AuthClient<tonic::transport::Channel>,
}

impl AuthService {
    pub async fn new() -> Self {
        let client = grpc::auth_client::AuthClient::connect("http://127.0.0.1:9090")
            .await
            .unwrap();
        Self { client }
    }

    pub async fn register(&mut self, email: String, password: String) -> Result<(), tonic::Status> {
        let request = tonic::Request::new(grpc::RegisterRequest { email, password });

        self.client.register(request).await?;
        Ok(())
    }
}
