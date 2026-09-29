use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Offer {
    pub id: [u8; 16],
    pub user: Pubkey,
    pub deposited_mint: Pubkey,
    pub deposited_amount: u64,
    pub wanted_mint: Pubkey,
    pub wanted_amount: u64,
    pub bump: u8,
}

impl Offer {
    pub const OFFER_TAG: &[u8; 5] = b"offer";
}
