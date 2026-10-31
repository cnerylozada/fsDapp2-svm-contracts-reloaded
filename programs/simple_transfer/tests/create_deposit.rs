use anchor_lang::system_program;
use borsh::BorshDeserialize;
use solana_sdk::instruction::InstructionError;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::TransactionError;

mod common;
use common::fixtures::{create_svm_and_actors, derive_create_deposit_accounts};
use common::instructions::build_create_deposit_tx;
use common::models::{DepositAccount, DepositArgs};
use common::utils::MIN_BUSINESS_DEPOSIT;

#[test]
fn create_deposit_below_minimum_returns_insufficient_deposit() {
    let mut env = create_svm_and_actors();

    let args = DepositArgs {
        amount: env.svm.minimum_balance_for_rent_exemption(0) + 100_000,
        goal: "a certain goal".to_string(),
    };
    let accounts = derive_create_deposit_accounts(&env.signer, &args.goal);

    let create_deposit_tx = build_create_deposit_tx(&env, &accounts, &args);
    let create_deposit_tx_result = env.svm.send_transaction(create_deposit_tx);

    // Anchor numbers errors from 6000 by position: InsufficientDeposit is the third variant.
    let failed_tx = create_deposit_tx_result.unwrap_err();
    assert_eq!(
        failed_tx.err,
        TransactionError::InstructionError(0, InstructionError::Custom(6002))
    );
}

#[test]
fn create_deposit_at_minimum_creates_deposit_account_and_funds_vault() {
    let mut env = create_svm_and_actors();

    let args = DepositArgs {
        amount: env.svm.minimum_balance_for_rent_exemption(0) + MIN_BUSINESS_DEPOSIT,
        goal: "a certain goal".to_string(),
    };
    let accounts = derive_create_deposit_accounts(&env.signer, &args.goal);

    let create_deposit_tx = build_create_deposit_tx(&env, &accounts, &args);
    let create_deposit_tx_result = env.svm.send_transaction(create_deposit_tx);
    assert_eq!(create_deposit_tx_result.is_ok(), true);

    let raw_deposit_account = env.svm.get_account(&accounts.deposit_account).unwrap();
    assert_eq!(raw_deposit_account.owner, simple_transfer::ID);

    let deposit_account = DepositAccount::deserialize(&mut &raw_deposit_account.data[8..]).unwrap();
    assert_eq!(deposit_account.owner, env.signer.pubkey());
    assert_eq!(deposit_account.goal, args.goal);

    let vault_account = env.svm.get_account(&accounts.vault_account).unwrap();
    assert_eq!(vault_account.lamports, args.amount);
    assert_eq!(vault_account.owner, system_program::ID);
}
