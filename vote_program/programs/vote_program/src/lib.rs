use anchor_lang::prelude::*;


pub mod context;
pub mod state;
pub use state::*;
declare_id!("6Xn8LRYdVC82AHcw8yegpjAjPatBRTnDqpdi9Jn62LJZ");

#[program]
pub mod vote_program {
    use super::*;
    use crate::context::InitializeTreasury;

    pub fn initializeTreasury(ctx: Context<InitializeTreasury>) -> Result<()> {
        Ok(())
    }
}
