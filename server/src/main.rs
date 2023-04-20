use std::net::TcpListener;
use diesel::{SqliteConnection, r2d2};

use server::startup;


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
	let configuration = server::configuration::get().expect("Failed to read configuration");
	let address = format!("0.0.0.0:{}", configuration.application_port);
	let listener = TcpListener::bind(address).expect("Failed to bind port: 1337");

	let manager = r2d2::ConnectionManager::<SqliteConnection>::new(configuration.database.path);
	let pool = r2d2::Pool::builder()
		.build(manager)
		.expect("Failed to load SQLite database");

	startup::run(listener, pool)?.await
}
