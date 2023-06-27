use actix_web::{web, HttpResponse, error::{self}, http::{StatusCode}, get, post};
use diesel::{SqliteConnection, RunQueryDsl, r2d2};
use log::debug;
use derive_more::{Display, Error};
use diesel::prelude::*;

use crate::{
	models::{Transaction, TransactionKind, User},
	schemas::{UserLookupIDRequest, TransactionLookupRequest, SoloTransaction, DuoTransaction}
};
use crate::models::APIError;

// TODO: move to database module
type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

#[derive(Debug, Display, Error)]
pub enum TransactionError {
	#[display(fmt = "No transactions found")]
	NoTransactions,
	#[display(fmt = "Transaction id is invalid")]
	BadTransactionId,
	#[display(fmt = "An internal error occurred. Please feed the maintainers")]
	InternalServerError, //TODO: Find a way to do general errors that are usable over more than a single controller
	#[display(fmt = "Mutation is larger than {}", i32::MAX)]
	MutationTooLarge
}

impl error::ResponseError for TransactionError {
	fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
			HttpResponse::build(self.status_code())
				.json(APIError { error: self.to_string() })
	}

	fn status_code(&self) -> StatusCode {
		match *self {
			Self::NoTransactions => StatusCode::NOT_FOUND,
			Self::BadTransactionId => StatusCode::BAD_REQUEST,
			Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
			Self::MutationTooLarge => StatusCode::BAD_REQUEST,
		}
	}
}

pub fn build_transaction_controller(cfg: &mut actix_web::web::ServiceConfig) {
	cfg.service(get_user_transactions);
	cfg.service(get_transaction);
	cfg.service(deposit);
	cfg.service(withdrawal);
	cfg.service(transfer);
	cfg.service(purchase);
}

/// Get a User's Transactions
#[utoipa::path(
	tag = "Transaction",
	params(
		UserLookupIDRequest
	),
	responses(
			(status = OK, description = "A User with the given ID was found, and that user had transactions", body = Vec<Transaction>),
			(status = NOT_FOUND, description = "A User with this ID was not found, or they didn't have any transactions"),
	)
)]
#[get("/users/{userId}/transactions")]
pub async fn get_user_transactions(path: web::Path<UserLookupIDRequest>,  pool: web::Data<DbPool>) -> Result<HttpResponse, TransactionError> {
	let user_id_in: String = path.into_inner().id;

	let user_transactions: Vec<Transaction> = web::block(move || {
		use crate::schema::transactions::dsl::*;
		let mut connection = pool.get()
			.expect("Failed to get connection from pool");

		transactions
			.filter(user_id.eq(user_id_in))
			.load::<Transaction>(&mut connection)
			.expect("Error fetching transactions")
	})
	.await
	.map_err(|_e| TransactionError::NoTransactions)?;

	// TODO: return empty array? Meaning is the same
	if user_transactions.is_empty() {
		return Err(TransactionError::NoTransactions);
	}

	Ok(HttpResponse::Ok().json(user_transactions))
}

/// Get details of a specific Transaction
#[utoipa::path(
	tag = "Transaction",
	params(
		TransactionLookupRequest
	),
	responses(
			(status = OK, description = "Details of the specific Transaction", body = Transaction),
			(status = NOT_FOUND, description = "No transaction with this ID was found"),
	)
)]
#[get("/transactions/{transactionId}")]
pub async fn get_transaction(path: web::Path<TransactionLookupRequest>,  pool: web::Data<DbPool>) -> Result<HttpResponse, TransactionError> {
	let transaction_id = path.into_inner().id;

	let transaction: Transaction = web::block(move || {
		use crate::schema::transactions::dsl::*;
		let mut connection = pool.get()
			.map_err(|_e| TransactionError::NoTransactions) // TODO: change this to a different error...
			.expect("Failed to get connection from pool");

		transactions
			.filter(id.eq(transaction_id))
			.load::<Transaction>(&mut connection)
			.expect("Error fetching transaction")
	})
	.await
	.map_err(|_e| TransactionError::InternalServerError)?
	.pop()
	.ok_or(TransactionError::NoTransactions)?;

	Ok(HttpResponse::Ok().json(transaction))
}

//TODO: Can we replace these with Macros?
/// Create a Deposit Transaction
#[utoipa::path(
	tag = "Transaction",
	params(
		UserLookupIDRequest
	),
	request_body(
		content = SoloTransaction,
		description = "Details of the Deposit Transaction"
	),
	responses(
			(status = OK, description = "Deposit Transaction was created", body = Transaction),
			(status = BAD_REQUEST, description = "Mutation was too large"),
			(status = INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
	)
)]
#[post("/users/{userId}/transactions/deposit")]
pub async fn deposit(path: web::Path<UserLookupIDRequest>, body: web::Json<SoloTransaction>, pool: web::Data<DbPool>) -> Result<HttpResponse, TransactionError> {
	let user_id = path.into_inner().id;
	let transaction = body.into_inner();

	if transaction.amount > i32::MAX as u32 {
		return Err(TransactionError::MutationTooLarge);
	}

	let new_transaction = Transaction::new(user_id, TransactionKind::Deposit, transaction.amount as i32, None);

	let inserted_transaction = execute_transaction(new_transaction, pool).await?;

	Ok(HttpResponse::Ok().json(inserted_transaction))
}

