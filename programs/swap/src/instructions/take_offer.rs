use crate::models::Offer;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct TakeOffer<'info> {
    #[account(mut)]
    taker: Signer<'info>,

    offered_mint_account: Account<'info, Mint>,

    maker: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [Offer::OFFER_TAG, offer.id.as_ref(), maker.key().as_ref()],
        bump = offer.bump
    )]
    offer: Account<'info, Offer>,

    #[account(
        mut,
        associated_token::mint = offered_mint_account,
        associated_token::authority = offer,
        associated_token::token_program = token_program,
    )]
    vault_offer_ata: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = offered_mint_account,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    taker_offered_ata: Account<'info, TokenAccount>,

    wanted_mint_account: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = wanted_mint_account,
        associated_token::authority = taker,
        associated_token::token_program =token_program
    )]
    taker_wanted_ata: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = wanted_mint_account,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    maker_wanted_ata: Account<'info, TokenAccount>,

    system_program: Program<'info, System>,
    associated_token_program: Program<'info, AssociatedToken>,
    token_program: Program<'info, Token>,
}

pub fn handler(_ctx: Context<TakeOffer>) -> Result<()> {
    Ok(())
}
