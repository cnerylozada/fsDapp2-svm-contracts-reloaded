use anchor_lang::system_program;
use borsh::BorshDeserialize;
use solana_sdk::signer::Signer;

mod utils;
use utils::{
    create_deposit_tx, get_create_deposit_pdas, setup_svm, CreateDepositInput, DepositAccount,
};

#[test]
fn create_deposit() {
    let (mut svm, signer) = setup_svm();

    let inputs = CreateDepositInput {
        amount: 1_000_000_000,
        goal: "a certain goal".to_string(),
    };

    let (deposit_account_pda, vault_account_pda) =
        get_create_deposit_pdas(signer.pubkey(), &inputs.goal);

    let create_deposit_tx_result = create_deposit_tx(
        &mut svm,
        &signer,
        deposit_account_pda,
        vault_account_pda,
        &inputs,
    );
    assert_eq!(create_deposit_tx_result.is_ok(), true);

    let deposit_account_raw = svm.get_account(&deposit_account_pda).unwrap();
    assert_eq!(deposit_account_raw.owner, simple_transfer::ID);

    let deposit_account = DepositAccount::deserialize(&mut &deposit_account_raw.data[8..]).unwrap();
    assert_eq!(deposit_account.owner, signer.pubkey());
    assert_eq!(deposit_account.amount, inputs.amount);
    assert_eq!(deposit_account.goal, inputs.goal);

    let vault_account_raw = svm.get_account(&vault_account_pda).unwrap();
    assert_eq!(vault_account_raw.owner, system_program::ID);
    assert_eq!(vault_account_raw.lamports, inputs.amount);
}
