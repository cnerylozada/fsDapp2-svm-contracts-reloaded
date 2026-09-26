use anchor_lang::prelude::*;

pub const ACCOUNT_DISCRIMINATOR: usize = 8;

#[constant]
pub const VAULT_OFFER_TAG: &[u8; 11] = b"vault_offer";
