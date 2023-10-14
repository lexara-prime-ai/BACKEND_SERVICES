use rocket::http::Status;
use rocket::response::status::{Custom, NoContent};
use rocket::serde::json::{Json, serde_json::json, Value};

use crate::rocket_routes::{DbConn, EditorUser, not_found, server_error};
use crate::models::*;
use crate::repositories::RustaceanRepository;

/*
* _user -> acts as a trait for implementing route protection
*/

// Route configuration
#[rocket::get("/rustaceans")]
pub async fn get_rustaceans(db: DbConn, _user: User) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        RustaceanRepository::find_multiple(c, 100)
            .map(|rustaceans| json!(rustaceans))
            .map_err(|e| server_error(e.into()))
    }).await
}

#[rocket::get("/rustaceans/<id>")]
pub async fn view_rustacean(id: i32, db: DbConn, _user: User) -> Result<Value, Custom<Value>> {
    // Move the ownership of id into the callback since the id isn't being used after db.run
    db.run(move |c| {
        // Implement '404 Not Found' if id does not exist
        match RustaceanRepository::find(c, id) {
            Ok(rustacean) => Ok(json!(rustacean)),
            Err(e) => {
                if let diesel::result::Error::NotFound = e {
                    Err(not_found(e.into()))
                } else {
                    Err(server_error(e.into()))
                }
            }
        }
    }).await
}

#[rocket::post("/rustaceans", format = "json", data = "<new_rustacean>")]
pub async fn create_rustacean(new_rustacean: Json<NewRustacean>, db: DbConn, _user: EditorUser) -> Result<Custom<Value>, Custom<Value>> {
    db.run(move |c| {
        // Use into_inner in order to unwrap new_rustacean from json
        RustaceanRepository::create(c, new_rustacean.into_inner())
            .map(|rustacean| Custom(Status::Created, json!(rustacean)))
            // Are you sure you want to expose info. about the database |e|, be sure to do away with 'e,to_string()'
            // This new implementation adds the necessary abstraction
            .map_err(|e| server_error(e.into()))
    }).await
}

#[rocket::put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
pub async fn update_rustacean(id: i32, rustacean: Json<Rustacean>, db: DbConn, _user: EditorUser) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        // Use into_inner in order to unwrap new_rustacean from json
        RustaceanRepository::update(c, id, rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|e| server_error(e.into()))
    }).await
}

#[rocket::delete("/rustaceans/<id>")]
pub async fn delete_rustacean(id: i32, db: DbConn, _user: EditorUser) -> Result<NoContent, Custom<Value>> {
    // Return <NoContent> on delete
    db.run(move |c| {
        RustaceanRepository::delete(c, id)
            .map(|_| NoContent)
            .map_err(|e| server_error(e.into()))
    }).await
}