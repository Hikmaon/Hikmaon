use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionType {
    Transfer,        // Transfer tokens
    Reward,          // PoS or PoW reward
    Certificate,     // Issue certificate
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub from: Option<String>,       // None for rewards
    pub to: String,
    pub amount: u64,
    pub transaction_type: TransactionType,
    pub timestamp: DateTime<Utc>,
}

impl Transaction {
    pub fn new(from: Option<String>, to: String, amount: u64, transaction_type: TransactionType) -> Self {
        Transaction {
            id: Uuid::new_v4().to_string(),
            from,
            to,
            amount,
            transaction_type,
            timestamp: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_creation() {
        let tx = Transaction::new(Some("Alice".to_string()), "Bob".to_string(), 100, TransactionType::Transfer);
        assert_eq!(tx.amount, 100);
        assert_eq!(tx.to, "Bob");
    }
}
