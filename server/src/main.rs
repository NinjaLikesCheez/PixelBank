#[macro_use]
extern crate actix_web;

use std::{io, env};

use actix_web::{App, HttpServer, middleware};
use actix_web::web::Data;
use diesel::r2d2::{Pool, PooledConnection, ConnectionManager};
use diesel::SqliteConnection;
use dotenvy::dotenv;

mod user;
mod response;
mod constants;
mod schema;

pub type DBPool = Pool<ConnectionManager<SqliteConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<SqliteConnection>>;

#[actix_rt::main]
async fn main() -> io::Result<()> {
	env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
	env_logger::init();
	dotenv().ok();

	// DB
	let database_url = env::var("DATABASE_URL").expect("Set DATABASE_URL in your environment");
	let manager = ConnectionManager::<SqliteConnection>::new(database_url);
	let pool = Pool::builder()
		.build(manager)
		.expect("Failed to create the database pool");

	HttpServer::new(move || {
		App::new()
			// used with web::Data<Pool> extractor
			.app_data(Data::new(pool.clone()))
			// enable logger - always register this last
			.wrap(middleware::Logger::default())
			// HTTP request handlers
			.service(user::all_users)
			.service(user::new_user)
	})
	.bind("0.0.0.0:1337")?
	.run()
	.await
}
