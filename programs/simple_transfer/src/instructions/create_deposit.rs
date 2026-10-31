use crate::constants::{ACCOUNT_DISCRIMINATOR, VAULT_TAG};
use crate::errors::SimpleTransferError;
use crate::models::DepositAccount;
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

#[derive(Accounts)]
#[instruction(goal: String)]
pub struct CreateDeposit<'info> {
    #[account(mut)]
    signer: Signer<'info>,

    #[account(
        init,
        space = ACCOUNT_DISCRIMINATOR + DepositAccount::INIT_SPACE,
        payer = signer,
        seeds = [DepositAccount::DEPOSIT_ACCOUNT_TAG, signer.key().as_ref(), goal.as_bytes()],
        bump
    )]
    deposit_account: Account<'info, DepositAccount>,

    #[account(
        mut,
        seeds = [VAULT_TAG, deposit_account.key().as_ref()],
        bump
    )]
    vault_account: SystemAccount<'info>,

    system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateDeposit>, goal: String, amount: u64) -> Result<()> {
    let goal_len = goal.len();
    if goal_len == 0 || goal_len > DepositAccount::MAX_GOAL_LEN {
        return Err(SimpleTransferError::InvalidGoalLength.into());
    }

    // The vault is a System account with no data, created by this first transfer. The runtime
    // rejects a new account that ends below its rent-exempt minimum (`minimum_balance(0)`), so the
    // deposit must cover that floor. It is not a fee: it stays in the vault and only comes back
    // when the vault is emptied. MIN_BUSINESS_DEPOSIT is our own rule, on top of the rent.
    let min_vault_deposit = Rent::get()?.minimum_balance(0) + DepositAccount::MIN_BUSINESS_DEPOSIT;
    if amount < min_vault_deposit {
        return Err(SimpleTransferError::InsufficientDeposit.into());
    }

    *ctx.accounts.deposit_account = DepositAccount {
        owner: ctx.accounts.signer.key(),
        goal,
        bump: ctx.bumps.deposit_account,
    };

    let cpi_accounts = Transfer {
        from: ctx.accounts.signer.to_account_info(),
        to: ctx.accounts.vault_account.to_account_info(),
    };
    let cpi_context = CpiContext::new(system_program::ID, cpi_accounts);

    transfer(cpi_context, amount)?;

    Ok(())
}
