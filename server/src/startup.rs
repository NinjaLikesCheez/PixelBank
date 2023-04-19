use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use diesel::r2d2::ConnectionManager;
use diesel::{SqliteConnection, r2d2};
use std::net::TcpListener;

use crate::routes;

pub fn run(listener: TcpListener, pool: r2d2::Pool<ConnectionManager<SqliteConnection>>) -> Result<Server, std::io::Error> {
	let server = HttpServer::new(move || {
			App::new()
			.app_data(web::Data::new(pool.clone()))
			.route(
				"/ping",
				web::get().to(routes::ping)
			)
			.route(
				"/user",
				web::post().to(routes::create_user)
			)
		})
		.listen(listener)?
		.run();

	Ok(server)
}