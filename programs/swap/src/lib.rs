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
        _ctx: Context<MakeOffer>,
        _id: String,
        _deposited_amount: u64,
        _wanted_amount: u64,
    ) -> Result<()> {
        instructions::make_offer::handler(_ctx, _id, _deposited_amount, _wanted_amount)
    }
}
