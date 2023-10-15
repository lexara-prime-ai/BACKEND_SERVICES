use std::str::FromStr;
use chrono::{Datelike, Utc};
use diesel::{Connection, PgConnection};

use lettre::transport::smtp::authentication::Credentials;

use tera::{Context, Tera};
use crate::auth;
use crate::mail::HtmlMailer;
use crate::models::{NewUser, RoleCode};
use crate::repositories::{CrateRepository, RoleRepository, UserRepository};

// Load database connection
fn load_db_connection() -> PgConnection {
    // Read DATABASE_URL from the env::variables
    let database_url = std::env::var("DATABASE_URL")
        .expect("Failed to load DB URL from env");
    PgConnection::establish(&database_url)
        .expect("Failed to establish connection with postgres")
}

fn load_template_engine() -> Tera {
    Tera::new("templates/**/*.html").unwrap_or_else(|e| {
        panic!("Parsing error(s): {}", e);
    })
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

pub fn send_digest(to: String, hours_since: i32) {
    println!("Establishing connection...");

    let mut c = load_db_connection();
    let tera = load_template_engine();

    println!("Connection established...\nProceeding...");
    let crates = CrateRepository::find_since(&mut c, hours_since).unwrap();
    // Check if crates are available | if not don't send an email
    if crates.len() > 0 {
        // Debugging...
        println!("Sending email digest for {} crates...", crates.len());

        let year = Utc::now().year();
        // Create Tera context
        let mut context = Context::new();
        // Add context
        context.insert("crates", &crates);
        context.insert("year", &year);


        // Load host info from ENV variables i.e SMTP_HOST etc
        let smtp_host = std::env::var("SMTP_HOST")
            .expect("Cannot load SMTP host from env");

        let smtp_username = std::env::var("SMTP_USERNAME")
            .expect("Cannot load SMTP username from env");

        let smtp_password = std::env::var("SMTP_PASSWORD")
            .expect("Cannot load SMTP password from env");

        // Create credential struct
        let credentials = Credentials::new(smtp_username, smtp_password);
        let mailer = HtmlMailer { smtp_host, credentials, template_engine: tera };
        // Send email with custom implementation -> send_email()
        mailer.send_email(&to, "email/digest.html", &context).unwrap();
    }
}