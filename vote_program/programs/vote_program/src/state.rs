#[account]
pub struct TreasuryAccount {
  pub authority: Pubkey,
  pub x_mint: Pubkey,
  pub treasury_token_account:Pubkey,
  sol_price: u64,
  pub token_perpurchased: u64,
  pub bump: u8,
}

