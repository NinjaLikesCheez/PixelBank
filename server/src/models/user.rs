use diesel::prelude::*;

#[derive(Queryable)]
pub struct User {
	pub id: String,
	pub username: String,
	pub created_at: String,
	pub balance: i32,
	pub role: i32
}