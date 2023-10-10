use diesel::{Connection, PgConnection};
use crate::models::NewUser;
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
    // Create a new_user
    let new_user = NewUser { username, password };
    let user = UserRepository::create(&mut c, new_user, role_codes).unwrap(); // unwrap otherwise, Result -> QueryResult<T>
    // Print user to console | for debugging
    println!("User created: {:?}", user);

    let roles = RoleRepository::find_by_user(&mut c, &user).unwrap();
    // Print all roles assigned to user
    println!("Role assigned : {:?}", roles)
}

pub fn list_users() {}

pub fn delete_user(id: i32) {}