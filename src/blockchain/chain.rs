use super::block::Block;
use crate::consensus::pos::{self, Staker};
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let genesis_block = Block::new(
            0,
            vec!["Genesis Block".to_string()],
            "0".to_string(),
            difficulty,
            None,
            None,
            None,
            None,
            None,
        );
        Blockchain {
            blocks: vec![genesis_block],
            difficulty,
        }
    }

    pub fn latest_hash(&self) -> String {
        self.blocks
            .last()
            .map(|block| block.hash.clone())
            .unwrap_or_else(|| "0".to_string())
    }

    pub fn add_block(
        &mut self,
        transactions: Vec<String>,
        validator: Option<String>,
        validator_public_key: Option<String>,
        validator_signature: Option<String>,
        staker_set_hash: Option<String>,
        staker_snapshot: Option<Vec<Staker>>,
    ) {
        let index = self.blocks.len() as u64;
        let previous_hash = self.latest_hash();
        let block = Block::new(
            index,
            transactions,
            previous_hash,
            self.difficulty,
            validator,
            validator_public_key,
            validator_signature,
            staker_set_hash,
            staker_snapshot,
        );
        self.blocks.push(block);
    }

    pub fn create_block(
        &self,
        transactions: Vec<String>,
        validator: Option<String>,
        validator_public_key: Option<String>,
        staker_set_hash: Option<String>,
        staker_snapshot: Option<Vec<Staker>>,
    ) -> Block {
        let index = self.blocks.len() as u64;
        let previous_hash = self.latest_hash();
        Block::new(
            index,
            transactions,
            previous_hash,
            self.difficulty,
            validator,
            validator_public_key,
            None,
            staker_set_hash,
            staker_snapshot,
        )
    }

    pub fn add_mined_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn validate_block_candidate(&self, block: &Block) -> Result<(), String> {
        let expected_index = self.blocks.len() as u64;
        if block.index != expected_index {
            return Err("Block index does not match chain tip".to_string());
        }

        if block.previous_hash != self.latest_hash() {
            return Err("Block previous hash does not match chain tip".to_string());
        }

        if block.validator.is_none()
            || block.validator_public_key.is_none()
            || block.validator_signature.is_none()
            || block.staker_snapshot.is_none()
            || block.staker_set_hash.is_none()
        {
            return Err("Block missing required validator or staker data".to_string());
        }

        let staker_snapshot = block.staker_snapshot.as_ref().unwrap();
        let staker_hash = pos::staker_set_hash(staker_snapshot);
        if Some(staker_hash) != block.staker_set_hash {
            return Err("Block staker set hash mismatch".to_string());
        }

        let expected_validator =
            pos::select_staker_with_seed(&block.previous_hash, staker_snapshot);
        if expected_validator != block.validator {
            return Err("Block validator does not match PoS selection".to_string());
        }

        let public_key = block.validator_public_key.as_ref().unwrap();
        let signature = block.validator_signature.as_ref().unwrap();
        if !pos::verify_block_signature(&block.hash, public_key, signature) {
            return Err("Block signature verification failed".to_string());
        }

        if !block.has_valid_pow() {
            return Err("Block PoW validation failed".to_string());
        }

        Ok(())
    }

    pub fn evaluate_slash_evidence(&self, block_index: u64) -> Result<SlashEvidence, String> {
        let index = block_index as usize;
        if index == 0 {
            return Err("Cannot slash genesis block".to_string());
        }
        if index >= self.blocks.len() {
            return Err("Block index out of range".to_string());
        }

        let block = &self.blocks[index];
        let previous = &self.blocks[index - 1];

        if block.previous_hash != previous.hash {
            return Ok(SlashEvidence {
                validator: block.validator.clone().unwrap_or_else(|| "unknown".to_string()),
                reason: "Previous hash mismatch".to_string(),
                timestamp: Utc::now().to_rfc3339(),
            });
        }

        let validator = block
            .validator
            .clone()
            .ok_or_else(|| "Block missing validator".to_string())?;
        let public_key = block
            .validator_public_key
            .as_ref()
            .ok_or_else(|| "Block missing validator public key".to_string())?;
        let signature = block
            .validator_signature
            .as_ref()
            .ok_or_else(|| "Block missing validator signature".to_string())?;

        let staker_snapshot = block
            .staker_snapshot
            .as_ref()
            .ok_or_else(|| "Block missing staker snapshot".to_string())?;
        let staker_hash = pos::staker_set_hash(staker_snapshot);
        if Some(staker_hash) != block.staker_set_hash {
            return Ok(SlashEvidence {
                validator,
                reason: "Staker set hash mismatch".to_string(),
                timestamp: Utc::now().to_rfc3339(),
            });
        }

        let expected_validator =
            pos::select_staker_with_seed(&block.previous_hash, staker_snapshot);
        if expected_validator.as_ref() != Some(&validator) {
            return Ok(SlashEvidence {
                validator,
                reason: "Validator does not match PoS selection".to_string(),
                timestamp: Utc::now().to_rfc3339(),
            });
        }

        if !pos::verify_block_signature(&block.hash, public_key, signature) {
            return Ok(SlashEvidence {
                validator,
                reason: "Invalid block signature".to_string(),
                timestamp: Utc::now().to_rfc3339(),
            });
        }

        if !block.has_valid_pow() {
            return Ok(SlashEvidence {
                validator,
                reason: "Invalid PoW".to_string(),
                timestamp: Utc::now().to_rfc3339(),
            });
        }

        Err("Block does not contain slashable behavior".to_string())
    }

    pub fn is_valid(&self) -> bool {
        if let Some(genesis) = self.blocks.first() {
            if !genesis.has_valid_pow() {
                return false;
            }
        }

        for i in 1..self.blocks.len() {
            let current = &self.blocks[i];
            let previous = &self.blocks[i - 1];

            if current.previous_hash != previous.hash {
                return false;
            }

            if current.validator.is_none()
                || current.validator_public_key.is_none()
                || current.validator_signature.is_none()
                || current.staker_snapshot.is_none()
                || current.staker_set_hash.is_none()
            {
                return false;
            }

            let staker_snapshot = current.staker_snapshot.as_ref().unwrap();
            let staker_hash = pos::staker_set_hash(staker_snapshot);
            if Some(staker_hash) != current.staker_set_hash {
                return false;
            }

            let expected_validator =
                pos::select_staker_with_seed(&current.previous_hash, staker_snapshot);
            if expected_validator != current.validator {
                return false;
            }

            let public_key = current.validator_public_key.as_ref().unwrap();
            let signature = current.validator_signature.as_ref().unwrap();
            if !pos::verify_block_signature(&current.hash, public_key, signature) {
                return false;
            }

            if !current.has_valid_pow() {
                return false;
            }
        }
        true
    }

    pub fn validate_and_slash(
        &self,
        stakers: &mut Vec<Staker>,
    ) -> (bool, Vec<(String, u64)>, Option<String>) {
        let mut slashed = Vec::new();

        if let Some(genesis) = self.blocks.first() {
            if !genesis.has_valid_pow() {
                return (false, slashed, Some("Genesis block failed PoW validation".to_string()));
            }
        }

        for i in 1..self.blocks.len() {
            let current = &self.blocks[i];
            let previous = &self.blocks[i - 1];

            if current.previous_hash != previous.hash {
                return (false, slashed, Some(format!("Block {} has invalid previous hash", i)));
            }

            let validator = match &current.validator {
                Some(value) => value,
                None => {
                    return (false, slashed, Some(format!("Block {} missing validator", i)));
                }
            };

            let public_key = match &current.validator_public_key {
                Some(value) => value,
                None => {
                    return (
                        false,
                        slashed,
                        Some(format!("Block {} missing validator public key", i)),
                    );
                }
            };

            let signature = match &current.validator_signature {
                Some(value) => value,
                None => {
                    return (
                        false,
                        slashed,
                        Some(format!("Block {} missing validator signature", i)),
                    );
                }
            };

            let staker_snapshot = match &current.staker_snapshot {
                Some(value) => value,
                None => {
                    return (
                        false,
                        slashed,
                        Some(format!("Block {} missing staker snapshot", i)),
                    );
                }
            };

            let staker_hash = pos::staker_set_hash(staker_snapshot);
            if Some(staker_hash) != current.staker_set_hash {
                return (
                    false,
                    slashed,
                    Some(format!("Block {} has invalid staker set hash", i)),
                );
            }

            let expected_validator =
                pos::select_staker_with_seed(&current.previous_hash, staker_snapshot);
            if expected_validator.as_ref() != Some(validator) {
                let amount = pos::slash_staker(stakers, validator);
                if amount > 0 {
                    slashed.push((validator.clone(), amount));
                }
                return (
                    false,
                    slashed,
                    Some(format!("Block {} validator does not match PoS selection", i)),
                );
            }

            if !pos::verify_block_signature(&current.hash, public_key, signature) {
                let amount = pos::slash_staker(stakers, validator);
                if amount > 0 {
                    slashed.push((validator.clone(), amount));
                }
                return (
                    false,
                    slashed,
                    Some(format!("Block {} failed signature verification", i)),
                );
            }

            if !current.has_valid_pow() {
                let amount = pos::slash_staker(stakers, validator);
                if amount > 0 {
                    slashed.push((validator.clone(), amount));
                }
                return (false, slashed, Some(format!("Block {} failed PoW validation", i)));
            }
        }

        (true, slashed, None)
    }
}

