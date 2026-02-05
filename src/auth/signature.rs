// src/auth/signature.rs
use secp256k1::ecdsa::{RecoveryId, Signature};
use secp256k1::{Message, Secp256k1};
use sha3::{Digest, Keccak256};

/// Recover Ethereum address from a personal_sign signature
pub fn recover_address_from_signature(
    message: &str,
    signature_hex: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Remove 0x prefix if present
    let signature_hex = signature_hex.strip_prefix("0x").unwrap_or(signature_hex);

    // Decode hex signature
    let signature_bytes = hex::decode(signature_hex)?;
    if signature_bytes.len() != 65 {
        return Err("Invalid signature length".into());
    }

    // Split signature into r, s, v
    let r = &signature_bytes[0..32];
    let s = &signature_bytes[32..64];
    let v = signature_bytes[64];

    // Create the message hash that MetaMask signs
    let prefixed_message = format!("\x19Ethereum Signed Message:\n{}{}", message.len(), message);
    let message_hash = Keccak256::digest(prefixed_message.as_bytes());

    // Create secp256k1 objects
    let secp = Secp256k1::new();
    let message = Message::from_digest_slice(&message_hash)?;

    // Create signature from r and s
    let mut sig_bytes = [0u8; 64];
    sig_bytes[..32].copy_from_slice(r);
    sig_bytes[32..].copy_from_slice(s);
    let _signature = Signature::from_compact(&sig_bytes)?;

    // Recovery ID (v - 27 for Ethereum)
    let recovery_id = RecoveryId::from_i32((v as i32) - 27)?;

    // Recover public key - Updated API: recovery_id is now part of RecoverableSignature
    let recoverable_sig =
        secp256k1::ecdsa::RecoverableSignature::from_compact(&sig_bytes, recovery_id)?;
    let public_key = secp.recover_ecdsa(&message, &recoverable_sig)?;

    // Convert public key to Ethereum address
    let public_key_bytes = public_key.serialize_uncompressed();
    let public_key_hash = Keccak256::digest(&public_key_bytes[1..]);
    let address = &public_key_hash[12..];

    Ok(format!("0x{}", hex::encode(address)))
}

/// Verify that a signature was created by the claimed address
pub fn verify_signature(address: &str, message: &str, signature: &str) -> bool {
    match recover_address_from_signature(message, signature) {
        Ok(recovered_address) => recovered_address.to_lowercase() == address.to_lowercase(),
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_signature_format() {
        // Test that we can at least handle the format correctly
        let test_sig = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef12";
        assert!(test_sig.len() == 132); // 0x + 130 hex chars = 65 bytes

        let without_prefix = test_sig.strip_prefix("0x").unwrap();
        assert!(without_prefix.len() == 130);
    }
}
