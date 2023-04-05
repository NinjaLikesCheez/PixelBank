use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;

use crate::routes;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
	let server = HttpServer::new(|| {
			App::new()
			.route("/ping", web::get().to(routes::ping))
			.route("/user", web::post().to(routes::create_user))
		})
		.listen(listener)?
		.run();

	Ok(server)
}