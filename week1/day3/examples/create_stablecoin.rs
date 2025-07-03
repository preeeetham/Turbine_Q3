// Stablecoin Creation Example
// Run with: cargo run --example create_stablecoin

use anchor_lang::prelude::*;
use create_your_own_spl_token::*;

fn main() {
    println!("💵 Stablecoin Creation Example");
    println!("==============================");

    // Example 1: Design a Stablecoin
    println!("\n🎯 Step 1: Design Your Stablecoin");
    
    let stablecoin_config = create_stablecoin_config(
        "USD Stable Coin".to_string(),
        "USDS".to_string(),
    );
    
    println!("Stablecoin Design:");
    println!("├── Name: {}", stablecoin_config.name);
    println!("├── Symbol: {}", stablecoin_config.symbol);
    println!("├── Decimals: {} (matches USD precision)", stablecoin_config.decimals);
    println!("├── Initial Supply: {} tokens", calculate_human_amount(stablecoin_config.initial_supply, stablecoin_config.decimals));
    println!("├── Total Supply: Unlimited (backed by reserves)");
    println!("├── Has Metadata: {}", stablecoin_config.has_metadata);
    println!("└── Can Freeze: {} (for compliance)", stablecoin_config.can_freeze);

    // Example 2: Stablecoin Types and Models
    println!("\n📊 Step 2: Stablecoin Types and Models");
    
    println!("Stablecoin Types:");
    println!("├── Fiat-Collateralized (USDC, USDT)");
    println!("│   ├── Backed by USD in bank accounts");
    println!("│   ├── 1:1 redemption ratio");
    println!("│   └── Centralized control");
    println!("├── Crypto-Collateralized (DAI, LUSD)");
    println!("│   ├── Backed by cryptocurrency");
    println!("│   ├── Over-collateralized");
    println!("│   └── Decentralized governance");
    println!("└── Algorithmic (FRAX, TERRA)");
    println!("    ├── No direct collateral");
    println!("    ├── Algorithm-controlled supply");
    println!("    └── Market-driven stability");

    // Example 3: Reserve Management
    println!("\n🏦 Step 3: Reserve Management");
    
    let reserve_scenarios = vec![
        ("Launch Phase", 100_000.0, 100_000), // $100k reserves, 100k tokens
        ("Growth Phase", 1_000_000.0, 1_000_000), // $1M reserves, 1M tokens
        ("Mature Phase", 100_000_000.0, 100_000_000), // $100M reserves, 100M tokens
    ];
    
    println!("Reserve Management Scenarios:");
    for (phase, reserves_usd, tokens_issued) in reserve_scenarios {
        let collateral_ratio = (reserves_usd / tokens_issued as f64) * 100.0;
        println!("├── {}:", phase);
        println!("│   ├── Reserves: ${:.0}", reserves_usd);
        println!("│   ├── Tokens Issued: {} USDS", tokens_issued);
        println!("│   └── Collateral Ratio: {:.1}%", collateral_ratio);
    }

    // Example 4: Minting and Redemption Process
    println!("\n🔄 Step 4: Minting and Redemption Process");
    
    println!("Minting Process:");
    println!("1. User deposits USD to bank account");
    println!("2. System verifies deposit");
    println!("3. Equivalent USDS tokens are minted");
    println!("4. Tokens sent to user's ATA");
    println!("5. Reserve balance updated");
    
    println!("\nRedemption Process:");
    println!("1. User requests redemption");
    println!("2. System burns user's USDS tokens");
    println!("3. USD transferred to user's bank");
    println!("4. Reserve balance updated");
    println!("5. Total supply decreased");

    // Example 5: Price Stability Mechanisms
    println!("\n⚖️ Step 5: Price Stability Mechanisms");
    
    println!("Stability Mechanisms:");
    println!("├── Direct Arbitrage");
    println!("│   ├── Buy tokens when < $1.00");
    println!("│   └── Sell tokens when > $1.00");
    println!("├── Reserve Management");
    println!("│   ├── Maintain adequate reserves");
    println!("│   └── Regular audits and attestations");
    println!("├── Market Making");
    println!("│   ├── Provide liquidity on DEXs");
    println!("│   └── Tight bid/ask spreads");
    println!("└── Emergency Controls");
    println!("    ├── Pause minting/redemption");
    println!("    └── Freeze suspicious accounts");

    // Example 6: Compliance and Regulations
    println!("\n⚖️ Step 6: Compliance and Regulations");
    
    println!("Compliance Requirements:");
    println!("├── KYC/AML Procedures");
    println!("│   ├── User identity verification");
    println!("│   ├── Transaction monitoring");
    println!("│   └── Suspicious activity reporting");
    println!("├── Banking Partnerships");
    println!("│   ├── FDIC-insured accounts");
    println!("│   ├── Segregated reserves");
    println!("│   └── Regular reconciliation");
    println!("├── Regulatory Licenses");
    println!("│   ├── Money transmitter licenses");
    println!("│   ├── Banking partnerships");
    println!("│   └── International compliance");
    println!("└── Transparency");
    println!("    ├── Regular attestations");
    println!("    ├── Public reserve reports");
    println!("    └── Open-source contracts");

    // Example 7: Token Operations
    println!("\n🔧 Step 7: Token Operations");
    
    let operations = vec![
        ("Mint 1,000 USDS", calculate_token_amount(1000.0, 6)),
        ("Mint 50,000 USDS", calculate_token_amount(50000.0, 6)),
        ("Burn 500 USDS", calculate_token_amount(500.0, 6)),
        ("Transfer 100 USDS", calculate_token_amount(100.0, 6)),
        ("Freeze Account", 0),
        ("Unfreeze Account", 0),
    ];
    
    println!("Common Operations:");
    for (operation, amount) in operations {
        if amount > 0 {
            println!("├── {}: {} raw units", operation, amount);
        } else {
            println!("├── {}: Administrative action", operation);
        }
    }

    // Example 8: Multi-Chain Deployment
    println!("\n🌐 Step 8: Multi-Chain Deployment");
    
    println!("Multi-Chain Strategy:");
    println!("├── Solana (Primary)");
    println!("│   ├── Fast and cheap transactions");
    println!("│   ├── Native SPL token standard");
    println!("│   └── DeFi ecosystem integration");
    println!("├── Ethereum (Bridge)");
    println!("│   ├── Wrapped version via bridge");
    println!("│   ├── ERC-20 compatibility");
    println!("│   └── DeFi liquidity access");
    println!("└── Other Chains");
    println!("    ├── Polygon for scaling");
    println!("    ├── BSC for alt ecosystem");
    println!("    └── Avalanche for speed");

    // Example 9: Risk Management
    println!("\n⚠️ Step 9: Risk Management");
    
    println!("Risk Mitigation:");
    println!("├── Counterparty Risk");
    println!("│   ├── Multiple banking partners");
    println!("│   ├── Insurance coverage");
    println!("│   └── Regular audits");
    println!("├── Technical Risk");
    println!("│   ├── Smart contract audits");
    println!("│   ├── Multi-sig controls");
    println!("│   └── Emergency pause");
    println!("├── Regulatory Risk");
    println!("│   ├── Proactive compliance");
    println!("│   ├── Legal framework");
    println!("│   └── Regulatory sandbox");
    println!("└── Market Risk");
    println!("    ├── Diversified reserves");
    println!("    ├── Stress testing");
    println!("    └── Liquidity management");

    // Example 10: Tokenomics and Economics
    println!("\n📈 Step 10: Tokenomics and Economics");
    
    println!("Economic Model:");
    println!("├── Revenue Sources");
    println!("│   ├── Reserve interest earnings");
    println!("│   ├── Exchange spread (0.1-0.5%)");
    println!("│   └── Premium services");
    println!("├── Cost Structure");
    println!("│   ├── Banking and custody fees");
    println!("│   ├── Compliance and audits");
    println!("│   ├── Technology infrastructure");
    println!("│   └── Insurance and reserves");
    println!("└── Growth Strategy");
    println!("    ├── DeFi protocol integrations");
    println!("    ├── Merchant adoption");
    println!("    ├── Cross-border payments");
    println!("    └── Institutional services");

    // Example 11: Technical Implementation
    println!("\n⚙️ Step 11: Technical Implementation");
    
    println!("Technical Stack:");
    println!("├── Smart Contract");
    println!("│   ├── SPL Token program");
    println!("│   ├── Custom minting controls");
    println!("│   └── Compliance features");
    println!("├── Backend Services");
    println!("│   ├── Banking API integration");
    println!("│   ├── KYC/AML services");
    println!("│   ├── Monitoring and alerts");
    println!("│   └── Compliance reporting");
    println!("├── Frontend Applications");
    println!("│   ├── Web dashboard");
    println!("│   ├── Mobile apps");
    println!("│   └── API for partners");
    println!("└── Infrastructure");
    println!("    ├── Cloud hosting");
    println!("    ├── Database systems");
    println!("    ├── Security monitoring");
    println!("    └── Backup and recovery");

    // Example 12: Launch Strategy
    println!("\n🚀 Step 12: Launch Strategy");
    
    println!("Launch Phases:");
    println!("├── Phase 1: Private Beta");
    println!("│   ├── Limited partners only");
    println!("│   ├── Small-scale testing");
    println!("│   └── Compliance validation");
    println!("├── Phase 2: Public Beta");
    println!("│   ├── Open to retail users");
    println!("│   ├── Limited mint amounts");
    println!("│   └── Community feedback");
    println!("├── Phase 3: Full Launch");
    println!("│   ├── Unrestricted minting");
    println!("│   ├── DeFi integrations");
    println!("│   └── Marketing campaign");
    println!("└── Phase 4: Expansion");
    println!("    ├── Multi-chain deployment");
    println!("    ├── Institutional services");
    println!("    └── Global compliance");

    // Example 13: Monitoring and Analytics
    println!("\n📊 Step 13: Monitoring and Analytics");
    
    println!("Key Metrics:");
    println!("├── Supply Metrics");
    println!("│   ├── Total tokens in circulation");
    println!("│   ├── Daily mint/burn volume");
    println!("│   └── Reserve ratio");
    println!("├── Market Metrics");
    println!("│   ├── Token price across exchanges");
    println!("│   ├── Trading volume");
    println!("│   ├── Liquidity depth");
    println!("│   └── Price stability (variance)");
    println!("├── User Metrics");
    println!("│   ├── Active addresses");
    println!("│   ├── Transaction frequency");
    println!("│   └── Geographic distribution");
    println!("└── Operational Metrics");
    println!("    ├── System uptime");
    println!("    ├── Transaction success rate");
    println!("    ├── Compliance incidents");
    println!("    └── Support tickets");

    // Example 14: Advanced Features
    println!("\n🔬 Step 14: Advanced Features");
    
    println!("Advanced Capabilities:");
    println!("├── Programmable Compliance");
    println!("│   ├── Automated KYC checks");
    println!("│   ├── Transaction limits");
    println!("│   └── Jurisdiction restrictions");
    println!("├── Yield Generation");
    println!("│   ├── Interest on holdings");
    println!("│   ├── Lending integration");
    println!("│   └── Staking rewards");
    println!("├── Cross-Chain Features");
    println!("│   ├── Atomic swaps");
    println!("│   ├── Bridge protocols");
    println!("│   └── Unified liquidity");
    println!("└── Enterprise Tools");
    println!("    ├── Bulk operations");
    println!("    ├── Treasury management");
    println!("    ├── Reporting APIs");
    println!("    └── White-label solutions");

    println!("\n✅ Stablecoin Design Complete!");
    println!("\n🎯 Success Metrics:");
    println!("1. Price stability within ±0.5% of $1.00");
    println!("2. 99.9% system uptime");
    println!("3. < 24h mint/redemption time");
    println!("4. 100% reserve backing");
    println!("5. Full regulatory compliance");
    println!("6. Growing ecosystem adoption");
}

