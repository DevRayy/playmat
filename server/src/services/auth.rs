use crate::features;

pub mod grpc {
    tonic::include_proto!("auth");
}

pub struct Service {
    register_feature: features::auth::register_user::Feature,
}

impl Service {
    pub fn new(feat: features::auth::register_user::Feature) -> Self {
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
            .run(features::auth::register_user::Query {
                email: request.email,
                password: request.password,
            })
            .await?;

        let response = grpc::RegisterResponse {};
        Ok(tonic::Response::new(response))
    }
}

impl From<features::auth::register_user::Error> for tonic::Status {
    fn from(value: features::auth::register_user::Error) -> Self {
        match value {
            features::auth::register_user::Error::Unknown(value) => tonic::Status::internal(value.to_string()),
            features::auth::register_user::Error::Hashing(value) => tonic::Status::internal(value.to_string()),
            features::auth::register_user::Error::DuplicateEmail => tonic::Status::already_exists(value.to_string()),
            features::auth::register_user::Error::InvalidEmail => tonic::Status::invalid_argument(value.to_string()),
        }
    }
}
