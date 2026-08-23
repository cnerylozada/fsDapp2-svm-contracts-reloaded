use crate::constants::VAUL_TAG;
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

#[derive(Accounts)]
#[instruction(_goal: String)]
pub struct Deposit<'info> {
    #[account(mut)]
    signer: Signer<'info>,

    #[account(
        mut,
        seeds = [VAUL_TAG, signer.key().as_ref(), _goal.as_bytes()],
        bump
    )]
    vault_account: SystemAccount<'info>,

    system_program: Program<'info, System>,
}

pub fn handler(_ctx: Context<Deposit>, _goal: String, _amount: u64) -> Result<()> {
    let cpi_accounts = Transfer {
        from: _ctx.accounts.signer.to_account_info(),
        to: _ctx.accounts.vault_account.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(system_program::ID, cpi_accounts);
    transfer(cpi_ctx, _amount)?;

    Ok(())
}