// Helper function to simulate stablecoin operations
pub fn simulate_stablecoin_operations() {
    println!("\n🔄 Simulating Stablecoin Operations");
    
    let config = create_stablecoin_config(
        "Demo Stable USD".to_string(),
        "DSUSD".to_string(),
    );
    
    println!("1. ✅ Stablecoin configuration created");
    println!("   └── {}, {}, {} decimals", config.name, config.symbol, config.decimals);
    
    // Simulate minting
    let mint_amounts = vec![
        ("User A deposit", 1000.0),
        ("User B deposit", 5000.0),
        ("Institution C", 100000.0),
    ];
    
    println!("2. ✅ Minting simulation:");
    let mut total_supply = 0.0;
    for (user, amount) in mint_amounts {
        total_supply += amount;
        let token_amount = calculate_token_amount(amount, 6);
        println!("   ├── {}: ${} → {} tokens", user, amount, calculate_human_amount(token_amount, 6));
    }
    
    println!("3. ✅ Current metrics:");
    println!("   ├── Total Supply: {} DSUSD", total_supply);
    println!("   ├── Reserve Ratio: 100%");
    println!("   └── Price: $1.00");
    
    // Simulate redemption
    let redemption_amount = 500.0;
    total_supply -= redemption_amount;
    println!("4. ✅ Redemption processed:");
    println!("   ├── Amount: {} DSUSD", redemption_amount);
    println!("   └── New Supply: {} DSUSD", total_supply);
    
    println!("\n🎯 Stablecoin operations completed!");
}

