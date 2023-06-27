use actix_web::{HttpResponse, get};

/// Is the Pixelbank open for business?
#[utoipa::path(
	tag = "Ping",
	responses(
		(status = OK, description = "Pixelbank is responding"),
	)
)]
#[get("/ping")]
pub async fn ping() -> HttpResponse {
	HttpResponse::Ok().finish()
}

pub fn build_ping_controller(cfg: &mut actix_web::web::ServiceConfig) {
	cfg.service(ping);
}