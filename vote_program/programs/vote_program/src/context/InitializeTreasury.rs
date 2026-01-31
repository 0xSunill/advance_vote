use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_lang::solana_program::pubkey::Pubkey;

#[derive(Accounts)]
pub struct InitializeTreasury<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init, payer = authority, space = 8 + TreasuryConfig::INIT_SPACE, seeds = [b"treasury_config"], bump)]
    pub treasury_config_account: Account<'info, TreasuryConfig>,

    #[account(init, payer = authority,mint::authority = mint_authority , mint::decimals = 6, seeds = [b"x_mint"], bump)]
    pub x_mint: Account<'info, Mint>,

    #[account(init, payer = authority, associated_token::mint = x_mint , associated_token::authority =authority )]
    pub treasury_token_account: Account<'info, TokenAccount>,

    #[account(mut, seeds = [b"sol_vault"], bump)]
    pub sol_vault: SystemAccount<'info>,

    #[account(init, payer = authority, space = 8, seeds = [b"mint_authority"], bump)]
    pub mint_authority: Account<'info, MintAuthority>,

    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
}

impl <'info> InitializeTreasury<'info> {
    
    pub fn init_treasury(&mut self, bumps: &Bumps) -> Result<()> {
        self.treasury_config_account.authority = self.authority.key();
        self.treasury_config_account.x_mint = self.x_mint.key();
        self.treasury_config_account.treasury_token_account = self.treasury_token_account.key();
        self.treasury_config_account.sol_price = 100;
        self.treasury_config_account.token_per_purchased = 0;
        self.treasury_config_account.bump = bumps.treasury_config_account;
        Ok(())
    }
}
