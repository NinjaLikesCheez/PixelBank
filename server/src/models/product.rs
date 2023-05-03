use diesel::prelude::*;
use serde::{Serialize, Deserialize};

use crate::schema::products;

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct Product {
	pub id: String,
	pub name: String,
	#[serde(default = "bool::default")]
	pub has_deposit: bool,
	pub price: i32,
}

impl Product {
	pub fn new(id: String, name: String, has_deposit: bool, price: i32) -> Self {
		Product { id, name, has_deposit, price}
	}
}