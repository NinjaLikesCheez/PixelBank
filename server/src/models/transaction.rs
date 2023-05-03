use chrono::Utc;
use serde::{Deserialize};
use std::fmt;

pub enum TransactionKind {
    Deposit,
    Withdrawal,
    Transfer,
    Purchase,
}

impl fmt::Display for TransactionKind {
    use std::fmt;

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

impl fmt::Debug for TransactionKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TransactionKind::Deposit => write!(f, "Deposit"),
            TransactionKind::Withdrawal => write!(f, "Withdrawal"),
            TransactionKind::Transfer => write!(f, "Transfer"),
            TransactionKind::Purchase => write!(f, "Purchase"),
        }
    }
}
}

#[derive(Deserialize)]
pub struct Transaction {
    pub id: i32,
	pub account: String,
	pub created_at: String,
    pub kind: String,
	pub mutation: f32,
    pub recipient: String
}

impl Transaction {
	pub fn new(account: String, transaction_kind: TransactionKind, mutation: f32, recipient: String) -> Self {
		Self {
            id: 0,
            account,
	    	created_at: Utc::now().to_rfc3339(),
            kind: transaction_kind.to_string(),
			mutation,
            recipient
		}
	}
}

