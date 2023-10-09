use rocket::http::Status;
use rocket::response::status::{Custom, NoContent};
use rocket::serde::json::{Json, serde_json::json, Value};

use crate::rocket_routes::DbConn;
use crate::models::*;
use crate::repositories::RustaceanRepository;

// Route configuration
#[rocket::get("/rustaceans")]
pub async fn get_rustaceans(db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        RustaceanRepository::find_multiple(c, 100)
            .map(|rustaceans| json!(rustaceans))
            .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
    }).await
}

#[rocket::get("/rustaceans/<id>")]
pub async fn view_rustacean(id: i32, db: DbConn) -> Result<Value, Custom<Value>> {
    // Move the ownership of id into the callback since the id isn't being used after db.run
    db.run(move |c| {
        RustaceanRepository::find(c, id)
            .map(|rustacean| json!(rustacean))
            .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
    }).await
}

#[rocket::post("/rustaceans", format = "json", data = "<new_rustacean>")]
pub async fn create_rustacean(new_rustacean: Json<NewRustacean>, db: DbConn) -> Result<Custom<Value>, Custom<Value>> {
    db.run(move |c| {
        // Use into_inner in order to unwrap new_rustacean from json
        RustaceanRepository::create(c, new_rustacean.into_inner())
            .map(|rustacean| Custom(Status::Created, json!(rustacean)))
            // Are you sure you want to expose info. about the database |e|, be sure to do away with 'e,to_string()'
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

#[rocket::put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
pub async fn update_rustacean(id: i32, rustacean: Json<Rustacean>, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        // Use into_inner in order to unwrap new_rustacean from json
        RustaceanRepository::update(c, id, rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
    }).await
}

#[rocket::delete("/rustaceans/<id>")]
pub async fn delete_rustacean(id: i32, db: DbConn) -> Result<NoContent, Custom<Value>> {
    // Return <NoContent> on delete
    db.run(move |c| {
        RustaceanRepository::delete(c, id)
            .map(|_| NoContent)
            .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
    }).await
}