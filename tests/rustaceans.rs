use reqwest::blocking::Client;
use reqwest::StatusCode;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;


// Utility function for creating test rustacean
// Passing in the client as a reference prevents its consumption
fn create_test_rustacean(client: &Client) -> Value {
    let response = client.post("http://127.0.0.1:8000/rustaceans")
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

// To do:: Refactor tests in order to reduce redundancy
#[test]
fn test_get_rustaceans() {
    // Implement http client for making requests
    let client = Client::new();
    let rustacean1 = create_test_rustacean(&client);
    let rustacean2 = create_test_rustacean(&client);

    // We don't care about error handling here, just unwrap XD
    let response = client.get("http://127.0.0.1:8000/rustaceans").send().unwrap();
    // Assertions
    assert_eq!(response.status(), StatusCode::OK);

    let json: Value = response.json().unwrap();

    assert!(json.as_array().unwrap().contains(&rustacean1));
    assert!(json.as_array().unwrap().contains(&rustacean2));
}

#[test]
fn test_create_rustacean() {
    let client = Client::new();
    let response = client.post("http://127.0.0.1:8000/rustaceans")
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
}


#[test]
fn test_view_rustacean() {
    let client = Client::new();
    let rustacean: Value = create_test_rustacean(&client);
    let response = client.get(format!("http://127.0.0.1:8000/rustaceans/{}", rustacean["id"]))
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
}

#[test]
fn test_update_rustacean() {
    let client = Client::new();
    let rustacean: Value = create_test_rustacean(&client);
    let response = client.put(format!("http://127.0.0.1:8000/rustaceans/{}", rustacean["id"]))
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
}


#[test]
fn test_delete_rustacean() {
    let client = Client::new();
    let rustacean: Value = create_test_rustacean(&client);
    let response = client.delete(format!("http://127.0.0.1:8000/rustaceans/{}", rustacean["id"]))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}