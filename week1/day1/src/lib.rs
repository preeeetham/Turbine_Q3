use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint};
use anchor_spl::associated_token::AssociatedToken;
use mpl_token_metadata::state::DataV2;

declare_id!("11111111111111111111111111111112");

#[program]
pub mod solana_examples {
    use super::*;

    /// Initialize a PDA (Program Derived Address) account
    pub fn initialize_pda(ctx: Context<InitializePDA>, bump: u8) -> Result<()> {
        let pda_account = &mut ctx.accounts.pda_account;
        pda_account.data = "Hello from PDA".to_string();
        pda_account.bump = bump;
        pda_account.authority = ctx.accounts.authority.key();
        Ok(())
    }

    /// Create a new SPL token mint
    pub fn create_token_mint(
        ctx: Context<CreateTokenMint>,
        decimals: u8,
        name: String,
        symbol: String,
    ) -> Result<()> {
        // Token mint is created through CPI to SPL Token program
        // Metadata can be added for additional token information
        msg!("Creating token mint with name: {}, symbol: {}", name, symbol);
        Ok(())
    }

    /// Mint SPL tokens to a token account
    pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        let cpi_accounts = token::MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::mint_to(cpi_ctx, amount)?;
        Ok(())
    }

    /// Transfer SPL tokens between accounts
    pub fn transfer_tokens(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
        let cpi_accounts = token::Transfer {
            from: ctx.accounts.from_account.to_account_info(),
            to: ctx.accounts.to_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::transfer(cpi_ctx, amount)?;
        Ok(())
    }

    /// Create an NFT (Non-Fungible Token)
    pub fn create_nft(
        ctx: Context<CreateNFT>,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        // First mint exactly 1 token (NFT characteristic)
        let cpi_accounts = token::MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::mint_to(cpi_ctx, 1)?;

        // Create metadata for the NFT
        msg!("Creating NFT: {} ({}) with URI: {}", name, symbol, uri);
        
        Ok(())
    }

    /// Update PDA data
    pub fn update_pda_data(ctx: Context<UpdatePDAData>, new_data: String) -> Result<()> {
        let pda_account = &mut ctx.accounts.pda_account;
        pda_account.data = new_data;
        Ok(())
    }
}

// Account structures for PDA operations
#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitializePDA<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 4 + 200 + 1, // discriminator + pubkey + string length + string + bump
        seeds = [b"pda", authority.key().as_ref()],
        bump
    )]
    pub pda_account: Account<'info, PDAAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdatePDAData<'info> {
    #[account(
        mut,
        seeds = [b"pda", authority.key().as_ref()],
        bump = pda_account.bump,
        has_one = authority
    )]
    pub pda_account: Account<'info, PDAAccount>,
    pub authority: Signer<'info>,
}

// Account structures for SPL Token operations
#[derive(Accounts)]
pub struct CreateTokenMint<'info> {
    #[account(
        init,
        payer = authority,
        mint::decimals = 9,
        mint::authority = authority,
        mint::freeze_authority = authority,
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
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

// Account structures for NFT operations
#[derive(Accounts)]
pub struct CreateNFT<'info> {
    #[account(
        init,
        payer = mint_authority,
        mint::decimals = 0, // NFTs have 0 decimals
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
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

// Custom account types
#[account]
pub struct PDAAccount {
    pub data: String,
    pub bump: u8,
    pub authority: Pubkey,
}

// Utility functions and helpers
impl PDAAccount {
    pub fn size() -> usize {
        8 + 32 + 4 + 200 + 1 // discriminator + pubkey + string length + max string + bump
    }
}

// Helper functions for working with PDAs
pub fn find_pda_address(authority: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"pda", authority.as_ref()], program_id)
}

// Helper functions for SPL tokens
pub fn calculate_token_amount(human_amount: f64, decimals: u8) -> u64 {
    (human_amount * 10_f64.powi(decimals as i32)) as u64
}

pub fn calculate_human_amount(token_amount: u64, decimals: u8) -> f64 {
    token_amount as f64 / 10_f64.powi(decimals as i32)
}

// Error handling
#[error_code]
pub enum CustomError {
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Invalid token amount")]
    InvalidAmount,
    #[msg("NFT supply must be exactly 1")]
    InvalidNFTSupply,
    #[msg("Invalid metadata")]
    InvalidMetadata,
}

// Constants
pub const MAX_DATA_SIZE: usize = 200;
pub const STANDARD_TOKEN_DECIMALS: u8 = 9;
pub const NFT_DECIMALS: u8 = 0;
pub const NFT_SUPPLY: u64 = 1;

// Tests
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
    fn test_pda_size() {
        let expected_size = 8 + 32 + 4 + 200 + 1;
        assert_eq!(PDAAccount::size(), expected_size);
    }
}

// Documentation examples
/// # PDA (Program Derived Address) Example
/// 
/// PDAs are deterministic addresses derived from seeds and a program ID.
/// They allow programs to own accounts and sign transactions.
/// 
/// ```rust
/// // Find PDA address
/// let (pda_address, bump) = find_pda_address(&authority.key(), &program_id);
/// 
/// // Initialize PDA
/// // This creates an account at the derived address
/// initialize_pda(ctx, bump)?;
/// ```

/// # SPL Token Example
/// 
/// SPL tokens are fungible tokens on Solana, similar to ERC-20 on Ethereum.
/// 
/// ```rust
/// // Create a new token mint
/// create_token_mint(ctx, 9, "My Token".to_string(), "MTK".to_string())?;
/// 
/// // Mint tokens to an account
/// mint_tokens(ctx, calculate_token_amount(100.0, 9))?;
/// 
/// // Transfer tokens
/// transfer_tokens(ctx, calculate_token_amount(10.0, 9))?;
/// ```

/// # NFT (Non-Fungible Token) Example
/// 
/// NFTs on Solana are SPL tokens with specific characteristics:
/// - 0 decimals
/// - Supply of exactly 1
/// - Associated metadata
/// 
/// ```rust
/// // Create an NFT
/// create_nft(
///     ctx,
///     "My NFT".to_string(),
///     "MNFT".to_string(),
///     "https://example.com/metadata.json".to_string()
/// )?;
/// ```