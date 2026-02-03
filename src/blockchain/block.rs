use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: DateTime<Utc>,
    pub transactions: Vec<String>,
    pub previous_hash: String,
    pub difficulty: usize,
    pub nonce: u64,
    pub hash: String,
    pub validator: Option<String>,
    pub validator_public_key: Option<String>,
    pub validator_signature: Option<String>,
    pub staker_set_hash: Option<String>,
    pub staker_snapshot: Option<Vec<crate::consensus::pos::Staker>>,
}


use crate::consensus::pow;

impl Block {
    pub fn new(
        index: u64,
        transactions: Vec<String>,
        previous_hash: String,
        difficulty: usize,
        validator: Option<String>,
        validator_public_key: Option<String>,
        validator_signature: Option<String>,
        staker_set_hash: Option<String>,
        staker_snapshot: Option<Vec<crate::consensus::pos::Staker>>,
    ) -> Self {
        let timestamp = Utc::now();
        let data = Block::hash_payload(
            &index,
            &transactions,
            &timestamp,
            &validator,
            &validator_public_key,
            &staker_set_hash,
            &previous_hash,
        );

        let (nonce, hash) = pow::mine_block(&data, difficulty);

        Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            difficulty,
            nonce,
            hash,
            validator,
            validator_public_key,
            validator_signature,
            staker_set_hash,
            staker_snapshot,
        }
    }

    fn hash_payload(
        index: &u64,
        transactions: &Vec<String>,
        timestamp: &DateTime<Utc>,
        validator: &Option<String>,
        validator_public_key: &Option<String>,
        staker_set_hash: &Option<String>,
        previous_hash: &str,
    ) -> String {
        format!(
            "{:?}{:?}{:?}{:?}{:?}{:?}{}",
            index,
            transactions,
            timestamp,
            validator,
            validator_public_key,
            staker_set_hash,
            previous_hash
        )
    }

    pub fn calculate_hash(&self) -> String {
        let data = Block::hash_payload(
            &self.index,
            &self.transactions,
            &self.timestamp,
            &self.validator,
            &self.validator_public_key,
            &self.staker_set_hash,
            &self.previous_hash,
        );
        let candidate = format!("{}{}", data, self.nonce);
        let mut hasher = Sha256::new();
        hasher.update(candidate.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn has_valid_pow(&self) -> bool {
        self.hash == self.calculate_hash()
            && self.hash.starts_with(&"0".repeat(self.difficulty))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_creation() {
        let block = Block::new(
            1,
            vec!["Tx".to_string()],
            "abc".to_string(),
            2,
            Some("validator-1".to_string()),
            Some("validator-pubkey".to_string()),
            None,
            Some("staker-set-hash".to_string()),
            None,
        );
        assert_eq!(block.index, 1);
    }
}
