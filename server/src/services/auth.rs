use crate::features;

pub mod grpc {
    tonic::include_proto!("auth");
}

pub struct Service {
    register_feature: features::AuthRegisterUser,
}

impl Service {
    pub fn new(feat: features::AuthRegisterUser) -> Self {
        Self {
            register_feature: feat,
        }
    }

    pub fn into_server(self) -> grpc::auth_server::AuthServer<Self> {
        grpc::auth_server::AuthServer::new(self)
    }
}

#[tonic::async_trait]
impl grpc::auth_server::Auth for Service {
    async fn register(
        &self,
        request: tonic::Request<grpc::RegisterRequest>,
    ) -> Result<tonic::Response<grpc::RegisterResponse>, tonic::Status> {
        let request = request.into_inner();

        self.register_feature
            .run(request.email, request.password)
            .await
            .unwrap();

        let response = grpc::RegisterResponse {};
        Ok(tonic::Response::new(response))
    }
}
