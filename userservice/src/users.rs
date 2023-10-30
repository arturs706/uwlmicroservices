#![allow(dead_code)]

use crate::AppState;
use argon2::PasswordHasher;
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2
};
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use axum_macros::debug_handler;
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use surrealdb::sql::Thing;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[allow(non_snake_case)]

pub struct User {
    #[allow(dead_code)]
    id: Thing,
    fullName: String,
    dob: String,
    gender: String,
    mob_phone: String,
    email: String,
    passwd: String,
    email_ver_token: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[allow(non_snake_case)]

pub struct UserReg {
    fullName: String,
    dob: String,
    gender: String,
    mob_phone: Option<String>,
    email: String,
    passwd: String,
    email_ver_token: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserAuthClaims {
    userid: String, 
    exp: i64,       
    iat: i64,       
}

impl UserAuthClaims {
    pub fn new(userid: String) -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::hours(72);

        Self {
            userid,
            exp: exp.timestamp(),
            iat: iat.timestamp(),
        }
    }
}

#[debug_handler]
pub async fn fetchusershandler(State(state): State<AppState>) -> impl IntoResponse {
    let db = &state.database.db;
    let users_result: Result<Vec<User>, surrealdb::Error> = db.select("users").await;
    // get incoming request headers
    match users_result {
        Ok(records) => {
            if records.is_empty() {
                // No users found
                return Err((
                    StatusCode::NOT_FOUND,
                    Json(serde_json::json!({ "error": "No users found" })),
                ));
            }

            println!("Users fetched successfully");
            Ok(
                Json(serde_json::json!({
                    "users": records
                })),
            )
        }
        Err(e) => {
            println!("Error fetching records: {:?}", e);
            Err((
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "error": e.to_string() })),
            ))
        }
    }
}

#[debug_handler]
pub async fn create_user(
    State(state): State<AppState>,
    req: Json<UserReg>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, axum::Json<JsonValue>)> {
    let db = &state.database.db;
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2
        .hash_password(req.passwd.as_bytes(), &salt)
        .unwrap()
        .to_string();
    let my_uuid = Uuid::new_v4().to_string();
    let email_ver_token = &state.email_secret.email_secret;
    let email_token = jsonwebtoken::encode(
        &Header::new(Algorithm::HS256),
        &UserAuthClaims::new(my_uuid),
        &EncodingKey::from_secret(email_ver_token.as_bytes()),
    )
    .unwrap();
    let result: Result<Vec<Record>, surrealdb::Error> = db
        .create("users")
        .content(UserReg {
            fullName: req.fullName.to_owned(),
            dob: req.dob.to_owned(),
            gender: req.gender.to_owned(),
            mob_phone: Some(req.mob_phone.to_owned().unwrap_or("".to_string())),
            email: req.email.to_owned(),
            passwd: password_hash.to_owned(),
            email_ver_token: Some(email_token.to_owned()),
        })
        .await;

    match result {
        Ok(ids) => {
            let id = ids.first().cloned();
            Ok((
                StatusCode::CREATED,
                Json(serde_json::json!({
                    "id": id.unwrap().id.id.to_string(),
                    "fullName": req.fullName,
                    "dob": req.dob,
                    "gender": req.gender,
                    "mob_phone": req.mob_phone,
                    "email": req.email,
                    "passwd": password_hash,
                    "email_ver_token": email_token.to_owned()
                })),
            ))
        }
        Err(e) => {
            println!("Error creating record: {:?}", e);
            Err((
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "error": e.to_string() })),
            ))
        }
    }
}
