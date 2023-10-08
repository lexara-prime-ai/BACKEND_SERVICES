use rocket::serde::json::Json;
use crate::DbConn;
use crate::models::*;
use crate::repositories::RustaceanRepository;

// Route configuration
#[rocket::get("/rustaceans")]
pub fn get_rustaceans(db: DbConn) {
    db.run(|c| {
        // Resume
        RustaceanRepository::find_multiple(c, 100)
    })
}

#[rocket::get("/rustaceans/<id>")]
pub fn view_rustacean(id: i32, db: DbConn) {}

#[rocket::post("/rustaceans", format = "json", data = "<new_rustacean>")]
pub fn create_rustacean(new_rustacean: Json<NewRustacean>, db: DbConn) {}

#[rocket::put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
pub fn update_rustacean(id: i32, rustacean: Json<Rustacean>, db: DbConn) {}

#[rocket::delete("/rustaceans/<id>")]
pub fn delete_rustacean(id: i32) {}