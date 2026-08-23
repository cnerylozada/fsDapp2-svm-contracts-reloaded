use anchor_lang::prelude::*;

mod constants;
mod instructions;
mod models;
use instructions::*;

declare_id!("CGUBBtJSaJXcrieV9x7KYGTsdJJuRb819NxbKzCo7EFM");

#[program]
pub mod simple_transfer {
    use super::*;

    pub fn create_account(_ctx: Context<CreateAccount>, _goal: String) -> Result<()> {
        create_account::handler(_ctx, _goal)
    }

    pub fn deposit(_ctx: Context<Deposit>, _goal: String, _amount: u64) -> Result<()> {
        deposit::handler(_ctx, _goal, _amount)
    }
}
