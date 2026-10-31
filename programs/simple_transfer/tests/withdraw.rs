mod common;
use common::fixtures::{create_svm_and_actors, derive_create_deposit_accounts};
use common::instructions::{build_create_deposit_tx, build_withdraw_tx};
use common::models::DepositArgs;

#[test]
fn withdraw() {
    let mut env = create_svm_and_actors();

    let create_deposit_args = DepositArgs {
        amount: env.svm.minimum_balance_for_rent_exemption(0) + 1_000_000,
        goal: "a certain goal".to_string(),
    };
    let create_deposit_accounts =
        derive_create_deposit_accounts(&env.signer, &create_deposit_args.goal);

    let create_deposit_tx =
        build_create_deposit_tx(&env, &create_deposit_accounts, &create_deposit_args);
    let create_deposit_tx_result = env.svm.send_transaction(create_deposit_tx);
    assert_eq!(create_deposit_tx_result.is_ok(), true);

    let withdraw_args = DepositArgs {
        amount: 1_000_000,
        goal: create_deposit_args.goal,
    };

    let withdraw_tx = build_withdraw_tx(&env, &create_deposit_accounts, &withdraw_args);
    let withdraw_tx_result = env.svm.send_transaction(withdraw_tx);
    assert_eq!(withdraw_tx_result.is_ok(), true);
}
