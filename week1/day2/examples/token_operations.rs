// This example demonstrates various SPL token operations
// Run with: cargo run --example token_operations

use anchor_lang::prelude::*;
use spl_token_deep_dive::*;

fn main() {
    println!("🚀 SPL Token Operations Examples");
    println!("=====================================");

    // Example 1: Token amount calculations
    println!("\n📊 Token Amount Calculations:");
    
    let human_amount = 100.5;
    let decimals = 9;
    let token_amount = calculate_token_amount(human_amount, decimals);
    
    println!("Human readable: {} tokens", human_amount);
    println!("Token amount: {} (with {} decimals)", token_amount, decimals);
    println!("Back to human: {} tokens", calculate_human_amount(token_amount, decimals));

    // Example 2: Different token scenarios
    println!("\n🎮 Gaming Token Examples:");
    
    // Gaming tokens (usually 9 decimals)
    let game_tokens = calculate_token_amount(1000.0, 9);
    println!("1,000 GAME tokens = {} raw units", game_tokens);
    
    // Stablecoin (6 decimals like USDC)
    let stable_tokens = calculate_token_amount(50.25, 6);
    println!("$50.25 USDC = {} raw units", stable_tokens);
    
    // Utility tokens (2 decimals for simple counting)
    let utility_tokens = calculate_token_amount(25.0, 2);
    println!("25.00 utility tokens = {} raw units", utility_tokens);

    // Example 3: Batch operation planning
    println!("\n🔄 Batch Operations Planning:");
    
    let batch_amounts = vec![
        calculate_token_amount(100.0, 9),   // 100 tokens
        calculate_token_amount(250.0, 9),   // 250 tokens
        calculate_token_amount(75.5, 9),    // 75.5 tokens
    ];
    
    let total: u64 = batch_amounts.iter().sum();
    println!("Batch mint amounts: {:?}", batch_amounts);
    println!("Total tokens to mint: {} ({} human-readable)", 
             total, calculate_human_amount(total, 9));

    // Example 4: Token economics calculations
    println!("\n💰 Token Economics Examples:");
    
    // Calculate inflation rewards
    let staking_pool = calculate_token_amount(1_000_000.0, 9); // 1M tokens staked
    let annual_rate = 0.05; // 5% APY
    let daily_rewards = (staking_pool as f64 * annual_rate / 365.0) as u64;
    
    println!("Staking pool: {} tokens", calculate_human_amount(staking_pool, 9));
    println!("Daily rewards (5% APY): {} tokens", calculate_human_amount(daily_rewards, 9));

    // Example 5: Associated Token Account addresses
    println!("\n🏦 Token Account Management:");
    
    // Mock pubkeys for demonstration
    let owner_pubkey = Pubkey::new_unique();
    let mint_pubkey = Pubkey::new_unique();
    
    let ata_address = find_associated_token_address(&owner_pubkey, &mint_pubkey);
    
    println!("Owner: {}", owner_pubkey);
    println!("Mint: {}", mint_pubkey);
    println!("ATA Address: {}", ata_address);

    println!("\n✅ Examples completed successfully!");
    println!("\n📝 Next steps:");
    println!("1. Study the code in src/lib.rs");
    println!("2. Run tests with: cargo test");
    println!("3. Deploy to devnet for testing");
    println!("4. Build your own token economy!");
}

// Helper function to demonstrate token economy design
pub fn design_gaming_economy() {
    println!("\n🎯 Gaming Token Economy Design:");
    
    // Primary utility token
    let total_supply = calculate_token_amount(10_000_000.0, 9); // 10M tokens
    let initial_mint = calculate_token_amount(1_000_000.0, 9);  // 1M initial
    
    println!("GAME Token Economy:");
    println!("├── Total Supply: {} GAME", calculate_human_amount(total_supply, 9));
    println!("├── Initial Mint: {} GAME", calculate_human_amount(initial_mint, 9));
    println!("├── Reserved for Rewards: {} GAME", calculate_human_amount(total_supply - initial_mint, 9));
    
    // Player allocation example
    let player_allocations = vec![
        ("New Player", calculate_token_amount(100.0, 9)),
        ("Active Player", calculate_token_amount(500.0, 9)),
        ("Veteran Player", calculate_token_amount(2000.0, 9)),
    ];
    
    println!("├── Player Tiers:");
    for (tier, amount) in player_allocations {
        println!("│   ├── {}: {} GAME", tier, calculate_human_amount(amount, 9));
    }
    
    println!("└── Decimals: 9 (allows micro-transactions)");
}

#[cfg(test)]
mod example_tests {
    use super::*;

    #[test]
    fn test_gaming_token_calculations() {
        let player_reward = calculate_token_amount(50.25, 9);
        assert_eq!(player_reward, 50_250_000_000);
        
        let back = calculate_human_amount(player_reward, 9);
        assert_eq!(back, 50.25);
    }

    #[test]
    fn test_stablecoin_precision() {
        // USDC-like token with 6 decimals
        let price = calculate_token_amount(1.50, 6); // $1.50
        assert_eq!(price, 1_500_000);
        
        let back = calculate_human_amount(price, 6);
        assert_eq!(back, 1.5);
    }

    #[test]
    fn test_batch_operations() {
        let amounts = vec![
            calculate_token_amount(10.0, 9),
            calculate_token_amount(20.0, 9),
            calculate_token_amount(30.0, 9),
        ];
        
        let total: u64 = amounts.iter().sum();
        let expected = calculate_token_amount(60.0, 9);
        
        assert_eq!(total, expected);
        assert!(amounts.len() <= MAX_BATCH_SIZE);
    }
} 