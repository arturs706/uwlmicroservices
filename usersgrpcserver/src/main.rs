use tonic::{transport::Server, Request, Response, Status};
use staff_users::staff_users_server::{StaffUsers, StaffUsersServer};
use staff_users::{RegisterUserRequest, RegisterUserResponse, EmptyReqRes, AllUserRetrieve};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Row};
use uuid::Uuid;
use crate::staff_users::SingleUserRetrieve;
use std::time::SystemTime;
pub mod staff_users {
    tonic::include_proto!("staffusers");
}




#[derive(Debug, Default)]
pub struct StaffUsersService {}

pub async fn establish_connection() -> Pool<Postgres> {
    dotenv().ok();
    let database_url = std::env::var("POSTGRES_URL").expect("POSTGRES_URL must be set");
    return PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error connecting to db");
}

pub async fn create_user(pool: &Pool<Postgres>, name: &str, username: &str, passwd: &str, mob_phone: &str) -> Result<u64, sqlx::Error> {
    let uuid = Uuid::new_v4();
    let uuid_str = uuid.to_string();
    let timestamp = SystemTime::now();
    let timestamp_in_milliseconds = (timestamp.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis()).to_string();

    let query = sqlx::query(
        "INSERT INTO staff_users (user_id, name, username, passwd, mob_phone, a_created) VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(uuid_str)
    .bind(name)
    .bind(username)
    .bind(passwd)
    .bind(mob_phone)
    .bind(timestamp_in_milliseconds)
    .execute(pool)
    .await;
    
    match query {
        Ok(result) => {
            if result.rows_affected() == 1 {
                Ok(1)
            } else {
                Ok(0)
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            Err(e)
        }
    }
}

#[tonic::async_trait]
impl StaffUsers for StaffUsersService {
    async fn register_user(
        &self,
        request: Request<RegisterUserRequest>,
    ) -> Result<Response<RegisterUserResponse>, Status> {
        println!("Got a request: {:?}", request);
        dotenv().ok();
        let req = request.into_inner();
        let pool = establish_connection().await;
        let result = create_user(&pool, &req.name, &req.username, &req.passwd, &req.mob_phone).await;
        
        match result {
            Ok(_) => {
                let reply = RegisterUserResponse {
                    name: "".into(),
                    message: format!("Welcome {}", req.name).into(),
                };
                Ok(Response::new(reply))
            },
            Err(e) => {
                eprintln!("Error creating user: {}", e);
                let status = if e.to_string().contains("duplicate key value violates unique constraint") {
                    Status::already_exists("User already exists")
                } else {
                    Status::internal("Error creating user")
                };
                Err(status)
            }
        }
    }
    async fn get_all_users(&self,_request: tonic::Request<EmptyReqRes>) -> Result<Response<AllUserRetrieve>, Status> {
        let pool = establish_connection().await;
        let mut v: Vec<SingleUserRetrieve> = Vec::new();
        let rows = sqlx::query("SELECT * FROM staff_users")
            .fetch_all(&pool)
            .await
            .expect("Error fetching users");

        for row in rows.iter() {
            v.push(
                SingleUserRetrieve {
                    user_id: row.get("user_id"),
                    name: row.get("name"),
                    username: row.get("username"),
                    mob_phone: row.get("mob_phone"),
                    access_level: row.get("access_level"),
                    status: row.get("status"),
                    a_created: row.get("a_created"),
                    // account_created: timestamp.to_string()
                }
            );
        }
        dbg!(
            AllUserRetrieve {
                users: v.clone()
            }
        );
   
        let reply = AllUserRetrieve {
            users: v,
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50050".parse()?;
    let staff_user_reg_service = StaffUsersService::default();
    Server::builder()
    .add_service(StaffUsersServer::new(staff_user_reg_service))
    .serve(addr)
    .await?;

    Ok(())
}

// let cors = CorsLayer::new()
// .allow_headers(Any)
// .allow_methods([Method::POST, Method::GET, Method::OPTIONS, Method::PUT, Method::DELETE])
// // .allow_origin(["http://localhost:2000".parse().unwrap(), "https://localhost:2001".parse().unwrap()]);
// .allow_origin(Any);
// let grpc = tonic::transport::Server::builder()
// .accept_http1(true)     
// .layer(cors)
// .layer(GrpcWebLayer::new()) 
// .add_service(tonic_web::enable(StaffUsersServer::new(us_prox)))    
// .into_service();
