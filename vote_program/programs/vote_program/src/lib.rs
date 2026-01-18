use anchor_lang::prelude::*;


pub mod context;
pub use context::*;
declare_id!("6Xn8LRYdVC82AHcw8yegpjAjPatBRTnDqpdi9Jn62LJZ");

#[program]
pub mod vote_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}
