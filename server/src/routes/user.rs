use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct UserData {
	pub username: String,
	pub balance: i32,
	pub role: String
}

async fn user_exists(username: &String) -> bool {
	false
}

pub async fn create_user(body: web::Json<UserData>) -> HttpResponse {
	// Can't create a user that already exists
	if user_exists(&body.username).await {
		return HttpResponse::Conflict().finish();
	}

	// Admin accounts cannot be created via the REST API
	if body.role == "admin" {
		return HttpResponse::Forbidden().finish();
	}

	// Don't owe us before you're part of us
	if body.balance < 0 {
		return HttpResponse::Forbidden().finish();
	}

	// Request is fine, create user and store it
	HttpResponse::Ok().finish()
}
