pub mod test_context;
use test_context::*;

#[tokio::test]
async fn ping_succeeds() {
	let _ctx = TestContext::new();
	let client = reqwest::Client::new();

	let response = client
		.get(format!("{}/ping", _ctx.address))
		.send()
		.await
		.expect("Failed to execute ping");

	assert!(response.status().is_success());
	assert_eq!(Some(0), response.content_length());
}
