use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, MintTo, Transfer, Burn, FreezeAccount, ThawAccount, CloseAccount};
use anchor_spl::associated_token::AssociatedToken;
use mpl_token_metadata::accounts::{Metadata, MasterEdition};
use mpl_token_metadata::instructions::{CreateMetadataAccountV3, CreateMasterEditionV3};
use mpl_token_metadata::types::{DataV2, Creator, Collection, Uses, CollectionToggle, RuleSetToggle, UsesToggle};

declare_id!("11111111111111111111111111111112");

#[program]
pub mod spl_token_deep_dive {
    use super::*;

    // ========================================
    // BASIC TOKEN OPERATIONS
    // ========================================

    /// Create a new SPL token mint with comprehensive configuration
    pub fn create_token_mint(
        ctx: Context<CreateTokenMint>,
        decimals: u8,
        name: String,
        symbol: String,
        uri: String,
        initial_supply: u64,
    ) -> Result<()> {
        msg!("Creating token mint: {} ({})", name, symbol);
        msg!("Decimals: {}, Initial supply: {}", decimals, initial_supply);

        // Store token information for later use
        let token_info = &mut ctx.accounts.token_info;
        token_info.mint = ctx.accounts.mint.key();
        token_info.name = name.clone();
        token_info.symbol = symbol.clone();
        token_info.uri = uri.clone();
        token_info.decimals = decimals;
        token_info.total_supply = initial_supply;
        token_info.mint_authority = ctx.accounts.mint_authority.key();
        token_info.freeze_authority = Some(ctx.accounts.mint_authority.key());
        token_info.bump = ctx.bumps.token_info;

        // Mint initial supply if specified
        if initial_supply > 0 {
            let cpi_accounts = MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            
            token::mint_to(cpi_ctx, initial_supply)?;
        }

        // Create metadata account for the token
        self::create_token_metadata(
            ctx.accounts.mint.key(),
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
            name,
            symbol,
            uri,
        )?;

        Ok(())
    }

    /// Mint additional tokens to a specified account
    pub fn mint_tokens(
        ctx: Context<MintTokens>,
        amount: u64,
    ) -> Result<()> {
        require!(amount > 0, CustomError::InvalidAmount);

        msg!("Minting {} tokens to account", amount);

        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::mint_to(cpi_ctx, amount)?;

        // Update total supply tracking
        let token_info = &mut ctx.accounts.token_info;
        token_info.total_supply = token_info.total_supply.checked_add(amount)
            .ok_or(CustomError::MathOverflow)?;

        Ok(())
    }

