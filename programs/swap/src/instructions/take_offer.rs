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
        // The address must be the PDA derived from this offer's id and this maker. Without it the
        // caller could pass any `Offer` owned by the program, and `maker` (a plain SystemAccount
        // with no checks of its own) could be someone other than the real maker, so the taker's
        // payment would go to the wrong wallet.
        seeds = [Offer::OFFER_TAG, offer.id.as_ref(), maker.key().as_ref()],
        // Reuse the bump stored at `make_offer`. It skips the search for a valid bump on every call
        // (cheaper) and pins the one canonical address.
        bump = offer.bump,
        // The accounts the caller passes must be the mints recorded in the offer
        // (`offer.offered_mint == offered_mint.key()`). Anchor matches by field name, so the names
        // here and in `Offer` must be equal.
        // offered_mint: the vault and the taker's receiving ATA are derived from it, so a different
        // mint would point at the wrong token accounts.
        has_one = offered_mint,
        // wanted_mint: what the maker asked to be paid in. Without it the taker could pass a mint
        // they created themselves, mint any amount of it, and "pay" with worthless tokens to take
        // the offered tokens for free.
        has_one = wanted_mint,
    )]
    offer: Account<'info, Offer>,

    #[account(
        mut,
        associated_token::mint = offered_mint,
        associated_token::authority = offer,
        associated_token::token_program = token_program,
    )]
    vault_offered_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = offered_mint,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    taker_offered_ata: Box<Account<'info, TokenAccount>>,

    wanted_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = wanted_mint,
        associated_token::authority = taker,
        associated_token::token_program =token_program
    )]
    taker_wanted_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = wanted_mint,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    maker_wanted_ata: Box<Account<'info, TokenAccount>>,

    system_program: Program<'info, System>,
    associated_token_program: Program<'info, AssociatedToken>,
    token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<TakeOffer>) -> Result<()> {
    ctx.accounts.pay_maker()?;
    ctx.accounts.withdraw_from_vault()?;

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
            from: self.vault_offered_ata.to_account_info(),
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

        transfer(cpi_context, self.vault_offered_ata.amount)
    }
}
