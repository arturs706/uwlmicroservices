use tonic::{transport::Server, Request, Response, Status};
use users::user_auth_server::{UserAuth, UserAuthServer};
use users::{UserAuthRequest, UserAuthResponse};
pub mod users {
    tonic::include_proto!("users");
}

#[derive(Debug, Default)]
pub struct UserAuthService {}

// #[tonic::async_trait]
// impl Greeter for GrpcServiceImpl {
//     async fn say_hello(
//         &self,
//         request: tonic::Request<HelloRequest>,
//     ) -> Result<TonicResponse<HelloReply>, Status> {
//         let info = request.extensions().get::<ConnectInfo<SocketAddr>>();
//         tracing::info!("Got a request from {:?}", info);

//         let reply = HelloReply {
//             message: format!("Hello {}!", request.into_inner().name),
//         };

//         Ok(TonicResponse::new(reply))
//     }
// }

#[tonic::async_trait]
impl UserAuth for UserAuthService {
    async fn register_user(
        &self,
        request: Request<UserAuthRequest>,
    ) -> Result<Response<UserAuthResponse>, Status> {
        println!("Got a request: {:?}", request);
        let req = request.into_inner();
        let reply = UserAuthResponse {
            user_id: "1234".into(),
            message: {
                format!(
                    "Welcome {}! Email sent to {} successfully!", req.full_name, req.email
                ).into()
            }
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50055".parse()?;
    let userauth_service = UserAuthService::default();

    Server::builder()
        .add_service(UserAuthServer::new(userauth_service))
        .serve(addr)
        .await?;

    Ok(())
}