use std::str::FromStr;
use diesel::{Connection, PgConnection};
use crate::auth;
use crate::models::{NewUser, RoleCode};
use crate::repositories::{RoleRepository, UserRepository};

// Load database connection
fn load_db_connection() -> PgConnection {
    // Read DATABASE_URL from the env::variables
    let database_url = std::env::var("DATABASE_URL")
        .expect("Failed load DB URL from env");
    PgConnection::establish(&database_url)
        .expect("Failed to establish connection with postgres")
}

pub fn create_user(username: String, password: String, role_codes: Vec<String>) {
    // Create a new connection
    let mut c = load_db_connection();
    // Hash password
    let password_hash = auth::hash_password(password).unwrap();
    // Create a new_user
    let new_user = NewUser { username, password: password_hash.to_string() };

    let role_codes = role_codes.iter().map(|v| RoleCode::from_str(&v).unwrap()).collect();
    let user = UserRepository::create(&mut c, new_user, role_codes).unwrap(); // unwrap otherwise, Result -> QueryResult<T>
    // Print user to console | for debugging
    println!("User created: {:?}", user);

    let roles = RoleRepository::find_by_user(&mut c, &user).unwrap();
    // Print all roles assigned to user
    // To do::Inspect this -> User Roles don't seem to be printed to the console on user creation
    println!("Role assigned : {:?}", roles)
}

pub fn list_users() {
    let mut c = load_db_connection();
    let users = UserRepository::find_with_roles(&mut c).unwrap();
    // Loop through users
    for user in users {
        println!("{:?}", user);
    }
}

pub fn delete_user(id: i32) {
    let mut c = load_db_connection();
    UserRepository::delete(&mut c, id).unwrap();
}