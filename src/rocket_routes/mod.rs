use diesel::PgConnection;
use rocket::request::Outcome;
use rocket::http::Status;
use rocket::Request;
use rocket::request::FromRequest;
use rocket::response::status::Custom;
use rocket_sync_db_pools;
use rocket_db_pools::{deadpool_redis, Database, Connection};

use rocket_db_pools::deadpool_redis::redis::AsyncCommands;

use rocket::serde::json::{serde_json::json, Value};

use rocket_sync_db_pools::database;
use crate::models::User;
use crate::repositories::UserRepository;

pub mod rustaceans;
pub mod crates;
pub mod authorization;

#[database("postgres")]
pub struct DbConn(PgConnection);

#[derive(Database)]
#[database("redis")]
pub struct CacheConn(deadpool_redis::Pool);

pub fn server_error(e: Box<dyn std::error::Error>) -> Custom<Value> {
    log::error!("{}", e);
    Custom(Status::InternalServerError, json!("Error"))
}

pub fn not_found(e: Box<dyn std::error::Error>) -> Custom<Value> {
    log::error!("{}", e);
    Custom(Status::NotFound, json!("404 Not Found"))
}

// Trait implemented by request guards to derive a value from incoming requests
// In this case:: _user: User will implement the trait FromRequest
#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // In coming requests should have the following header::
        // Authorization: Bearer SESSION_ID_128_CHARS_LONG
        let session_header = request.headers().get_one("Authorization")
            .map(|v| v.split_whitespace().collect::<Vec<_>>())
            .filter(|v| v.len() == 2 && v[0] == "Bearer");

        if let Some(session_value) = session_header {
            // Establish Redis connection
            let mut cache = request.guard::<Connection<CacheConn>>().await
                .expect("Cannot connect to Redis in request guard...");
            // Establish postgres connection
            let db = request.guard::<DbConn>().await
                .expect("Cannot connect to Postgres in request guard...");
            let result = cache.get::<_, i32>(format!("sessions/{}", session_value[1])).await;
            // Check if result is OK()
            if let Ok(user_id) = result {
                return match db.run(move |c| UserRepository::find(c, user_id)).await {
                    Ok(user) => Outcome::Success(user),
                    _ => Outcome::Failure((Status::Unauthorized, ()))
                }
            }
        }

        Outcome::Failure((Status::Unauthorized, ()))
    }
}
