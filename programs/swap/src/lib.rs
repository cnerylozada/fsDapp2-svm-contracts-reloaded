use anchor_lang::prelude::*;

mod constants;
mod errors;
mod instructions;
mod models;
use instructions::*;

declare_id!("uEd5zro44qqnFg67EAYxZ6uYYVFhg5Hkdvkx4mo7RQE");

#[program]
pub mod swap {
    use super::*;

    pub fn make_offer(
        ctx: Context<MakeOffer>,
        id: [u8; 16],
        offered_amount: u64,
        wanted_amount: u64,
    ) -> Result<()> {
        instructions::make_offer::handler(ctx, id, offered_amount, wanted_amount)
    }

    pub fn take_offer(ctx: Context<TakeOffer>) -> Result<()> {
        instructions::take_offer::handler(ctx)
    }
}
