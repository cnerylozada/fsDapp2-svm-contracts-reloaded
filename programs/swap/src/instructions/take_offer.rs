use crate::constants::VAULT_OFFER_TAG;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, transfer, Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
#[instruction(_id: [u8; 16])]
pub struct TakeOffer<'info> {
    #[account(mut)]
    taker: Signer<'info>,

    offered_mint_account: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [VAULT_OFFER_TAG, _id.as_ref(), taker.key().as_ref()],
        bump
    )]
    vault_offer_ata_authority: SystemAccount<'info>,

    #[account(
        mut,
        associated_token::mint = offered_mint_account,
        associated_token::authority = vault_offer_ata_authority,
        associated_token::token_program = token_program,
    )]
    vault_offer_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = offered_mint_account,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    taker_offered_ata: Account<'info, TokenAccount>,

    token_program: Program<'info, Token>,
}

pub fn handler(_ctx: Context<TakeOffer>, _id: [u8; 16]) -> Result<()> {
    Ok(())
}
