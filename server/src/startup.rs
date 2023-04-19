use actix_web::{web, App, HttpServer, error, HttpRequest, HttpResponse};
use actix_web::dev::Server;
use diesel::r2d2::ConnectionManager;
use diesel::{SqliteConnection, r2d2};
use serde_json::json;
use std::net::TcpListener;

use crate::routes;

// Ideas on making this better:
// https://github.com/serde-rs/json/issues/759???
// https://www.reddit.com/r/rust/comments/doexmf/gracefully_handling_json_validation_errors_with/
fn json_decoding_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
	use actix_web::error::JsonPayloadError;

	let detail = err.to_string();
	let resp = match &err {
		JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType().body(detail),
		JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
			let json = json!({
				"error": "JSON Error",
				"message": detail
				// TODO detail needs to be more... customizable, don't want to have to manually deserialize in every route
		});
			HttpResponse::UnprocessableEntity().json(json)
		}
		_ => HttpResponse::BadRequest().body(detail),
	};

	error::InternalError::from_response(err, resp).into()
}

pub fn run(listener: TcpListener, pool: r2d2::Pool<ConnectionManager<SqliteConnection>>) -> Result<Server, std::io::Error> {
	let server = HttpServer::new(move || {
			App::new()
			.app_data(web::Data::new(pool.clone()))
			.app_data(
				web::JsonConfig::default().error_handler(json_decoding_error_handler)
			)
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