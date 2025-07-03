// Gaming Token Creation Example
// Run with: cargo run --example create_gaming_token

use anchor_lang::prelude::*;
use create_your_own_spl_token::*;

fn main() {
    println!("🎮 Gaming Token Creation Example");
    println!("=================================");

    // Example 1: Design a Gaming Token
    println!("\n🎯 Step 1: Design Your Gaming Token");
    
    let gaming_token_config = create_gaming_token_config(
        "Epic Adventure Coins".to_string(),
        "EPIC".to_string(),
        1_000_000, // 1M initial supply
    );
    
    println!("Token Design:");
    println!("├── Name: {}", gaming_token_config.name);
    println!("├── Symbol: {}", gaming_token_config.symbol);
    println!("├── Decimals: {} (allows micro-transactions)", gaming_token_config.decimals);
    println!("├── Initial Supply: {} tokens", calculate_human_amount(gaming_token_config.initial_supply, gaming_token_config.decimals));
    println!("├── Total Supply: {} tokens", calculate_human_amount(gaming_token_config.total_supply, gaming_token_config.decimals));
    println!("├── Has Metadata: {}", gaming_token_config.has_metadata);
    println!("└── Can Freeze: {}", gaming_token_config.can_freeze);

    // Example 2: Gaming Economy Distribution
    println!("\n💰 Step 2: Plan Token Distribution");
    
    let total_supply = gaming_token_config.total_supply;
    let initial_supply = gaming_token_config.initial_supply;
    let reserved_for_rewards = total_supply - initial_supply;
    
    println!("Token Distribution:");
    println!("├── Initial Mint: {} EPIC ({}%)", 
             calculate_human_amount(initial_supply, 9),
             (initial_supply * 100) / total_supply);
    println!("├── Reserved for Rewards: {} EPIC ({}%)", 
             calculate_human_amount(reserved_for_rewards, 9),
             (reserved_for_rewards * 100) / total_supply);
    println!("└── Team/Marketing: {} EPIC ({}%)", 
             calculate_human_amount(initial_supply / 10, 9),
             10); // 10% for team/marketing

    // Example 3: Player Reward Tiers
    println!("\n🏆 Step 3: Player Reward System");
    
    let reward_tiers = vec![
        ("New Player Bonus", calculate_token_amount(50.0, 9)),
        ("Daily Login", calculate_token_amount(5.0, 9)),
        ("Quest Completion", calculate_token_amount(25.0, 9)),
        ("Boss Kill", calculate_token_amount(100.0, 9)),
        ("Tournament Winner", calculate_token_amount(1000.0, 9)),
        ("Rare Item Discovery", calculate_token_amount(200.0, 9)),
    ];
    
    println!("Reward Tiers:");
    for (reward_type, amount) in reward_tiers {
        println!("├── {}: {} EPIC tokens", reward_type, calculate_human_amount(amount, 9));
    }

    // Example 4: In-Game Purchases
    println!("\n🛍️ Step 4: In-Game Economy");
    
    let item_prices = vec![
        ("Health Potion", calculate_token_amount(0.1, 9)),
        ("Magic Sword", calculate_token_amount(25.0, 9)),
        ("Armor Set", calculate_token_amount(50.0, 9)),
        ("Mount", calculate_token_amount(100.0, 9)),
        ("Character Skin", calculate_token_amount(15.0, 9)),
        ("Inventory Expansion", calculate_token_amount(10.0, 9)),
    ];
    
    println!("Item Prices:");
    for (item, price) in item_prices {
        println!("├── {}: {} EPIC tokens", item, calculate_human_amount(price, 9));
    }

    // Example 5: Batch Operations for Airdrops
    println!("\n🎁 Step 5: Airdrop Planning");
    
    let airdrop_scenarios = vec![
        ("Launch Event", vec![
            calculate_token_amount(100.0, 9),  // Top players
            calculate_token_amount(50.0, 9),   // Active players
            calculate_token_amount(25.0, 9),   // New players
        ]),
        ("Season End Rewards", vec![
            calculate_token_amount(500.0, 9),  // Champion
            calculate_token_amount(250.0, 9),  // Second place
            calculate_token_amount(125.0, 9),  // Third place
        ]),
        ("Community Event", vec![
            calculate_token_amount(10.0, 9),   // Participation reward
            calculate_token_amount(10.0, 9),   // Everyone gets the same
            calculate_token_amount(10.0, 9),   // Equal distribution
        ]),
    ];
    
    for (event, amounts) in airdrop_scenarios {
        let total: u64 = amounts.iter().sum();
        println!("├── {}: {} EPIC tokens total", event, calculate_human_amount(total, 9));
        println!("│   └── Recipients: {}", amounts.len());
    }

    // Example 6: Advanced Features
    println!("\n🔧 Step 6: Advanced Gaming Features");
    
    println!("Advanced Features:");
    println!("├── Staking System: Earn 5% APY for holding tokens");
    println!("├── Governance: Vote on game updates with tokens");
    println!("├── Trading: Player-to-player marketplace");
    println!("├── Tournaments: Entry fees and prize pools");
    println!("├── NFT Integration: Buy/sell game items as NFTs");
    println!("└── Cross-Game Compatibility: Use tokens across multiple games");

    // Example 7: Token Utility Matrix
    println!("\n📊 Step 7: Token Utility Matrix");
    
    let utilities = vec![
        ("Purchase Items", "✅ Primary use case"),
        ("Earn Rewards", "✅ Playing the game"),
        ("Stake for Yield", "✅ Passive income"),
        ("Vote on Updates", "✅ Community governance"),
        ("Trade with Players", "✅ Peer-to-peer economy"),
        ("Tournament Entry", "✅ Competitive gaming"),
        ("Unlock Features", "✅ Premium content"),
        ("Character Upgrades", "✅ Progress enhancement"),
    ];
    
    println!("Token Utilities:");
    for (utility, status) in utilities {
        println!("├── {}: {}", utility, status);
    }

    // Example 8: Tokenomics Summary
    println!("\n📈 Step 8: Tokenomics Summary");
    
    println!("EPIC Token Tokenomics:");
    println!("├── Type: Utility Token");
    println!("├── Use Case: In-game currency and rewards");
    println!("├── Supply Model: Fixed supply with gradual release");
    println!("├── Distribution: 10% initial, 90% through gameplay");
    println!("├── Inflation: None (fixed supply)");
    println!("├── Deflation: Optional token burning for rare items");
    println!("├── Liquidity: DEX listing planned");
    println!("└── Governance: Community-driven development");

    // Example 9: Technical Implementation
    println!("\n⚙️ Step 9: Technical Implementation");
    
    println!("Implementation Steps:");
    println!("1. Deploy token program to Solana");
    println!("2. Create token mint with proper authorities");
    println!("3. Set up Associated Token Accounts for players");
    println!("4. Implement reward distribution system");
    println!("5. Create in-game purchase functionality");
    println!("6. Add metadata with token information");
    println!("7. Set up monitoring and analytics");
    println!("8. Plan for future upgrades");

    // Example 10: Security and Best Practices
    println!("\n🔒 Step 10: Security Considerations");
    
    println!("Security Measures:");
    println!("├── Multi-signature wallet for treasury");
    println!("├── Time-locked mint authority");
    println!("├── Regular security audits");
    println!("├── Bug bounty program");
    println!("├── Gradual authority revocation");
    println!("├── Emergency pause functionality");
    println!("├── Rate limiting on rewards");
    println!("└── Anti-bot measures");

    println!("\n✅ Gaming Token Design Complete!");
    println!("\n🚀 Next Steps:");
    println!("1. Deploy to devnet for testing");
    println!("2. Create game integration APIs");
    println!("3. Set up player onboarding");
    println!("4. Launch with community");
    println!("5. Monitor and optimize");
}

