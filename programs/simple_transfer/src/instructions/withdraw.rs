use crate::constants::VAULT_TAG;
use crate::models::DepositAccount;
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

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

    system_program: Program<'info, System>,
}

pub fn handler(_ctx: Context<Withdraw>, _goal: String, _amount: u64) -> Result<()> {
    let cpi_accounts = Transfer {
        from: _ctx.accounts.vault_account.to_account_info(),
        to: _ctx.accounts.signer.to_account_info(),
    };

    let deposit_account = _ctx.accounts.deposit_account.key();
    let bump_seed = _ctx.bumps.vault_account;
    let signer_seeds: &[&[&[u8]]] = &[&[VAULT_TAG, deposit_account.as_ref(), &[bump_seed]]];

    let cpi_context = CpiContext::new(system_program::ID, cpi_accounts).with_signer(signer_seeds);

    transfer(cpi_context, _amount)?;

    Ok(())
}
