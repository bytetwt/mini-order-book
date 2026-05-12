use anchor_lang::prelude::*;

use crate::state::Market;

#[derive(Accounts)]
pub struct InitializeMarket<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        seeds = [b"market"],
        space = Market::DISCRIMINATOR.len() + Market::INIT_SPACE,
        bump
    )]
    pub market: Account<'info, Market>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeMarket<'info> {
    pub fn initialize_market(&mut self, bumps: InitializeMarketBumps) -> Result<()> {
        self.market.set_inner(Market {
            authority: self.signer.key(),
            ask_count: 0,
            bid_count: 0,
            bump: bumps.market,
        });

        Ok(())
    }
}