impl Default for Blockchain {
    fn default() -> Self {
        Blockchain::new(2)
    }
}

pub struct SlashEvidence {
    pub validator: String,
    pub reason: String,
    pub timestamp: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use secp256k1::{PublicKey, Secp256k1, SecretKey};
    
    fn test_keys() -> (String, String) {
        let secret_bytes = [1u8; 32];
        let secret_key = SecretKey::from_slice(&secret_bytes).unwrap();
        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        (
            hex::encode(public_key.serialize_uncompressed()),
            hex::encode(secret_key.secret_bytes()),
        )
    }

    #[test]
    fn test_blockchain_addition() {
        let mut chain = Blockchain::default();
        assert_eq!(chain.blocks.len(), 1); // Only genesis block

        let (public_key, private_key) = test_keys();
        let stakers = vec![Staker {
            address: "validator-1".to_string(),
            stake: 10,
            public_key: Some(public_key.clone()),
            private_key: Some(private_key.clone()),
        }];
        let staker_hash = pos::staker_set_hash(&stakers);
        let mut block = chain.create_block(
            vec!["Tx1".to_string()],
            Some("validator-1".to_string()),
            Some(public_key),
            Some(staker_hash),
            Some(stakers),
        );
        let signature = pos::sign_block_hash(&block.hash, &private_key).unwrap();
        block.validator_signature = Some(signature);
        chain.add_mined_block(block);
        assert_eq!(chain.blocks.len(), 2);
    }

    #[test]
    fn test_chain_validation() {
        let mut chain = Blockchain::default();
        let (public_key, private_key) = test_keys();
        let stakers = vec![Staker {
            address: "validator-1".to_string(),
            stake: 10,
            public_key: Some(public_key.clone()),
            private_key: Some(private_key.clone()),
        }];
        let staker_hash = pos::staker_set_hash(&stakers);
        let mut block = chain.create_block(
            vec!["Tx1".to_string()],
            Some("validator-1".to_string()),
            Some(public_key),
            Some(staker_hash),
            Some(stakers),
        );
        let signature = pos::sign_block_hash(&block.hash, &private_key).unwrap();
        block.validator_signature = Some(signature);
        chain.add_mined_block(block);
        assert!(chain.is_valid());
    }
}
