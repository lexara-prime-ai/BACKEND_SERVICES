use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use serde_json::{json, Value};
use crate::auth;
use crate::repositories::UserRepository;
use crate::rocket_routes::{CacheConn, server_error};
use super::DbConn;

// Wrap 'CacheConn' inside a Connection from:: rocket_db_pools::Connection in order to be able to use it
// Referencing will not guaranttee an available connection
#[rocket::post("/login", format = "json", data = "<credentials>")]
pub async fn login(credentials: Json<auth::Credentials>, db: DbConn, mut cache: Connection<CacheConn>) -> Result<Value, Custom<Value>> {
    // Store username in a local variable | clone in order to make it available for future referencing
    let username = credentials.username.clone();

    let user = db.run(move |c| {
        // Modified to display unauthorized if a non existing username is passed
        UserRepository::find_by_username(c, &username)
            .map_err(|e| {
                match e {
                    diesel::result::Error::NotFound => Custom(Status::Unauthorized, json!("Wrong credentials!")),
                    _ => server_error(e.into()),
                }
            })
    }).await?;

    // Create session_id a.k.a token
    let session_id = auth::authorize_user(&user, &credentials)
        .map_err(|_| Custom(Status::Unauthorized, json!("Wrong credentials!")))?;

    cache.set_ex::<_, _, ()>(
        format!("sessions/{}", session_id),
        user.id,
        3 * 60 * 60,
    )
        .await
        .map(|_| json!({"token": session_id}))
        .map_err(|e| server_error(e.into()))
}


