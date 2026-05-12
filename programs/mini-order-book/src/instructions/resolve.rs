use std::cmp::min;

use anchor_lang::prelude::*;

use crate::state::{Ask, Bid, Market, OrderStatus};

#[derive(Accounts)]
pub struct Resolve<'info> {
    #[account(mut)]
    pub resolver: Signer<'info>,

    #[account(
        mut,
        seeds = [b"market"],
        bump = market.bump
    )]
    pub market: Account<'info, Market>,

    #[account(
        mut,
        seeds = [b"ask", ask.seller.as_ref(), ask.id.to_le_bytes().as_ref()],
        bump
    )]
    pub ask: Account<'info, Ask>,

    #[account(
        mut,
        seeds = [b"bid", bid.buyer.as_ref(), bid.id.to_le_bytes().as_ref()],
        bump
    )]
    pub bid: Account<'info, Bid>,
}

impl<'info> Resolve<'info> {
    pub fn resolve(&mut self) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;

        let fill_size = min(self.ask.remaining_size, self.bid.remaining_size);
        let execution_price = self.ask.price;

        let slippage = self.bid.price - self.ask.price;
        let national = execution_price * fill_size;

        self.ask.remaining_size -= fill_size;
        self.bid.remaining_size -= fill_size;

        if self.ask.remaining_size == 0 {
            self.ask.status = OrderStatus::Filled
        } else if self.ask.original_size > self.ask.remaining_size {
            self.ask.status = OrderStatus::PartiallyFilled
        } else {
            self.ask.status = OrderStatus::Open
        }

        if self.bid.remaining_size == 0 {
            self.bid.status = OrderStatus::Filled
        } else if self.bid.original_size > self.bid.remaining_size {
            self.bid.status = OrderStatus::PartiallyFilled
        } else {
            self.bid.status = OrderStatus::Open
        }

        Ok(())
    }
}
