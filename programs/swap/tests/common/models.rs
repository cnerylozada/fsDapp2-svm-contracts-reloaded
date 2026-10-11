use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;

pub struct MakeOfferAccounts {
    pub offer_pda: Pubkey,
    pub offered_mint: Pubkey,
    pub maker_offered_ata: Pubkey,
    pub vault_offer_ata_pda: Pubkey,
    pub wanted_mint: Pubkey,
}

pub struct MakeOfferArgs {
    pub id: [u8; 16],
    pub offered_amount: u64,
    pub wanted_amount: u64,
}

#[derive(Debug, BorshDeserialize)]
pub struct Offer {
    pub id: [u8; 16],
    pub maker: Pubkey,
    pub offered_mint: Pubkey,
    pub offered_amount: u64,
    pub wanted_mint: Pubkey,
    pub wanted_amount: u64,
    pub bump: u8,
}
