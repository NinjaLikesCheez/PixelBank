use std::{net::TcpListener};
use diesel::prelude::*;
use diesel::{SqliteConnection, r2d2};
use diesel::r2d2::{ConnectionManager, Pool};
extern crate diesel_migrations;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use serde_json::json;
use server::models::{User, Transaction};
use server::schema;
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub struct TestContext {
	pub address: String,
	pub pool: Pool<ConnectionManager<SqliteConnection>>,
	pub client: reqwest::Client,
}

impl TestContext {
	pub fn new() -> Self {
		let listener = TcpListener::bind("127.0.0.1:0")
		.expect("Failed to bind random port");

		let port = listener.local_addr().unwrap().port();

		let manager = r2d2::ConnectionManager::<SqliteConnection>::new(":memory:");
		let pool = r2d2::Pool::builder()
			.build(manager)
			.expect("Failed to load SQLite database");

		let mut connection = pool.get().expect("Failed to get connection from pool");
		connection.run_pending_migrations(MIGRATIONS).expect("Failed to run migrations");

		let server = server::startup::run(listener, pool.clone())
			.expect("Failed to bind address");

		let _ = tokio::spawn(server);

		Self {
			address: format!("http://127.0.0.1:{}", port),
			pool: pool,
			client: reqwest::Client::new()
		}
	}

	pub async fn get_user(_ctx: &TestContext, _user_id: &str) -> User {
		let response = _ctx.client
			.get(&format!("{}/users/{}", _ctx.address, _user_id))
			.send()
			.await
			.expect("Failed to execute request");

		response
			.json::<User>()
			.await
			.expect("Failed to decode to User model")
	}

	pub async fn create_user(_ctx: &TestContext, _username: &str, _balance: i32, _role: &str) -> User {
		let json = json!({
			"username": _username,
			"balance": _balance,
			"role": _role,
		});
	
		_ctx.client
			.post(&format!("{}/users", _ctx.address))
			.json(&json)
			.send()
			.await
			.expect("Failed to execute request");

		// Return the user we just created
		let connection = &mut _ctx.pool.get().expect("Failed to get connection from pool");
		use schema::users::dsl::*;

		users
			.filter(username.eq(json.get("username").unwrap().as_str().unwrap()))
			.limit(1)
			.load::<User>(connection)
			.expect("Error loading user")
			.pop()
			.expect("No results returned from database")
	}

	pub async fn create_transaction(_ctx: &TestContext, user_id_in: &str, transaction_type: &str, mutation: &i32) -> Transaction {
		//Serde really doesn't like floats, so we're just not making it variable for now
		let json = json!({
			"mutation": mutation
		});

		let response = _ctx.client
			.post(&format!("{}/users/{}/transactions/{}", _ctx.address, user_id_in, transaction_type))
			.json(&json)
			.send()
			.await
			.expect("Failed to execute request");

		response
			.json::<Transaction>()
			.await
			.expect("Failed to decode to Transaction model")
	}
	
}

// impl Drop for TestContext {
// 	fn drop(&mut self) {
// 		println!("Tear down");
// 	}
// }