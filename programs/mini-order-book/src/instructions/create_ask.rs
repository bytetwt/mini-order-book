use anchor_lang::prelude::*;

use crate::state::{Ask, Market, OrderStatus};

#[derive(Accounts)]
pub struct CreateAsk<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
        mut,
        seeds = [b"market"],
        bump = market.bump
    )]
    pub market: Account<'info, Market>,

    #[account(
        init,
        payer = seller,
        seeds = [b"ask", seller.key.as_ref(), market.ask_count.to_le_bytes().as_ref()],
        space = Ask::DISCRIMINATOR.len() + Ask::INIT_SPACE,
        bump
    )]
    pub ask: Account<'info, Ask>,

    pub system_program: Program<'info, System>,
}

impl<'info> CreateAsk<'info> {
    pub fn create_ask(&mut self, price: u64, size: u64, bumps: CreateAskBumps) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;

        self.ask.set_inner(Ask {
            id: self.market.ask_count,
            market: self.market.key(),
            seller: self.seller.key(),
            price,
            original_size: size,
            remaining_size: size,
            status: OrderStatus::Open,
            created_at: current_time,
            bump: bumps.ask,
        });

        self.market.ask_count += 1;
        Ok(())
    }
}
