use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Market {
    pub authority: Pubkey,
    pub ask_count: u64,
    pub bid_count: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Ask {
    pub id: u64,
    pub market: Pubkey,
    pub seller: Pubkey,
    pub price: u64,
    pub original_size: u64,
    pub remaining_size: u64,
    pub status: OrderStatus,
    pub created_at: i64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Bid {
    pub id: u64,
    pub market: Pubkey,
    pub buyer: Pubkey,
    pub price: u64,
    pub original_size: u64,
    pub remaining_size: u64,
    pub status: OrderStatus,
    pub created_at: i64,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub enum OrderStatus {
    Open,
    PartiallyFilled,
    Filled,
    Cancelled,
}
