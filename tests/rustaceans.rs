use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde_json::{json, Value};

pub mod common;

/**
 *
 * Note::Cargo runs each test on a separate thread
 * Tip::It's better not to make the tests interdependent
 *
 */

// To do:: Refactor tests in order to reduce redundancy
#[test]
fn test_get_rustaceans() {
    // SETUP
    // Implement http client for making requests
    let client = common::get_client_with_logged_in_admin();
    let rustacean1 = common::create_test_rustacean(&client);
    let rustacean2 = common::create_test_rustacean(&client);

    // TEST SUITE
    // Switch -> Viewer :: for viewing
    let client = common::get_client_with_logged_in_viewer();
    // We don't care about error handling here, just unwrap XD
    let response = client.get(format!("{}/rustaceans", common::APP_HOST)).send().unwrap();
    // Assertions
    assert_eq!(response.status(), StatusCode::OK);

    let json: Value = response.json().unwrap();

    assert!(json.as_array().unwrap().contains(&rustacean1));
    assert!(json.as_array().unwrap().contains(&rustacean2));

    // Switch back -> Admin :: for clean up
    let client = common::get_client_with_logged_in_admin();
    // CLEAN UP
    common::delete_test_rustacean(&client, rustacean1);
    common::delete_test_rustacean(&client, rustacean2);
}

#[test]
fn test_get_rustaceans_without_session_id() {
    // SETUP
    // Implement http client for making requests
    let client = Client::new();

    // TEST SUITE
    // We don't care about error handling here, just unwrap XD
    let response = client.get(format!("{}/rustaceans", common::APP_HOST)).send().unwrap();
    // Assertions
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[test]
fn test_create_rustacean() {
    let client = common::get_client_with_logged_in_admin();
    let response = client.post(format!("{}/rustaceans", common::APP_HOST))
        .json(&json!({
            "name": "test user",
            "email": "test@test.com",
        }))
        .send()
        .unwrap();

    // Assertions
    assert_eq!(response.status(), StatusCode::CREATED);

    // Grab the response for assertion
    let rustacean: Value = response.json().unwrap();

    assert_eq!(rustacean, json!({
        "id": rustacean["id"],
        "name": "test user",
        "email": "test@test.com",
        "created_at": rustacean["created_at"],
    }));

    // CLEAN UP
    common::delete_test_rustacean(&client, rustacean);
}


#[test]
fn test_view_rustacean() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean: Value = common::create_test_rustacean(&client);

    // Switch -> Viewer :: for viewing
    let client = common::get_client_with_logged_in_viewer();
    let response = client.get(format!("{}/rustaceans/{}", common::APP_HOST, rustacean["id"]))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Grab the response for assertion
    let rustacean: Value = response.json().unwrap();

    assert_eq!(rustacean, json!({
        "id": rustacean["id"],
        "name": "test user",
        "email": "test@test.com",
        "created_at": rustacean["created_at"],
    }));

    // Switch back -> Admin :: for clean up
    let client = common::get_client_with_logged_in_admin();
    common::delete_test_rustacean(&client, rustacean);
}

// Test for non existent rustacean
#[test]
fn test_view_nonexistent_rustacean() {
    let client = common::get_client_with_logged_in_admin();
    let non_existent_id = 64290;

    let response = client.get(format!("{}/rustaceans/{}", common::APP_HOST, non_existent_id))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[test]
fn test_update_rustacean() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean: Value = common::create_test_rustacean(&client);
    let response = client.put(format!("{}/rustaceans/{}", common::APP_HOST, rustacean["id"]))
        .json(&json!({
            "name": "updated user",
            "email": "updated@updated.com",
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Grab the response for assertion
    let rustacean: Value = response.json().unwrap();

    assert_eq!(rustacean, json!({
        "id": rustacean["id"],
        "name": "updated user",
        "email": "updated@updated.com",
        "created_at": rustacean["created_at"],
    }));

    common::delete_test_rustacean(&client, rustacean);
}


#[test]
fn test_delete_rustacean() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean: Value = common::create_test_rustacean(&client);
    let response = client.delete(format!("{}/rustaceans/{}", common::APP_HOST, rustacean["id"]))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}