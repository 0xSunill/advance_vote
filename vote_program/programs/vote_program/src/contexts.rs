use anchor_lang::prelude::*;
#[derive(Accounts)]
pub struct Initialize<'info> {
  #[account(init, payer = authority, space = 8 + 32 + 32 + 32 + 8 + 8 + 1, seeds = [b"treasury"], bump)]
  pub treasury_account: Account<'info, TreasuryAccount>,
  pub authority: Signer<'info>,
  pub system_program: Program<'info, System>,
}
 