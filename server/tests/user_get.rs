use chrono::{Utc, DateTime};
use server::{models::*};

pub mod test_context;
use test_context::*;

#[tokio::test]
async fn get_user_returns_200_for_existing_user_by_username() {
	let _ctx = TestContext::new();

	TestContext::create_user(&_ctx, "Ninja", 0, "member").await;

	let response = _ctx.client
		.get(format!("{}/users/name/Ninja", _ctx.address))
		.send()
		.await
		.expect("Failed to fetch user");

	let user = response
		.json::<User>()
		.await
		.expect("Failed to decode to User model");

	assert_eq!("Ninja", user.username);
	assert_eq!(0, user.balance);
	assert_eq!("member", user.role);

	let created = DateTime::parse_from_rfc3339(&user.created_at)
		.expect("Failed to parse rfc3339 date string");

	let difference = created.signed_duration_since(Utc::now());

	assert_eq!(difference.num_seconds() < 5, true);
}

#[tokio::test]
async fn get_user_returns_200_for_existing_user() {
	let _ctx = TestContext::new();

	let user = TestContext::create_user(&_ctx, "Ninja", 0, "member").await;

	let response = _ctx.client
		.get(format!("{}/users/{}", _ctx.address, user.id))
		.send()
		.await
		.expect("Failed to fetch user");

	let user = response
		.json::<User>()
		.await
		.expect("Failed to decode to User model");

	assert_eq!("Ninja", user.username);
	assert_eq!(0, user.balance);
	assert_eq!("member", user.role);

	let created = DateTime::parse_from_rfc3339(&user.created_at)
		.expect("Failed to parse rfc3339 date string");

	let difference = created.signed_duration_since(Utc::now());

	assert_eq!(difference.num_seconds() < 5, true);
}


#[tokio::test]
async fn get_user_returns_404_for_nonexistent_user() {
	let _ctx = TestContext::new();

	let response = _ctx.client
		.get(format!("{}/users/ninja", _ctx.address))
		.send()
		.await
		.expect("Failed to fetch user")
		.json::<APIError>()
		.await
		.expect("Failed to decode API Error");

	assert_eq!(response.error, "User doesn't exist");
}

#[tokio::test]
async fn get_all_users_returns_200() {
	let _ctx = TestContext::new();

	TestContext::create_user(&_ctx, "Ninja", 100, "member").await;

	let response = _ctx.client
		.get(format!("{}/users", _ctx.address))
		.send()
		.await
		.expect("Failed to fetch user");

	assert_eq!(200, response.status());

	let users = response
		.json::<Vec<User>>()
		.await
		.expect("Failed to decode API Error");

	assert_eq!(users.len(), 1);
	let ninja_user = users.first().unwrap();
	assert_eq!(ninja_user.username, "Ninja");
}