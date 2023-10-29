use self::multiplex_service::MultiplexService;
use axum::{routing::get, Router, error_handling::HandleErrorLayer, BoxError};
use proto::{
    greeter_server::{Greeter, GreeterServer},
    HelloReply, HelloRequest,
};
use std::net::SocketAddr;
use tonic::{Response as TonicResponse, Status};
use axum::{extract::ConnectInfo, ServiceExt};
mod multiplex_service;
use axum_client_ip::{InsecureClientIp, SecureClientIp, SecureClientIpSource};
use tower::{limit::ConcurrencyLimitLayer, ServiceBuilder};
use tower_governor::{errors::display_error, governor::GovernorConfigBuilder, GovernorLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod proto {
    tonic::include_proto!("helloworld");
}

#[derive(Default)]
struct GrpcServiceImpl {}

#[tonic::async_trait]
impl Greeter for GrpcServiceImpl{async fn say_hello(&self, request: tonic::Request<HelloRequest>) -> Result<TonicResponse<HelloReply>, Status> {
        tracing::info!("Got a request from {:?}", request.remote_addr());
        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(TonicResponse::new(reply))
    }
}

async fn web_root(insecure_ip: InsecureClientIp, secure_ip: SecureClientIp, ConnectInfo(addr): ConnectInfo<SocketAddr>) -> String {
    format!("{insecure_ip:?} {secure_ip:?} {addr}", insecure_ip = insecure_ip, secure_ip = secure_ip, addr = addr)
}


#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
    .with(
        tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "debuglogs".into()),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();

    let governor_conf = Box::new(
        GovernorConfigBuilder::default()
            .per_second(2)
            .burst_size(5)
            .finish()
            .unwrap(),
    );
    let concurrency_limi_layer = ConcurrencyLimitLayer::new(100);
    let rest = Router::new().route("/", get(web_root))
    .layer(SecureClientIpSource::ConnectInfo.into_extension())
    .layer(concurrency_limi_layer)
    .layer(
        ServiceBuilder::new()
            .layer(HandleErrorLayer::new(|e: BoxError| async move {
                display_error(e)
            }))
            .layer(GovernorLayer {
                config: Box::leak(governor_conf),
            }),
    );
    let grpc = tonic::transport::Server::builder()
        .add_service(GreeterServer::new(GrpcServiceImpl::default()))
        .into_service();
    let service = MultiplexService::new(rest, grpc);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(service.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}