use actix_web::{HttpResponse, get};

#[get("/ping")]
pub async fn ping() -> HttpResponse {
	HttpResponse::Ok().finish()
}

pub fn actix_config(cfg: &mut actix_web::web::ServiceConfig) {
	cfg.service(ping);
}