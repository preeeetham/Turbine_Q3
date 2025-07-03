// Associated Token Account Management Example
// Run with: cargo run --example manage_associated_accounts

use anchor_lang::prelude::*;
use create_your_own_spl_token::*;
use std::str::FromStr;

fn main() {
    println!("🏦 Associated Token Account Management Example");
    println!("==============================================");

    // Example 1: Understanding ATA Basics
    println!("\n📚 Step 1: Understanding ATA Basics");
    
    println!("What are Associated Token Accounts (ATAs)?");
    println!("├── Deterministic addresses derived from owner + mint");
    println!("├── One ATA per owner per token type");
    println!("├── Simplifies token account management");
    println!("├── Standard pattern used by all wallets");
    println!("└── Eliminates user confusion about token accounts");

    // Example 2: ATA Address Generation
    println!("\n🔑 Step 2: ATA Address Generation");
    
    // Mock pubkeys for demonstration
    let alice_wallet = Pubkey::new_unique();
    let bob_wallet = Pubkey::new_unique();
    let charlie_wallet = Pubkey::new_unique();
    
    let game_token_mint = Pubkey::new_unique();
    let stable_token_mint = Pubkey::new_unique();
    
    println!("Wallet Addresses:");
    println!("├── Alice: {}", alice_wallet);
    println!("├── Bob: {}", bob_wallet);
    println!("└── Charlie: {}", charlie_wallet);
    
    println!("\nToken Mint Addresses:");
    println!("├── Game Token: {}", game_token_mint);
    println!("└── Stable Token: {}", stable_token_mint);

    // Example 3: Calculate ATA Addresses
    println!("\n🧮 Step 3: Calculate ATA Addresses");
    
    let alice_game_ata = get_associated_token_address(&alice_wallet, &game_token_mint);
    let alice_stable_ata = get_associated_token_address(&alice_wallet, &stable_token_mint);
    
    let bob_game_ata = get_associated_token_address(&bob_wallet, &game_token_mint);
    let bob_stable_ata = get_associated_token_address(&bob_wallet, &stable_token_mint);
    
    println!("Alice's ATAs:");
    println!("├── Game Token ATA: {}", alice_game_ata);
    println!("└── Stable Token ATA: {}", alice_stable_ata);
    
    println!("\nBob's ATAs:");
    println!("├── Game Token ATA: {}", bob_game_ata);
    println!("└── Stable Token ATA: {}", bob_stable_ata);

    // Example 4: ATA Properties
    println!("\n🔍 Step 4: ATA Properties");
    
    println!("Key Properties:");
    println!("├── Deterministic: Same inputs = Same address");
    println!("├── Unique: Each owner+mint combination is unique");
    println!("├── Discoverable: Can be calculated by anyone");
    println!("├── Standard: All wallets use the same derivation");
    println!("└── Efficient: No need to store mappings");

    // Demonstrate deterministic nature
    let alice_game_ata_again = get_associated_token_address(&alice_wallet, &game_token_mint);
    println!("\nDeterministic Verification:");
    println!("├── First calculation: {}", alice_game_ata);
    println!("├── Second calculation: {}", alice_game_ata_again);
    println!("└── Match: {}", alice_game_ata == alice_game_ata_again);

    // Example 5: ATA vs Regular Token Account
    println!("\n⚖️ Step 5: ATA vs Regular Token Account");
    
    let comparison = vec![
        ("Address Type", "ATA: Deterministic", "Regular: Random"),
        ("Creation", "ATA: Standard instruction", "Regular: Custom setup"),
        ("Discovery", "ATA: Easy calculation", "Regular: Need to store/track"),
        ("User Experience", "ATA: Seamless", "Regular: Complex"),
        ("Wallet Support", "ATA: Universal", "Regular: Limited"),
        ("Recommendation", "ATA: ✅ Always use", "Regular: ❌ Avoid"),
    ];
    
    println!("Comparison:");
    for (feature, ata, regular) in comparison {
        println!("├── {}: {} | {}", feature, ata, regular);
    }

    // Example 6: Multi-Token User Portfolio
    println!("\n💼 Step 6: Multi-Token User Portfolio");
    
    let tokens = vec![
        ("GAME", "Epic Adventure Coins", game_token_mint),
        ("STABLE", "USD Stablecoin", stable_token_mint),
        ("GOV", "Governance Token", Pubkey::new_unique()),
        ("REWARD", "Reward Points", Pubkey::new_unique()),
    ];
    
    println!("Alice's Token Portfolio:");
    for (symbol, name, mint) in tokens {
        let ata = get_associated_token_address(&alice_wallet, &mint);
        println!("├── {}: {} | ATA: {}", symbol, name, ata);
    }

    // Example 7: ATA Creation Workflow
    println!("\n🔄 Step 7: ATA Creation Workflow");
    
    println!("ATA Creation Process:");
    println!("1. User wants to receive tokens");
    println!("2. Check if ATA exists for user+token");
    println!("3. If not exists, create ATA");
    println!("4. Transfer tokens to ATA");
    println!("5. User can now see tokens in wallet");

    // Example 8: Batch ATA Creation
    println!("\n📦 Step 8: Batch ATA Creation");
    
    let users = vec![
        ("alice", alice_wallet),
        ("bob", bob_wallet),
        ("charlie", charlie_wallet),
    ];
    
    let token_mint = game_token_mint;
    
    println!("Batch ATA Creation for Game Token:");
    for (name, wallet) in users {
        let ata = get_associated_token_address(&wallet, &token_mint);
        println!("├── {}: {} → {}", name, wallet, ata);
    }

    // Example 9: ATA Utility Functions
    println!("\n🛠️ Step 9: ATA Utility Functions");
    
    println!("Utility Functions:");
    println!("├── get_associated_token_address() - Calculate ATA address");
    println!("├── create_associated_token_account() - Create ATA");
    println!("├── get_or_create_ata() - Get existing or create new");
    println!("├── is_ata_initialized() - Check if ATA exists");
    println!("└── get_ata_balance() - Get token balance");

    // Example 10: ATA in Different Scenarios
    println!("\n🎯 Step 10: ATA in Different Scenarios");
    
    println!("Gaming Scenario:");
    println!("├── Player receives quest rewards → ATA");
    println!("├── Player buys items → Transfer from ATA");
    println!("├── Player trades with others → ATA to ATA");
    println!("└── Player stakes tokens → ATA to staking pool");
    
    println!("\nDeFi Scenario:");
    println!("├── User provides liquidity → ATA to pool");
    println!("├── User receives LP tokens → Pool to ATA");
    println!("├── User claims rewards → Rewards to ATA");
    println!("└── User swaps tokens → ATA to ATA");
    
    println!("\nNFT Scenario:");
    println!("├── User mints NFT → Mint to ATA");
    println!("├── User lists NFT → ATA to marketplace");
    println!("├── User buys NFT → Marketplace to ATA");
    println!("└── User receives royalties → Royalties to ATA");

    // Example 11: ATA Security Considerations
    println!("\n🔒 Step 11: ATA Security Considerations");
    
    println!("Security Best Practices:");
    println!("├── Always verify ATA ownership before transfers");
    println!("├── Check ATA exists before sending tokens");
    println!("├── Use proper account constraints in programs");
    println!("├── Validate mint matches expected token");
    println!("├── Handle ATA creation fees properly");
    println!("└── Implement proper error handling");

    // Example 12: ATA Cost Analysis
    println!("\n💰 Step 12: ATA Cost Analysis");
    
    let ata_rent = 2_039_280; // lamports (typical ATA rent)
    let sol_price = 50.0; // assume $50 per SOL
    let lamports_per_sol = 1_000_000_000;
    
    let ata_cost_sol = ata_rent as f64 / lamports_per_sol as f64;
    let ata_cost_usd = ata_cost_sol * sol_price;
    
    println!("ATA Creation Cost:");
    println!("├── Rent: {} lamports", ata_rent);
    println!("├── SOL: {} SOL", ata_cost_sol);
    println!("├── USD: ${:.4}", ata_cost_usd);
    println!("└── Note: Rent is recoverable when ATA is closed");

    // Example 13: ATA Lifecycle Management
    println!("\n🔄 Step 13: ATA Lifecycle Management");
    
    println!("ATA Lifecycle:");
    println!("1. Calculate ATA address");
    println!("2. Check if ATA exists");
    println!("3. Create ATA if needed");
    println!("4. Use ATA for token operations");
    println!("5. Monitor ATA balance");
    println!("6. Close ATA when no longer needed (recover rent)");

    // Example 14: Common ATA Patterns
    println!("\n📋 Step 14: Common ATA Patterns");
    
    println!("Pattern 1: Lazy ATA Creation");
    println!("├── Create ATA only when first needed");
    println!("├── Reduces upfront costs");
    println!("└── Common in dApps");
    
    println!("\nPattern 2: Batch ATA Setup");
    println!("├── Create multiple ATAs in one transaction");
    println!("├── Efficient for multi-token operations");
    println!("└── Common in token launches");
    
    println!("\nPattern 3: ATA Rent Recovery");
    println!("├── Close unused ATAs to recover rent");
    println!("├── Requires zero token balance");
    println!("└── Good for account hygiene");

    // Example 15: Advanced ATA Features
    println!("\n🚀 Step 15: Advanced ATA Features");
    
    println!("Advanced Features:");
    println!("├── Delegate: Allow others to transfer from ATA");
    println!("├── Close Authority: Control who can close ATA");
    println!("├── Multisig ATA: Require multiple signatures");
    println!("├── Program-Owned ATA: ATA owned by program");
    println!("└── Immutable ATA: Cannot be closed");

    println!("\n✅ ATA Management Complete!");
    println!("\n📝 Key Takeaways:");
    println!("1. Always use ATAs for token storage");
    println!("2. ATAs are deterministic and discoverable");
    println!("3. One ATA per owner per token type");
    println!("4. Create ATAs lazily when needed");
    println!("5. Handle creation costs appropriately");
    println!("6. Close unused ATAs to recover rent");
}