/// Create a Withdrawal Transaction
#[utoipa::path(
	tag = "Transaction",
	params(
		UserLookupIDRequest
	),
	request_body(
		content = SoloTransaction,
		description = "Details of the Withdrawal Transaction"
	),
	responses(
			(status = OK, description = "Withdrawal Transaction was created", body = Transaction),
			(status = BAD_REQUEST, description = "Mutation was too large"),
			(status = INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
	)
)]
#[post("/users/{userId}/transactions/withdrawal")]
pub async fn withdrawal(path: web::Path<String>, body: web::Json<SoloTransaction>, pool: web::Data<DbPool>) -> Result<HttpResponse, TransactionError> {
	let user_id = path.into_inner();
	let transaction = body.into_inner();

	if transaction.amount > i32::MAX as u32 {
		return Err(TransactionError::MutationTooLarge);
	}

	let new_transaction = Transaction::new(user_id, TransactionKind::Withdrawal, -(transaction.amount as i32), None);

	let inserted_transaction = execute_transaction(new_transaction, pool).await?;

	Ok(HttpResponse::Ok().json(inserted_transaction))
}

/// Create a Transfer Transaction
#[utoipa::path(
	tag = "Transaction",
	params(
		UserLookupIDRequest
	),
	request_body(
		content = DuoTransaction,
		description = "Details of the Transfer Transaction"
	),
	responses(
			(status = OK, description = "Transfer Transaction was created", body = Transaction),
			(status = BAD_REQUEST, description = "Mutation was too large"),
			(status = INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
	)
)]
#[post("/users/{userId}/transactions/transfer")]
pub async fn transfer(path: web::Path<String>, body: web::Json<DuoTransaction>, pool: web::Data<DbPool>) -> Result<HttpResponse, TransactionError> {
	let user_id = path.into_inner();
	let transaction = body.into_inner();

	if transaction.amount > i32::MAX as u32 {
		return Err(TransactionError::MutationTooLarge);
	}

	let new_transaction = Transaction::new(user_id, TransactionKind::Transfer, -(transaction.amount as i32), Some(transaction.recipient));

	let inserted_transaction = execute_transaction(new_transaction, pool).await?;

	Ok(HttpResponse::Ok().json(inserted_transaction))
}

/// Create a Purchase Transaction
#[utoipa::path(
	tag = "Transaction",
	params(
		UserLookupIDRequest
	),
	request_body(
		content = SoloTransaction,
		description = "Details of the Transfer Transaction"
	),
	responses(
			(status = OK, description = "Purchase Transaction was created", body = Transaction),
			(status = BAD_REQUEST, description = "Mutation was too large"),
			(status = INTERNAL_SERVER_ERROR, description = "Internal Server Error"),
	)
)]
#[post("/users/{userId}/transactions/purchase")]
pub async fn purchase(path: web::Path<String>, body: web::Json<SoloTransaction>, pool: web::Data<DbPool>) -> Result<HttpResponse, TransactionError> {
	let user_id = path.into_inner();
	let transaction = body.into_inner();

	if transaction.amount > i32::MAX as u32 {
		return Err(TransactionError::MutationTooLarge);
	}

	let new_transaction = Transaction::new(user_id, TransactionKind::Purchase, -(transaction.amount as i32), None);

	let inserted_transaction = execute_transaction(new_transaction, pool).await?;

	Ok(HttpResponse::Ok().json(inserted_transaction))
}

// TODO: Move this to a database module, since you'll have to touch this for products sales too :)
async fn execute_transaction(transaction: Transaction, pool: web::Data<DbPool>) -> Result<Transaction, TransactionError> {
	// TODO: Find a way to map the blockingerrors properly. I want the user to know what went wrong.
	web::block(move || {
		use crate::schema::transactions::dsl::*;
		use crate::schema::users::dsl::*;
		let mut connection = pool.get()
			.expect("Failed to get connection from pool");

		let user = users
		.filter(username.eq(username))
		.first::<User>(&mut connection)
		.expect("Error fetching user");

		debug!("Found user: {} with balance of {}", user.id, user.balance);

		connection.transaction(|connection| {
			if transaction.kind == TransactionKind::Transfer.to_string() {
				let recipient_user = users
					.filter(crate::schema::users::id.eq(transaction.recipient_id.clone().expect("No recipient id provided")))
					.first::<User>(connection)
					.expect("Error fetching recipient");

				debug!("Found recipient: {} with balance of {}", recipient_user.id, recipient_user.balance);

				diesel::update(&recipient_user)
					.set(balance.eq(balance - transaction.mutation))
					.execute(connection)
					.expect("Error updating recipient balance");

				debug!("Updating recipient balance: {} + {} = {}", recipient_user.balance, -transaction.mutation, recipient_user.balance + -transaction.mutation);
			}

			diesel::update(&user)
				.set(balance.eq(balance + transaction.mutation))
				.execute(connection)
				.expect("Error updating user balance");

			debug!("Updating user balance: {} + {} = {}", user.balance, transaction.mutation, user.balance + transaction.mutation);

			diesel::insert_into(transactions)
				.values(&transaction)
				.execute(connection)
				.expect("Error inserting new transaction");

			transactions
				.order(crate::schema::transactions::dsl::id.desc())
				.first::<Transaction>(connection)
		}).expect("Error executing transaction")
	})
	.await
	.map_err(|_e| TransactionError::InternalServerError)
}