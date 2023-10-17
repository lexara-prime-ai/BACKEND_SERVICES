use std::str::FromStr;
use chrono::{Datelike, Utc};
use diesel::{Connection, PgConnection};

use lettre::transport::smtp::authentication::Credentials;

use tera::{Context, Tera};
use crate::auth;

use crate::models::{NewUser, RoleCode};
use crate::repositories::{CrateRepository, RoleRepository, UserRepository};

use lettre::message::header::ContentType;
use lettre::{Message, SmtpTransport, Transport};


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

/**********************
    ::MAILING LOGIC::
************************/
pub fn send_digest(to: String, hours_since: i32) {
    let mut c = load_db_connection();
    let tera = load_template_engine();
    let crates = CrateRepository::find_since(&mut c, hours_since).unwrap();
    if crates.len() > 0 {
        println!("Sending email digest for {} crates...", crates.len());
        let year = Utc::now().year();

        let mut context = Context::new();
        context.insert("crates", &crates);
        context.insert("year", &year);

        let html_body = tera.render("email/digest.html", &context).unwrap();

        // TO DO::Load host info from ENV variables i.e SMTP_HOST etc
        let smtp_key = "";
        let from_email = "";
        let host = "";

        let email: Message = Message::builder()
            .from(from_email.parse().unwrap())
            .to(to.parse().unwrap())
            .subject("Cr8s Digest")
            .header(ContentType::TEXT_HTML)
            .body(html_body)
            .unwrap();

        let mailer: SmtpTransport = SmtpTransport::relay(&host)
            .unwrap()
            .credentials(Credentials::new(
                from_email.to_string(),
                smtp_key.to_string(),
            ))
            .build();

        match mailer.send(&email) {
            Ok(_) => println!("Email sent successfully"),
            Err(e) => panic!("Could not send email: {:?}", e)
        }
    }
}




