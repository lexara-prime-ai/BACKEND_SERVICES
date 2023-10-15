use std::process::Command;
use reqwest::{blocking::Client, StatusCode};
use rocket::form::validate::Len;
use serde_json::{json, Value};

pub mod common;

// Automated tests for login endpoint
#[test]
fn test_login() {
    // Create 'test_admin'
    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("cli")
        .arg("users")
        .arg("create")
        .arg("test_admin")
        .arg("1234")
        .arg("admin")
        .output()
        .unwrap();

    println!("{:?}", output);

    let client = Client::new();

    let response = client.post(format!("{}/login", common::APP_HOST))
        .json(&json!({
            "username": "test_admin",
            "password": "1234",
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.get("token").is_some());
    assert_eq!(json["token"].as_str().len(), 128);

    // Test for wrong credentials
    let response = client.post(format!("{}/login", common::APP_HOST))
        .json(&json!({
            "username": "test_admi",
            "password": "12345",
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

// Test 'me' endpoint
// #[test]
// fn test_me() {
//     let client = common::get_client_with_logged_in_viewer();
//     let response = client.get(format!("{}/me", common::APP_HOST))
//         .send()
//         .unwrap();
//
//     assert_eq!(response.status(), StatusCode::OK);
//     let json: Value = response.json().unwrap();
//     assert!(json.get("id").is_some());
//     assert!(json.get("username").is_some());
//     assert_eq!(json["username"], "test_viewer");
//     assert!(json.get("created_at").is_some());
//     assert!(json.get("password").is_none());
// }
