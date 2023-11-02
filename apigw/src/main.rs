use self::merg_serv::MergServ;
use axum::{
    error_handling::HandleErrorLayer,
    extract::MatchedPath,
    http::Request,
    middleware::{self, Next},
    response::IntoResponse,
    routing::get,
    BoxError, Router
};
mod errors;
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};
use tokio::time::Instant;
use axum::{extract::ConnectInfo, ServiceExt};
use tonic_web::GrpcWebLayer;
use std::{future::ready, net::SocketAddr};
mod merg_serv;
use axum_client_ip::{InsecureClientIp, SecureClientIp, SecureClientIpSource};
use tower::{limit::ConcurrencyLimitLayer, ServiceBuilder};
use tower_governor::{errors::display_error, governor::GovernorConfigBuilder, GovernorLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use dotenv::dotenv;
mod rest_routes;
mod mware;
mod grpc_routes;
use crate::{grpc_routes::StaffUsersProxyService, staff_users::staff_users_server::StaffUsersServer};
pub mod staff_users {
    tonic::include_proto!("staffusers");
}
use tower_http::cors::{Any, CorsLayer};
use http::Method;
use tonic::{codegen::http::header::HeaderName, transport::Server};



async fn web_root(
    insecure_ip: InsecureClientIp,
    secure_ip: SecureClientIp,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> String {
    format!(
        "{insecure_ip:?} {secure_ip:?} {addr}",
        insecure_ip = insecure_ip,
        secure_ip = secure_ip,
        addr = addr
    )
}

async fn track_metrics<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let start = Instant::now();
    let path = if let Some(matched_path) = req.extensions().get::<MatchedPath>() {
        matched_path.as_str().to_owned()
    } else {
        req.uri().path().to_owned()
    };
    let method = req.method().clone();
    let response = next.run(req).await;
    let latency = start.elapsed().as_secs_f64();
    let status = response.status().as_u16().to_string();
    let labels = [
        ("method", method.to_string()),
        ("path", path),
        ("status", status),
    ];

    metrics::increment_counter!("http_requests_total", &labels);
    metrics::histogram!("http_requests_duration_seconds", latency, &labels);
    response
}


fn setup_metrics_recorder() -> PrometheusHandle {
    const EXPONENTIAL_SECONDS: &[f64] = &[
        0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
    ];

    PrometheusBuilder::new()
        .set_buckets_for_metric(
            Matcher::Full("http_requests_duration_seconds".to_string()),
            EXPONENTIAL_SECONDS,
        )
        .unwrap()
        .install_recorder()
        .unwrap()
}


fn metrics_app() -> Router {
    let recorder_handle = setup_metrics_recorder();
    Router::new().route("/metrics", get(move || ready(recorder_handle.render())))
}

fn main_app() -> Router {
        let governor_conf = Box::new(
            GovernorConfigBuilder::default()
                .per_second(2)
                .burst_size(5)
                .finish()
                .unwrap(),
        );
        let concurrency_limi_layer = ConcurrencyLimitLayer::new(10);
        let cors = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods([Method::POST, Method::GET, Method::OPTIONS, Method::PUT, Method::DELETE])
        .allow_origin(["http://localhost:2000".parse().unwrap(), "http://localhost:2001".parse().unwrap(), "http://localhost:2002".parse().unwrap()]);
        // .allow_origin(Any);
        Router::new()
            .route("/", get(web_root))
            .route("/api/v1/users", get(rest_routes::fetchusershandler))
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
            )            
            .layer(middleware::from_fn(mware::add_token))
            .route_layer(middleware::from_fn(track_metrics))
            .layer(cors)
            
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debuglogs".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();



    async fn start_api_gw() {
        let us_prox: StaffUsersProxyService = StaffUsersProxyService::default();
        let cors = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods([Method::POST, Method::GET, Method::OPTIONS, Method::PUT, Method::DELETE])
        // .allow_origin(["http://localhost:2000".parse().unwrap(), "https://localhost:2001".parse().unwrap()]);
        .allow_origin(Any);
        let grpc = tonic::transport::Server::builder()
        .accept_http1(true)     
        .layer(cors)
        .layer(GrpcWebLayer::new()) 
        .add_service(tonic_web::enable(StaffUsersServer::new(us_prox)))    
        .into_service();


        let rest = main_app();
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        tracing::debug!("listening on {}", addr);
        let service = MergServ::new(rest, grpc);
        let _ = axum::Server::bind(&addr)
        .serve(service.into_make_service_with_connect_info::<SocketAddr>())
        
        .await;
    }

    async fn start_metrics_server() {
        let app = metrics_app();
        let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
        tracing::debug!("listening on {}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap()
    }
    let (_api_gw, _metrics_server) = tokio::join!(start_api_gw(), start_metrics_server());
}
