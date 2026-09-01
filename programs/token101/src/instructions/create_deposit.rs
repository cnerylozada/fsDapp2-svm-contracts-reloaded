use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct CreateDeposit<'info> {
    #[account(mut)]
    signer: Signer<'info>,

    mint_account: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::authority = signer,
        associated_token::mint = mint_account,
        associated_token::token_program = token_program

    )]
    ata_vault: Account<'info, TokenAccount>,

    system_program: Program<'info, System>,
    associated_token_program: Program<'info, AssociatedToken>,
    token_program: Program<'info, Token>,
}

pub fn handler(_ctx: Context<CreateDeposit>, _amount: u64) -> Result<()> {
    Ok(())
}
