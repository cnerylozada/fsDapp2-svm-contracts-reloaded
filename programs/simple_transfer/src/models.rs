use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct DepositAccount {
    pub owner: Pubkey,

    pub amount: u64,

    #[max_len(15)]
    pub goal: String,

    pub bump: u8,
}

impl DepositAccount {
    pub const MIN_VAULT_DEPOSIT: u64 = 500_000_000;
    pub const MAX_GOAL_LEN: usize = 15;
    pub const DEPOSIT_ACCOUNT_TAG: &[u8; 15] = b"deposit_account";
}
