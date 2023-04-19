use std::{net::TcpListener, vec, collections::HashMap};
use diesel::{Connection, SqliteConnection};
use diesel::r2d2::{ConnectionManager, Pool};
use server::configuration::{get_configuration, self};

pub struct TestContext {
	pub address: String
}

impl TestContext {
	pub fn new() -> Self {
		let address = Self::spawn_app();

		Self { address }
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

	pub fn establish_connection(&self) -> SqliteConnection {
		// TODO: create a new, unique database each time
		let configuration = get_configuration()
			.expect("Failed to get configuration");

		SqliteConnection::establish(&configuration.database.path)
			.expect("Error connecting to database")
	}
}

impl Drop for TestContext {
	fn drop(&mut self) {
		println!("Tear down");
	}
}