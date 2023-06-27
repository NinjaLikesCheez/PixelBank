use serde::Deserialize;
use utoipa::{ToSchema, IntoParams};

#[derive(Deserialize, ToSchema)]
pub struct UserCreateRequest {
	pub username: String,
	pub balance: i32,
	pub role: String
}

#[derive(Deserialize, ToSchema, IntoParams)]
pub struct UserLookupIDRequest {
	pub id: String
}

#[derive(Deserialize, ToSchema, IntoParams)]
pub struct UserLookupNameRequest {
	pub username: String
}
