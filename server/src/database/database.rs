use chrono::prelude::*;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use crate::models::User;
use crate::schema::users::dsl::*;

pub type DBPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(Clone)]
pub struct Database {
	pool: DBPool
}

impl Database {
	pub fn new(path: String) -> Self {
		let manager = r2d2::ConnectionManager::<SqliteConnection>::new(path);
		let pool = r2d2::Pool::builder()
			.build(manager)
			.expect("Failed to load SQLite database");

		Database { pool }
	}

	pub fn get_all_users() -> Result<Vec<User>, Error>
}