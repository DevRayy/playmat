use crate::repository;

pub mod grpc {
    tonic::include_proto!("auth");
}

pub struct Service {
    users_repo: repository::users::Users,
}

impl Service {
    pub fn new(users_repo: repository::users::Users) -> Self {
        Self { users_repo }
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
        self.users_repo
            .create(
                request.email,
                bcrypt::hash(request.password, bcrypt::DEFAULT_COST).unwrap(),
            )
            .await
            .unwrap();

        let response = grpc::RegisterResponse {};
        Ok(tonic::Response::new(response))
    }
}
