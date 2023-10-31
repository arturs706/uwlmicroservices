use tonic::transport::Endpoint;
use users_proto::user_auth_server::UserAuth;
use users_proto::user_auth_client::UserAuthClient;
use users_proto::{UserAuthRequest, UserAuthResponse};
use tonic::{Response, Status};
use crate::users_proto;
use crate::mware;

#[derive(Debug, Default)]
pub struct UserAuthProxyService {}

#[tonic::async_trait]
impl UserAuth for UserAuthProxyService {
    async fn register_user(
        &self,
        request: tonic::Request<UserAuthRequest>,
    ) -> Result<Response<UserAuthResponse>, Status> {
        let channel = Endpoint::from_static("http://[::1]:50055")
        .connect()
        .await.unwrap();
        let mut client = UserAuthClient::with_interceptor(channel, mware::check_auth);
        let response = client.register_user(request).await?;
        Ok(Response::new(response.into_inner()))
    }
}