// Helper functions to demonstrate ATA operations
pub fn demonstrate_ata_operations() {
    println!("\n🔧 ATA Operations Demonstration");
    
    // Mock data
    let user_wallet = Pubkey::new_unique();
    let token_mint = Pubkey::new_unique();
    
    // Step 1: Calculate ATA address
    let ata_address = get_associated_token_address(&user_wallet, &token_mint);
    println!("1. ✅ ATA address calculated: {}", ata_address);
    
    // Step 2: Simulate ATA creation
    println!("2. ✅ ATA would be created with create_associated_token_account()");
    
    // Step 3: Simulate token transfer
    let transfer_amount = calculate_token_amount(100.0, 9);
    println!("3. ✅ Would transfer {} tokens to ATA", calculate_human_amount(transfer_amount, 9));
    
    // Step 4: Simulate balance check
    println!("4. ✅ Would check ATA balance");
    
    // Step 5: Simulate ATA closing
    println!("5. ✅ Would close ATA to recover rent (if balance is zero)");
    
    println!("\n🎯 ATA operations completed!");
}

pub fn ata_best_practices() {
    println!("\n💡 ATA Best Practices");
    
    println!("✅ DO:");
    println!("├── Always use ATAs for token storage");
    println!("├── Verify ATA ownership in programs");
    println!("├── Handle ATA creation gracefully");
    println!("├── Close unused ATAs to recover rent");
    println!("├── Use standard ATA derivation");
    println!("└── Check ATA exists before operations");
    
    println!("\n❌ DON'T:");
    println!("├── Use random token accounts");
    println!("├── Skip ATA existence checks");
    println!("├── Ignore ATA creation costs");
    println!("├── Hardcode ATA addresses");
    println!("├── Mix up different token ATAs");
    println!("└── Leave empty ATAs unclosed");
}

