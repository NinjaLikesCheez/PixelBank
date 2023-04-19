use actix_web::{web, HttpResponse, error::{self}, http::{StatusCode}, Responder};
use diesel::{SqliteConnection, RunQueryDsl, r2d2};
use serde::Serialize;
use derive_more::{Display, Error};

use crate::models::User;

// TODO: move to database module
type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

#[derive(serde::Deserialize)]
pub struct UserData {
	pub username: String,
	pub balance: i32,
	pub role: String
}

#[derive(Serialize)]
struct APIError {
	error: String
}

#[derive(Debug, Display, Error)]
pub enum UserError {
	#[display(fmt = "User already exists")]
	UserExistsError,

	#[display(fmt = "Admin accounts cannot be created via API")]
	CantCreateAdminError,

	#[display(fmt = "New user balance cannot be below zero")]
	NewUserBalanceTooLow,

	#[display(fmt = "An internal error occurred. Please feed the maintainers")]
	InternalError
}

impl error::ResponseError for UserError {
	fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
			HttpResponse::build(self.status_code())
				.json(APIError { error: self.to_string() })
	}

	fn status_code(&self) -> StatusCode {
		match *self {
			UserError::InternalError => StatusCode::BAD_REQUEST,
			UserError::NewUserBalanceTooLow => StatusCode::BAD_REQUEST,

			UserError::UserExistsError => StatusCode::CONFLICT,
			UserError::CantCreateAdminError => StatusCode::FORBIDDEN,
		}
	}
}

pub async fn create_user(body: web::Json<UserData>, pool: web::Data<DbPool>) -> Result<HttpResponse, UserError> {
	// Admin accounts cannot be created via the REST API
	if body.role == "admin" {
		return Err(UserError::CantCreateAdminError);
	}

	// Don't owe us before you're part of us
	if body.balance < 0 {
		return Err(UserError::NewUserBalanceTooLow);
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
	.await
	.map_err(|_e| UserError::UserExistsError)?;

	// TODO: Ensure user was stored by returning the user from the database and returning it - Ok(web::Json(user))
	Ok(HttpResponse::Ok().finish())
}
