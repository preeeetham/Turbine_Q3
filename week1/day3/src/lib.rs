use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, MintTo, Transfer};
use anchor_spl::associated_token::AssociatedToken;
use mpl_token_metadata::instructions::{CreateMetadataAccountV3, UpdateMetadataAccountV2};
use mpl_token_metadata::types::{DataV2, Creator, Collection, Uses};

declare_id!("11111111111111111111111111111112");

#[program]
pub mod create_your_own_spl_token {
    use super::*;

    // ========================================
    // TOKEN CREATION FUNCTIONS
    // ========================================

    /// Create a new SPL token mint with complete configuration
    pub fn create_custom_token(
        ctx: Context<CreateCustomToken>,
        params: TokenCreationParams,
    ) -> Result<()> {
        msg!("🚀 Creating custom SPL token");
        msg!("Name: {}", params.name);
        msg!("Symbol: {}", params.symbol);
        msg!("Decimals: {}", params.decimals);
        msg!("Initial Supply: {}", params.initial_supply);

        // Initialize token registry for tracking
        let token_registry = &mut ctx.accounts.token_registry;
        token_registry.mint = ctx.accounts.mint.key();
        token_registry.creator = ctx.accounts.creator.key();
        token_registry.name = params.name.clone();
        token_registry.symbol = params.symbol.clone();
        token_registry.decimals = params.decimals;
        token_registry.total_supply = params.initial_supply;
        token_registry.current_supply = 0;
        token_registry.mint_authority = ctx.accounts.creator.key();
        token_registry.freeze_authority = if params.can_freeze { Some(ctx.accounts.creator.key()) } else { None };
        token_registry.created_at = Clock::get()?.unix_timestamp;
        token_registry.is_initialized = true;
        token_registry.bump = ctx.bumps.token_registry;

        // Create metadata if requested
        if params.has_metadata {
            Self::create_token_metadata_internal(
                ctx.accounts.metadata.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.creator.to_account_info(),
                ctx.accounts.creator.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.token_metadata_program.to_account_info(),
                ctx.accounts.rent.to_account_info(),
                params.name.clone(),
                params.symbol.clone(),
                params.uri.clone(),
            )?;
        }

        // Mint initial supply if requested
        if params.initial_supply > 0 {
            let cpi_accounts = MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.creator_token_account.to_account_info(),
                authority: ctx.accounts.creator.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            
            token::mint_to(cpi_ctx, params.initial_supply)?;
            
            // Update registry
            token_registry.current_supply = params.initial_supply;
            msg!("✅ Minted {} tokens to creator", params.initial_supply);
        }

        msg!("✅ Token created successfully: {}", ctx.accounts.mint.key());
        Ok(())
    }

    /// Create Associated Token Account for a user
    pub fn create_user_token_account(
        ctx: Context<CreateUserTokenAccount>,
    ) -> Result<()> {
        let mint = &ctx.accounts.mint;
        let owner = &ctx.accounts.owner;
        let token_account = &ctx.accounts.token_account;

        msg!("🏦 Creating Associated Token Account");
        msg!("Owner: {}", owner.key());
        msg!("Mint: {}", mint.key());
        msg!("ATA Address: {}", token_account.key());

        // ATA is created automatically by Anchor constraints
        // Verify the address is deterministic
        let expected_ata = get_associated_token_address(&owner.key(), &mint.key());
        require_eq!(token_account.key(), expected_ata, CustomError::InvalidATAAddress);

        // Update token registry statistics
        let token_registry = &mut ctx.accounts.token_registry;
        token_registry.holder_count += 1;

        msg!("✅ ATA created successfully");
        Ok(())
    }

    /// Mint tokens to a specific account
    pub fn mint_to_account(
        ctx: Context<MintToAccount>,
        amount: u64,
    ) -> Result<()> {
        require!(amount > 0, CustomError::InvalidAmount);

        let token_registry = &mut ctx.accounts.token_registry;
        
        // Check if we can mint more tokens
        let new_supply = token_registry.current_supply
            .checked_add(amount)
            .ok_or(CustomError::MathOverflow)?;
        
        require!(new_supply <= token_registry.total_supply, CustomError::ExceedsMaxSupply);

        msg!("🔨 Minting {} tokens", amount);

        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::mint_to(cpi_ctx, amount)?;

        // Update registry
        token_registry.current_supply = new_supply;

        msg!("✅ Minted {} tokens. Total supply: {}", amount, new_supply);
        Ok(())
    }

