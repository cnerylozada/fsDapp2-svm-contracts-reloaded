use crate::constants::RECIPIENT_VAULT_TAG;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, transfer, Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(mut)]
    signer: Signer<'info>,

    mint_account: Account<'info, Mint>,

    #[account(
        seeds = [RECIPIENT_VAULT_TAG, signer.key().as_ref()],
        bump
    )]
    vault_authority: SystemAccount<'info>,

    #[account(
        associated_token::authority = vault_authority,
        associated_token::mint = mint_account,
        associated_token::token_program = token_program
    )]
    vault_ata: Account<'info, TokenAccount>,

    recipient_authority: SystemAccount<'info>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::authority = recipient_authority,
        associated_token::mint = mint_account,
        associated_token::token_program = token_program,
    )]
    recipient_ata: Account<'info, TokenAccount>,

    associated_token_program: Program<'info, AssociatedToken>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

pub fn handler(_ctx: Context<TransferTokens>, _amount: u64) -> Result<()> {
    let cpi_accounts = Transfer {
        authority: _ctx.accounts.signer.to_account_info(),
        from: _ctx.accounts.vault_ata.to_account_info(),
        to: _ctx.accounts.recipient_ata.to_account_info(),
    };

    let cpi_context = CpiContext::new(token::ID, cpi_accounts);

    let amount_to_transfer = _amount * 10u64.pow(_ctx.accounts.mint_account.decimals as u32);
    transfer(cpi_context, amount_to_transfer)?;
    Ok(())
}
