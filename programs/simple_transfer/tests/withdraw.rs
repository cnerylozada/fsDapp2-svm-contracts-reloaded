use anchor_lang::system_program;
use anchor_lang::InstructionData;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    signature::Signer,
};
use solana_transaction::Transaction;

mod utils;
use utils::{create_deposit_tx, get_create_deposit_pdas, setup_svm, CreateDepositInput};

#[test]
fn withdraw() {
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

    let amount_to_withdraw = 500_000_000;

    let withdraw_ix = Instruction {
        program_id: simple_transfer::ID,
        accounts: vec![
            AccountMeta::new(deposit_account_pda, false),
            AccountMeta::new(vault_account_pda, false),
            AccountMeta::new(signer.pubkey(), true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: simple_transfer::instruction::Withdraw {
            _amount: amount_to_withdraw,
            _goal: inputs.goal,
        }
        .data(),
    };

    let withdraw_tx = Transaction::new_signed_with_payer(
        &[withdraw_ix],
        Some(&signer.pubkey()),
        &[&signer],
        svm.latest_blockhash(),
    );

    let withdraw_tx_result = svm.send_transaction(withdraw_tx);
    assert_eq!(withdraw_tx_result.is_ok(), true);

    let vault_account_balance = svm.get_balance(&vault_account_pda).unwrap();
    assert_eq!(vault_account_balance, inputs.amount - amount_to_withdraw);
}