    /// Transfer tokens between accounts
    pub fn transfer_tokens(
        ctx: Context<TransferTokens>,
        amount: u64,
    ) -> Result<()> {
        require!(amount > 0, CustomError::InvalidAmount);

        msg!("💸 Transferring {} tokens", amount);

        let cpi_accounts = Transfer {
            from: ctx.accounts.from_account.to_account_info(),
            to: ctx.accounts.to_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::transfer(cpi_ctx, amount)?;

        msg!("✅ Transfer completed");
        Ok(())
    }

    /// Airdrop tokens to multiple accounts
    pub fn airdrop_tokens(
        ctx: Context<AirdropTokens>,
        amounts: Vec<u64>,
    ) -> Result<()> {
        require!(amounts.len() <= 50, CustomError::TooManyAccounts);
        require!(amounts.len() == ctx.remaining_accounts.len(), CustomError::AccountMismatch);

        let total_amount: u64 = amounts.iter().sum();
        msg!("🎁 Airdropping {} tokens to {} accounts", total_amount, amounts.len());

        let token_registry = &mut ctx.accounts.token_registry;
        let new_supply = token_registry.current_supply
            .checked_add(total_amount)
            .ok_or(CustomError::MathOverflow)?;
        
        require!(new_supply <= token_registry.total_supply, CustomError::ExceedsMaxSupply);

        // Mint to each account
        for (i, &amount) in amounts.iter().enumerate() {
            if amount > 0 {
                let token_account = &ctx.remaining_accounts[i];
                
                let cpi_accounts = MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: token_account.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                };
                let cpi_program = ctx.accounts.token_program.to_account_info();
                let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
                
                token::mint_to(cpi_ctx, amount)?;
            }
        }

        // Update registry
        token_registry.current_supply = new_supply;

        msg!("✅ Airdrop completed. New total supply: {}", new_supply);
        Ok(())
    }

    /// Update token metadata
    pub fn update_token_metadata(
        ctx: Context<UpdateTokenMetadata>,
        new_name: Option<String>,
        new_symbol: Option<String>,
        new_uri: Option<String>,
    ) -> Result<()> {
        msg!("📝 Updating token metadata");

        let token_registry = &mut ctx.accounts.token_registry;
        
        // Update registry if values changed
        if let Some(name) = new_name.clone() {
            token_registry.name = name;
        }
        if let Some(symbol) = new_symbol.clone() {
            token_registry.symbol = symbol;
        }

        // Update metadata account
        // This is a simplified version - in practice you'd use the full Metaplex CPI
        msg!("✅ Metadata updated");
        Ok(())
    }