// Helper function to simulate token creation flow
pub fn simulate_gaming_token_creation() {
    println!("\n🔄 Simulating Token Creation Flow:");
    
    // Step 1: Create token configuration
    let config = create_gaming_token_config(
        "Demo Gaming Token".to_string(),
        "DEMO".to_string(),
        500_000,
    );
    
    println!("1. ✅ Token configuration created");
    println!("   └── {}, {}, {} decimals", config.name, config.symbol, config.decimals);
    
    // Step 2: Calculate initial distribution
    let creator_tokens = config.initial_supply * 60 / 100; // 60% to creator
    let reward_pool = config.initial_supply * 40 / 100;    // 40% to reward pool
    
    println!("2. ✅ Initial distribution calculated");
    println!("   ├── Creator: {} tokens", calculate_human_amount(creator_tokens, 9));
    println!("   └── Reward Pool: {} tokens", calculate_human_amount(reward_pool, 9));
    
    // Step 3: Plan player accounts
    let player_accounts = vec![
        ("alice.sol", calculate_token_amount(100.0, 9)),
        ("bob.sol", calculate_token_amount(150.0, 9)),
        ("charlie.sol", calculate_token_amount(75.0, 9)),
    ];
    
    println!("3. ✅ Player accounts planned");
    for (player, amount) in player_accounts {
        println!("   ├── {}: {} tokens", player, calculate_human_amount(amount, 9));
    }
    
    // Step 4: Calculate total usage
    let total_distributed: u64 = creator_tokens + reward_pool;
    let remaining = config.total_supply - total_distributed;
    
    println!("4. ✅ Supply allocation verified");
    println!("   ├── Distributed: {} tokens", calculate_human_amount(total_distributed, 9));
    println!("   └── Remaining: {} tokens", calculate_human_amount(remaining, 9));
    
    println!("\n🎯 Ready for deployment!");
}

