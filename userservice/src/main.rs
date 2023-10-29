
#[allow(dead_code)]
use axum::routing::post;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
mod errors;
mod users;
use dotenv::dotenv;
mod mware;


#[derive(Clone)]
pub struct AppState {
    pub database: Database,
    pub accesstoken: AccessToken,
    pub email_secret: EmailSecret,
}

#[derive(Clone)]
pub struct Database {
    db: Surreal<Client>,
}
#[derive(Clone)]
pub struct AccessToken {
    pub accesstoken: String,
}
#[derive(Clone)]
pub struct EmailSecret {
    pub email_secret: String,
}

async fn connect_to_server() -> surrealdb::Result<Surreal<Client>> {
    dotenv().ok();
    let database_acc: String = std::env::var("DB_ACC").expect("DB_ACC must be set");
    let db_ip: String = std::env::var("DB_IP").expect("DB_IP must be set");
    let db_pass: String = std::env::var("DB_PASS").expect("DB_PASS must be set");
    let db_ns: String = std::env::var("DB_NS").expect("DB_NS must be set");
    let db_name: String = std::env::var("DB_NAME").expect("DB_NAME must be set");
    let db: Surreal<Client> = Surreal::new::<Ws>(db_ip).await?;
    db.signin(Root {
        username: &database_acc,
        password: &db_pass,
    })
    .await?;
    db.use_ns(&db_ns).use_db(&db_name).await?;
    Ok(db)
}

#[tokio::main]
async fn main() {
    let db_result = connect_to_server().await;
    let db = match db_result {
        Ok(db) => db,
        Err(e) => panic!("Error connecting to the server: {}", e),
    };
    let access_token_secret: String = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let email_secret_token =
        std::env::var("EMAIL_VER_SECRET").expect("EMAIL_VER_SECRET must be set");

    let state = AppState {
        database: Database { db },
        accesstoken: AccessToken {
            accesstoken: access_token_secret,
        },
        email_secret: EmailSecret {
            email_secret: email_secret_token,
        },
    };

    let app = Router::new()
        // .route_layer(middleware::from_fn_with_state(state.clone(), auth_middleware))
        .route("/api/v1/users", get(users::fetchusershandler))
        .route("/api/v1/users/register", post(users::create_user))
        .with_state(state);
    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 11000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
