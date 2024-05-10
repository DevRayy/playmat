use crate::features;

pub mod grpc {
    tonic::include_proto!("auth");
}

pub struct Service {
    register_feature: features::auth_register_user::Feature,
}

impl Service {
    pub fn new(feat: features::auth_register_user::Feature) -> Self {
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
            .await?;

        let response = grpc::RegisterResponse {};
        Ok(tonic::Response::new(response))
    }
}

impl From<features::auth_register_user::errors::FeatureError> for tonic::Status {
    fn from(value: features::auth_register_user::errors::FeatureError) -> Self {
        tonic::Status::internal(value.to_string())
    }
}
