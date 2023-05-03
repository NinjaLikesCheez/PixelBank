use actix_web::{web, HttpResponse, error::{self}, http::{StatusCode}};
use diesel::{SqliteConnection, RunQueryDsl, r2d2};
use serde::{Deserialize};
use derive_more::{Display, Error};
use diesel::prelude::*;

use crate::{models::Product};
use crate::models::APIError;

// TODO: move to database module
type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

#[derive(Debug, Display, Error)]
pub enum ProductError {
	#[display(fmt = "An internal error occurred. Please feed the maintainers")]
	InternalError
}

impl error::ResponseError for ProductError {
	fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
			HttpResponse::build(self.status_code())
				.json(APIError { error: self.to_string() })
	}

	fn status_code(&self) -> StatusCode {
		match *self {
			ProductError::InternalError => StatusCode::BAD_REQUEST,
		}
	}
}

// fn get_all_products(pool: web::Data<DbPool>) -> Result<HttpResponse, ProductError> {
// 	Ok()
// }