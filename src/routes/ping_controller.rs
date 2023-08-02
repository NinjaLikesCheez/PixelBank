use actix_web::{HttpResponse, get};

#[get("/ping")]
pub async fn ping() -> HttpResponse {
	HttpResponse::Ok().finish()
}

pub fn build_ping_controller(cfg: &mut actix_web::web::ServiceConfig) {
	cfg.service(ping);
}