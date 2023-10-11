use argon2::{password_hash::{rand_core::OsRng, SaltString}, PasswordHasher, PasswordHash, PasswordVerifier, Argon2, password_hash::Error};
use rand::distributions::Alphanumeric;
use rand::Rng;

use crate::models::User;

#[derive(serde::Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

pub fn authorize_user(user: &User, credentials: &Credentials) -> Result<String, Error> {
    // Get stored password | hash
    // Avoid unwrap and use '?' instead
    let db_hash = PasswordHash::new(&user.password)?;
    let argon = Argon2::default();
    // Verify password hash
    argon.verify_password(
        credentials.password.as_bytes(), &db_hash,
    )?;

    // Use the OS to generate a random number
    Ok(rand::thread_rng()
        // Generate an alphanumeric ID to make sure it's safe for use on the web
        .sample_iter(&Alphanumeric)
        .take(128)
        .map(char::from)
        // Collecting will result in a string
        .collect()
    )
}

pub fn hash_password(password: String) -> Result<String, Error> {
    // Create a new salt
    let salt = SaltString::generate(OsRng);
    let argon = Argon2::default();
    // hash_password only accepts the first argument as_bytes & the second as a reference &
    let password_hash = argon.hash_password(password.as_bytes(), &salt)?;
    // Convert password_hash -> to_string()
    Ok(password_hash.to_string())
}