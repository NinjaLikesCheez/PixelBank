use chrono::Utc;
use diesel::{Queryable, Insertable, expression::ValidGrouping, Selectable};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::schema::transactions;

#[derive(PartialEq, Clone, Copy)]
pub enum TransactionKind {
    Deposit,
    Withdrawal,
    Transfer,
    Purchase,
}

impl fmt::Display for TransactionKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TransactionKind::Deposit => write!(f, "Deposit"),
            TransactionKind::Withdrawal => write!(f, "Withdrawal"),
            TransactionKind::Transfer => write!(f, "Transfer"),
            TransactionKind::Purchase => write!(f, "Purchase"),
        }
    }
}

//NOTE: Mutations are in cents and reflect the effect of the transaction on the account balance. This is done so with a simple sql query we can check the current balance of an account by summing all mutations for that account.
#[derive(Serialize, Deserialize, Queryable, Insertable, ValidGrouping, Selectable)]
pub struct Transaction {
    pub id: i32,
	pub user_id: String,
	pub created_at: String,
    pub kind: String,
	pub mutation: i32,
    pub recipient_id: String
}

impl Transaction {
	pub fn new(user_id: String, transaction_kind: TransactionKind, mutation: i32, recipient_id: String) -> Self {
		Self {
            id: 0,
            user_id,
	    	created_at: Utc::now().to_rfc3339(),
            kind: transaction_kind.to_string(),
			mutation,
            recipient_id
		}
	}
}