#[cfg(test)]
mod gaming_token_tests {
    use super::*;

    #[test]
    fn test_gaming_token_config() {
        let config = create_gaming_token_config(
            "Test Gaming Token".to_string(),
            "TGT".to_string(),
            1_000_000,
        );
        
        assert_eq!(config.name, "Test Gaming Token");
        assert_eq!(config.symbol, "TGT");
        assert_eq!(config.decimals, 9);
        assert_eq!(config.initial_supply, 1_000_000);
        assert_eq!(config.total_supply, 10_000_000);
        assert_eq!(config.has_metadata, true);
        assert_eq!(config.can_freeze, true);
    }

    #[test]
    fn test_reward_calculations() {
        let daily_login = calculate_token_amount(5.0, 9);
        let quest_reward = calculate_token_amount(25.0, 9);
        let boss_kill = calculate_token_amount(100.0, 9);
        
        assert_eq!(daily_login, 5_000_000_000);
        assert_eq!(quest_reward, 25_000_000_000);
        assert_eq!(boss_kill, 100_000_000_000);
    }

    #[test]
    fn test_item_pricing() {
        let health_potion = calculate_token_amount(0.1, 9);
        let magic_sword = calculate_token_amount(25.0, 9);
        let mount = calculate_token_amount(100.0, 9);
        
        assert_eq!(health_potion, 100_000_000); // 0.1 tokens
        assert_eq!(magic_sword, 25_000_000_000); // 25 tokens
        assert_eq!(mount, 100_000_000_000); // 100 tokens
    }

    #[test]
    fn test_airdrop_amounts() {
        let launch_airdrop = vec![
            calculate_token_amount(100.0, 9),
            calculate_token_amount(50.0, 9),
            calculate_token_amount(25.0, 9),
        ];
        
        let total: u64 = launch_airdrop.iter().sum();
        assert_eq!(total, calculate_token_amount(175.0, 9));
    }

    #[test]
    fn test_supply_distribution() {
        let config = create_gaming_token_config(
            "Test".to_string(),
            "TST".to_string(),
            1_000_000,
        );
        
        // 10% initial, 90% reserved
        let initial_percentage = (config.initial_supply * 100) / config.total_supply;
        let reserved_percentage = ((config.total_supply - config.initial_supply) * 100) / config.total_supply;
        
        assert_eq!(initial_percentage, 10);
        assert_eq!(reserved_percentage, 90);
    }
} 