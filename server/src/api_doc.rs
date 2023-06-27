use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
	paths(
		// Ping
		crate::routes::ping_controller::ping,

		// Users
		crate::routes::user_controller::create_user,
		crate::routes::user_controller::get_user,
		crate::routes::user_controller::get_all_users,

		// Transactions
		crate::routes::transaction_controller::get_user_transactions,
		crate::routes::transaction_controller::get_transaction,
		crate::routes::transaction_controller::deposit,
		crate::routes::transaction_controller::withdrawal,
		crate::routes::transaction_controller::transfer,
		crate::routes::transaction_controller::purchase,

	),
	components(
		schemas(
			// User
			crate::schemas::UserCreateRequest,
			crate::schemas::UserLookupIDRequest,
			crate::schemas::UserLookupNameRequest,

			crate::models::User,

			// Transaction
			crate::schemas::TransactionLookupRequest,
			crate::schemas::SoloTransaction,
			crate::schemas::DuoTransaction
		)
	)
)]
pub struct ApiDoc;