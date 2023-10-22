#![allow(dead_code)]

use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::net::IpAddr;
use std::time::SystemTime;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{Header, Algorithm, EncodingKey};
use chrono::{Utc, Duration};

#[derive(Debug)]

struct RateLimiter {
    visitors: Arc<Mutex<HashMap<IpAddr, (u32, SystemTime)>>>,
}

impl RateLimiter {
    fn new() -> Self {
        RateLimiter {
            visitors: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn allow(&self, addr: IpAddr) -> bool {

        // grabbing the current time
        let request_time = SystemTime::now();
        // locking the hashmap preventing multiple threads from accessing it at the same time
        let mut visitors = self.visitors.lock().unwrap();
        // checking if the ip address is in the hashmap, and if it is there, we are returning the value
        let counter = visitors.entry(addr).or_insert((0, request_time));
    
        // Check if 30 seconds have passed since the first request
        if request_time.duration_since(counter.1).unwrap().as_secs() >= 30 {
            // Reset the request count and timestamp
            counter.0 = 0;
            counter.1 = request_time;
        }
        
        if counter.0 >= 50 {
            false
        } else {
            counter.0 += 1;
            true
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct GatewayAuthClaims {
    iss: String, // Issuer
    gateway_access: bool, // Custom claim for gateway access
    exp: i64, // Expiration Time
    iat: i64, // Issued At
}

impl GatewayAuthClaims {
    pub fn new(issuer: String, gateway_access: bool) -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::hours(72);
    
        Self {
            iss: issuer,
            gateway_access,
            exp: exp.timestamp(),
            iat: iat.timestamp(),
        }
    }
}


async fn handle_request(req: Request<Body>, rate_limiter: Arc<RateLimiter>, addr: IpAddr) -> Result<Response<Body>, hyper::Error> {
    let issuer = "gateway".to_string();
    let gateway_access = true;
    let jwt_secret: String = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let jwt_token = jsonwebtoken::encode(&Header::new(Algorithm::HS256), &GatewayAuthClaims::new(issuer, gateway_access, ),&EncodingKey::from_secret(jwt_secret.as_bytes())).unwrap();
    match req.uri().path() {
        "/service" => {
           
            if !rate_limiter.allow(addr) {
                return Ok(Response::new(Body::from("Rate limit exceeded for service")))
            } 
            let client = hyper::Client::new();
            let forwarded_req = Request::builder()
                .method(req.method())
                .uri("http://localhost:3000")
                .header("Authorization", format!("Bearer {}", jwt_token))
                .body(req.into_body())
                .unwrap();
            let mut resp: Response<Body> = client.request(forwarded_req).await?;
            *resp.status_mut() = hyper::StatusCode::CREATED;
            Ok(resp)
        },
        
        _ => Ok(Response::new(Body::from("Not Found"))), 
    }
}



#[tokio::main]
async fn main() {
    dotenv().ok();
    let rate_limiter = Arc::new(RateLimiter::new());
    let make_svc = make_service_fn(|conn: &hyper::server::conn::AddrStream| {
        let rate_limiter = rate_limiter.clone();
        let address = conn.remote_addr().ip();
        let service = service_fn(move |req| handle_request(req, rate_limiter.clone(), address));
        async move { Ok::<_, hyper::Error>(service) }
    });
    let addr = ([0, 0, 0, 0], 10000).into();
    let server = Server::bind(&addr).serve(make_svc);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}