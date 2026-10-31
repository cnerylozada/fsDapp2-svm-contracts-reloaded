use anchor_lang::{system_program, InstructionData};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    signature::Signer,
    transaction::Transaction,
};

use crate::common::models::{DepositArgs, DepositPdas, TestEnv};

pub fn build_create_deposit_tx(
    env: &TestEnv,
    accounts: &DepositPdas,
    args: &DepositArgs,
) -> Transaction {
    let TestEnv { svm, signer } = env;

    let DepositPdas {
        deposit_account,
        vault_account,
    } = *accounts;

    let create_deposit_ix = Instruction {
        program_id: simple_transfer::ID,
        accounts: vec![
            AccountMeta::new(signer.pubkey(), true),
            AccountMeta::new(deposit_account, false),
            AccountMeta::new(vault_account, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: simple_transfer::instruction::CreateDeposit {
            amount: args.amount,
            goal: args.goal.clone(),
        }
        .data(),
    };

    Transaction::new_signed_with_payer(
        &[create_deposit_ix],
        Some(&signer.pubkey()),
        &[signer],
        svm.latest_blockhash(),
    )
}

pub fn build_withdraw_tx(env: &TestEnv, accounts: &DepositPdas, args: &DepositArgs) -> Transaction {
    let TestEnv { svm, signer } = env;

    let DepositPdas {
        deposit_account,
        vault_account,
    } = *accounts;

    let withdraw_ix = Instruction {
        program_id: simple_transfer::ID,
        accounts: vec![
            AccountMeta::new(signer.pubkey(), true),
            AccountMeta::new_readonly(deposit_account, false),
            AccountMeta::new(vault_account, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: simple_transfer::instruction::Withdraw {
            goal: args.goal.clone(),
            amount: args.amount,
        }
        .data(),
    };

    Transaction::new_signed_with_payer(
        &[withdraw_ix],
        Some(&signer.pubkey()),
        &[signer],
        svm.latest_blockhash(),
    )
}
