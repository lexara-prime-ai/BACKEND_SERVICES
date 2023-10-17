use diesel::PgConnection;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::request::Outcome;
use rocket::http::Status;
use rocket::{Request, Response};
use rocket::request::FromRequest;
use rocket::response::status::Custom;
use rocket_sync_db_pools;
use rocket_db_pools::{deadpool_redis, Database, Connection};

use rocket_db_pools::deadpool_redis::redis::AsyncCommands;

use rocket::serde::json::{serde_json::json, Value};

use rocket_sync_db_pools::database;
use crate::models::{RoleCode, User};
use crate::repositories::{RoleRepository, UserRepository};

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

// Handle response to preflight requests
#[rocket::options("/<_route_args..>")]
pub fn options(_route_args: Option<std::path::PathBuf>) {
    // Handle CORS header via -> fairing
}

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Append CORS headers in responses",
            kind: Kind::Response,
        }
    }
    async fn on_response<'r>(&self, _req: &'r Request<'_>, res: &mut Response<'r>) {
        res.set_raw_header("Access-Control-Allow-Origin", "*");
        res.set_raw_header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE");
        res.set_raw_header("Access-Control-Allow-Headers", "*");
        res.set_raw_header("Access-Control-Allow-Credentials", "true");
    }
}

// Define a User with 'edit' permissions
pub struct EditorUser(User);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for EditorUser {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user = request.guard::<User>().await
            .expect("Cannot retrieve logged in user from request guard...");

        let db = request.guard::<DbConn>().await
            .expect("Cannot connect to Postgres in request guard...");

        let editor_option = db.run(|c| {
            // Only give access to roles that match the result
            // Any route containing the route guard EditorUser will evaluate the roles -> Admin & Editor
            match RoleRepository::find_by_user(c, &user) {
                Ok(roles) => {
                    // Log::for debugging purposes
                    log::info!("Assigned roles {:?}", roles);
                    let is_editor = roles.iter().any(|r| match r.code {
                        RoleCode::Admin => true,
                        RoleCode::Editor => true,
                        _ => false,
                    });

                    // Log::for debugging purposes
                    log::info!("Is editor is: {:?}", is_editor);
                    is_editor.then_some(EditorUser(user))
                }
                // If result is empty
                _ => None
            }
        }).await;

        match editor_option {
            Some(editor) => Outcome::Success(editor),
            _ => Outcome::Failure((Status::Unauthorized, ()))
        }
    }
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
                };
            }
        }

        Outcome::Failure((Status::Unauthorized, ()))
    }
}
