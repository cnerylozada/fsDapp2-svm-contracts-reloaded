use anchor_lang::{system_program, InstructionData};
use borsh::BorshDeserialize;
use litesvm::{types::TransactionResult, LiteSVM};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use solana_transaction::Transaction;

pub struct CreateDepositInput {
    pub goal: String,
    pub amount: u64,
}

#[derive(Debug, BorshDeserialize)]
pub struct DepositAccount {
    pub owner: Pubkey,
    pub amount: u64,
    pub goal: String,
    pub bump: u8,
}

pub fn setup_svm() -> (LiteSVM, Keypair) {
    let mut svm: LiteSVM = LiteSVM::new();
    let program_id = simple_transfer::ID;
    let program_bytes = include_bytes!("../../../target/deploy/simple_transfer.so");
    svm.add_program(program_id, program_bytes);

    let signer = Keypair::new();
    svm.airdrop(&signer.pubkey(), 10_000_000_000).unwrap();

    (svm, signer)
}

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

pub fn create_deposit_tx(
    svm: &mut LiteSVM,
    signer: &Keypair,
    deposit_account_pda: Pubkey,
    vault_account_pda: Pubkey,
    inputs: &CreateDepositInput,
) -> TransactionResult {
    let create_deposit_ix = Instruction {
        program_id: simple_transfer::ID,
        accounts: vec![
            AccountMeta::new(deposit_account_pda, false),
            AccountMeta::new(vault_account_pda, false),
            AccountMeta::new(signer.pubkey(), true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: simple_transfer::instruction::CreateDeposit {
            _amount: inputs.amount,
            _goal: inputs.goal.clone(),
        }
        .data(),
    };

    let create_deposit_tx = Transaction::new_signed_with_payer(
        &[create_deposit_ix],
        Some(&signer.pubkey()),
        &[signer],
        svm.latest_blockhash(),
    );

    svm.send_transaction(create_deposit_tx)
}