pub fn stablecoin_best_practices() {
    println!("\n💡 Stablecoin Best Practices");
    
    println!("✅ DO:");
    println!("├── Maintain full reserve backing");
    println!("├── Implement robust compliance");
    println!("├── Conduct regular audits");
    println!("├── Provide transparent reporting");
    println!("├── Build strong banking partnerships");
    println!("├── Monitor price stability constantly");
    println!("├── Plan for regulatory changes");
    println!("└── Prioritize user experience");
    
    println!("\n❌ DON'T:");
    println!("├── Fractional reserve without disclosure");
    println!("├── Skip regulatory compliance");
    println!("├── Ignore price deviations");
    println!("├── Concentrate counterparty risk");
    println!("├── Overcomplicate user flows");
    println!("├── Neglect security measures");
    println!("├── Launch without proper testing");
    println!("└── Ignore market feedback");
}

#[cfg(test)]
mod stablecoin_tests {
    use super::*;

    #[test]
    fn test_stablecoin_config() {
        let config = create_stablecoin_config(
            "Test Stable".to_string(),
            "TSTABLE".to_string(),
        );
        
        assert_eq!(config.name, "Test Stable");
        assert_eq!(config.symbol, "TSTABLE");
        assert_eq!(config.decimals, 6); // USD precision
        assert_eq!(config.initial_supply, 0);
        assert_eq!(config.total_supply, u64::MAX);
        assert_eq!(config.has_metadata, true);
        assert_eq!(config.can_freeze, true);
    }

