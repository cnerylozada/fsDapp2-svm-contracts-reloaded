use anchor_lang::prelude::*;

mod constants;
mod instructions;
use instructions::*;

declare_id!("8qcRPrAkfUy9RtTUktUkjZufkmZmSMiWem9HQSgYpwEn");

#[program]
pub mod token101 {
    use super::*;

    pub fn create_deposit(_ctx: Context<CreateDeposit>, _amount: u64) -> Result<()> {
        instructions::create_deposit::handler(_ctx, _amount)
    }

    pub fn transfer_tokens(_ctx: Context<TransferTokens>, _amount: u64) -> Result<()> {
        instructions::transfer_tokens::handler(_ctx, _amount)
    }
}
