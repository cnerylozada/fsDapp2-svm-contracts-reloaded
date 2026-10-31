use solana_sdk::pubkey::Pubkey;

pub fn get_create_deposit_pdas(user: Pubkey, goal: &String) -> (Pubkey, Pubkey) {
    let (deposit_pda_account, _) = Pubkey::find_program_address(
        &[b"deposit_account", user.as_ref(), goal.as_bytes()],
        &simple_transfer::ID,
    );

    let (vault_account_pda, _) = Pubkey::find_program_address(
        &[b"vault", deposit_pda_account.as_ref()],
        &simple_transfer::ID,
    );

    (deposit_pda_account, vault_account_pda)
}

pub const MIN_BUSINESS_DEPOSIT: u64 = 1_000_000;