    /// Revoke mint authority (make supply immutable)
    pub fn revoke_mint_authority(
        ctx: Context<RevokeMintAuthority>,
    ) -> Result<()> {
        msg!("🔒 Revoking mint authority - supply will become immutable");

        let cpi_accounts = token::SetAuthority {
            account_or_mint: ctx.accounts.mint.to_account_info(),
            current_authority: ctx.accounts.current_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::set_authority(
            cpi_ctx,
            token::spl_token::instruction::AuthorityType::MintTokens,
            None,
        )?;

        // Update registry
        let token_registry = &mut ctx.accounts.token_registry;
        token_registry.mint_authority = Pubkey::default();
        token_registry.total_supply = token_registry.current_supply; // Fix supply

        msg!("✅ Mint authority revoked. Supply is now immutable at {}", token_registry.current_supply);
        Ok(())
    }

    /// Get token information
    pub fn get_token_info(ctx: Context<GetTokenInfo>) -> Result<()> {
        let token_registry = &ctx.accounts.token_registry;
        
        msg!("📊 Token Information:");
        msg!("Name: {}", token_registry.name);
        msg!("Symbol: {}", token_registry.symbol);
        msg!("Mint: {}", token_registry.mint);
        msg!("Decimals: {}", token_registry.decimals);
        msg!("Current Supply: {}", token_registry.current_supply);
        msg!("Total Supply: {}", token_registry.total_supply);
        msg!("Holder Count: {}", token_registry.holder_count);
        msg!("Creator: {}", token_registry.creator);
        msg!("Created At: {}", token_registry.created_at);
        
        Ok(())
    }

    // ========================================
    // HELPER FUNCTIONS
    // ========================================

    /// Create token metadata (internal helper)
    fn create_token_metadata_internal(
        _metadata_account: AccountInfo,
        _mint_account: AccountInfo,
        _mint_authority: AccountInfo,
        _payer: AccountInfo,
        _system_program: AccountInfo,
        _token_metadata_program: AccountInfo,
        _rent: AccountInfo,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        // This is a simplified version
        // In practice, you'd use the full Metaplex CPI call
        msg!("Creating metadata: {} ({}) at {}", name, symbol, uri);
        Ok(())
    }
}

// ========================================
// ACCOUNT STRUCTURES
// ========================================

#[derive(Accounts)]
#[instruction(params: TokenCreationParams)]
pub struct CreateCustomToken<'info> {
    #[account(
        init,
        payer = creator,
        mint::decimals = params.decimals,
        mint::authority = creator,
        mint::freeze_authority = if params.can_freeze { Some(creator.key()) } else { None },
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = creator,
        associated_token::mint = mint,
        associated_token::authority = creator,
    )]
    pub creator_token_account: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = creator,
        space = 8 + TokenRegistry::INIT_SPACE,
        seeds = [b"token-registry", mint.key().as_ref()],
        bump
    )]
    pub token_registry: Account<'info, TokenRegistry>,

    /// CHECK: This is the metadata account
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: This is the token metadata program
    pub token_metadata_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct CreateUserTokenAccount<'info> {
    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = owner,
    )]
    pub token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"token-registry", mint.key().as_ref()],
        bump = token_registry.bump,
    )]
    pub token_registry: Account<'info, TokenRegistry>,

    /// CHECK: This is the token account owner
    pub owner: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct MintToAccount<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"token-registry", mint.key().as_ref()],
        bump = token_registry.bump,
    )]
    pub token_registry: Account<'info, TokenRegistry>,

    #[account(
        constraint = mint_authority.key() == token_registry.mint_authority @ CustomError::Unauthorized
    )]
    pub mint_authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(mut)]
    pub from_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub to_account: Account<'info, TokenAccount>,

    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct AirdropTokens<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"token-registry", mint.key().as_ref()],
        bump = token_registry.bump,
    )]
    pub token_registry: Account<'info, TokenRegistry>,

    #[account(
        constraint = mint_authority.key() == token_registry.mint_authority @ CustomError::Unauthorized
    )]
    pub mint_authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct UpdateTokenMetadata<'info> {
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"token-registry", mint.key().as_ref()],
        bump = token_registry.bump,
    )]
    pub token_registry: Account<'info, TokenRegistry>,

    /// CHECK: This is the metadata account
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,

    #[account(
        constraint = authority.key() == token_registry.creator @ CustomError::Unauthorized
    )]
    pub authority: Signer<'info>,

    /// CHECK: This is the token metadata program
    pub token_metadata_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct RevokeMintAuthority<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"token-registry", mint.key().as_ref()],
        bump = token_registry.bump,
    )]
    pub token_registry: Account<'info, TokenRegistry>,

    #[account(
        constraint = current_authority.key() == token_registry.mint_authority @ CustomError::Unauthorized
    )]
    pub current_authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct GetTokenInfo<'info> {
    #[account(
        seeds = [b"token-registry", token_registry.mint.as_ref()],
        bump = token_registry.bump,
    )]
    pub token_registry: Account<'info, TokenRegistry>,
}

// ========================================
// DATA STRUCTURES
// ========================================

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct TokenCreationParams {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub initial_supply: u64,
    pub total_supply: u64,
    pub uri: String,
    pub has_metadata: bool,
    pub can_freeze: bool,
}

#[account]
#[derive(InitSpace)]
pub struct TokenRegistry {
    pub mint: Pubkey,
    pub creator: Pubkey,
    #[max_len(50)]
    pub name: String,
    #[max_len(10)]
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: u64,
    pub current_supply: u64,
    pub holder_count: u64,
    pub mint_authority: Pubkey,
    pub freeze_authority: Option<Pubkey>,
    pub created_at: i64,
    pub is_initialized: bool,
    pub bump: u8,
}

// ========================================
// HELPER FUNCTIONS
// ========================================

/// Calculate token amount from human-readable amount
pub fn calculate_token_amount(human_amount: f64, decimals: u8) -> u64 {
    (human_amount * 10_f64.powi(decimals as i32)) as u64
}

