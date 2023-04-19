use actix_web::{web, HttpResponse, error};
use diesel::{QueryDsl, SqliteConnection, RunQueryDsl, r2d2};
use serde_json::json;

use crate::models::User;

// TODO: move to database module
type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

#[derive(serde::Deserialize)]
pub struct UserData {
	pub username: String,
	pub balance: i32,
	pub role: String
}

async fn user_exists(username: &String) -> bool {
	username.is_empty()
}

pub async fn create_user(body: web::Json<UserData>, pool: web::Data<DbPool>) -> HttpResponse {
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

	let user = User::new(body.username.clone(), body.balance, body.role.clone());

	use crate::schema::users::dsl::*;

	let rows_inserted = web::block(move || {
		let mut connection = pool.get()
			.expect("Failed to get connection from pool");

		diesel::insert_into(users)
			.values(&user)
			.execute(&mut connection)
			.expect("Couldn't insert user")
	})
	.await;

	match rows_inserted {
		Ok(rows) => HttpResponse::Ok().json(format!("{:?}", rows)),
		Err(err) => HttpResponse::Conflict().json(json!({"error": err.to_string()})),
	}
}
