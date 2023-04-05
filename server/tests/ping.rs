use std::{net::TcpListener, vec, collections::HashMap};
use diesel::{Connection, SqliteConnection};
use diesel::r2d2::{ConnectionManager, Pool};
use serde_json::json;
use server::configuration::{get_configuration, self};
use server::{models::*, schema};
use diesel::prelude::*;

fn establish_connection() -> SqliteConnection {
	let configuration = get_configuration().expect("Failed to get configuration");
	SqliteConnection::establish(&configuration.database.url).expect("Error connecting to database")
}


#[tokio::test]
async fn ping_succeeds() {
	let address = spawn_app();
	let client = reqwest::Client::new();

	let response = client
		.get(format!("{}/ping", address))
		.send()
		.await
		.expect("Failed to execute ping");

	assert!(response.status().is_success());
	assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn create_user_returns_200_for_valid_data() {
	let address = spawn_app();
	let client = reqwest::Client::new();
	let configuration = get_configuration().expect("Failed to get configuration");
	let connection = &mut establish_connection();
	use schema::users::dsl::*;

	// let body = "name=ninja&email=ninja@industries";
	let mut body = HashMap::new();
	body.insert("username", "ninja");
	body.insert("balance", "0" );
	body.insert("role", "0");

	let json = json!({
		"username": "ninja",
		"balance": 0,
		"role": 0,
	});

	let response = client
		.post(&format!("{}/user", address))
		// .header("Content-Type", "application/json")
		.json(&json)
		// .body(body)
		.send()
		.await
		.expect("Failed to execute request");

	assert_eq!(200, response.status().as_u16());

	let ninja_user = users
		.filter(username.eq("ninja"))
		.limit(1)
		.load::<User>(connection)
		.expect("Error loading user")
		.pop()
		.expect("No results returned from database");

	assert_eq!(ninja_user.username, "ninja");
	assert_eq!(ninja_user.balance, 0);
	assert_eq!(ninja_user.role, 0);

}

#[tokio::test]
async fn create_user_returns_400_when_data_is_invalid() {
	let address = spawn_app();
	let client = reqwest::Client::new();

	// TODO: rewrite this to use json :D

	let test_cases = vec![
		("name=ninja", "missing email"),
		("email=ninja@industries", "missing name"),
		("", "missing name and email")
	];

	for (body, error_message) in test_cases {
		let response = client
			.post(format!("{}/user", address))
			.header("Content-Type", "application/x-www-form-urlencoded")
			.body(body)
			.send()
			.await
			.expect("Failed to execute request.");

		assert_eq!(400, response.status().as_u16(), "API didn't fail with 400 on when payload was {}.", error_message);
	}
}

fn spawn_app() -> String {
	let listener = TcpListener::bind("127.0.0.1:0")
		.expect("Failed to bind random port");

	let port = listener.local_addr().unwrap().port();

	let server = server::startup::run(listener)
		.expect("Failed to bind address");

	let _ = tokio::spawn(server);

	format!("http://127.0.0.1:{}", port)
}