use serde_json::json;
use server::{models::*};

pub mod test_context;
use test_context::*;

#[tokio::test]
async fn create_deposit_returns_transaction() {
	let _ctx = TestContext::new();

	let user = TestContext::create_user(&_ctx, "Ninja", 0, "member").await;

    let mutation = 42.50;
    let mutation_in_cents = (mutation * 100.0) as i32;

    let json = json!({
        "mutation": mutation
    });
	
	let response = _ctx.client
			.post(&format!("{}/users/{}/transactions/deposit", _ctx.address, user.id))
			.json(&json)
			.send()
			.await
			.expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());

    let response_json = response
    .json::<Transaction>()
    .await
    .expect("Failed to decode to Transaction model");

    assert_eq!(response_json.mutation, mutation_in_cents, "Mutation is {}, not {}", response_json.mutation, mutation_in_cents); //Looks like currently fetched transactions are in cents. Should probably change that up.
    assert_eq!(response_json.kind, "Deposit", "Transaction type is {}, not Deposit", response_json.kind);

    let updated_user = TestContext::get_user(&_ctx, &user.id).await;
    assert_eq!(updated_user.balance, user.balance + mutation_in_cents, "User balance is {}, not {}", updated_user.balance, user.balance + mutation_in_cents); //here too
}

#[tokio::test]
async fn create_withdrawal_returns_transaction() {
	let _ctx = TestContext::new();

	let user = TestContext::create_user(&_ctx, "Ninja", 1000, "member").await;

    let mutation = -5.0;
    let mutation_in_cents = (mutation * 100.0) as i32;

    let json = json!({
        "mutation": mutation
    });
	
	let response = _ctx.client
			.post(&format!("{}/users/{}/transactions/withdrawal", _ctx.address, user.id))
			.json(&json)
			.send()
			.await
			.expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());

    let response_json = response
    .json::<Transaction>()
    .await
    .expect("Failed to decode to Transaction model");

    assert_eq!(response_json.mutation, mutation_in_cents, "Mutation is {}, not {}", response_json.mutation, mutation_in_cents); //Looks like currently fetched transactions are in cents. Should probably change that up.
    assert_eq!(response_json.kind, "Withdrawal", "Transaction type is {}, not Withdrawal", response_json.kind);

    let updated_user = TestContext::get_user(&_ctx, &user.id).await;
    assert_eq!(updated_user.balance, user.balance + mutation_in_cents, "User balance is {}, not {}", updated_user.balance, user.balance + mutation_in_cents); //here too
}

#[tokio::test]
async fn create_purchase_returns_transaction() {
	let _ctx = TestContext::new();

	let user = TestContext::create_user(&_ctx, "Ninja", 1000, "member").await;

    let mutation = -5.0;
    let mutation_in_cents = (mutation * 100.0) as i32;

    let json = json!({
        "mutation": mutation
    });
	
	let response = _ctx.client
			.post(&format!("{}/users/{}/transactions/purchase", _ctx.address, user.id))
			.json(&json)
			.send()
			.await
			.expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());

    let response_json = response
    .json::<Transaction>()
    .await
    .expect("Failed to decode to Transaction model");

    assert_eq!(response_json.mutation, mutation_in_cents, "Mutation is {}, not {}", response_json.mutation, mutation_in_cents); //Looks like currently fetched transactions are in cents. Should probably change that up.
    assert_eq!(response_json.kind, "Purchase", "Transaction type is {}, not Purchase", response_json.kind);

    let updated_user = TestContext::get_user(&_ctx, &user.id).await;
    assert_eq!(updated_user.balance, user.balance + mutation_in_cents, "User balance is {}, not {}", updated_user.balance, user.balance + mutation_in_cents); //here too
}

#[tokio::test]
async fn create_transfer_returns_transaction() {
	let _ctx = TestContext::new();

	let user = TestContext::create_user(&_ctx, "Ninja", 1000, "member").await;
    let recipient = TestContext::create_user(&_ctx, "Aiko", 1000, "member").await;

    let mutation = -5.0;
    let mutation_in_cents = (mutation * 100.0) as i32;

    let json = json!({
        "mutation": mutation,
        "recipient": recipient.id
    });
	
	let response = _ctx.client
			.post(&format!("{}/users/{}/transactions/transfer", _ctx.address, user.id))
			.json(&json)
			.send()
			.await
			.expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());

    let response_json = response
    .json::<Transaction>()
    .await
    .expect("Failed to decode to Transaction model");

    assert_eq!(response_json.mutation, mutation_in_cents, "Mutation is {}, not {}", response_json.mutation, mutation_in_cents); //Looks like currently fetched transactions are in cents. Should probably change that up.
    assert_eq!(response_json.kind, "Transfer", "Transaction type is {}, not Transfer", response_json.kind);

    let updated_user = TestContext::get_user(&_ctx, &user.id).await;
    assert_eq!(updated_user.balance, user.balance + mutation_in_cents, "User balance is {}, not {}", user.balance, user.balance + mutation_in_cents); //here too

    let updated_recipient = TestContext::get_user(&_ctx, &recipient.id).await;
    assert_eq!(updated_recipient.balance, recipient.balance - mutation_in_cents, "Recipient balance is {}, not {}", updated_recipient.balance, recipient.balance - mutation_in_cents); //here too

}