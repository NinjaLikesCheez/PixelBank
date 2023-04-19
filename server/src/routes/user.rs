use actix_web::{web, HttpResponse, error::{self}, http::{StatusCode}};
use diesel::{SqliteConnection, RunQueryDsl, r2d2};
use serde::{Deserialize};
use derive_more::{Display, Error};
use diesel::prelude::*;

use crate::{models::User, routes::user};
use crate::models::APIError;

// TODO: move to database module
type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

#[derive(Deserialize)]
pub struct UserData {
	pub username: String,
	pub balance: i32,
	pub role: String
}

#[derive(Debug, Display, Error)]
pub enum UserError {
	#[display(fmt = "User already exists")]
	UserExistsError,

	#[display(fmt = "Admin accounts cannot be created via API")]
	CantCreateAdminError,

	#[display(fmt = "New user balance cannot be below zero")]
	NewUserBalanceTooLow,

	#[display(fmt = "User doesn't exist")]
	NoSuchUser,

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

			UserError::NoSuchUser => StatusCode::NOT_FOUND,
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

	use crate::schema::users::dsl::*;

	let user = web::block(move || {
		let mut connection = pool.get()
			.expect("Failed to get connection from pool");

		let user = User::new(body.username.clone(), body.balance, body.role.clone());

		diesel::insert_into(users)
			.values(&user)
			.execute(&mut connection)
			.expect("Couldn't insert user");

		users
			.filter(username.eq(body.username.clone()))
			.first::<User>(&mut connection)
			.expect("Error fetching user")
	})
	.await
	.map_err(|_e| UserError::UserExistsError)?;

	Ok(HttpResponse::Ok().json(user))
}

pub async fn get_user(path: web::Path<String>,  pool: web::Data<DbPool>) -> Result<HttpResponse, UserError> {
	let username = path.into_inner();

	println!("username: {}", username);

	let user = web::block(move || {
		use crate::schema::users::dsl::*;
		let mut connection = pool.get()
			.expect("Failed to get connection from pool");

		users
			.filter(username.eq(username.clone()))
			.first::<User>(&mut connection)
			.expect("Error fetching user")
	})
	.await
	.map_err(|_e| UserError::NoSuchUser)?;

	Ok(HttpResponse::Ok().json(user))
}

pub async fn get_all_users(pool: web::Data<DbPool>) -> Result<HttpResponse, UserError> {
	let users = web::block(move || {
		use crate::schema::users::dsl::*;
		let mut connection = pool.get()
			.expect("Failed to get connection from pool");

		users
			.load::<User>(&mut connection)
			.expect("Failed to load all users")
	})
	.await
	.map_err(|_e| UserError::NoSuchUser)?;

	Ok(HttpResponse::Ok().json(users))
}