    #[test]
    fn test_usd_precision() {
        // Test that 6 decimals provides proper USD precision
        let one_cent = calculate_token_amount(0.01, 6);
        assert_eq!(one_cent, 10_000);
        
        let one_dollar = calculate_token_amount(1.0, 6);
        assert_eq!(one_dollar, 1_000_000);
        
        let thousand_dollars = calculate_token_amount(1000.0, 6);
        assert_eq!(thousand_dollars, 1_000_000_000);
    }

    #[test]
    fn test_mint_burn_operations() {
        // Simulate mint operation
        let deposit_amount = 1000.0; // $1000 USD
        let tokens_to_mint = calculate_token_amount(deposit_amount, 6);
        assert_eq!(tokens_to_mint, 1_000_000_000); // 1000 tokens with 6 decimals
        
        // Simulate burn operation
        let redemption_amount = 500.0; // $500 USD
        let tokens_to_burn = calculate_token_amount(redemption_amount, 6);
        assert_eq!(tokens_to_burn, 500_000_000); // 500 tokens with 6 decimals
        
        // Check remaining
        let remaining = tokens_to_mint - tokens_to_burn;
        assert_eq!(calculate_human_amount(remaining, 6), 500.0);
    }

    #[test]
    fn test_reserve_ratio() {
        let reserves_usd = 1_000_000.0; // $1M reserves
        let tokens_issued = 1_000_000.0; // 1M tokens
        
        let ratio = (reserves_usd / tokens_issued) * 100.0;
        assert_eq!(ratio, 100.0); // 100% backing
        
        // Test over-collateralized
        let over_collat_reserves = 1_100_000.0; // $1.1M reserves
        let over_collat_ratio = (over_collat_reserves / tokens_issued) * 100.0;
        assert_eq!(over_collat_ratio, 110.0); // 110% backing
    }

    #[test]
    fn test_price_stability_bounds() {
        let target_price = 1.0;
        let acceptable_deviation = 0.005; // 0.5%
        
        let price_scenarios = vec![0.995, 1.0, 1.005];
        
        for price in price_scenarios {
            let deviation = (price - target_price).abs();
            assert!(deviation <= acceptable_deviation, 
                   "Price {} exceeds acceptable deviation", price);
        }
    }

    #[test]
    fn test_bulk_operations() {
        let mint_operations = vec![
            calculate_token_amount(1000.0, 6),
            calculate_token_amount(5000.0, 6),
            calculate_token_amount(100000.0, 6),
        ];
        
        let total: u64 = mint_operations.iter().sum();
        let expected = calculate_token_amount(106000.0, 6);
        
        assert_eq!(total, expected);
    }

    #[test]
    fn test_fractional_cents() {
        // Test handling of fractions of cents
        let fraction = 0.001; // 0.1 cent
        let tokens = calculate_token_amount(fraction, 6);
        assert_eq!(tokens, 1000); // 0.001 * 1,000,000
        
        let back_to_usd = calculate_human_amount(tokens, 6);
        assert_eq!(back_to_usd, fraction);
    }
} 