#[account]
pub struct TreasuryAccount {
  pub authority: Pubkey,
  pub x_mint: Pubkey,
  pub treasury_token_account:Pubkey,
  sol_price: u64,
  pub token_perpurchased: u64,
  pub bump: u8,
}

#[derive(Accounts)]
pub struct InitializeTreasuryAccount<'info> {
  #[account(init, payer = authority, space = 8 + 32 + 32 + 32 + 8 + 8 + 1, seeds = [b"treasury"], bump)]
  pub treasury_account: Account<'info, TreasuryAccount>,
  pub authority: Signer<'info>,
  pub system_program: Program<'info, System>,
}