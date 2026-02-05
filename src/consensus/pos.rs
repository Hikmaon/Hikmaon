use rand::Rng;
use secp256k1::{ecdsa::Signature, Message, PublicKey, Secp256k1, SecretKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Staker {
    pub address: String,
    pub stake: u64,
    pub public_key: Option<String>,
    pub private_key: Option<String>,
}

const SLASH_PERCENT: u64 = 10;

pub fn staker_set_hash(stakers: &[Staker]) -> String {
    let mut hasher = Sha256::new();
    for staker in stakers {
        hasher.update(staker.address.as_bytes());
        hasher.update(staker.stake.to_be_bytes());
        if let Some(public_key) = &staker.public_key {
            hasher.update(public_key.as_bytes());
        }
    }
    format!("{:x}", hasher.finalize())
}

fn seed_to_u64(seed: &str) -> u64 {
    let digest = Sha256::digest(seed.as_bytes());
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&digest[..8]);
    u64::from_be_bytes(bytes)
}

pub fn select_staker_with_seed(seed: &str, stakers: &[Staker]) -> Option<String> {
    let total_stake: u64 = stakers.iter().map(|s| s.stake).sum();

    if total_stake == 0 {
        return None;
    }

    let selection_point = seed_to_u64(seed) % total_stake;
    let mut running_point = selection_point;

    for staker in stakers {
        if running_point < staker.stake {
            return Some(staker.address.clone());
        }
        running_point -= staker.stake;
    }

    None
}

pub fn select_staker(stakers: &[Staker]) -> Option<String> {
    let total_stake: u64 = stakers.iter().map(|s| s.stake).sum();

    if total_stake == 0 {
        return None;
    }

    let mut rng = rand::rng();
    let selection_point = rng.random_range(0..total_stake);
    let seed = format!("random-{}", selection_point);

    select_staker_with_seed(&seed, stakers)
}

pub fn sign_block_hash(block_hash: &str, private_key_hex: &str) -> Result<String, String> {
    let hash_bytes = hex::decode(block_hash).map_err(|err| err.to_string())?;
    let message = Message::from_digest_slice(&hash_bytes).map_err(|err| err.to_string())?;
    let secret_key_bytes = hex::decode(private_key_hex).map_err(|err| err.to_string())?;
    let secret_key = SecretKey::from_slice(&secret_key_bytes).map_err(|err| err.to_string())?;
    let secp = Secp256k1::new();
    let signature = secp.sign_ecdsa(&message, &secret_key);
    Ok(hex::encode(signature.serialize_compact()))
}

pub fn verify_block_signature(block_hash: &str, public_key_hex: &str, signature_hex: &str) -> bool {
    let hash_bytes = match hex::decode(block_hash) {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };
    let message = match Message::from_digest_slice(&hash_bytes) {
        Ok(msg) => msg,
        Err(_) => return false,
    };
    let signature_bytes = match hex::decode(signature_hex) {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };
    let signature = match Signature::from_compact(&signature_bytes) {
        Ok(sig) => sig,
        Err(_) => return false,
    };
    let public_key_bytes = match hex::decode(public_key_hex) {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };
    let public_key = match PublicKey::from_slice(&public_key_bytes) {
        Ok(key) => key,
        Err(_) => return false,
    };
    let secp = Secp256k1::new();
    secp.verify_ecdsa(&message, &signature, &public_key).is_ok()
}

pub fn slash_staker(stakers: &mut Vec<Staker>, address: &str) -> u64 {
    slash_staker_with_percent(stakers, address, SLASH_PERCENT)
}

pub fn slash_staker_with_percent(stakers: &mut Vec<Staker>, address: &str, percent: u64) -> u64 {
    for staker in stakers.iter_mut() {
        if staker.address == address {
            let slashed = staker.stake.saturating_mul(percent) / 100;
            staker.stake = staker.stake.saturating_sub(slashed);
            return slashed;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_staker() {
        let stakers = vec![
            Staker {
                address: "Alice".to_string(),
                stake: 100,
                public_key: None,
                private_key: None,
            },
            Staker {
                address: "Bob".to_string(),
                stake: 50,
                public_key: None,
                private_key: None,
            },
        ];

        let winner = select_staker_with_seed("seed", &stakers);
        assert!(winner.is_some());
    }
}
