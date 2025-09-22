use axum::{
    extract::{State, Path},
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use crate::{
    blockchain::{chain::Blockchain, transaction::{Transaction, TransactionType}}, 
    contract::contract::ContractExecutor, 
    token::fungible::Token,
    auth::AuthManager,
};

#[derive(Clone)]
pub struct AppState {
    pub chain: Arc<Mutex<Blockchain>>,
    pub token: Arc<Mutex<Token>>,
    pub contracts: Arc<Mutex<ContractExecutor>>,
    pub pending_transactions: Arc<Mutex<Vec<Transaction>>>,
    pub auth_manager: Arc<Mutex<AuthManager>>,
}

#[derive(Deserialize)]
pub struct CertificateRequest {
    pub id: String,
    pub issued_to: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct VerifyCertificateRequest {
    pub id: String,
}

#[derive(Deserialize)]
pub struct TokenTransferRequest {
    pub from: String,
    pub to: String,
    pub amount: u64,
}

#[derive(Deserialize)]
pub struct DifficultyRequest {
    pub difficulty: usize,
}

#[derive(Serialize)]
pub struct ApiResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct BalanceResponse {
    pub account: String,
    pub balance: u64,
}

#[derive(Serialize)]
pub struct MiningResponse {
    pub status: String,
    pub message: String,
    pub block_index: u64,
    pub transactions_count: usize,
}

#[derive(Serialize)]
pub struct BlockchainStats {
    pub total_blocks: usize,
    pub pending_transactions: usize,
    pub difficulty: usize,
    pub is_valid: bool,
    pub latest_hash: String,
}

#[derive(Serialize)]
pub struct ValidationResponse {
    pub is_valid: bool,
    pub message: String,
    pub details: Option<String>,
}

#[derive(Serialize)]
pub struct DifficultyResponse {
    pub current_difficulty: usize,
}

pub fn api_routes() -> Router<AppState> {
    Router::new()
        // Certificate routes
        .route("/certificates/issue", post(issue_certificate))
        .route("/certificates/verify", post(verify_certificate))
        
        // Token routes
        .route("/tokens/transfer", post(transfer_tokens))
        .route("/tokens/balance/{account}", get(get_token_balance))
        
        // Blockchain routes
        .route("/blocks", get(get_blocks))
        .route("/blocks/{index}", get(get_block_by_index))
        .route("/blockchain/stats", get(get_blockchain_stats))
        
        // Mining routes
        .route("/mine", post(mine_block))
        .route("/mining/difficulty", get(get_mining_difficulty))
        .route("/mining/difficulty", post(set_mining_difficulty))
        
        // Validation routes
        .route("/blockchain/validate", get(validate_blockchain))
        .route("/blocks/{index}/validate", get(validate_block))
        .route("/validate", get(validate_chain)) // Tutorial compatibility
        
        // Transaction routes
        .route("/transactions/pending", get(get_pending_transactions))
}

// ===== CERTIFICATE ENDPOINTS =====

async fn issue_certificate(State(state): State<AppState>, Json(payload): Json<CertificateRequest>) -> Json<ApiResponse> {
    // Update contract state
    let mut contracts = state.contracts.lock().unwrap();
    contracts.issue_certificate(&payload.id, &payload.issued_to, &payload.description);
    drop(contracts);

    // Create blockchain transaction
    let transaction = Transaction::new(
        None, // No sender for certificate issuance
        payload.issued_to.clone(),
        0, // Certificates don't transfer tokens
        TransactionType::Certificate,
    );

    // Add to pending transactions
    let mut pending = state.pending_transactions.lock().unwrap();
    pending.push(transaction);
    drop(pending);

    Json(ApiResponse {
        status: "success".to_string(),
        message: format!("Certificate {} issued to {} and added to pending transactions", payload.id, payload.issued_to),
    })
}

async fn verify_certificate(State(state): State<AppState>, Json(payload): Json<VerifyCertificateRequest>) -> Json<ApiResponse> {
    let mut contracts = state.contracts.lock().unwrap();
    let success = contracts.verify_certificate(&payload.id);

    Json(ApiResponse {
        status: if success { "success" } else { "error" }.to_string(),
        message: if success {
            format!("Certificate {} verified", payload.id)
        } else {
            format!("Failed to verify certificate {}", payload.id)
        },
    })
}

// ===== TOKEN ENDPOINTS =====

async fn transfer_tokens(State(state): State<AppState>, Json(payload): Json<TokenTransferRequest>) -> Json<ApiResponse> {
    // Update token balances
    let mut token = state.token.lock().unwrap();
    let success = token.transfer(&payload.from, &payload.to, payload.amount);
    drop(token);

    if success {
        // Create blockchain transaction
        let transaction = Transaction::new(
            Some(payload.from.clone()),
            payload.to.clone(),
            payload.amount,
            TransactionType::Transfer,
        );

        // Add to pending transactions
        let mut pending = state.pending_transactions.lock().unwrap();
        pending.push(transaction);
        drop(pending);

        Json(ApiResponse {
            status: "success".to_string(),
            message: format!("Transferred {} tokens from {} to {} and added to blockchain", payload.amount, payload.from, payload.to),
        })
    } else {
        Json(ApiResponse {
            status: "error".to_string(),
            message: format!("Failed to transfer tokens from {} to {}", payload.from, payload.to),
        })
    }
}

async fn get_token_balance(State(state): State<AppState>, Path(account): Path<String>) -> Json<BalanceResponse> {
    let token = state.token.lock().unwrap();
    let balance = token.balance_of(&account);
    
    Json(BalanceResponse {
        account,
        balance,
    })
}

// ===== BLOCKCHAIN ENDPOINTS =====

async fn get_blocks(State(state): State<AppState>) -> Json<Vec<String>> {
    let chain = state.chain.lock().unwrap();
    let block_data: Vec<String> = chain.blocks.iter().map(|b| format!("{:?}", b)).collect();
    Json(block_data)
}

async fn get_block_by_index(State(state): State<AppState>, Path(index): Path<usize>) -> Json<Option<String>> {
    let chain = state.chain.lock().unwrap();
    
    if index < chain.blocks.len() {
        Json(Some(format!("{:?}", chain.blocks[index])))
    } else {
        Json(None)
    }
}

async fn get_blockchain_stats(State(state): State<AppState>) -> Json<BlockchainStats> {
    let chain = state.chain.lock().unwrap();
    let pending = state.pending_transactions.lock().unwrap();
    
    Json(BlockchainStats {
        total_blocks: chain.blocks.len(),
        pending_transactions: pending.len(),
        difficulty: chain.difficulty,
        is_valid: chain.is_valid(),
        latest_hash: chain.latest_hash(),
    })
}

// ===== MINING ENDPOINTS =====

async fn mine_block(State(state): State<AppState>) -> Json<MiningResponse> {
    let mut pending = state.pending_transactions.lock().unwrap();
    let mut chain = state.chain.lock().unwrap();
    
    // Since genesis block is auto-created, we only need to check for pending transactions
    // Allow mining if there are pending transactions OR if there's only the genesis block
    let has_only_genesis = chain.blocks.len() == 1;
    
    if pending.is_empty() && !has_only_genesis {
        drop(chain);
        drop(pending);
        return Json(MiningResponse {
            status: "info".to_string(),
            message: "No pending transactions to mine".to_string(),
            block_index: 0,
            transactions_count: 0,
        });
    }

    let transactions_count: usize;
    let transaction_strings: Vec<String>;

    if has_only_genesis && pending.is_empty() {
        // For the first user-initiated mining after genesis, create a welcome transaction
        transaction_strings = vec!["First mined block - Blockchain is now active!".to_string()];
        transactions_count = 1;
    } else {
        // Convert pending transactions to strings for the block
        transaction_strings = pending
            .iter()
            .map(|tx| format!("{:?}", tx))
            .collect();
        transactions_count = transaction_strings.len();
        
        // Clear pending transactions after copying them
        pending.clear();
    }

    // Add block to blockchain
    chain.add_block(transaction_strings);
    let block_index = chain.blocks.len() as u64 - 1;
    
    // Release locks
    drop(chain);
    drop(pending);

    Json(MiningResponse {
        status: "success".to_string(),
        message: if has_only_genesis {
            "Successfully mined the first block! 🎉 Your blockchain is now active with 2 blocks.".to_string()
        } else {
            format!("Successfully mined block with {} transactions", transactions_count)
        },
        block_index,
        transactions_count,
    })
}

async fn get_mining_difficulty(State(state): State<AppState>) -> Json<DifficultyResponse> {
    let chain = state.chain.lock().unwrap();
    Json(DifficultyResponse {
        current_difficulty: chain.difficulty,
    })
}

async fn set_mining_difficulty(State(state): State<AppState>, Json(payload): Json<DifficultyRequest>) -> Json<ApiResponse> {
    let mut chain = state.chain.lock().unwrap();
    let old_difficulty = chain.difficulty;
    chain.difficulty = payload.difficulty;
    
    Json(ApiResponse {
        status: "success".to_string(),
        message: format!("Mining difficulty changed from {} to {}", old_difficulty, payload.difficulty),
    })
}

// ===== VALIDATION ENDPOINTS =====

async fn validate_blockchain(State(state): State<AppState>) -> Json<ValidationResponse> {
    let chain = state.chain.lock().unwrap();
    let is_valid = chain.is_valid();
    
    Json(ValidationResponse {
        is_valid,
        message: if is_valid {
            "Blockchain is valid".to_string()
        } else {
            "Blockchain validation failed".to_string()
        },
        details: if is_valid {
            Some("All blocks properly linked with valid hashes".to_string())
        } else {
            Some("Hash chain broken - invalid block detected".to_string())
        },
    })
}

async fn validate_block(State(state): State<AppState>, Path(index): Path<usize>) -> Json<ValidationResponse> {
    let chain = state.chain.lock().unwrap();
    
    if index >= chain.blocks.len() {
        return Json(ValidationResponse {
            is_valid: false,
            message: "Block not found".to_string(),
            details: Some(format!("Block index {} does not exist", index)),
        });
    }

    if index == 0 {
        // Genesis block is always valid
        return Json(ValidationResponse {
            is_valid: true,
            message: "Genesis block is valid".to_string(),
            details: Some("Genesis block validation passed".to_string()),
        });
    }

    let current_block = &chain.blocks[index];
    let previous_block = &chain.blocks[index - 1];
    
    let is_valid = current_block.previous_hash == previous_block.hash;
    
    Json(ValidationResponse {
        is_valid,
        message: if is_valid {
            format!("Block {} is valid", index)
        } else {
            format!("Block {} validation failed", index)
        },
        details: if is_valid {
            Some("Block properly linked to previous block".to_string())
        } else {
            Some(format!(
                "Hash mismatch: expected {}, got {}", 
                previous_block.hash, 
                current_block.previous_hash
            ))
        },
    })
}

// Tutorial compatibility endpoint
async fn validate_chain(State(state): State<AppState>) -> Json<ApiResponse> {
    let chain = state.chain.lock().unwrap();
    let is_valid = chain.is_valid();

    Json(ApiResponse {
        status: if is_valid { "success" } else { "error" }.to_string(),
        message: if is_valid {
            "Blockchain is valid.".to_string()
        } else {
            "Blockchain is invalid!".to_string()
        },
    })
}

// ===== TRANSACTION ENDPOINTS =====

async fn get_pending_transactions(State(state): State<AppState>) -> Json<Vec<String>> {
    let pending = state.pending_transactions.lock().unwrap();
    let transaction_strings: Vec<String> = pending
        .iter()
        .map(|tx| format!("{:?}", tx))
        .collect();
    Json(transaction_strings)
}
