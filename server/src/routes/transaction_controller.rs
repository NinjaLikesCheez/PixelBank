use actix_web::{web, HttpResponse, error::{self}, http::{StatusCode}, get, post};
use diesel::{SqliteConnection, RunQueryDsl, r2d2};
use serde::{Deserialize};
use derive_more::{Display, Error};
use diesel::prelude::*;

use crate::{models::{Transaction, TransactionKind, User}};
use crate::models::APIError;

// TODO: move to database module
type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

#[derive(Deserialize)]
pub struct SoloTransaction {
	pub username: String,
	pub mutation: f32,
	pub role: String
}

#[derive(Deserialize)]
pub struct DuoTransaction {
	pub username: String,
	pub mutation: f32,
	pub role: String,
	pub recipient: String
}

#[derive(Debug, Display, Error)]
pub enum TransactionError {
	#[display(fmt = "No transactions found")]
	NoTransactions,
	#[display(fmt = "Transaction id is invalid")]
	BadTransactionId,
	#[display(fmt = "An internal error occurred. Please feed the maintainers")]
	InternalServerError, //TODO: Find a way to do general errors that are usable over more than a single controller
	#[display(fmt = "Mutation is not a float")]
	BadMutation,
	#[display(fmt = "Mutation has to be positive for this transaction type")]
	BadPositiveMutation,
	#[display(fmt = "Mutation has to be negative for this transaction type")]
	BadNegativeMutation
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
			Self::BadMutation => StatusCode::BAD_REQUEST,
			Self::BadPositiveMutation => StatusCode::BAD_REQUEST,
			Self::BadNegativeMutation => StatusCode::BAD_REQUEST
		}
	}
}

//Getting transactions

#[get("/users/{userId}/transactions")]
pub async fn get_user_transactions(path: web::Path<String>,  pool: web::Data<DbPool>) -> Result<HttpResponse, TransactionError> {
	let user_id = path.into_inner();

	let user_transactions: Vec<Transaction> = web::block(move || {
		use crate::schema::transactions::dsl::*;
		let mut connection = pool.get()
			.expect("Failed to get connection from pool");

		transactions
			.filter(account.eq(user_id))
			.load::<Transaction>(&mut connection)
			.expect("Error fetching transactions")
	})
	.await
	.map_err(|_e| TransactionError::NoTransactions)?;

	if user_transactions.is_empty() {
		return Err(TransactionError::NoTransactions);
	}

	Ok(HttpResponse::Ok().json(user_transactions))
}

#[get("/transactions/{transactionId}")]
pub async fn get_transaction(path: web::Path<String>,  pool: web::Data<DbPool>) -> Result<HttpResponse, TransactionError> {
	let transaction_id = path.into_inner().parse::<i32>().map_err(|_e| TransactionError::BadTransactionId)?;

	let user_transactions: Vec<Transaction> = web::block(move || {
		use crate::schema::transactions::dsl::*;
		let mut connection = pool.get()
			.map_err(|_e| TransactionError::NoTransactions)
			.expect("Failed to get connection from pool");

		transactions
			.filter(id.eq(transaction_id))
			.load::<Transaction>(&mut connection)
			.expect("Error fetching transaction")
	})
	.await
	.map_err(|_e| TransactionError::InternalServerError)?;

	if user_transactions.is_empty() {
		return Err(TransactionError::NoTransactions);
	}

	Ok(HttpResponse::Ok().json(user_transactions))
}

//Creating transactions
//TODO: Can we replace these with Macros?

#[post("/users/{userId}/transactions/deposit")]
pub async fn deposit(path: web::Path<String>, body: web::Json<SoloTransaction>, pool: web::Data<DbPool>) -> Result<HttpResponse, TransactionError> {
	let user_id = path.into_inner();
	let transaction = body.into_inner();
	
	if transaction.mutation <= 0.0 {
		return Err(TransactionError::BadNegativeMutation);
	}

	let new_transaction = Transaction::new(user_id, TransactionKind::Deposit, (transaction.mutation * 100.0) as i32, "".to_string());

	let inserted_transaction = execute_transaction(new_transaction, pool).await?;

	Ok(HttpResponse::Ok().json(inserted_transaction))
}

