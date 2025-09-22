use chrono::prelude::*;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: DateTime<Utc>,
    pub transactions: Vec<String>,
    pub previous_hash: String,
    pub nonce: u64,
    pub hash: String,
}


use crate::consensus::pow;

impl Block {
    pub fn new(index: u64, transactions: Vec<String>, previous_hash: String, difficulty: usize) -> Self {
        let timestamp = Utc::now();
        let data = format!("{:?}{:?}{:?}{}", index, &transactions, &timestamp, &previous_hash);

        let (nonce, hash) = pow::mine_block(&data, difficulty);

        Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            nonce,
            hash,
        }
    }

    fn calculate_hash(data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_creation() {
        let block = Block::new(1, vec!["Tx".to_string()], "abc".to_string(), 2);
        assert_eq!(block.index, 1);
    }
}
