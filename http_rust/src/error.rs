use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Solana RPC error: {0}")]
    SolanaRpc(#[from] solana_client::client_error::ClientError),
    
    #[error("Invalid public key: {0}")]
    InvalidPublicKey(String),
    
    #[error("Wallet not found: {0}")]
    WalletNotFound(String),
    
    #[error("Invalid signature: {0}")]
    InvalidSignature(String),
    
    #[error("Invalid amount: {0}")]
    InvalidAmount(String),
    
    #[error("Bad request: {0}")]
    BadRequest(String),
    
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    
    #[error("Internal server error: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match &self {
            AppError::SolanaRpc(_) => (StatusCode::BAD_GATEWAY, "Solana RPC error"),
            AppError::InvalidPublicKey(_) => (StatusCode::BAD_REQUEST, "Invalid public key"),
            AppError::WalletNotFound(_) => (StatusCode::NOT_FOUND, "Wallet not found"),
            AppError::InvalidSignature(_) => (StatusCode::BAD_REQUEST, "Invalid signature"),
            AppError::InvalidAmount(_) => (StatusCode::BAD_REQUEST, "Invalid amount"),
            AppError::BadRequest(_) => (StatusCode::BAD_REQUEST, "Bad request"),
            AppError::Unauthorized(_) => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            AppError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };

        let body = Json(json!({
            "error": error_message,
            "message": self.to_string(),
        }));

        (status, body).into_response()
    }
} 