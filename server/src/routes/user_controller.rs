use core::panic;

use actix_web::{web, HttpResponse, error::{self}, http::{StatusCode}, post, get,};
use diesel::{SqliteConnection, RunQueryDsl, r2d2};
use derive_more::{Display, Error};
use diesel::prelude::*;
use utoipa::{ToSchema};

use crate::{
	models::{User, APIError},
	schemas::{UserCreateRequest, UserLookupIDRequest, UserLookupNameRequest}
};

// TODO: move to database module
type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

#[derive(Debug, Display, Error, ToSchema)]
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
			Self::InternalError => StatusCode::BAD_REQUEST,
			Self::NewUserBalanceTooLow => StatusCode::BAD_REQUEST,

			Self::NoSuchUser => StatusCode::NOT_FOUND,
			Self::UserExistsError => StatusCode::CONFLICT,
			Self::CantCreateAdminError => StatusCode::FORBIDDEN,
		}
	}
}

pub fn build_user_controller(cfg: &mut actix_web::web::ServiceConfig) {
	cfg.service(get_user);
	cfg.service(get_all_users);
	cfg.service(create_user);
	cfg.service(get_user_by_name);
}

/// Create a User
#[utoipa::path(
	tag = "User",
	request_body(
		content = UserCreateRequest,
		description = "\
		Details of the User to create\n\
		Note:\n\
		\n\
		* Users cannot be created with a negative balance\n\
		* Admin users cannot be created via the API",
	),
	responses(
			(status = OK, description = "Created User", body = User),
			(status = FORBIDDEN, description = "Can't create admin account"),
			(status = CONFLICT, description = "This user already exists"),
			(status = BAD_REQUEST, description = "New user's balance is too low")
	)
)]
#[post("/users")]
pub async fn create_user(body: web::Json<UserCreateRequest>, pool: web::Data<DbPool>) -> Result<HttpResponse, UserError> {
	use crate::schema::users::dsl::*;

	// Admin accounts cannot be created via the REST API
	if body.role == "admin" {
		return Err(UserError::CantCreateAdminError);
	}

	// Don't owe us before you're part of us
	if body.balance < 0 {
		return Err(UserError::NewUserBalanceTooLow);
	}

	let user = web::block(move || {
		let mut connection = pool.get()
			.expect("Failed to get connection from pool");

		let user = User::new(body.username.clone(), body.balance, body.role.clone());

		// Check is username already exists
		let count:i64 = users
			.filter(username.eq(body.username.clone()))
			.count()
			.get_result(&mut connection)
			.expect("Couldn't get count of users with username");

		if count > 0 {
			panic!("User already exists"); // You get to have fun with this when we move to real error handling
		}

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

/// Get a User by id
#[utoipa::path(
	tag = "User",
	params(
		UserLookupIDRequest
	),
	responses(
			(status = OK, description = "A User with the given ID was found", body = User),
			(status = NOT_FOUND, description = "A User with this ID does not exist"),
	)
)]
#[get("/users/{id}")]
pub async fn get_user(path: web::Path<UserLookupIDRequest>,  pool: web::Data<DbPool>) -> Result<HttpResponse, UserError> {
	let in_id = path.into_inner().id;

	let user = web::block(move || {
		use crate::schema::users::dsl::*;
		let mut connection = pool.get()
			.expect("Failed to get connection from pool");

		users
			.filter(id.eq(in_id))
			.first::<User>(&mut connection)
			.expect("Error fetching user")
	})
	.await
	.map_err(|_e| UserError::NoSuchUser)?;

	Ok(HttpResponse::Ok().json(user))
}

/// Get a User by name
#[utoipa::path(
	tag = "User",
	params(
		UserLookupNameRequest
	),
	responses(
			(status = OK, description = "A User with the given name was found", body = User),
			(status = NOT_FOUND, description = "A User with this name does not exist"),
	)
)]
#[get("/users/name/{username}")]
pub async fn get_user_by_name(path: web::Path<UserLookupNameRequest>,  pool: web::Data<DbPool>) -> Result<HttpResponse, UserError> {
	let in_username = path.into_inner().username;

	let user = web::block(move || {
		use crate::schema::users::dsl::*;
		let mut connection = pool.get()
			.expect("Failed to get connection from pool");

		users
			.filter(username.eq(in_username))
			.first::<User>(&mut connection)
			.expect("Error fetching user")
	})
	.await
	.map_err(|_e| UserError::NoSuchUser)?;

	Ok(HttpResponse::Ok().json(user))
}

/// Gets all Users
#[utoipa::path(
	tag = "User",
	responses(
			(status = OK, description = "A list of all Users", body = Vec::<User>),
			(status = NOT_FOUND, description = "No users exist"), // TODO: we should have an internal error for 'failed to connect to db'
	)
)]
#[get("/users")]
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