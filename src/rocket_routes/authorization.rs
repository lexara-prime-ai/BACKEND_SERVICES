use rocket::response::status::Custom;
use rocket::serde::json::Json;
use serde_json::{json, Value};
use crate::auth;
use crate::repositories::UserRepository;
use crate::rocket_routes::server_error;
use super::DbConn;


#[rocket::post("/login", format = "json", data = "<credentials>")]
pub async fn login(credentials: Json<auth::Credentials>, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        UserRepository::find_by_username(c, &credentials.username)
            .map(|user| {
                // Create new session ID
                if let Ok(token) = auth::authorize_user(&user, &credentials) {
                    return json!(token);
                }
                json!("Unauthorized!")
            })
            .map_err(|e| server_error(e.into()))
    }).await
}


