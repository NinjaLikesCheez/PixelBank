use actix_web::{web, App, HttpServer, error, HttpRequest, HttpResponse};
use actix_web::dev::Server;
use diesel::r2d2::ConnectionManager;
use diesel::{SqliteConnection, r2d2};
use log::{info, debug};
use serde_json::json;
use utoipa::OpenApi;
use std::net::TcpListener;
use utoipa_swagger_ui::SwaggerUi;

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
	let openapi = crate::api_doc::ApiDoc::openapi();
	debug!("{}", openapi.to_yaml().unwrap());

	let server = HttpServer::new(move || {
			App::new()
			.app_data(web::Data::new(pool.clone()))
			.app_data(
				web::JsonConfig::default().error_handler(json_decoding_error_handler)
			)
			.configure(crate::routes::ping_controller::build_ping_controller)
			.configure(crate::routes::user_controller::build_user_controller)
			.configure(crate::routes::transaction_controller::build_transaction_controller)
			.service(
				SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
			)
		})
		.listen(listener)?
		.run();

	info!("✔️  Server started successfully");

	Ok(server)
}