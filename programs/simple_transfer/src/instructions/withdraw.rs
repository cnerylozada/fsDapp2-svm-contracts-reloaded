use crate::constants::VAULT_TAG;
use crate::models::DepositAccount;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(_goal: String)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        seeds = [DepositAccount::DEPOSIT_ACCOUNT_TAG, signer.key().as_ref(), _goal.as_bytes()],
        bump = deposit_account.bump
    )]
    deposit_account: Account<'info, DepositAccount>,

    #[account(
        mut,
        seeds = [VAULT_TAG, deposit_account.key().as_ref()],
        bump
    )]
    vault_account: SystemAccount<'info>,

    #[account(mut)]
    signer: Signer<'info>,
}

pub fn handler(_ctx: Context<Withdraw>, _goal: String, _amount: u64) -> Result<()> {
    Ok(())
}
