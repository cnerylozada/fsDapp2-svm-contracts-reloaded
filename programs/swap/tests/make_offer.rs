use borsh::BorshDeserialize;
use litesvm_token::{get_spl_account, spl_token::state::Account as TokenAccount};
use solana_sdk::signer::Signer;

mod common;
use common::fixtures::{derive_make_offer_accounts, setup_test_env};
use common::instructions::build_make_offer_tx;
use common::models::{MakeOfferArgs, Offer};
use common::utils::{
    supabase_id_to_bytes, to_base_units, INITIAL_TOKEN_BALANCE_UI, OFFERED_AMOUNT_UI,
    WANTED_AMOUNT_UI,
};

const OFFER_SUPABASE_ID: &str = "c29c3e20-931d-4b75-9de3-103c48ac2976";

#[test]
fn make_offer_moves_tokens_to_vault_and_stores_offer() {
    let maker_offered_ata_initial_balance = to_base_units(INITIAL_TOKEN_BALANCE_UI);

    let mut env = setup_test_env(maker_offered_ata_initial_balance);

    let maker_offered_ata_account_before: TokenAccount =
        get_spl_account(&env.svm, &env.maker_offered_ata).unwrap();
    assert_eq!(maker_offered_ata_account_before.mint, env.offered_mint);
    assert_eq!(maker_offered_ata_account_before.owner, env.maker.pubkey());
    assert_eq!(
        maker_offered_ata_account_before.amount,
        maker_offered_ata_initial_balance
    );

    let offer_id = supabase_id_to_bytes(OFFER_SUPABASE_ID);

    let accounts = derive_make_offer_accounts(&env, offer_id);
    let args = MakeOfferArgs {
        id: offer_id,
        offered_amount: to_base_units(OFFERED_AMOUNT_UI),
        wanted_amount: to_base_units(WANTED_AMOUNT_UI),
    };

    let make_offer_tx = build_make_offer_tx(&env.svm, &env.maker, &accounts, args);
    let make_offer_tx_result = env.svm.send_transaction(make_offer_tx);
    assert_eq!(make_offer_tx_result.is_ok(), true);

    let maker_offered_ata_account_after: TokenAccount =
        get_spl_account(&env.svm, &env.maker_offered_ata).unwrap();
    assert_eq!(
        maker_offered_ata_account_after.amount,
        to_base_units(INITIAL_TOKEN_BALANCE_UI - OFFERED_AMOUNT_UI)
    );

    let vault_offer_ata_account: TokenAccount =
        get_spl_account(&env.svm, &accounts.vault_offer_ata_pda).unwrap();
    assert_eq!(
        vault_offer_ata_account.amount,
        to_base_units(OFFERED_AMOUNT_UI)
    );

    let offer_account_raw = env.svm.get_account(&accounts.offer_pda).unwrap();
    let offer_account =
        Offer::deserialize(&mut &offer_account_raw.data[8..]).expect("Failed to deserialize Offer");

    assert_eq!(offer_account.id, offer_id);
    assert_eq!(offer_account.maker, env.maker.pubkey());
    assert_eq!(
        offer_account.offered_amount,
        to_base_units(OFFERED_AMOUNT_UI)
    );
}

#[test]
pub fn make_offer_test_2() {
    let maker_offered_ata_initial_balance = to_base_units(INITIAL_TOKEN_BALANCE_UI);

    let mut env = setup_test_env(maker_offered_ata_initial_balance);

    let offer_id = supabase_id_to_bytes(OFFER_SUPABASE_ID);

    let accounts = derive_make_offer_accounts(&env, offer_id);
    let args = MakeOfferArgs {
        id: offer_id,
        offered_amount: to_base_units(OFFERED_AMOUNT_UI),
        wanted_amount: to_base_units(WANTED_AMOUNT_UI),
    };

    let make_offer_tx = build_make_offer_tx(&env.svm, &env.maker, &accounts, args);
    let make_offer_tx_result = env.svm.send_transaction(make_offer_tx);
    assert_eq!(make_offer_tx_result.is_ok(), true);
}
