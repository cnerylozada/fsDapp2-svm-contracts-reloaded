use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, transfer, Mint, Token, TokenAccount, Transfer},
};
use crate::models::{Offer};
use crate::constants::{VAULT_OFFER_TAG,ACCOUNT_DISCRIMINATOR};


#[derive(Accounts)]
#[instruction(_id: [u8; 16])]
pub struct MakeOffer<'info> {
    #[account(mut)]
    maker: Signer<'info>,

    #[account(
        init,
        payer = maker,
        space = ACCOUNT_DISCRIMINATOR + Offer::INIT_SPACE,
        seeds = [Offer::OFFER_TAG, _id.as_ref(), maker.key().as_ref()],
        bump
    )]
    offer: Account<'info, Offer>,

    offered_mint_account: Account<'info, Mint>,

    #[account(
        mut, 
        associated_token::mint = offered_mint_account,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    maker_offered_ata: Account<'info,TokenAccount>,

    #[account(
        seeds = [VAULT_OFFER_TAG, _id.as_ref(), maker.key().as_ref()],
        bump
    )]
    vault_offer_ata_authority: SystemAccount<'info>,

    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = offered_mint_account,
        associated_token::authority = vault_offer_ata_authority,
        associated_token::token_program = token_program
    )]
    vault_offer_ata: Account<'info, TokenAccount>,

    wanted_mint_account: Account<'info, Mint>,

    system_program: Program<'info, System>,
    associated_token_program: Program<'info, AssociatedToken>,
    token_program: Program<'info, Token>,
}

pub fn handler(_ctx: Context<MakeOffer>, _id: [u8; 16], _offered_amount: u64, _wanted_amount: u64) -> Result<()> {
    let cpi_accounts = Transfer {
        authority: _ctx.accounts.maker.to_account_info(),
        from: _ctx.accounts.maker_offered_ata.to_account_info(),
        to: _ctx.accounts.vault_offer_ata.to_account_info(),
    };

    let cpi_context = CpiContext::new(token::ID, cpi_accounts);

    transfer(cpi_context, _offered_amount)?;

    *_ctx.accounts.offer = Offer {
        id: _id,
        maker: _ctx.accounts.maker.key(),
        offered_mint: _ctx.accounts.offered_mint_account.key(),
        offered_amount: _offered_amount,
        wanted_mint: _ctx.accounts.wanted_mint_account.key(),
        wanted_amount: _wanted_amount,
        bump: _ctx.bumps.offer
    };

    Ok(())
}
