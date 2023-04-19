use std::{net::TcpListener};
use diesel::{Connection, SqliteConnection, r2d2};
use diesel::r2d2::{ConnectionManager, Pool};
use server::configuration::{get_configuration, self};

pub struct TestContext {
	pub address: String,
	pub pool: Pool<ConnectionManager<SqliteConnection>>
}

impl TestContext {
	pub fn new() -> Self {
		let listener = TcpListener::bind("127.0.0.1:0")
		.expect("Failed to bind random port");

		let port = listener.local_addr().unwrap().port();

		// TODO: we need to randomize and initalize a fresh database for every context here
		let configuration = get_configuration()
			.expect("Failed to read configuration");
		let manager = r2d2::ConnectionManager::<SqliteConnection>::new(configuration.database.path);
		let pool = r2d2::Pool::builder()
			.build(manager)
			.expect("Failed to load SQLite database");


		let server = server::startup::run(listener, pool.clone())
			.expect("Failed to bind address");

		let _ = tokio::spawn(server);

		Self {
			address: format!("http://127.0.0.1:{}", port),
			pool: pool
		}
	}
}

impl Drop for TestContext {
	fn drop(&mut self) {
		println!("Tear down");
	}
}