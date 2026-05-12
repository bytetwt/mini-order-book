use anchor_lang::prelude::*;

use crate::state::{Bid, Market, OrderStatus};

#[derive(Accounts)]
pub struct CreateBid<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"market"],
        bump = market.bump
    )]
    pub market: Account<'info, Market>,

    #[account(
        init,
        payer = buyer,
        seeds = [b"bid", buyer.key.as_ref(), market.bid_count.to_le_bytes().as_ref()],
        space = Bid::DISCRIMINATOR.len() + Bid::INIT_SPACE,
        bump
    )]
    pub bid: Account<'info, Bid>,

    pub system_program: Program<'info, System>,
}

impl<'info> CreateBid<'info> {
    pub fn create_bid(&mut self, price: u64, size: u64, bumps: CreateBidBumps) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;

        self.bid.set_inner(Bid {
            market: self.market.key(),
            buyer: self.buyer.key(),
            price,
            original_size: size,
            remaining_size: size,
            status: OrderStatus::Open,
            created_at: current_time,
            bump: bumps.bid,
        });

        self.market.bid_count += 1;
        Ok(())
    }
}
