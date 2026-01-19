use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeTreasury<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init, payer = authority, space = 8 + TreasuryConfig::INIT_SPACE, seeds = [b"treasury_config"], bump)]
    pub treasury_config_account: Account<'info, TreasuryConfig>,

    #[account(init, payer = authority, space = 8 + Mint::INIT_SPACE, seeds = [b"x_mint"], bump)]
    pub x_mint: Account<'info, Mint>,

    pub treasury_token_account: Account<'info, TokenAccount>,

    #[account(mut,seeds = [b"sol_vault"], bump)]  
    pub sol_vault: Account<'info, TokenAccount>,

    #[account(seeds = [b"mint_authority"], bump)]
    pub mint_authority: Account<'info, MintAuthority>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
