use diesel::PgConnection;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket_sync_db_pools;
use rocket_db_pools::{deadpool_redis, Database};
use rocket::serde::json::{serde_json::json, Value};
use rocket_sync_db_pools::database;

pub mod rustaceans;
pub mod crates;
pub mod authorization;

#[rocket_sync_db_pools::database("postgres")]
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