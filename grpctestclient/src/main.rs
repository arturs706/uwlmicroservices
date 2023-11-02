// use staff_users::staff_users_client::StaffUsersClient;
// use staff_users::EmptyReqRes;
// use tonic::{metadata::MetadataValue, transport::Channel, Request};
// pub mod staff_users {
//     tonic::include_proto!("staffusers");
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let channel = Channel::from_static("http://127.0.0.1:3000").connect().await?;
//     let token: MetadataValue<_> = "1".parse()?;
//     let mut client = StaffUsersClient::with_interceptor(channel, move |mut req: Request<()>| {
//         req.metadata_mut().insert("authorization", token.clone());
//         Ok(req)
//     });
//        let request = tonic::Request::new(
//         EmptyReqRes {}
//     );
//     let response = client.get_all_users(request).await?;
//     println!("RESPONSE={:?}", response);


// Ok(())
// }


use staff_users::staff_users_client::StaffUsersClient;
use tonic::{metadata::MetadataValue, transport::Channel, Request};

use crate::staff_users::RegisterUserRequest;
pub mod staff_users {
    tonic::include_proto!("staffusers");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static("http://127.0.0.1:3000").connect().await?;
    let token: MetadataValue<_> = "1".parse()?;
    let mut client = StaffUsersClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut().insert("authorization", token.clone());
        Ok(req)
    });

    let request = tonic::Request::new(
        RegisterUserRequest {
            name: "John Dowe".into(),
            username: "112222040".into(),
            mob_phone: "113230".into(),
            passwd: "123546".into(),
        }
    );

    let response = client.register_user(request).await?;

    println!("RESPONSE={:?}", response);
Ok(())
}