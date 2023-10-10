use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable};
use rocket::serde::Deserialize;
use serde::Serialize;
use crate::schema::*;

#[derive(Queryable, AsChangeset, Deserialize, Serialize)]
pub struct Rustacean {
    // Make the id optional e.g when creating the payload for the update <readonly>
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub email: String,
    // This value should never change so we should skip it as well <readonly>
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = rustaceans)]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}

#[derive(Queryable, AsChangeset, Deserialize, Serialize)]
pub struct Crate {
    // Make the id optional e.g when creating the payload for the update <readonly>
    #[serde(skip_deserializing)]
    pub id: i32,
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    // This value should never change so we should skip it as well <readonly>
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crates)]
pub struct NewCrate {
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
}

// User models
#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

// Role models
#[derive(Queryable)]
pub struct Role {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = roles)]
pub struct NewRole {
    pub code: String,
    pub name: String,
}

// Most ORMs hide the fact that a join/pivot table exists
#[derive(Queryable)]
#[diesel(belongs_to(User))] // Define table relationship
#[diesel(belongs_to(Role))]
pub struct UserRole {
    pub user_id: i32,
    pub role_id: i32,
    pub id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = users_roles)]
pub struct NewUserRole {
    pub user_id: i32,
    pub role_id: i32,
}