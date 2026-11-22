use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, transfer, Mint, Token, TokenAccount, Transfer},
};
use crate::models::{Offer};
use crate::constants::{ACCOUNT_DISCRIMINATOR};
use crate::errors::ErrorCode;


#[derive(Accounts)]
#[instruction(id: [u8; 16])]
pub struct MakeOffer<'info> {
    #[account(mut)]
    maker: Signer<'info>,

    #[account(
        init,
        payer = maker,
        space = ACCOUNT_DISCRIMINATOR + Offer::INIT_SPACE,
        seeds = [Offer::OFFER_TAG, id.as_ref(), maker.key().as_ref()],
        bump
    )]
    offer: Account<'info, Offer>,

    offered_mint: Account<'info, Mint>,

    #[account(
        mut, 
        associated_token::mint = offered_mint,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    maker_offered_ata: Account<'info,TokenAccount>,

    #[account(
        init,
        payer = maker,
        associated_token::mint = offered_mint,
        associated_token::authority = offer,
        associated_token::token_program = token_program
    )]
    vault_offered_ata: Account<'info, TokenAccount>,

    wanted_mint: Account<'info, Mint>,

    system_program: Program<'info, System>,
    associated_token_program: Program<'info, AssociatedToken>,
    token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<MakeOffer>, id: [u8; 16], offered_amount: u64, wanted_amount: u64) -> Result<()> {
    // A zero offered_amount would lock an empty vault, and a zero wanted_amount would let anyone
    // take the offered tokens for free.
    if offered_amount == 0 || wanted_amount == 0 {
        return Err(ErrorCode::InvalidAmount.into());
    }

    let cpi_accounts = Transfer {
        authority: ctx.accounts.maker.to_account_info(),
        from: ctx.accounts.maker_offered_ata.to_account_info(),
        to: ctx.accounts.vault_offered_ata.to_account_info(),
    };

    let cpi_context = CpiContext::new(token::ID, cpi_accounts);

    transfer(cpi_context, offered_amount)?;

    *ctx.accounts.offer = Offer {
        id,
        maker: ctx.accounts.maker.key(),
        offered_mint: ctx.accounts.offered_mint.key(),
        offered_amount,
        wanted_mint: ctx.accounts.wanted_mint.key(),
        wanted_amount,
        bump: ctx.bumps.offer
    };

    Ok(())
}
