pub mod grpc {
    tonic::include_proto!("auth");
}

pub struct Service {

}

impl Service {
    pub fn new() -> Self {
        Self {}
    }

    pub fn into_server(self) -> grpc::auth_server::AuthServer<Self> {
        grpc::auth_server::AuthServer::new(self)
    }
}

#[tonic::async_trait]
impl grpc::auth_server::Auth for Service {
    async fn login(&self, request: tonic::Request<grpc::LoginRequest>) -> Result<tonic::Response<grpc::LoginResponse>, tonic::Status> {
        let response = grpc::LoginResponse {
            token: request.into_inner().username,
        };
        Ok(tonic::Response::new(response))
    }
}