use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct APIError {
	pub error: String
}