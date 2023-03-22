use std::str::FromStr;

use actix_web::{HttpResponse, web};
use actix_web::web::{Path, Json, Data};
use serde::{Deserialize, Serialize};
use uuid::{Uuid};
use chrono::{DateTime,Utc};
use diesel::{Queryable, Insertable, RunQueryDsl};
use diesel::result::Error;

use crate::{DBPool, DBPooledConnection};
use crate::response::Response;
use crate::constants::{APPLICATION_JSON, CONNECTION_POOL_ERROR};
use super::schema::{users};

pub type Users = Response<User>;

// Models
#[derive(Debug, Deserialize, Serialize)]
pub struct User {
	pub id: Uuid,
	pub username: String,
	pub created_at: DateTime<Utc>,
	pub balance: i32,
  pub role: UserRole
}

impl User {
	pub fn new(username: String, balance: i32, role: UserRole) -> Self {
		Self {
			id: Uuid::new_v4(),
			username,
			created_at: Utc::now(),
			balance,
			role,
		}
	}

	pub fn to_user_db(&self) -> UserDB {
		UserDB {
			id: self.id.to_string(),
			username: self.username.clone(),
			created_at: self.created_at.to_rfc3339(),
			balance: self.balance,
			role: self.role.to_i32(),
		}
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub enum UserRole {
	Admin,
	Member,
	Guest,
}

impl UserRole {
	fn from_i32(i: &i32) -> Result<Self, ()> {
		match i {
			0 => Ok(UserRole::Admin),
			1 => Ok(UserRole::Member),
			2 => Ok(UserRole::Guest),
			_ => Err(())
		}
	}

	fn to_i32(&self) -> i32 {
		match self {
			UserRole::Admin => 0,
			UserRole::Member => 1,
			UserRole::Guest => 2,
		}
	}
}

impl FromStr for UserRole {
	type Err = ();
	fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s {
				"Admin" => Ok(UserRole::Admin),
				"Member" => Ok(UserRole::Member),
				"Guest" => Ok(UserRole::Guest),
				_ => Err(())
			}
	}
}

// Database
#[derive(Queryable, Insertable)]
#[diesel(table_name = users)]
pub struct UserDB {
	pub id: String,
	pub username: String,
	pub created_at: String,
	pub balance: i32,
	pub role: i32
}

impl UserDB {
	fn to_user(&self) -> User {
		User {
			id: Uuid::parse_str(&self.id).expect("Invalid UUID String"),
			username: self.username.clone(),
			created_at: DateTime::parse_from_rfc3339(&self.created_at).expect("Invalid created_at date time string").into(),
			balance: self.balance,
			role: UserRole::from_i32(&self.role).expect("Invalid role")
		}
	}
}

// Request models
#[derive(Debug, Deserialize, Serialize)]
pub struct UserRequest {
	pub username: String,
	pub balance: i32,
	pub role: String,
}

impl UserRequest {
	pub fn to_user(&self) -> User {
		User::new(self.username, self.balance, UserRole::from_str(self.role.as_str()).expect("Invalid role"))
	}
}



// API

// Create new user
#[post("/user")]
pub async fn new_user(user_request: Json<UserRequest>, pool: Data<DBPool>) -> HttpResponse {
	let connection = pool.get().expect(CONNECTION_POOL_ERROR);

	let _user = web::block(move || create_user(user_request.to_user(), &connection)).await;

	// TODO: improve error
	match _user {
		Ok(_user) => HttpResponse::Created()
			.content_type(APPLICATION_JSON)
			.json(_user),
		_ => HttpResponse::NoContent().await.unwrap(),
	}
}

fn create_user(_user: User, connection: &DBPooledConnection) -> Result<User, Error> {
	use crate::schema::users::dsl::users;

	let user_db = _user.to_user_db();
	let _ = diesel::insert_into(users).values(&user_db).execute(connection);

	Ok(user_db.to_user())
}

#[get("/user/all")]
pub async fn all_users() -> HttpResponse {
	// TODO: impl
	let users = Users { results: vec![] };

	HttpResponse::Ok()
		.content_type(APPLICATION_JSON)
		.json(users)
}

#[get("/user/{id}")]
pub async fn user(path: Path<(String,)>) -> HttpResponse {
	// TODO: impl
	let found_user: Option<User> = Some(
		User::new("ninjalikescheez".to_string(), 1337, UserRole::Admin)
	);

	match found_user {
		Some(user) => HttpResponse::Ok()
			.content_type(APPLICATION_JSON)
			.json(user),
		None => HttpResponse::NotFound()
			.content_type(APPLICATION_JSON)
			.await
			.unwrap(),
	}
}