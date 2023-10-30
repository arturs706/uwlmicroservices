use users_proto::user_auth_server::UserAuth;
use users_proto::user_auth_client::UserAuthClient;
use users_proto::{UserAuthRequest, UserAuthResponse};
use tonic::{Response, Status};
use crate::users_proto;

#[derive(Debug, Default)]
pub struct UserAuthProxyService {}

#[tonic::async_trait]
impl UserAuth for UserAuthProxyService {
    async fn register_user(
        &self,
        request: tonic::Request<UserAuthRequest>,
    ) -> Result<Response<UserAuthResponse>, Status> {
        let mut client = UserAuthClient::connect("http://[::1]:50055").await.unwrap();
        let response = client.register_user(request).await?;

        // Send the response back to the client.
        Ok(Response::new(response.into_inner()))
    }
}

