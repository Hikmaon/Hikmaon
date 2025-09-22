use sha2::{Digest, Sha256};

/// Tries different nonces to find a hash starting with `difficulty` zeroes.
pub fn mine_block(data: &str, difficulty: usize) -> (u64, String) {
    let mut nonce = 0;

    loop {
        let candidate = format!("{}{}", data, nonce);
        let mut hasher = Sha256::new();
        hasher.update(candidate.as_bytes());
        let hash = format!("{:x}", hasher.finalize());

        if hash.starts_with(&"0".repeat(difficulty)) {
            return (nonce, hash);
        }

        nonce += 1;
    }
}
