use axum::{
    response::Json,
    routing::{get, post},
    Router,
};
use solana_client::rpc_client::RpcClient;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, warn};

mod error;
mod handlers;
mod types;

use handlers::*;

#[derive(Clone)]
pub struct AppState {
    pub rpc_client: Arc<RpcClient>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // Load environment variables
    dotenv::dotenv().ok();
    
    // Get RPC URL from environment or use default
    let rpc_url = std::env::var("SOLANA_RPC_URL")
        .unwrap_or_else(|_| "https://api.devnet.solana.com".to_string());
    
    info!("Connecting to Solana RPC: {}", rpc_url);
    
    // Create RPC client
    let rpc_client = Arc::new(RpcClient::new(rpc_url));
    
    // Test connection
    match rpc_client.get_version() {
        Ok(version) => info!("Connected to Solana node: {:?}", version),
        Err(e) => warn!("Failed to connect to Solana node: {}", e),
    }
    
    // Create app state
    let state = AppState { rpc_client };
    
    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    
    // Build router
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/balance/:address", get(get_balance))
        .route("/account/:address", get(get_account_info))
        .route("/transfer", post(transfer_sol))
        .route("/transaction/:signature", get(get_transaction))
        .layer(cors)
        .with_state(state);
    
    // Get port from environment or use default
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    
    info!("🚀 Solana HTTP API server starting on {}", addr);
    info!("📖 API Documentation:");
    info!("  GET  /              - API information");
    info!("  GET  /health        - Health check");
    info!("  GET  /balance/:addr - Get SOL balance");
    info!("  GET  /account/:addr - Get account info");
    info!("  POST /transfer      - Transfer SOL");
    info!("  GET  /transaction/:sig - Get transaction info");
    
    // Start server
    axum::serve(listener, app).await?;
    
    Ok(())
}

// Root endpoint
async fn root() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "message": "Solana HTTP API Server",
        "version": "1.0.0",
        "description": "HTTP server for interacting with Solana blockchain",
        "endpoints": {
            "health": "GET /health - Health check",
            "balance": "GET /balance/{address} - Get SOL balance for address",
            "account": "GET /account/{address} - Get account information",
            "transfer": "POST /transfer - Transfer SOL between addresses",
            "transaction": "GET /transaction/{signature} - Get transaction details"
        },
        "examples": {
            "balance": "/balance/11111111111111111111111111111111",
            "transfer": {
                "method": "POST",
                "url": "/transfer",
                "body": {
                    "from": "source_address",
                    "to": "destination_address", 
                    "amount": 0.1,
                    "private_key": "base58_encoded_private_key"
                }
            }
        }
    }))
}

// Health check endpoint
async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "service": "solana-http-api"
    }))
}
