use http::HeaderValue;
use jsonwebtoken::{Header as JsonWebTokenHeader, Algorithm, EncodingKey};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use axum::{
    http::Request,
    middleware::Next,
    response::Response,
};
use tonic::{Request as TonicRequest, Status};
use tonic::metadata::MetadataValue;


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

pub async fn add_token<B>(req: Request<B>, next: Next<B>) -> Result<Response, Response> {
    let issuer = "gateway".to_string();
    let gateway_access = true;
    let jwt_secret: String = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let jwt_token = jsonwebtoken::encode(&JsonWebTokenHeader::new(Algorithm::HS256), &GatewayAuthClaims::new(issuer, gateway_access), &EncodingKey::from_secret(jwt_secret.as_bytes())).unwrap();
    let mut req = req;
    req.headers_mut().insert("Authorization", HeaderValue::from_str(&jwt_token).unwrap());
    let response = next.run(req).await;
    Ok(response)
}

pub fn check_auth(req: TonicRequest<()>) -> Result<TonicRequest<()>, Status> {
    let token: MetadataValue<_> = "1".parse().unwrap();
    match req.metadata().get("authorization") {
        Some(t) => {
            let split_token = t.to_str().unwrap();
            let split_token = split_token.split(" ").collect::<Vec<&str>>();
            let t = split_token[1].to_string();
            if t == token.to_str().unwrap() {
                println!("{}", token.to_str().unwrap());
                Ok(req)
            } else {
                println!("Here 1 - {}", t);
                println!("Here 2 - {}", token.to_str().unwrap());
                Err(Status::unauthenticated(
                    "No valid auth token",
                ))
            }
        },
   
        _ => {
            println!("{}", token.to_str().unwrap());
            Err(Status::unauthenticated(
                "No valid auth token",
            ))
        }
    }
}