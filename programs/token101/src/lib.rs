use anchor_lang::prelude::*;

mod constants;
mod instructions;
use instructions::*;

declare_id!("8qcRPrAkfUy9RtTUktUkjZufkmZmSMiWem9HQSgYpwEn");

#[program]
pub mod token101 {
    use super::*;

    pub fn create_deposit(ctx: Context<CreateDeposit>, amount: u64) -> Result<()> {
        instructions::create_deposit::handler(ctx, amount)
    }

    pub fn transfer_tokens(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
        instructions::transfer_tokens::handler(ctx, amount)
    }
}
