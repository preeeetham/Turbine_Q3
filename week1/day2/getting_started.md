# Getting Started with SPL Token Deep Dive

## Quick Start Guide

### 1. Build the Project
```bash
cd week1/day2
cargo build
```

### 2. Run Tests
```bash
cargo test
```

### 3. Run Examples
```bash
cargo run --example token_operations
```

### 4. Key Concepts to Understand

#### Token Creation Flow
```rust
// 1. Create Token Mint
create_token_mint(
    ctx,
    9,                                    // decimals
    "My Game Token".to_string(),          // name
    "GAME".to_string(),                   // symbol  
    "https://example.com/meta.json".to_string(), // metadata URI
    1_000_000_000_000_000,               // initial supply (1M tokens)
)

// 2. Mint Additional Tokens
mint_tokens(ctx, calculate_token_amount(5000.0, 9))

// 3. Transfer Tokens
transfer_tokens(ctx, calculate_token_amount(100.0, 9))
```

#### Batch Operations
```rust
// Mint to multiple accounts at once
let amounts = vec![
    calculate_token_amount(100.0, 9),
    calculate_token_amount(200.0, 9), 
    calculate_token_amount(150.0, 9),
];
batch_mint_tokens(ctx, amounts)
```

### 5. Common Token Patterns

#### Gaming Token (9 decimals)
- Allows micro-transactions
- Good for in-game currencies
- Example: 0.001 tokens for small actions

#### Stablecoin (6 decimals) 
- Matches USD precision ($0.01)
- Used for payments and trading
- Example: USDC pattern

#### Utility Token (2-4 decimals)
- Simple counting tokens
- Good for NFTs or limited items
- Example: Event tickets, collectibles

### 6. Security Best Practices

1. **Authority Management**
   ```rust
   // Revoke mint authority when supply is final
   revoke_mint_authority(ctx)
   
   // Revoke freeze authority for immutable tokens
   revoke_freeze_authority(ctx)
   ```

2. **Input Validation**
   ```rust
   require!(amount > 0, CustomError::InvalidAmount);
   require!(amounts.len() <= MAX_BATCH_SIZE, CustomError::TooManyAccounts);
   ```

3. **Overflow Protection**
   ```rust
   token_info.total_supply = token_info.total_supply
       .checked_add(amount)
       .ok_or(CustomError::MathOverflow)?;
   ```

### 7. Testing Your Token

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_token_amount_calculation

# Test with output
cargo test -- --nocapture
```

### 8. Next Steps

1. **Study the Code**: Read through `src/lib.rs` carefully
2. **Modify Examples**: Try changing token parameters 
3. **Deploy to Devnet**: Test with real Solana transactions
4. **Build Applications**: Create programs that use your tokens

### 9. Common Issues

#### Build Errors
- Make sure you have the latest Rust and Anchor versions
- Check that all dependencies are properly specified

#### Test Failures  
- Verify your token calculations are correct
- Check that account constraints are properly set

#### Deployment Issues
- Ensure you have enough SOL for transaction fees
- Verify your program ID is correctly set

### 10. Resources

- [Solana Documentation](https://docs.solana.com/)
- [Anchor Book](https://book.anchor-lang.com/)
- [SPL Token Program](https://spl.solana.com/token)

Happy coding! 🚀 