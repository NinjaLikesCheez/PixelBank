use std::{vec};
use serde_json::json;
use server::{models::*, schema};
use diesel::prelude::*;

pub mod test_context;
use test_context::*;

#[tokio::test]
async fn create_user_returns_200_for_valid_data() {
	let _ctx = TestContext::new();
	let client = reqwest::Client::new();

	let json = json!({
		"username": "ninja",
		"balance": 0,
		"role": "member",
	});

	let response = client
		.post(&format!("{}/user", _ctx.address))
		.json(&json)
		.send()
		.await
		.expect("Failed to execute request");

	assert_eq!(200, response.status().as_u16());

	// Check the data made it to the database
	let connection = &mut _ctx.pool.get().expect("Failed to get connection from pool");
	use schema::users::dsl::*;

	let ninja_user = users
		.filter(username.eq("ninja"))
		.limit(1)
		.load::<User>(connection)
		.expect("Error loading user")
		.pop()
		.expect("No results returned from database");

	assert_eq!(ninja_user.username, "ninja");
	assert_eq!(ninja_user.balance, 0);
	assert_eq!(ninja_user.role, "member");

}

#[tokio::test]
async fn create_user_returns_422_when_data_is_invalid() {
	let _ctx = TestContext::new();
	let client = reqwest::Client::new();

	// TODO: rewrite this to use json - check error codes etc
	let test_cases = vec![
		(json!({"name": "ninja"}), "missing balance & role"),
		(json!({"balance": 0, "role": "member"}), "missing username"),
		(json!({"name": "ninja", "balance": "0", "role": "member"}), "balance is a string"),
	];

	for (body, error_message) in test_cases {
		let response = client
			.post(format!("{}/user", _ctx.address))
			.json(&body)
			.send()
			.await
			.expect("Failed to execute request.");

		assert_eq!(422, response.status().as_u16(), "API didn't fail with 400 on when payload was {}.", error_message);
	}
}

#[tokio::test]
async fn create_user_returns_409_when_user_exists() {
	let _ctx = TestContext::new();
	let client = reqwest::Client::new();

	// First, create a user
	let json = json!({
		"username": "ninja",
		"balance": 0,
		"role": "member",
	});

	let response = client
		.post(&format!("{}/user", _ctx.address))
		.json(&json)
		.send()
		.await
		.expect("Failed to execute request");

	assert_eq!(200, response.status().as_u16());

	// Try to create the user again
	let response = client
		.post(&format!("{}/user", _ctx.address))
		.json(&json)
		.send()
		.await
		.expect("Failed to execute request");

	assert_eq!(409, response.status().as_u16());

	let error = response
		.json::<APIError>()
		.await
		.expect("Failed to decode to API Error");

	assert_eq!(error.error, "User already exists");
}

#[tokio::test]
async fn create_user_returns_400_when_balance_too_low() {
	let _ctx = TestContext::new();
	let client = reqwest::Client::new();

	// First, create a user
	let json = json!({
		"username": "ninja",
		"balance": -100,
		"role": "member",
	});

	let response = client
		.post(&format!("{}/user", _ctx.address))
		.json(&json)
		.send()
		.await
		.expect("Failed to execute request");

	assert_eq!(400, response.status().as_u16());

	let error = response
		.json::<APIError>()
		.await
		.expect("Failed to decode API error");

	assert_eq!(error.error, "New user balance cannot be below zero")
}

#[tokio::test]
async fn create_user_returns_403_when_creating_admin() {
	let _ctx = TestContext::new();
	let client = reqwest::Client::new();

	// First, create a user
	let json = json!({
		"username": "ninja",
		"balance": 0,
		"role": "admin",
	});

	let response = client
		.post(&format!("{}/user", _ctx.address))
		.json(&json)
		.send()
		.await
		.expect("Failed to execute request");

	assert_eq!(403, response.status().as_u16());

	let error = response
		.json::<APIError>()
		.await
		.expect("Failed to decode API error");

	assert_eq!(error.error, "Admin accounts cannot be created via API")
}