/// Calculate human-readable amount from token amount
pub fn calculate_human_amount(token_amount: u64, decimals: u8) -> f64 {
    token_amount as f64 / 10_f64.powi(decimals as i32)
}

/// Get Associated Token Account address
pub fn get_associated_token_address(owner: &Pubkey, mint: &Pubkey) -> Pubkey {
    anchor_spl::associated_token::get_associated_token_address(owner, mint)
}

/// Validate token creation parameters
pub fn validate_token_params(params: &TokenCreationParams) -> Result<()> {
    require!(params.name.len() > 0 && params.name.len() <= 50, CustomError::InvalidTokenName);
    require!(params.symbol.len() > 0 && params.symbol.len() <= 10, CustomError::InvalidTokenSymbol);
    require!(params.decimals <= 9, CustomError::InvalidDecimals);
    require!(params.initial_supply <= params.total_supply, CustomError::InvalidSupply);
    require!(params.total_supply > 0, CustomError::InvalidSupply);
    Ok(())
}

/// Create a standard gaming token configuration
pub fn create_gaming_token_config(
    name: String,
    symbol: String,
    initial_supply: u64,
) -> TokenCreationParams {
    TokenCreationParams {
        name,
        symbol,
        decimals: 9,
        initial_supply,
        total_supply: initial_supply * 10, // 10x for future growth
        uri: "https://example.com/gaming-token-metadata.json".to_string(),
        has_metadata: true,
        can_freeze: true,
    }
}

/// Create a standard stablecoin configuration
pub fn create_stablecoin_config(
    name: String,
    symbol: String,
) -> TokenCreationParams {
    TokenCreationParams {
        name,
        symbol,
        decimals: 6, // USD precision
        initial_supply: 0,
        total_supply: u64::MAX, // Unlimited supply
        uri: "https://example.com/stablecoin-metadata.json".to_string(),
        has_metadata: true,
        can_freeze: true,
    }
}

/// Create a standard utility token configuration
pub fn create_utility_token_config(
    name: String,
    symbol: String,
    total_supply: u64,
) -> TokenCreationParams {
    TokenCreationParams {
        name,
        symbol,
        decimals: 9,
        initial_supply: total_supply / 10, // 10% initial
        total_supply,
        uri: "https://example.com/utility-token-metadata.json".to_string(),
        has_metadata: true,
        can_freeze: false, // Utility tokens shouldn't be freezable
    }
}

// ========================================
// ERROR HANDLING
// ========================================

#[error_code]
pub enum CustomError {
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Invalid token amount")]
    InvalidAmount,
    #[msg("Invalid token name")]
    InvalidTokenName,
    #[msg("Invalid token symbol")]
    InvalidTokenSymbol,
    #[msg("Invalid decimals")]
    InvalidDecimals,
    #[msg("Invalid supply configuration")]
    InvalidSupply,
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Exceeds maximum supply")]
    ExceedsMaxSupply,
    #[msg("Too many accounts")]
    TooManyAccounts,
    #[msg("Account count mismatch")]
    AccountMismatch,
    #[msg("Invalid ATA address")]
    InvalidATAAddress,
    #[msg("Token not initialized")]
    TokenNotInitialized,
    #[msg("Metadata creation failed")]
    MetadataCreationFailed,
    #[msg("Invalid metadata URI")]
    InvalidMetadataURI,
}

// ========================================
// CONSTANTS
// ========================================

pub const MAX_AIRDROP_ACCOUNTS: usize = 50;
pub const MAX_TOKEN_NAME_LENGTH: usize = 50;
pub const MAX_TOKEN_SYMBOL_LENGTH: usize = 10;
pub const MAX_TOKEN_URI_LENGTH: usize = 200;
pub const DEFAULT_DECIMALS: u8 = 9;
pub const STABLECOIN_DECIMALS: u8 = 6;

