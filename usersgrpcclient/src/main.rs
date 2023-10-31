use users::user_auth_client::UserAuthClient;
use users::UserAuthRequest;
use tonic::{metadata::MetadataValue, transport::Channel, Request};
pub mod users {
    tonic::include_proto!("users");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static("http://127.0.0.1:3000").connect().await?;
    let token: MetadataValue<_> = "1".parse()?;
    let mut client = UserAuthClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut().insert("authorization", token.clone());
        Ok(req)
    });

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
