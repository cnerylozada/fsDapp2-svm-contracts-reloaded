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
    signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = ACCOUNT_DISCRIMINATOR + Offer::INIT_SPACE,
        seeds = [Offer::OFFER_TAG, _id.as_ref(), signer.key().as_ref()],
        bump
    )]
    offer: Account<'info, Offer>,

    deposited_mint_account: Account<'info, Mint>,

    #[account(
        mut, 
        associated_token::authority = signer,
        associated_token::mint = deposited_mint_account,
        associated_token::token_program = token_program
    )]
    sender_ata: Account<'info,TokenAccount>,

    #[account(
        mut,
        seeds = [VAULT_OFFER_TAG, _id.as_ref(), signer.key().as_ref()],
        bump
    )]
    vault_offer_ata_authority: SystemAccount<'info>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::authority = vault_offer_ata_authority,
        associated_token::mint = deposited_mint_account,
        associated_token::token_program = token_program
    )]
    vault_offer_ata: Account<'info, TokenAccount>,

    wanted_mint_account: Account<'info, Mint>,

    system_program: Program<'info, System>,
    associated_token_program: Program<'info, AssociatedToken>,
    token_program: Program<'info, Token>,
}

pub fn handler(_ctx: Context<MakeOffer>, _id: [u8; 16], _deposited_amount: u64, _wanted_amount: u64) -> Result<()> {
    let cpi_accounts = Transfer {
        authority: _ctx.accounts.signer.to_account_info(),
        from: _ctx.accounts.sender_ata.to_account_info(),
        to: _ctx.accounts.vault_offer_ata.to_account_info(),
    };

    let cpi_context = CpiContext::new(token::ID, cpi_accounts);

    let amount_to_transfer = _deposited_amount * 10u64.pow(_ctx.accounts.deposited_mint_account.decimals as u32);
    transfer(cpi_context, amount_to_transfer)?;

    *_ctx.accounts.offer = Offer {
        id: _id,
        user: _ctx.accounts.signer.key(),
        deposited_mint: _ctx.accounts.deposited_mint_account.key(),
        deposited_amount: _deposited_amount,
        wanted_mint: _ctx.accounts.wanted_mint_account.key(),
        wanted_amount: _wanted_amount,
        bump: _ctx.bumps.offer
    };

    Ok(())
}
