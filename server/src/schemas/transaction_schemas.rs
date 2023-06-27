use serde::Deserialize;
use utoipa::{ToSchema, IntoParams};

#[derive(Deserialize, ToSchema, IntoParams)]
pub struct TransactionLookupRequest {
	pub id: i32, // TODO: All IDs should be UUIDs... probably - privacy, enumeration attacks, or something idk
}

#[derive(Deserialize, ToSchema, IntoParams)]
pub struct SoloTransaction {
	pub amount: u32
}

#[derive(Deserialize, ToSchema, IntoParams)]
pub struct DuoTransaction {
	pub amount: u32,
	pub recipient: String
}
