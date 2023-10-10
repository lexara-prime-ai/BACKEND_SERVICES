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

