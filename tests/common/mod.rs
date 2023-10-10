use reqwest::{blocking::Client, StatusCode};
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