use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

use crate::error::AppError;

// Request types
#[derive(Debug, Deserialize)]
pub struct TransferRequest {
    pub from: String,
    pub to: String,
    pub amount: f64,
    #[serde(default)]
    pub private_key: Option<String>,
}

// Response types
#[derive(Debug, Serialize)]
pub struct BalanceResponse {
    pub address: String,
    pub balance: f64,
    pub lamports: u64,
}

#[derive(Debug, Serialize)]
pub struct AccountInfoResponse {
    pub address: String,
    pub lamports: u64,
    pub owner: String,
    pub executable: bool,
    pub rent_epoch: u64,
}

#[derive(Debug, Serialize)]
pub struct TransferResponse {
    pub signature: String,
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct TransactionResponse {
    pub signature: String,
    pub slot: u64,
    pub block_time: Option<i64>,
    pub success: bool,
    pub fee: Option<u64>,
    pub accounts: Vec<String>,
}

// Utility functions
pub fn validate_pubkey(address: &str) -> Result<Pubkey, AppError> {
    Pubkey::from_str(address)
        .map_err(|_| AppError::InvalidPublicKey(format!("Invalid public key: {}", address)))
}

pub fn lamports_to_sol(lamports: u64) -> f64 {
    lamports as f64 / 1_000_000_000.0
}

pub fn sol_to_lamports(sol: f64) -> u64 {
    (sol * 1_000_000_000.0) as u64
} 