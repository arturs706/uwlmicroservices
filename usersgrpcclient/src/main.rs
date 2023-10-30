use users::user_auth_client::UserAuthClient;
use tonic::{transport::Server, Request, Response, Status};
use users::user_auth_server::{UserAuth, UserAuthServer};
use users::{UserAuthRequest, UserAuthResponse};
pub mod users {
    tonic::include_proto!("users");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = UserAuthClient::connect("http://127.0.0.1:3000").await?;

    let request = tonic::Request::new(
        UserAuthRequest {
            full_name: "John Doe".into(),
            dob: "01-01-2000".into(),
            gender: "male".into(),
            mob_phone: "1234567890".into(),
            email: "a@a.com".into(),
            passwd: "123456".into(),
            email_ver_token: "random".into(),
        }
    );

    let response = client.register_user(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
