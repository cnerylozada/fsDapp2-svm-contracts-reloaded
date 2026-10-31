use borsh::BorshDeserialize;
use litesvm::LiteSVM;
use solana_sdk::{pubkey::Pubkey, signature::Keypair};

pub struct TestEnv {
    pub svm: LiteSVM,
    pub signer: Keypair,
}

pub struct DepositPdas {
    pub deposit_account: Pubkey,
    pub vault_account: Pubkey,
}

pub struct DepositArgs {
    pub goal: String,
    pub amount: u64,
}

#[derive(Debug, BorshDeserialize)]
pub struct DepositAccount {
    pub owner: Pubkey,
    pub goal: String,
    pub bump: u8,
}
