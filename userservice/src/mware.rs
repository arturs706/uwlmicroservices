

use crate::{errors::CustomErrors, AppState};
use axum::{extract::State, http::Request, middleware::Next, response::Response};
use chrono::{Duration, Utc};
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct GatewayAuthClaims {
    iss: String,          // Issuer
    gateway_access: bool, // Custom claim for gateway access
    exp: i64,             // Expiration Time
    iat: i64,             // Issued At
}

impl GatewayAuthClaims {
    pub fn _new(issuer: String, gateway_access: bool) -> Self {
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

#[allow(dead_code)]

pub async fn auth_middleware<B>(
    State(state): State<AppState>,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, CustomErrors>
where
    B: Send,
{
    request
        .headers()
        .get("Authorization")
        .ok_or(CustomErrors::NotLoggedIn)?;
    let auth = request
        .headers()
        .get("Authorization")
        .ok_or(CustomErrors::MissingCreds)?;
    let token = auth.to_str().map_err(|_| CustomErrors::MissingCreds)?;
    let authtoken = token.replace("Bearer ", "");
    let validation = Validation::new(Algorithm::HS256);
    let access_token_secret: String = state.accesstoken.accesstoken.clone();
    let access_secret = &access_token_secret.as_bytes();
    let access_verify = jsonwebtoken::decode::<GatewayAuthClaims>(
        &authtoken,
        &DecodingKey::from_secret(access_secret),
        &validation,
    );
    match access_verify {
        Ok(_) => Ok(next.run(request).await),
        Err(e) => {
            println!("access_verify: {:?}", e);
            match e.kind() {
                ErrorKind::InvalidToken => {
                    println!("access_verify: {:?}", e);
                    Err(CustomErrors::InvalidToken)
                }
                _ => {
                    println!("access_verify: {:?}", e);
                    Err(CustomErrors::InvalidKey)
                }
            }
        }
    }
}
