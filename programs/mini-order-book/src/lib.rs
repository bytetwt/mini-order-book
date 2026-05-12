use anchor_lang::prelude::*;

declare_id!("HhYnYsWtT6nCWL3Mw4dZgoicScQd8VcwHUJfk8fdv8Lq");

#[program]
pub mod mini_order_book {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
