use std::process::Command;

use reqwest::{blocking::Client, header, StatusCode};
use reqwest::blocking::ClientBuilder;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::{json, Value};

// Define a static lifetime for the BASE URL
pub static APP_HOST: &'static str = "http://127.0.0.1:8000";

// Utility function for creating test rustacean
// Passing in the client as a reference prevents its consumption
pub fn create_test_rustacean(client: &Client) -> Value {
    let response = client.post(format!("{}/rustaceans", APP_HOST))
        .json(&json!({
            "name": "test user",
            "email": "test@test.com",
        }))
        .send()
        .unwrap();

    // Assertions
    assert_eq!(response.status(), StatusCode::CREATED);

    // Grab the response for assertion
    response.json().unwrap()
}

// Utility function for creating a 'test crate'
pub fn create_test_crate(client: &Client, rustacean: &Value) -> Value {
    let response = client.post(format!("{}/crates", APP_HOST))
        .json(&json!({
            "rustacean_id": rustacean["id"],
            "code": "test code",
            "name": "test crate",
            "version": "test version",
            "description": "test description",
        }))
        .send()
        .unwrap();

    // Assertions
    assert_eq!(response.status(), StatusCode::CREATED);

    // Grab the response for assertion
    response.json().unwrap()
}

// Clean up -> Post test script
// The 'rustacean' can be consumed, we don't need to pass it as a reference
pub fn delete_test_rustacean(client: &Client, rustacean: Value) {
    let response = client.delete(format!("{}/rustaceans/{}", APP_HOST, rustacean["id"]))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

pub fn delete_test_crate(client: &Client, a_crate: Value) {
    let response = client.delete(format!("{}/crates/{}", APP_HOST, a_crate["id"]))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

fn get_logged_in_client(username: &str, role: &str) -> Client {
    // Create 'test_admin'
    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("cli")
        .arg("users")
        .arg("create")
        .arg(username)
        .arg("1234")
        .arg(role)
        .output()
        .unwrap();

    println!("{:?}", output);

    let client = Client::new();

    let response = client.post(format!("{}/login", APP_HOST))
        .json(&json!({
            "username": username,
            "password": "1234",
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.get("token").is_some());

    let header_value = format!("Bearer {}", json["token"].as_str().unwrap());

    let mut headers = HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        HeaderValue::from_str(&header_value).unwrap(),
    );

    // ClientBuilder::new().default_headers(headers).build()
    ClientBuilder::new().default_headers(headers).build().unwrap()
}

// Create proxy function -> get_client_with_logged_in_viewer()
pub fn get_client_with_logged_in_viewer() -> Client {
    get_logged_in_client("test_viewer", "viewer")
}

// Create proxy function -> get_client_with_logged_in_viewer()

pub fn get_client_with_logged_in_admin() -> Client {
    get_logged_in_client("test_admin", "admin")
}