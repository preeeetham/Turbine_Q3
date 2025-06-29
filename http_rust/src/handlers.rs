use axum::{extract::{Path, State}, response::Json};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use std::str::FromStr;
use tracing::info;

use crate::{error::AppError, types::*, AppState};

pub async fn get_balance(
    State(state): State<AppState>,
    Path(address): Path<String>,
) -> Result<Json<BalanceResponse>, AppError> {
    info!("Getting balance for address: {}", address);
    
    let pubkey = validate_pubkey(&address)?;
    let balance = state.rpc_client.get_balance_with_commitment(&pubkey, CommitmentConfig::confirmed())?;
    let sol_balance = lamports_to_sol(balance.value);
    
    info!("Balance for {}: {} SOL ({} lamports)", address, sol_balance, balance.value);
    
    Ok(Json(BalanceResponse {
        address,
        balance: sol_balance,
        lamports: balance.value,
    }))
}

pub async fn get_account_info(
    State(state): State<AppState>,
    Path(address): Path<String>,
) -> Result<Json<AccountInfoResponse>, AppError> {
    info!("Getting account info for address: {}", address);
    
    let pubkey = validate_pubkey(&address)?;
    let account = state.rpc_client.get_account_with_commitment(&pubkey, CommitmentConfig::confirmed())?;
    
    let account_info = account.value.ok_or_else(|| {
        AppError::WalletNotFound(format!("Account not found: {}", address))
    })?;
    
    info!("Account info retrieved for: {}", address);
    
    Ok(Json(AccountInfoResponse {
        address,
        lamports: account_info.lamports,
        owner: account_info.owner.to_string(),
        executable: account_info.executable,
        rent_epoch: account_info.rent_epoch,
    }))
}

pub async fn transfer_sol(
    State(state): State<AppState>,
    Json(transfer_req): Json<TransferRequest>,
) -> Result<Json<TransferResponse>, AppError> {
    info!("Processing transfer: {} SOL from {} to {}", 
          transfer_req.amount, transfer_req.from, transfer_req.to);
    
    // Validate addresses
    let from_pubkey = validate_pubkey(&transfer_req.from)?;
    let to_pubkey = validate_pubkey(&transfer_req.to)?;
    
    // Validate amount
    if transfer_req.amount <= 0.0 {
        return Err(AppError::InvalidAmount("Amount must be greater than 0".to_string()));
    }
    
    let lamports = sol_to_lamports(transfer_req.amount);
    
    // Handle private key
    let keypair = if let Some(private_key) = transfer_req.private_key {
        // Parse private key from string (base58 format)
        let private_key_bytes = bs58::decode(&private_key)
            .into_vec()
            .map_err(|_| AppError::BadRequest("Invalid private key format".to_string()))?;
        
        Keypair::from_bytes(&private_key_bytes)
            .map_err(|_| AppError::BadRequest("Invalid private key".to_string()))?
    } else {
        // Create a new keypair for demo (this will have 0 balance)
        return Err(AppError::BadRequest("Private key is required for transfers".to_string()));
    };
    
    // Check if the from address matches the keypair
    if keypair.pubkey() != from_pubkey {
        return Err(AppError::BadRequest("Private key doesn't match from address".to_string()));
    }
    
    // Create transfer instruction
    let transfer_instruction = system_instruction::transfer(
        &from_pubkey,
        &to_pubkey,
        lamports,
    );
    
    // Get recent blockhash
    let recent_blockhash = state.rpc_client.get_latest_blockhash()?;
    
    // Create and sign transaction
    let transaction = Transaction::new_signed_with_payer(
        &[transfer_instruction],
        Some(&from_pubkey),
        &[&keypair],
        recent_blockhash,
    );
    
    // Send transaction
    let signature = state.rpc_client.send_and_confirm_transaction(&transaction)?;
    
    info!("Transfer successful! Signature: {}", signature);
    
    Ok(Json(TransferResponse {
        signature: signature.to_string(),
        success: true,
        message: format!("Successfully transferred {} SOL", transfer_req.amount),
    }))
}

pub async fn get_transaction(
    State(state): State<AppState>,
    Path(signature): Path<String>,
) -> Result<Json<TransactionResponse>, AppError> {
    info!("Getting transaction info for signature: {}", signature);
    
    let sig = solana_sdk::signature::Signature::from_str(&signature)
        .map_err(|_| AppError::InvalidSignature(format!("Invalid signature: {}", signature)))?;
    
    let transaction = state.rpc_client.get_transaction_with_config(
        &sig, 
        solana_client::rpc_config::RpcTransactionConfig {
            encoding: Some(solana_transaction_status::UiTransactionEncoding::Json),
            commitment: Some(CommitmentConfig::confirmed()),
            max_supported_transaction_version: Some(0),
        }
    )?;
    
    // transaction is already the actual transaction data, not an Option
    let tx_info = transaction;
    
    let accounts: Vec<String> = match &tx_info.transaction.transaction {
        solana_transaction_status::EncodedTransaction::Json(ui_tx) => {
            match &ui_tx.message {
                solana_transaction_status::UiMessage::Parsed(parsed) => {
                    parsed.account_keys.iter().map(|key| key.pubkey.to_string()).collect()
                }
                solana_transaction_status::UiMessage::Raw(raw) => {
                    raw.account_keys.iter().map(|key| key.to_string()).collect()
                }
            }
        }
        _ => vec![], // Handle other encoding types if needed
    };
    
    info!("Transaction info retrieved for: {}", signature);
    
    Ok(Json(TransactionResponse {
        signature,
        slot: tx_info.slot,
        block_time: tx_info.block_time,
        success: tx_info.transaction.meta.as_ref().map(|meta| meta.err.is_none()).unwrap_or(false),
        fee: tx_info.transaction.meta.as_ref().map(|meta| meta.fee),
        accounts,
    }))
} 