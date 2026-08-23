use crate::constants::ACCOUNT_DISCRIMINATOR;
use crate::models::DepositAccount;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(_goal: String)]
pub struct CreateAccount<'info> {
    #[account(
        init,
        space = ACCOUNT_DISCRIMINATOR + DepositAccount::INIT_SPACE,
        payer = signer,
        seeds = [DepositAccount::DEPOSIT_ACCOUNT_TAG, signer.key().as_ref(), _goal.as_bytes()],
        bump
    )]
    deposit_account: Account<'info, DepositAccount>,

    #[account(mut)]
    signer: Signer<'info>,

    system_program: Program<'info, System>,
}

pub fn handler(_ctx: Context<CreateAccount>, _goal: String) -> Result<()> {
    *_ctx.accounts.deposit_account = DepositAccount {
        owner: _ctx.accounts.signer.key(),
        amount: 0,
        goal: _goal,
        bump: _ctx.bumps.deposit_account,
    };

    Ok(())
}
