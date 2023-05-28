use actix_web::{HttpResponse, get};

#[get("/ping")]
pub async fn ping() -> HttpResponse {
	HttpResponse::Ok().finish()
}