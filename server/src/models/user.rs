use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;
use serde::Serialize;

use crate::schema::users;

#[derive(Serialize, Queryable, Insertable)]
pub struct User {
	pub id: String,
	pub created_at: String,
	pub username: String,
	pub balance: i32,
	pub role: String
}

impl User {
		pub fn new(username: String, balance: i32, role: String) -> Self {
			User {
				id: Uuid::new_v4().to_string(),
				created_at: Utc::now().to_rfc3339(),
				username,
				balance,
				role
			}
		}
}