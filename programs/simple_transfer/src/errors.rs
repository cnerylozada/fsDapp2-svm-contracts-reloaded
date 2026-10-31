use anchor_lang::prelude::*;

#[error_code]
pub enum SimpleTransferError {
    #[msg("Goal must be between 1 and 15 characters")]
    InvalidGoalLength,

    #[msg("Amount must be greater than zero")]
    InvalidAmount,

    #[msg("Insufficient amount: below the minimum deposit, or more than the vault holds")]
    InsufficientDeposit,

    #[msg("Withdrawal would leave the vault below the minimum balance")]
    VaultBelowMinimumBalance,
}
