use server::{models::*};

pub mod test_context;
use test_context::*;

#[tokio::test]
async fn get_transaction_returns_200_for_existing_transaction() {
	let _ctx = TestContext::new();

	let user = TestContext::create_user(&_ctx, "Ninja", 0, "member").await;
	TestContext::create_transaction(&_ctx, &user.id).await;

	let response = _ctx.client
		.get(format!("{}/users/{}/transactions", _ctx.address, user.id))
		.send()
		.await
		.expect("Failed to fetch transactions");

	let transactions = response
		.json::<Vec<User>>()
		.await
		.expect("Failed to decode to Transaction model");

	assert_eq!(transactions.len(), 1);
}