use anchor_lang::prelude::*;

mod constants;
mod error;
mod instructions;
mod models;
use constants::*;
use instructions::*;
use models::*;

declare_id!("uEd5zro44qqnFg67EAYxZ6uYYVFhg5Hkdvkx4mo7RQE");

#[program]
pub mod swap {
    use super::*;

    pub fn make_offer(ctx: Context<MakeOffer>) -> Result<()> {
        instructions::make_offer::handler(ctx)
    }
}
