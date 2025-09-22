use std::collections::HashMap;

use crate::token::fungible::Token;

/// Represents a simple on-chain certificate
#[derive(Debug)]
pub struct Certificate {
    pub id: String,
    pub issued_to: String,
    pub description: String,
    pub verified: bool,
}

/// The contract executor that manages certificates and token issuance
pub struct ContractExecutor {
    pub certificates: HashMap<String, Certificate>,
}

impl ContractExecutor {
    pub fn new() -> Self {
        ContractExecutor {
            certificates: HashMap::new(),
        }
    }

    /// Issues a new certificate
    pub fn issue_certificate(&mut self, id: &str, issued_to: &str, description: &str) {
        let cert = Certificate {
            id: id.to_string(),
            issued_to: issued_to.to_string(),
            description: description.to_string(),
            verified: false,
        };
        self.certificates.insert(id.to_string(), cert);
        println!("Certificate {} issued to {}", id, issued_to);
    }

    /// Verifies a certificate by ID
    pub fn verify_certificate(&mut self, id: &str) -> bool {
        if let Some(cert) = self.certificates.get_mut(id) {
            cert.verified = true;
            println!("Certificate {} verified!", id);
            true
        } else {
            println!("Certificate {} not found.", id);
            false
        }
    }

    /// Issues tokens as a reward for certificate verification
    pub fn reward_certificate_holder(&self, token: &mut Token, cert_id: &str, amount: u64) -> bool {
        if let Some(cert) = self.certificates.get(cert_id) {
            if cert.verified {
                token.mint(&cert.issued_to, amount);
                println!("Rewarded {} {} tokens for certificate {}", cert.issued_to, amount, cert_id);
                return true;
            } else {
                println!("Certificate {} is not verified yet.", cert_id);
            }
        } else {
            println!("Certificate {} not found.", cert_id);
        }
        false
    }
}
