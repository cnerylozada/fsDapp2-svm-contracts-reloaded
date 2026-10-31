mod common;
use common::fixtures::{derive_make_offer_accounts, derive_take_offer_accounts, setup_test_env};
use common::instructions::{build_make_offer_tx, build_take_offer_tx};
use common::models::MakeOfferArgs;
use common::utils::{
    supabase_id_to_bytes, to_base_units, INITIAL_TOKEN_BALANCE_UI, OFFERED_AMOUNT_UI,
    WANTED_AMOUNT_UI,
};

const OFFER_SUPABASE_ID: &str = "c29c3e20-931d-4b75-9de3-103c48ac2976";

#[test]
fn take_offer() {
    let initial_offered_balance = to_base_units(INITIAL_TOKEN_BALANCE_UI);

    let mut env = setup_test_env(initial_offered_balance);

    let offer_id = supabase_id_to_bytes(OFFER_SUPABASE_ID);

    let make_offer_accounts = derive_make_offer_accounts(&env, offer_id);
    let args = MakeOfferArgs {
        id: offer_id,
        offered_amount: to_base_units(OFFERED_AMOUNT_UI),
        wanted_amount: to_base_units(WANTED_AMOUNT_UI),
    };

    let make_offer_tx = build_make_offer_tx(&env.svm, &env.maker, make_offer_accounts, args);
    let make_offer_tx_result = env.svm.send_transaction(make_offer_tx);
    assert_eq!(make_offer_tx_result.is_ok(), true);

    let take_offer_accounts = derive_take_offer_accounts(
        &env,
        make_offer_accounts.offer_pda,
        make_offer_accounts.vault_offered_ata,
    );
    let take_offer_tx = build_take_offer_tx(&env.svm, &env.taker, take_offer_accounts);
    let take_offer_tx_result = env.svm.send_transaction(take_offer_tx);
    assert_eq!(take_offer_tx_result.is_ok(), true);
}
