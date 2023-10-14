use rocket::http::Status;
use rocket::response::status::{Custom, NoContent};
use rocket::serde::json::{Json, serde_json::json, Value};

use crate::rocket_routes::{DbConn, EditorUser, not_found, server_error};
use crate::models::*;
use crate::repositories::CrateRepository;

// Route configuration
#[rocket::get("/crates")]
pub async fn get_crates(db: DbConn, _user: User) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        CrateRepository::find_multiple(c, 100)
            .map(|crates| json!(crates))
            .map_err(|e| server_error(e.into()))
    }).await
}

#[rocket::get("/crates/<id>")]
pub async fn view_crate(id: i32, db: DbConn, _user: User) -> Result<Value, Custom<Value>> {
    // Move the ownership of id into the callback since the id isn't being used after db.run
    db.run(move |c| {
        // Implement '404 Not Found' if id does not exist
        match CrateRepository::find(c, id) {
            Ok(a_crate) => Ok(json!(a_crate)),
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

#[rocket::post("/crates", format = "json", data = "<new_crate>")]
pub async fn create_crate(new_crate: Json<NewCrate>, db: DbConn, _user: EditorUser) -> Result<Custom<Value>, Custom<Value>> {
    db.run(move |c| {
        // Use into_inner in order to unwrap new_crate from json
        CrateRepository::create(c, new_crate.into_inner())
            .map(|a_crate| Custom(Status::Created, json!(a_crate)))
            // Are you sure you want to expose info. about the database |e|, be sure to do away with 'e,to_string()'
            .map_err(|e| server_error(e.into()))
    }).await
}

#[rocket::put("/crates/<id>", format = "json", data = "<a_crate>")]
pub async fn update_crate(id: i32, a_crate: Json<Crate>, db: DbConn, _user: EditorUser) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        // Use into_inner in order to unwrap new_crate from json
        CrateRepository::update(c, id, a_crate.into_inner())
            .map(|a_crate| json!(a_crate))
            // Use the into() method to transform the error to a boxed version
            .map_err(|e| server_error(e.into()))
    }).await
}

#[rocket::delete("/crates/<id>")]
pub async fn delete_crate(id: i32, db: DbConn, _user: EditorUser) -> Result<NoContent, Custom<Value>> {
    // Return <NoContent> on delete
    db.run(move |c| {
        CrateRepository::delete(c, id)
            .map(|_| NoContent)
            .map_err(|e| server_error(e.into()))
    }).await
}