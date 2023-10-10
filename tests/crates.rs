use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

pub mod common;

/*
    REFERENCE STRUCT

    pub id: i32,
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
*/


#[test]
fn test_get_crates() {
    // SETUP
    // Implement http client for making requests
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);

    let a_crate = common::create_test_crate(&client, &rustacean);
    let b_crate = common::create_test_crate(&client, &rustacean);

    // TEST SUITE
    // We don't care about error handling here, just unwrap XD
    let response = client.get(format!("{}/crates", common::APP_HOST)).send().unwrap();
    // Assertions
    assert_eq!(response.status(), StatusCode::OK);

    let json: Value = response.json().unwrap();

    assert!(json.as_array().unwrap().contains(&a_crate));
    assert!(json.as_array().unwrap().contains(&b_crate));

    // CLEAN UP
    common::delete_test_crate(&client, a_crate);
    common::delete_test_crate(&client, b_crate);
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_create_crate() {
    let client = Client::new();
    // Create a rustacean for testing
    let rustacean = common::create_test_rustacean(&client);
    let response = client.post(format!("{}/crates", common::APP_HOST))
        .json(&json!({
            "rustacean_id": rustacean["id"],
            "code": "test code",
            "name": "test crate",
            "version": "test version",
            "description": "test description",
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    // Verify payload
    let a_crate: Value = response.json().unwrap();

    assert_eq!(a_crate, json!({
        "id": a_crate["id"],
        "rustacean_id": rustacean["id"],
        "code": "test code",
        "name": "test crate",
        "version": "test version",
        "description": "test description",
        "created_at": a_crate["created_at"],
    }));

    // CLEAN UP
    // In this case order matters due to the table relationships
    // -> Attempting to delete the rustacean first will result in a error
    common::delete_test_crate(&client, a_crate);
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_view_crate() {
    let client = Client::new();
    // Create a rustacean for testing
    let rustacean = common::create_test_rustacean(&client);
    // Create test crate
    let a_crate = common::create_test_crate(&client, &rustacean);
    let response = client.get(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Verify payload
    let a_crate: Value = response.json().unwrap();

    assert_eq!(a_crate, json!({
        "id": a_crate["id"],
        "rustacean_id": rustacean["id"],
        "code": "test code",
        "name": "test crate",
        "version": "test version",
        "description": "test description",
        "created_at": a_crate["created_at"],
    }));

    // CLEAN UP
    // In this case order matters due to the table relationships
    // -> Attempting to delete the rustacean first will result in a error
    common::delete_test_crate(&client, a_crate);
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_update_crate() {
    let client = Client::new();
    // Create a rustacean for testing
    let rustacean = common::create_test_rustacean(&client);
    // Create test crate
    let a_crate = common::create_test_crate(&client, &rustacean);
    let response = client.put(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
        .json(&json!({
            "rustacean_id": rustacean["id"],
            "code": "updated code",
            "name": "updated crate",
            "version": "updated version",
            "description": "updated description",
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Verify payload
    let a_crate: Value = response.json().unwrap();

    assert_eq!(a_crate, json!({
        "id": a_crate["id"],
        "rustacean_id": rustacean["id"],
        "code": "updated code",
        "name": "updated crate",
        "version": "updated version",
        "description": "updated description",
        "created_at": a_crate["created_at"],
    }));

    let rustacean2 = common::create_test_rustacean(&client);
    // Test author switching
    let response = client.put(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
        .json(&json!({
            "rustacean_id": rustacean2["id"],
            "code": "updated code",
            "name": "updated crate",
            "version": "updated version",
            "description": "updated description",
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Reassign new response value to a_crate
    let a_crate: Value = response.json().unwrap();


    // Assert the new payload
    assert_eq!(a_crate, json!({
        "id": a_crate["id"],
        "rustacean_id": rustacean2["id"],
        "code": "updated code",
        "name": "updated crate",
        "version": "updated version",
        "description": "updated description",
        "created_at": a_crate["created_at"],
    }));


    // CLEAN UP
    // In this case order matters due to the table relationships
    // -> Attempting to delete the rustacean first will result in a error
    common::delete_test_crate(&client, a_crate);
    common::delete_test_rustacean(&client, rustacean);
}


#[test]
fn test_delete_crate() {
    let client = Client::new();
    // Create a rustacean for testing
    let rustacean = common::create_test_rustacean(&client);
    // Create test crate
    let a_crate = common::create_test_crate(&client, &rustacean);
    let response = client.delete(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    // Clean up
    common::delete_test_rustacean(&client, rustacean);
}