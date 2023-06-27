use chrono::Utc;
use diesel::prelude::*;
use utoipa::ToSchema;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::schema::users;

#[derive(Serialize, Deserialize, Queryable, Insertable, Identifiable, Debug, ToSchema)]
pub struct User {
	pub id: String,
	pub created_at: String,
	pub username: String,
	pub balance: i32,
	pub role: String
}

impl User {
		pub fn new(username: String, balance: i32, role: String) -> Self {
			Self {
				id: Uuid::new_v4().to_string(),
				created_at: Utc::now().to_rfc3339(),
				username,
				balance,
				role
			}
		}
}