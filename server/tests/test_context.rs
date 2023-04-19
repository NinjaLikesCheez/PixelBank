use std::{net::TcpListener};
use actix_web::dev::RequestHead;
use diesel::{SqliteConnection, r2d2};
use diesel::r2d2::{ConnectionManager, Pool};
extern crate diesel_migrations;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub struct TestContext {
	pub address: String,
	pub pool: Pool<ConnectionManager<SqliteConnection>>,
	pub client: reqwest::Client
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
}

// impl Drop for TestContext {
// 	fn drop(&mut self) {
// 		println!("Tear down");
// 	}
// }