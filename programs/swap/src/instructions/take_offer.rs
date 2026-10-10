use crate::models::Offer;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, transfer, Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
pub struct TakeOffer<'info> {
    #[account(mut)]
    taker: Signer<'info>,

    offered_mint: Account<'info, Mint>,

    maker: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [Offer::OFFER_TAG, offer.id.as_ref(), maker.key().as_ref()],
        bump = offer.bump,
        has_one = offered_mint,
        has_one = wanted_mint,
    )]
    offer: Account<'info, Offer>,

    #[account(
        mut,
        associated_token::mint = offered_mint,
        associated_token::authority = offer,
        associated_token::token_program = token_program,
    )]
    vault_offer_ata: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = offered_mint,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    taker_offered_ata: Account<'info, TokenAccount>,

    wanted_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = wanted_mint,
        associated_token::authority = taker,
        associated_token::token_program =token_program
    )]
    taker_wanted_ata: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = wanted_mint,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    maker_wanted_ata: Account<'info, TokenAccount>,

    system_program: Program<'info, System>,
    associated_token_program: Program<'info, AssociatedToken>,
    token_program: Program<'info, Token>,
}

pub fn handler(_ctx: Context<TakeOffer>) -> Result<()> {
    _ctx.accounts.pay_maker()?;
    _ctx.accounts.withdraw_from_vault()?;

    Ok(())
}

impl<'info> TakeOffer<'info> {
    fn pay_maker(&self) -> Result<()> {
        let cpi_accounts = Transfer {
            from: self.taker_wanted_ata.to_account_info(),
            to: self.maker_wanted_ata.to_account_info(),
            authority: self.taker.to_account_info(),
        };
        let cpi_context = CpiContext::new(token::ID, cpi_accounts);

        transfer(cpi_context, self.offer.wanted_amount)
    }

    fn withdraw_from_vault(&self) -> Result<()> {
        let cpi_acccounts = Transfer {
            from: self.vault_offer_ata.to_account_info(),
            to: self.taker_offered_ata.to_account_info(),
            authority: self.offer.to_account_info(),
        };

        let maker_key = self.maker.key();

        let signer_seeds: &[&[&[u8]]] = &[&[
            Offer::OFFER_TAG,
            self.offer.id.as_ref(),
            maker_key.as_ref(),
            &[self.offer.bump],
        ]];
        let cpi_context = CpiContext::new_with_signer(token::ID, cpi_acccounts, signer_seeds);

        transfer(cpi_context, self.vault_offer_ata.amount)
    }
}
