use tonic::transport::Endpoint;
use staff_users::staff_users_server::StaffUsers;
use staff_users::staff_users_client::StaffUsersClient;
use staff_users::{RegisterUserRequest, RegisterUserResponse, AllUserRetrieve, EmptyReqRes};
use tonic::{Response, Status};
use crate::staff_users;
use crate::mware;

// rpc GetAllUsers (Empty) returns (AllUserRetrieve) {}


#[derive(Debug, Default)]
pub struct StaffUsersProxyService {}


#[tonic::async_trait]
impl StaffUsers for StaffUsersProxyService {
    async fn register_user(
        &self,
        request: tonic::Request<RegisterUserRequest>,
    ) -> Result<Response<RegisterUserResponse>, Status> {
        println!("Received request: {:?}", request);

        let channel = Endpoint::from_static("http://[::1]:50050")
        .connect()
        .await.unwrap();
        let mut client = StaffUsersClient::with_interceptor(channel, mware::check_auth);
        let response = client.register_user(request).await?;
        Ok(Response::new(response.into_inner()))
    }



    async fn get_all_users(
        &self,
        request: tonic::Request<EmptyReqRes>,
    ) -> Result<Response<AllUserRetrieve>, Status> {
        println!("Received request here: {:?}", request);
        println!("Bearer token: {:?}", request.metadata().get("authorization"));
        let channel = Endpoint::from_static("http://[::1]:50050")
        .connect()
        .await.unwrap();
        let mut client = StaffUsersClient::with_interceptor(channel, mware::check_auth);
        let response = client.get_all_users(request).await?;
        Ok(Response::new(response.into_inner()))
    }
}

