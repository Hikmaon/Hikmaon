mod blockchain;
mod consensus;
mod token;
mod contract;
mod api;
mod auth;

use api::routes::{api_routes, AppState};
use auth::{routes::auth_routes, AuthManager};
use blockchain::{chain::Blockchain, transaction::Transaction};
use contract::contract::ContractExecutor;
use token::fungible::Token;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tower_http::cors::{CorsLayer, Any};
use axum::http::Method;

#[tokio::main]
async fn main() {
    let difficulty = 2;

    // Initialize Blockchain, Token, Contracts, and Pending Transactions
    let chain = Arc::new(Mutex::new(Blockchain::new(difficulty)));
    let token = Arc::new(Mutex::new(Token::new("Metacation Token", "MCT", 1000, "admin")));
    let contracts = Arc::new(Mutex::new(ContractExecutor::new()));
    let pending_transactions = Arc::new(Mutex::new(Vec::<Transaction>::new()));
    let auth_manager = Arc::new(Mutex::new(AuthManager::new()));

    let app_state = AppState {
        chain,
        token,
        contracts,
        pending_transactions,
        auth_manager,
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