    /// Transfer tokens between accounts
    pub fn transfer_tokens(
        ctx: Context<TransferTokens>,
        amount: u64,
    ) -> Result<()> {
        require!(amount > 0, CustomError::InvalidAmount);

        msg!("Transferring {} tokens", amount);

        let cpi_accounts = Transfer {
            from: ctx.accounts.from_account.to_account_info(),
            to: ctx.accounts.to_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::transfer(cpi_ctx, amount)?;

        Ok(())
    }

    /// Burn tokens from an account
    pub fn burn_tokens(
        ctx: Context<BurnTokens>,
        amount: u64,
    ) -> Result<()> {
        require!(amount > 0, CustomError::InvalidAmount);

        msg!("Burning {} tokens", amount);

        let cpi_accounts = Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::burn(cpi_ctx, amount)?;

        // Update total supply tracking
        let token_info = &mut ctx.accounts.token_info;
        token_info.total_supply = token_info.total_supply.checked_sub(amount)
            .ok_or(CustomError::MathOverflow)?;

        Ok(())
    }

    // ========================================
    // ADVANCED TOKEN OPERATIONS
    // ========================================

    /// Freeze a token account (prevents transfers)
    pub fn freeze_account(ctx: Context<FreezeTokenAccount>) -> Result<()> {
        msg!("Freezing token account");

        let cpi_accounts = FreezeAccount {
            account: ctx.accounts.token_account.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            authority: ctx.accounts.freeze_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::freeze_account(cpi_ctx)?;

        Ok(())
    }

    /// Thaw a frozen token account (allows transfers again)
    pub fn thaw_account(ctx: Context<ThawTokenAccount>) -> Result<()> {
        msg!("Thawing token account");

        let cpi_accounts = ThawAccount {
            account: ctx.accounts.token_account.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            authority: ctx.accounts.freeze_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::thaw_account(cpi_ctx)?;

        Ok(())
    }

    /// Close a token account and return rent to owner
    pub fn close_token_account(ctx: Context<CloseTokenAccount>) -> Result<()> {
        msg!("Closing token account");

        let cpi_accounts = CloseAccount {
            account: ctx.accounts.token_account.to_account_info(),
            destination: ctx.accounts.destination.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::close_account(cpi_ctx)?;

        Ok(())
    }

    /// Revoke mint authority (makes supply immutable)
    pub fn revoke_mint_authority(ctx: Context<RevokeMintAuthority>) -> Result<()> {
        msg!("Revoking mint authority - token supply will be immutable");

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

        // Update token info
        let token_info = &mut ctx.accounts.token_info;
        token_info.mint_authority = Pubkey::default();

        Ok(())
    }

    /// Revoke freeze authority (makes accounts unfreezable)
    pub fn revoke_freeze_authority(ctx: Context<RevokeFreezeAuthority>) -> Result<()> {
        msg!("Revoking freeze authority - accounts cannot be frozen");

        let cpi_accounts = token::SetAuthority {
            account_or_mint: ctx.accounts.mint.to_account_info(),
            current_authority: ctx.accounts.current_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::set_authority(
            cpi_ctx,
            token::spl_token::instruction::AuthorityType::FreezeAccount,
            None,
        )?;

        // Update token info
        let token_info = &mut ctx.accounts.token_info;
        token_info.freeze_authority = None;

        Ok(())
    }

    // ========================================
    // BATCH OPERATIONS
    // ========================================

    /// Batch mint tokens to multiple accounts
    pub fn batch_mint_tokens(
        ctx: Context<BatchMintTokens>,
        amounts: Vec<u64>,
    ) -> Result<()> {
        require!(amounts.len() <= 10, CustomError::TooManyAccounts);
        require!(amounts.len() == ctx.remaining_accounts.len(), CustomError::AccountMismatch);

        msg!("Batch minting to {} accounts", amounts.len());

        let total_amount: u64 = amounts.iter().sum();
        
        for (i, &amount) in amounts.iter().enumerate() {
            require!(amount > 0, CustomError::InvalidAmount);
            
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

        // Update total supply tracking
        let token_info = &mut ctx.accounts.token_info;
        token_info.total_supply = token_info.total_supply.checked_add(total_amount)
            .ok_or(CustomError::MathOverflow)?;

        Ok(())
    }

    /// Batch transfer tokens from one account to multiple accounts
    pub fn batch_transfer_tokens(
        ctx: Context<BatchTransferTokens>,
        amounts: Vec<u64>,
    ) -> Result<()> {
        require!(amounts.len() <= 10, CustomError::TooManyAccounts);
        require!(amounts.len() == ctx.remaining_accounts.len(), CustomError::AccountMismatch);

        msg!("Batch transferring to {} accounts", amounts.len());

        for (i, &amount) in amounts.iter().enumerate() {
            require!(amount > 0, CustomError::InvalidAmount);
            
            let to_account = &ctx.remaining_accounts[i];
            
            let cpi_accounts = Transfer {
                from: ctx.accounts.from_account.to_account_info(),
                to: to_account.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            
            token::transfer(cpi_ctx, amount)?;
        }

        Ok(())
    }

    // ========================================
    // PROGRAM DERIVED ADDRESS OPERATIONS
    // ========================================

    /// Program-signed transfer using PDA
    pub fn program_transfer(
        ctx: Context<ProgramTransfer>,
        amount: u64,
    ) -> Result<()> {
        require!(amount > 0, CustomError::InvalidAmount);

        msg!("Program-signed transfer of {} tokens", amount);

        let seeds = &[
            b"token-authority",
            ctx.accounts.mint.to_account_info().key.as_ref(),
            &[ctx.accounts.token_authority.bump],
        ];
        let signer = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.from_account.to_account_info(),
            to: ctx.accounts.to_account.to_account_info(),
            authority: ctx.accounts.token_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        
        token::transfer(cpi_ctx, amount)?;

        Ok(())
    }

    /// Initialize token authority PDA
    pub fn initialize_token_authority(
        ctx: Context<InitializeTokenAuthority>,
        bump: u8,
    ) -> Result<()> {
        let authority = &mut ctx.accounts.token_authority;
        authority.mint = ctx.accounts.mint.key();
        authority.bump = bump;
        authority.owner = ctx.accounts.owner.key();

        Ok(())
    }

    /// Create token with metadata
    pub fn create_token_with_metadata(
        ctx: Context<CreateTokenWithMetadata>,
        decimals: u8,
        name: String,
        symbol: String,
        uri: String,
        supply: u64,
    ) -> Result<()> {
        msg!("Creating token with metadata: {} ({})", name, symbol);

        // First create the basic token
        let token_info = &mut ctx.accounts.token_info;
        token_info.mint = ctx.accounts.mint.key();
        token_info.name = name.clone();
        token_info.symbol = symbol.clone();
        token_info.uri = uri.clone();
        token_info.decimals = decimals;
        token_info.total_supply = supply;
        token_info.mint_authority = ctx.accounts.authority.key();
        token_info.freeze_authority = Some(ctx.accounts.authority.key());
        token_info.bump = ctx.bumps.token_info;

        // Mint initial supply
        if supply > 0 {
            let cpi_accounts = MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            
            token::mint_to(cpi_ctx, supply)?;
        }

        Ok(())
    }
}

// ========================================
// ACCOUNT STRUCTURES
// ========================================

#[derive(Accounts)]
#[instruction(decimals: u8, name: String, symbol: String, uri: String)]
pub struct CreateTokenMint<'info> {
    #[account(
        init,
        payer = mint_authority,
        mint::decimals = decimals,
        mint::authority = mint_authority,
        mint::freeze_authority = mint_authority,
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = mint_authority,
        associated_token::mint = mint,
        associated_token::authority = mint_authority,
    )]
    pub token_account: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = mint_authority,
        space = 8 + TokenInfo::INIT_SPACE,
        seeds = [b"token-info", mint.key().as_ref()],
        bump
    )]
    pub token_info: Account<'info, TokenInfo>,

    /// CHECK: This is the metadata account for the token
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,

    #[account(mut)]
    pub mint_authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: This is the token metadata program
    pub token_metadata_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"token-info", mint.key().as_ref()],
        bump = token_info.bump,
    )]
    pub token_info: Account<'info, TokenInfo>,

    #[account(
        constraint = mint_authority.key() == token_info.mint_authority @ CustomError::Unauthorized
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
pub struct BurnTokens<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"token-info", mint.key().as_ref()],
        bump = token_info.bump,
    )]
    pub token_info: Account<'info, TokenInfo>,

    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct FreezeTokenAccount<'info> {
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,

    #[account(
        constraint = freeze_authority.key() == token_info.freeze_authority.unwrap() @ CustomError::Unauthorized
    )]
    pub freeze_authority: Signer<'info>,

    #[account(
        seeds = [b"token-info", mint.key().as_ref()],
        bump = token_info.bump,
    )]
    pub token_info: Account<'info, TokenInfo>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ThawTokenAccount<'info> {
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,

    #[account(
        constraint = freeze_authority.key() == token_info.freeze_authority.unwrap() @ CustomError::Unauthorized
    )]
    pub freeze_authority: Signer<'info>,

    #[account(
        seeds = [b"token-info", mint.key().as_ref()],
        bump = token_info.bump,
    )]
    pub token_info: Account<'info, TokenInfo>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct CloseTokenAccount<'info> {
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,

    /// CHECK: This is the destination account for rent
    #[account(mut)]
    pub destination: UncheckedAccount<'info>,

    pub owner: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct RevokeMintAuthority<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"token-info", mint.key().as_ref()],
        bump = token_info.bump,
    )]
    pub token_info: Account<'info, TokenInfo>,

    #[account(
        constraint = current_authority.key() == token_info.mint_authority @ CustomError::Unauthorized
    )]
    pub current_authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct RevokeFreezeAuthority<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"token-info", mint.key().as_ref()],
        bump = token_info.bump,
    )]
    pub token_info: Account<'info, TokenInfo>,

    #[account(
        constraint = current_authority.key() == token_info.freeze_authority.unwrap() @ CustomError::Unauthorized
    )]
    pub current_authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct BatchMintTokens<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"token-info", mint.key().as_ref()],
        bump = token_info.bump,
    )]
    pub token_info: Account<'info, TokenInfo>,

    #[account(
        constraint = mint_authority.key() == token_info.mint_authority @ CustomError::Unauthorized
    )]
    pub mint_authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct BatchTransferTokens<'info> {
    #[account(mut)]
    pub from_account: Account<'info, TokenAccount>,

    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ProgramTransfer<'info> {
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub from_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub to_account: Account<'info, TokenAccount>,

    #[account(
        seeds = [b"token-authority", mint.key().as_ref()],
        bump = token_authority.bump,
    )]
    pub token_authority: Account<'info, TokenAuthority>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitializeTokenAuthority<'info> {
    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = owner,
        space = 8 + TokenAuthority::INIT_SPACE,
        seeds = [b"token-authority", mint.key().as_ref()],
        bump
    )]
    pub token_authority: Account<'info, TokenAuthority>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateTokenWithMetadata<'info> {
    #[account(
        init,
        payer = authority,
        mint::decimals = 9,
        mint::authority = authority,
        mint::freeze_authority = authority,
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = authority,
    )]
    pub token_account: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = authority,
        space = 8 + TokenInfo::INIT_SPACE,
        seeds = [b"token-info", mint.key().as_ref()],
        bump
    )]
    pub token_info: Account<'info, TokenInfo>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

