use std::{vec, collections::HashMap};
use serde_json::json;
use server::{models::*, schema};
use diesel::prelude::*;

pub mod test_context;
use test_context::*;

#[tokio::test]
async fn create_user_returns_200_for_valid_data() {
	let _ctx = TestContext::new();
	let client = reqwest::Client::new();

	let mut body = HashMap::new();
	body.insert("username", "ninja");
	body.insert("balance", "0" );
	body.insert("role", "0");

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
async fn create_user_returns_400_when_data_is_invalid() {
	let _ctx = TestContext::new();
	let client = reqwest::Client::new();

	// TODO: rewrite this to use json - check error codes etc
	let test_cases = vec![
		("name=ninja", "missing email"),
		("email=ninja@industries", "missing name"),
		("", "missing name and email")
	];

	for (body, error_message) in test_cases {
		let response = client
			.post(format!("{}/user", _ctx.address))
			.header("Content-Type", "application/x-www-form-urlencoded")
			.body(body)
			.send()
			.await
			.expect("Failed to execute request.");

		assert_eq!(400, response.status().as_u16(), "API didn't fail with 400 on when payload was {}.", error_message);
	}
}