use server::{models::*};

pub mod test_context;
use test_context::*;

#[tokio::test]
async fn get_transactions_returns_200() {
	let _ctx = TestContext::new();

	let user = TestContext::create_user(&_ctx, "Ninja", 0, "member").await;
	TestContext::create_transaction(&_ctx, &user.id).await;

	let response = _ctx.client
		.get(format!("{}/users/{}/transactions", _ctx.address, user.id))
		.send()
		.await
		.expect("Failed to fetch transactions");

	let transactions = response
		.json::<Vec<Transaction>>()
		.await
		.expect("Failed to decode to Transaction model");

	assert_eq!(transactions.len(), 1);
}


#[tokio::test]
async fn get_transaction_returns_200_for_existing_transaction() {
	let _ctx = TestContext::new();

	let user = TestContext::create_user(&_ctx, "Ninja", 0, "member").await;
	assert_eq!(user.balance, 0);
	let transaction = TestContext::create_transaction(&_ctx, &user.id).await;

	let response = _ctx.client
		.get(format!("{}/transactions/{}", _ctx.address, transaction.id.expect("Create Transaction Failed")))
		.send()
		.await
		.expect("Failed to fetch transaction");

	let transactions = response
		.json::<Transaction>()
		.await
		.expect("Failed to decode to Transaction model");

	assert_eq!(transactions.mutation, 6942);
	assert_eq!(transactions.kind, "Deposit");
	assert_eq!(transactions.user_id, user.id);
}