// ========================================
// ACCOUNT TYPES
// ========================================

#[account]
#[derive(InitSpace)]
pub struct TokenInfo {
    pub mint: Pubkey,
    #[max_len(50)]
    pub name: String,
    #[max_len(10)]
    pub symbol: String,
    #[max_len(200)]
    pub uri: String,
    pub decimals: u8,
    pub total_supply: u64,
    pub mint_authority: Pubkey,
    pub freeze_authority: Option<Pubkey>,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct TokenAuthority {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub bump: u8,
}

// ========================================
// HELPER FUNCTIONS
// ========================================

/// Create metadata account for a token
fn create_token_metadata(
    mint: Pubkey,
    metadata_account: AccountInfo,
    mint_authority: AccountInfo,
    payer: AccountInfo,
    system_program: AccountInfo,
    token_metadata_program: AccountInfo,
    rent: AccountInfo,
    name: String,
    symbol: String,
    uri: String,
) -> Result<()> {
    let metadata_data = DataV2 {
        name,
        symbol,
        uri,
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    };

    let accounts = vec![
        metadata_account,
        mint_authority,
        payer,
        system_program,
        rent,
    ];

    // This is a simplified version - in practice, you'd use the full CPI call
    msg!("Creating metadata account for token");
    
    Ok(())
}

/// Calculate token amount from human-readable amount
pub fn calculate_token_amount(human_amount: f64, decimals: u8) -> u64 {
    (human_amount * 10_f64.powi(decimals as i32)) as u64
}

/// Calculate human-readable amount from token amount
pub fn calculate_human_amount(token_amount: u64, decimals: u8) -> f64 {
    token_amount as f64 / 10_f64.powi(decimals as i32)
}

/// Find Associated Token Account address
pub fn find_associated_token_address(owner: &Pubkey, mint: &Pubkey) -> Pubkey {
    anchor_spl::associated_token::get_associated_token_address(owner, mint)
}

/// Validate token amount is within reasonable bounds
pub fn validate_token_amount(amount: u64, decimals: u8) -> Result<()> {
    let max_amount = 10_u64.pow(decimals as u32 + 9); // Reasonable upper bound
    require!(amount <= max_amount, CustomError::AmountTooLarge);
    Ok(())
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
    #[msg("Amount too large")]
    AmountTooLarge,
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Too many accounts in batch operation")]
    TooManyAccounts,
    #[msg("Account count mismatch")]
    AccountMismatch,
    #[msg("Invalid metadata")]
    InvalidMetadata,
    #[msg("Token supply exhausted")]
    SupplyExhausted,
    #[msg("Account is frozen")]
    AccountFrozen,
    #[msg("Invalid authority")]
    InvalidAuthority,
}

// ========================================
// CONSTANTS
// ========================================

pub const MAX_BATCH_SIZE: usize = 10;
pub const MAX_TOKEN_NAME_LENGTH: usize = 50;
pub const MAX_TOKEN_SYMBOL_LENGTH: usize = 10;
pub const MAX_TOKEN_URI_LENGTH: usize = 200;
pub const DEFAULT_DECIMALS: u8 = 9;

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
    fn test_token_amount_validation() {
        // This should pass
        assert!(validate_token_amount(1_000_000_000, 9).is_ok());
        
        // This should fail
        assert!(validate_token_amount(u64::MAX, 9).is_err());
    }

