mod blockchain;
mod consensus;
mod token;
mod contract;
mod api;
mod auth;
mod persistence;
mod governance;

use api::routes::{api_routes, AppState};
use auth::{routes::auth_routes, AuthManager};
use blockchain::{chain::Blockchain, transaction::Transaction};
use consensus::pos::Staker;
use contract::contract::ContractExecutor;
use persistence::load_state;
use token::fungible::Token;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::TcpListener;
use tower_http::cors::{CorsLayer, Any};
use axum::http::Method;

#[tokio::main]
async fn main() {
    let difficulty = 2;

    // Initialize Blockchain, Token, Contracts, and Pending Transactions
    let snapshot = load_state();
    let chain = Arc::new(Mutex::new(
        snapshot
            .as_ref()
            .map(|state| state.chain.clone())
            .unwrap_or_else(|| Blockchain::new(difficulty)),
    ));
    let token = Arc::new(Mutex::new(
        snapshot
            .as_ref()
            .map(|state| state.token.clone())
            .unwrap_or_else(|| Token::new("Metacation Token", "MCT", 1000, "admin")),
    ));
    let contracts = Arc::new(Mutex::new(
        snapshot
            .as_ref()
            .map(|state| state.contracts.clone())
            .unwrap_or_else(ContractExecutor::new),
    ));
    let pending_transactions = Arc::new(Mutex::new(
        snapshot
            .as_ref()
            .map(|state| state.pending_transactions.clone())
            .unwrap_or_else(|| Vec::<Transaction>::new()),
    ));
    let auth_manager = Arc::new(Mutex::new(AuthManager::new()));
    let stakers = Arc::new(Mutex::new(
        snapshot
            .as_ref()
            .map(|state| state.stakers.clone())
            .unwrap_or_else(|| Vec::<Staker>::new()),
    ));
    let peers = Arc::new(Mutex::new(
        snapshot
            .as_ref()
            .map(|state| state.peers.clone())
            .unwrap_or_else(Vec::new),
    ));
    let governance = Arc::new(Mutex::new(
        snapshot
            .as_ref()
            .map(|state| state.governance.clone())
            .unwrap_or_default(),
    ));
    let slash_evidence = Arc::new(Mutex::new(
        snapshot
            .as_ref()
            .map(|state| state.slash_evidence.clone())
            .unwrap_or_default(),
    ));
    let metrics = Arc::new(Mutex::new(api::routes::Metrics::default()));

    let app_state = AppState {
        chain,
        token,
        contracts,
        pending_transactions,
        auth_manager,
        stakers,
        peers,
        governance,
        slash_evidence,
        metrics,
    };

    // Configure CORS to allow React app on localhost:5173
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<axum::http::HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers(Any)
        .allow_credentials(false);

    // Combine API routes with auth routes
    let app = api_routes()
        .merge(auth_routes())
        .with_state(app_state)
        .layer(cors);

    println!("🚀 HybridChain REST API running on http://127.0.0.1:3000");
    println!("🌐 CORS enabled for React app on http://localhost:5173");
    println!("📋 Available endpoints:");
    println!("  🔐 AUTHENTICATION:");
    println!("      🎫 POST /auth/nonce");
    println!("      ✅ POST /auth/verify");  
    println!("      🚪 DELETE /auth/logout");
    println!("  🎓 CERTIFICATES:");
    println!("      📜 POST /certificates/issue");
    println!("      ✅ POST /certificates/verify");
    println!("  💰 TOKENS:");
    println!("      💸 POST /tokens/transfer");
    println!("      📊 GET  /tokens/balance/{{account}}");
    println!("  📦 BLOCKCHAIN:");
    println!("      📚 GET  /blocks");
    println!("      🔢 GET  /blocks/{{index}}");
    println!("      📊 GET  /blockchain/stats");
    println!("  ⛏️  MINING:");
    println!("      ⚡ POST /mine");
    println!("      ⚙️  GET  /mining/difficulty");
    println!("      ⚙️  POST /mining/difficulty");
    println!("  🧮 STAKING:");
    println!("      ➕ POST /staking/deposit");
    println!("      ➖ POST /staking/withdraw");
    println!("      👥 GET  /staking/validators");
    println!("  ✔️  VALIDATION:");
    println!("      🔍 GET  /blockchain/validate");
    println!("      🔎 GET  /blocks/{{index}}/validate");
    println!("      📋 GET  /validate (tutorial compat)");
    println!("  📄 TRANSACTIONS:");
    println!("      ⏳ GET  /transactions/pending");
    println!("");
    println!("🌟 Complete blockchain with wallet authentication & smart contracts!");
    
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