#[post("/users/{userId}/transactions/withdrawal")]
pub async fn withdrawal(path: web::Path<String>, body: web::Json<SoloTransaction>, pool: web::Data<DbPool>) -> Result<HttpResponse, TransactionError> {
	let user_id = path.into_inner();
	let transaction = body.into_inner();

	if transaction.mutation >= 0.0 {
		return Err(TransactionError::BadPositiveMutation);
	}

	let new_transaction = Transaction::new(user_id, TransactionKind::Withdrawal, (transaction.mutation * 100.0) as i32, "".to_string());

	let inserted_transaction = execute_transaction(new_transaction, pool).await?;

	Ok(HttpResponse::Ok().json(inserted_transaction))
}

#[post("/users/{userId}/transactions/transfer")]
pub async fn transfer(path: web::Path<String>, body: web::Json<DuoTransaction>, pool: web::Data<DbPool>) -> Result<HttpResponse, TransactionError> {
	let user_id = path.into_inner();
	let transaction = body.into_inner();

	if transaction.mutation >= 0.0 {
		return Err(TransactionError::BadPositiveMutation);
	}

	let new_transaction = Transaction::new(user_id, TransactionKind::Transfer, (transaction.mutation * 100.0) as i32, "".to_string());

	let inserted_transaction = execute_transaction(new_transaction, pool).await?;

	Ok(HttpResponse::Ok().json(inserted_transaction))
}

//More for completeness.
#[post("/users/{userId}/transactions/purchase")]
pub async fn purchase(path: web::Path<String>, body: web::Json<SoloTransaction>, pool: web::Data<DbPool>) -> Result<HttpResponse, TransactionError> {
	let user_id = path.into_inner();
	let transaction = body.into_inner();

	if transaction.mutation >= 0.0 {
		return Err(TransactionError::BadPositiveMutation);
	}

	let new_transaction = Transaction::new(user_id, TransactionKind::Purchase, (transaction.mutation * 100.0) as i32, "".to_string());

	let inserted_transaction = execute_transaction(new_transaction, pool).await?;

	Ok(HttpResponse::Ok().json(inserted_transaction))
}

//TODO: Move this to a database module, since you'll have to touch this for products sales too :)
async fn execute_transaction(transaction: Transaction, pool: web::Data<DbPool>) -> Result<Transaction, TransactionError> {
	//TODO: Find a way to map the blockingerrors properly. I want the user to know what went wrong.
	web::block(move || {
		use crate::schema::transactions::dsl::*;
		use crate::schema::users::dsl::*;
		let mut connection = pool.get()
			.expect("Failed to get connection from pool");

		let user = users
		.filter(username.eq(username))
		.first::<User>(&mut connection)
		.expect("Error fetching user");

		connection.transaction(|connection| {
			if transaction.kind == TransactionKind::Transfer.to_string() {
				let recipient_user = users
					.filter(username.eq(&transaction.recipient))
					.first::<User>(connection)
					.expect("Error fetching recipient");

				diesel::update(&recipient_user)
					.set(balance.eq(recipient_user.balance - transaction.mutation))
					.execute(connection)
					.expect("Error updating recipient balance");
			}

			diesel::insert_into(transactions)
				.values(&transaction)
				.execute(connection)
				.expect("Error inserting new transaction");

			diesel::update(&user)
				.set(balance.eq(user.balance + transaction.mutation))
				.execute(connection)
				.expect("Error updating user balance");

			transactions
				.order(crate::schema::transactions::dsl::id.desc())
				.first::<Transaction>(connection)
		}).expect("Error executing transaction")
	})
	.await
	.map_err(|_e| TransactionError::InternalServerError)
}