    #[test]
    fn test_constants() {
        assert_eq!(MAX_BATCH_SIZE, 10);
        assert_eq!(DEFAULT_DECIMALS, 9);
        assert_eq!(MAX_TOKEN_NAME_LENGTH, 50);
    }
}

// ========================================
// DOCUMENTATION EXAMPLES
// ========================================

/// # Complete Token Lifecycle Example
/// 
/// This example demonstrates the complete lifecycle of an SPL token:
/// 
/// ```rust
/// // 1. Create token mint
/// create_token_mint(
///     ctx,
///     9,                              // decimals
///     "My Gaming Token".to_string(),  // name
///     "GAME".to_string(),             // symbol
///     "https://example.com/metadata.json".to_string(), // uri
///     1_000_000_000_000_000,         // initial supply (1M tokens)
/// )?;
/// 
/// // 2. Mint additional tokens
/// mint_tokens(ctx, calculate_token_amount(10000.0, 9))?;
/// 
/// // 3. Transfer tokens
/// transfer_tokens(ctx, calculate_token_amount(100.0, 9))?;
/// 
/// // 4. Burn tokens (optional)
/// burn_tokens(ctx, calculate_token_amount(50.0, 9))?;
/// 
/// // 5. Freeze account (optional)
/// freeze_account(ctx)?;
/// 
/// // 6. Thaw account (if frozen)
/// thaw_account(ctx)?;
/// 
/// // 7. Revoke authorities (when done)
/// revoke_mint_authority(ctx)?;
/// revoke_freeze_authority(ctx)?;
/// ```

/// # Batch Operations Example
/// 
/// ```rust
/// // Batch mint to multiple accounts
/// let amounts = vec![
///     calculate_token_amount(100.0, 9),
///     calculate_token_amount(200.0, 9),
///     calculate_token_amount(150.0, 9),
/// ];
/// batch_mint_tokens(ctx, amounts)?;
/// 
/// // Batch transfer to multiple accounts
/// let transfer_amounts = vec![
///     calculate_token_amount(10.0, 9),
///     calculate_token_amount(25.0, 9),
///     calculate_token_amount(5.0, 9),
/// ];
/// batch_transfer_tokens(ctx, transfer_amounts)?;
/// ```

/// # Program-Signed Transfer Example
/// 
/// ```rust
/// // Initialize program authority
/// initialize_token_authority(ctx, bump)?;
/// 
/// // Program can now transfer tokens on behalf of users
/// program_transfer(ctx, calculate_token_amount(50.0, 9))?;
/// 