use anchor_lang::prelude::*;

mod constants;
mod errors;
mod instructions;
mod models;
use instructions::*;

declare_id!("CGUBBtJSaJXcrieV9x7KYGTsdJJuRb819NxbKzCo7EFM");

#[program]
pub mod simple_transfer {
    use super::*;

    pub fn create_deposit(ctx: Context<CreateDeposit>, goal: String, amount: u64) -> Result<()> {
        create_deposit::handler(ctx, goal, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, goal: String, amount: u64) -> Result<()> {
        withdraw::handler(ctx, goal, amount)
    }
}
