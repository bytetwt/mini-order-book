use anchor_lang::prelude::*;

declare_id!("HhYnYsWtT6nCWL3Mw4dZgoicScQd8VcwHUJfk8fdv8Lq");

pub mod instructions;
pub mod state;

pub use instructions::*;

#[program]
pub mod mini_order_book {
    use super::*;

    pub fn initialize_market(ctx: Context<InitializeMarket>) -> Result<()> {
        ctx.accounts.initialize_market(ctx.bumps)?;

        Ok(())
    }

    pub fn create_ask(ctx: Context<CreateAsk>, price: u64, size: u64) -> Result<()> {
        ctx.accounts.create_ask(price, size, ctx.bumps)?;

        Ok(())
    }

    pub fn create_bid(ctx: Context<CreateBid>, price: u64, size: u64) -> Result<()> {
        ctx.accounts.create_bid(price, size, ctx.bumps)?;

        Ok(())
    }
}
