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
    pub const DEPOSIT_ACCOUNT_TAG: &[u8; 15] = b"deposit_account";
}
