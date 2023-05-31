use std::{net::TcpListener, path::Path};
use diesel::{SqliteConnection, r2d2};
use dotenvy::dotenv;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

use server::startup;


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
	dotenv().ok();
	pretty_env_logger::init();

	let configuration = server::configuration::get().expect("Failed to read configuration");
	let address = format!("0.0.0.0:{}", configuration.application_port);
	info!("▶️ Attempting to start server on http://{address}");

	let listener = TcpListener::bind(address).expect("Failed to bind port: {address}");

	use std::fs;

	let db_path = Path::new(&configuration.database.path);
	if !db_path.exists() {
		warn!("📁 Database file does not exist, creating it...");
		fs::File::create(&db_path).expect("Failed to create database file");
		info! ("📁 Database file created");
	}
	else {
		info!("📁 Database file exists");
	}

	let manager = r2d2::ConnectionManager::<SqliteConnection>::new(db_path.to_str().unwrap());
	let pool = r2d2::Pool::builder()
		.build(manager)
		.expect("Failed to load SQLite database");

	info!("✔️  Successfully connected to database");
	
	startup::run(listener, pool)?.await
}