#[cfg(test)]
mod ata_tests {
    use super::*;

    #[test]
    fn test_ata_deterministic() {
        let owner = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        
        let ata1 = get_associated_token_address(&owner, &mint);
        let ata2 = get_associated_token_address(&owner, &mint);
        
        assert_eq!(ata1, ata2);
    }

    #[test]
    fn test_ata_unique_per_owner() {
        let alice = Pubkey::new_unique();
        let bob = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        
        let alice_ata = get_associated_token_address(&alice, &mint);
        let bob_ata = get_associated_token_address(&bob, &mint);
        
        assert_ne!(alice_ata, bob_ata);
    }

    #[test]
    fn test_ata_unique_per_mint() {
        let owner = Pubkey::new_unique();
        let mint1 = Pubkey::new_unique();
        let mint2 = Pubkey::new_unique();
        
        let ata1 = get_associated_token_address(&owner, &mint1);
        let ata2 = get_associated_token_address(&owner, &mint2);
        
        assert_ne!(ata1, ata2);
    }

    #[test]
    fn test_ata_address_format() {
        let owner = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        
        let ata = get_associated_token_address(&owner, &mint);
        
        // ATA should be a valid Pubkey
        assert_ne!(ata, Pubkey::default());
        
        // ATA should be different from owner and mint
        assert_ne!(ata, owner);
        assert_ne!(ata, mint);
    }

    #[test]
    fn test_multiple_tokens_per_owner() {
        let owner = Pubkey::new_unique();
        let tokens = vec![
            Pubkey::new_unique(),
            Pubkey::new_unique(),
            Pubkey::new_unique(),
        ];
        
        let atas: Vec<Pubkey> = tokens.iter()
            .map(|mint| get_associated_token_address(&owner, mint))
            .collect();
        
        // All ATAs should be different
        for i in 0..atas.len() {
            for j in i+1..atas.len() {
                assert_ne!(atas[i], atas[j]);
            }
        }
    }

    #[test]
    fn test_ata_cost_calculation() {
        let ata_rent = 2_039_280;
        let lamports_per_sol = 1_000_000_000;
        
        let ata_cost_sol = ata_rent as f64 / lamports_per_sol as f64;
        
        // ATA should cost less than 0.01 SOL
        assert!(ata_cost_sol < 0.01);
        
        // ATA should cost more than 0.001 SOL
        assert!(ata_cost_sol > 0.001);
    }

    #[test]
    fn test_token_amount_calculations() {
        let transfer_amount = calculate_token_amount(100.0, 9);
        assert_eq!(transfer_amount, 100_000_000_000);
        
        let back_to_human = calculate_human_amount(transfer_amount, 9);
        assert_eq!(back_to_human, 100.0);
    }
} 