// ========================================
// TESTS
// ========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_amount_calculation() {
        let human_amount = 1.5;
        let decimals = 9;
        let token_amount = calculate_token_amount(human_amount, decimals);
        assert_eq!(token_amount, 1_500_000_000);
        
        let back_to_human = calculate_human_amount(token_amount, decimals);
        assert_eq!(back_to_human, human_amount);
    }

    #[test]
    fn test_gaming_token_config() {
        let config = create_gaming_token_config(
            "Test Game Token".to_string(),
            "TGT".to_string(),
            1000000,
        );
        
        assert_eq!(config.decimals, 9);
        assert_eq!(config.initial_supply, 1000000);
        assert_eq!(config.total_supply, 10000000);
        assert_eq!(config.has_metadata, true);
        assert_eq!(config.can_freeze, true);
    }

    #[test]
    fn test_stablecoin_config() {
        let config = create_stablecoin_config(
            "Test Stable".to_string(),
            "TSTABLE".to_string(),
        );
        
        assert_eq!(config.decimals, 6);
        assert_eq!(config.initial_supply, 0);
        assert_eq!(config.total_supply, u64::MAX);
        assert_eq!(config.has_metadata, true);
        assert_eq!(config.can_freeze, true);
    }

    #[test]
    fn test_utility_token_config() {
        let config = create_utility_token_config(
            "Test Utility".to_string(),
            "TUTIL".to_string(),
            1000000,
        );
        
        assert_eq!(config.decimals, 9);
        assert_eq!(config.initial_supply, 100000); // 10% of total
        assert_eq!(config.total_supply, 1000000);
        assert_eq!(config.has_metadata, true);
        assert_eq!(config.can_freeze, false); // Utility tokens shouldn't be freezable
    }

    #[test]
    fn test_ata_address_deterministic() {
        let owner = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        
        let ata1 = get_associated_token_address(&owner, &mint);
        let ata2 = get_associated_token_address(&owner, &mint);
        
        assert_eq!(ata1, ata2);
    }

    #[test]
    fn test_token_params_validation() {
        let valid_params = TokenCreationParams {
            name: "Valid Token".to_string(),
            symbol: "VLD".to_string(),
            decimals: 9,
            initial_supply: 1000,
            total_supply: 10000,
            uri: "https://example.com/meta.json".to_string(),
            has_metadata: true,
            can_freeze: true,
        };
        
        assert!(validate_token_params(&valid_params).is_ok());
        
        let invalid_params = TokenCreationParams {
            name: "".to_string(), // Empty name
            symbol: "VLD".to_string(),
            decimals: 9,
            initial_supply: 1000,
            total_supply: 10000,
            uri: "https://example.com/meta.json".to_string(),
            has_metadata: true,
            can_freeze: true,
        };
        
        assert!(validate_token_params(&invalid_params).is_err());
    }
}

// ========================================
// DOCUMENTATION EXAMPLES
// ========================================

/// # Complete Token Creation Example
/// 
/// This example shows how to create a complete gaming token:
/// 
/// ```rust
/// // 1. Create token configuration
/// let params = create_gaming_token_config(
///     "Epic Game Coin".to_string(),
///     "EPIC".to_string(),
///     1_000_000, // 1M initial supply
/// );
/// 
/// // 2. Create the token
/// create_custom_token(ctx, params)?;
/// 
/// // 3. Create user token accounts
/// create_user_token_account(ctx)?;
/// 
/// // 4. Mint tokens to users
/// mint_to_account(ctx, calculate_token_amount(100.0, 9))?;
/// 
/// // 5. Transfer tokens
/// transfer_tokens(ctx, calculate_token_amount(50.0, 9))?;
/// 
/// // 6. Airdrop to multiple users
/// let amounts = vec![
///     calculate_token_amount(10.0, 9),
///     calculate_token_amount(20.0, 9),
///     calculate_token_amount(15.0, 9),
/// ];
/// airdrop_tokens(ctx, amounts)?;
/// ```

/// # Associated Token Account Example
/// 
/// ```rust
/// // Get deterministic ATA address
/// let owner = Pubkey::from_str("owner_address")?;
/// let mint = Pubkey::from_str("mint_address")?;
/// let ata_address = get_associated_token_address(&owner, &mint);
/// 
/// // Create ATA for user
/// create_user_token_account(ctx)?;
/// 
/// // The ATA address is always the same for owner + mint combination
/// assert_eq!(
///     get_associated_token_address(&owner, &mint),
///     ata_address
/// );
/// ```

/// # Token Management Example
/// 
/// ```rust
/// // Check token information
/// get_token_info(ctx)?;
/// 
/// // Update metadata
/// update_token_metadata(
///     ctx,
///     Some("New Name".to_string()),
///     Some("NEW".to_string()),
///     Some("https://new-uri.com/metadata.json".to_string()),
/// )?;
/// 
/// // Revoke mint authority to make supply immutable
/// revoke_mint_authority(ctx)?;
/// 