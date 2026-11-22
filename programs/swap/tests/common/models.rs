use borsh::BorshDeserialize;
use litesvm::LiteSVM;
use solana_sdk::{pubkey::Pubkey, signature::Keypair};

pub struct TestEnv {
    pub svm: LiteSVM,
    pub maker: Keypair,
    pub offered_mint: Pubkey,
    pub maker_offered_ata: Pubkey,
    pub wanted_mint: Pubkey,
    pub taker: Keypair,
    pub taker_wanted_ata: Pubkey,
}

#[derive(Clone, Copy)]
pub struct MakeOfferAccounts {
    pub offer_pda: Pubkey,
    pub offered_mint: Pubkey,
    pub maker_offered_ata: Pubkey,
    pub vault_offered_ata: Pubkey,
    pub wanted_mint: Pubkey,
}

#[derive(Clone, Copy)]
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
    pub created_at: i64,
    pub bump: u8,
}

#[derive(Clone, Copy)]
pub struct TakeOfferAccounts {
    pub offered_mint: Pubkey,
    pub maker: Pubkey,
    pub offer_pda: Pubkey,
    pub vault_offered_ata: Pubkey,
    pub taker_offered_ata: Pubkey,
    pub wanted_mint: Pubkey,
    pub taker_wanted_ata: Pubkey,
    pub maker_wanted_ata: Pubkey,
}
