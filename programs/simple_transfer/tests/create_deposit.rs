use anchor_lang::{system_program, InstructionData};
use litesvm::LiteSVM;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use solana_transaction::Transaction;

use simple_transfer;

#[test]
fn create_deposit() {
    // Initialize the test environment
    let mut svm = LiteSVM::new();
    // Deploy your program to the test environment
    let program_id = Pubkey::from(simple_transfer::ID);
    let program_bytes = include_bytes!("../../../target/deploy/simple_transfer.so");
    svm.add_program(program_id, program_bytes);
    // Create and fund test accounts
    let signer = Keypair::new();
    svm.airdrop(&signer.pubkey(), 10_000_000_000).unwrap();

    let goal = "a certain goal".to_string();

    let (deposit_account_pda, _) = Pubkey::find_program_address(
        &[
            b"deposit_account",
            signer.pubkey().as_ref(),
            goal.as_bytes(),
        ],
        &simple_transfer::ID,
    );

    let (vault_account_pda, _) = Pubkey::find_program_address(
        &[b"vault", deposit_account_pda.as_ref()],
        &simple_transfer::ID,
    );

    let create_deposit_ix = Instruction {
        program_id: simple_transfer::ID,
        accounts: vec![
            AccountMeta::new(deposit_account_pda, false),
            AccountMeta::new(vault_account_pda, false),
            AccountMeta::new(signer.pubkey(), true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: simple_transfer::instruction::CreateDeposit {
            _amount: 1_000_000_000,
            _goal: goal,
        }
        .data(),
    };

    let create_deposit_tx = Transaction::new_signed_with_payer(
        &[create_deposit_ix],
        Some(&signer.pubkey()),
        &[&signer],
        svm.latest_blockhash(),
    );

    // Send transaction
    let create_deposit_tx_result = svm.send_transaction(create_deposit_tx);
    assert_eq!(create_deposit_tx_result.is_ok(), true);

    let vault_account_balance = svm.get_balance(&vault_account_pda).unwrap();
    assert_eq!(vault_account_balance, 1_000_000_000);
}
