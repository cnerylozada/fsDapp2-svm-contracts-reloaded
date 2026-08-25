use anchor_lang::prelude::*;

mod constants;
mod instructions;
mod models;
use instructions::*;

declare_id!("CGUBBtJSaJXcrieV9x7KYGTsdJJuRb819NxbKzCo7EFM");

#[program]
pub mod simple_transfer {
    use super::*;

    pub fn create_deposit(_ctx: Context<CreateDeposit>, _goal: String, _amount: u64) -> Result<()> {
        create_deposit::handler(_ctx, _goal, _amount)
    }

    pub fn withdraw(_ctx: Context<Withdraw>, _goal: String, _amount: u64) -> Result<()> {
        withdraw::handler(_ctx, _goal, _amount)
    }
}
