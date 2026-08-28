use crate::constants::{ACCOUNT_DISCRIMINATOR, VAULT_TAG};
use crate::errors::SimpleTransferError;
use crate::models::DepositAccount;
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

#[derive(Accounts)]
#[instruction(_goal: String)]
pub struct CreateDeposit<'info> {
    #[account(
        init,
        space = ACCOUNT_DISCRIMINATOR + DepositAccount::INIT_SPACE,
        payer = signer,
        seeds = [DepositAccount::DEPOSIT_ACCOUNT_TAG, signer.key().as_ref(), _goal.as_bytes()],
        bump
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

pub fn handler(_ctx: Context<CreateDeposit>, _goal: String, _amount: u64) -> Result<()> {
    let goal_len = _goal.len();
    if goal_len == 0 || goal_len > DepositAccount::MAX_GOAL_LEN {
        return Err(SimpleTransferError::InvalidGoalLength.into());
    }

    if _amount <= DepositAccount::MIN_VAULT_DEPOSIT {
        return Err(SimpleTransferError::InsufficientDeposit.into());
    }

    *_ctx.accounts.deposit_account = DepositAccount {
        owner: _ctx.accounts.signer.key(),
        amount: _amount,
        goal: _goal,
        bump: _ctx.bumps.deposit_account,
    };

    let cpi_accounts = Transfer {
        from: _ctx.accounts.signer.to_account_info(),
        to: _ctx.accounts.vault_account.to_account_info(),
    };
    let cpi_context = CpiContext::new(system_program::ID, cpi_accounts);

    transfer(cpi_context, _amount)?;

    Ok(())
}
