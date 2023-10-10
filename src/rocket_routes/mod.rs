use diesel::PgConnection;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket_sync_db_pools;
use rocket::serde::json::{serde_json::json, Value};

pub mod rustaceans;
pub mod crates;

#[rocket_sync_db_pools::database("postgres")]
pub struct DbConn(PgConnection);

pub fn server_error(e: Box<dyn std::error::Error>) -> Custom<Value> {
    log::error!("{}", e);
    Custom(Status::InternalServerError, json!("Error"))
}

pub fn not_found(e: Box<dyn std::error::Error>) -> Custom<Value> {
    log::error!("{}", e);
    Custom(Status::NotFound, json!("404 Not Found"))
}