use borsh::BorshDeserialize;
use litesvm_token::{get_spl_account, spl_token::state::Account as TokenAccount, TOKEN_ID};
use solana_sdk::signer::Signer;
use spl_associated_token_account::get_associated_token_address;

mod common;
use common::models::{MakeOfferAccounts, MakeOfferArgs, Offer};
use common::transactions::{
    build_make_offer_tx, create_and_fund_ata, create_offered_and_wanted_mint,
};
use common::utils::{create_pda_by_seeds, setup_svm, supabase_id_to_bytes, to_base_units};

const INITIAL_TOKEN_BALANCE_UI: u64 = 15;
const OFFERED_AMOUNT_UI: u64 = 2;
const WANTED_AMOUNT_UI: u64 = 3;
const OFFER_TAG: &[u8; 5] = b"offer";

#[test]
fn make_offer() {
    let (mut svm, token_admin, maker, _taker) = setup_svm();

    let (offered_mint, wanted_mint) = create_offered_and_wanted_mint(&mut svm, &token_admin);

    let maker_offered_ata_initial_balance = to_base_units(INITIAL_TOKEN_BALANCE_UI);
    let maker_offered_ata = create_and_fund_ata(
        &mut svm,
        &token_admin,
        &offered_mint,
        &maker.pubkey(),
        maker_offered_ata_initial_balance,
    );

    let maker_offered_ata_account_before: TokenAccount =
        get_spl_account(&svm, &maker_offered_ata).unwrap();
    assert_eq!(maker_offered_ata_account_before.mint, offered_mint);
    assert_eq!(maker_offered_ata_account_before.owner, maker.pubkey());
    assert_eq!(
        maker_offered_ata_account_before.amount,
        maker_offered_ata_initial_balance
    );

    let offer_supabase_id = String::from("c29c3e20-931d-4b75-9de3-103c48ac2976");
    let offer_id = supabase_id_to_bytes(&offer_supabase_id);

    let (offer_pda, _) =
        create_pda_by_seeds(&[OFFER_TAG, offer_id.as_ref(), maker.pubkey().as_ref()]);

    let vault_offer_ata_pda = get_associated_token_address(&offer_pda, &offered_mint);

    let accounts = MakeOfferAccounts {
        offer_pda,
        offered_mint,
        maker_offered_ata,
        vault_offer_ata_pda,
        wanted_mint,
    };
    let args = MakeOfferArgs {
        id: offer_id,
        offered_amount: to_base_units(OFFERED_AMOUNT_UI),
        wanted_amount: to_base_units(WANTED_AMOUNT_UI),
    };
    let make_offer_tx = build_make_offer_tx(&mut svm, &maker, accounts, args);

    let make_offer_tx_result = svm.send_transaction(make_offer_tx);
    assert_eq!(make_offer_tx_result.is_ok(), true);

    let maker_offered_ata_account_after: TokenAccount =
        get_spl_account(&svm, &maker_offered_ata).unwrap();
    assert_eq!(
        maker_offered_ata_account_after.amount,
        to_base_units(INITIAL_TOKEN_BALANCE_UI - OFFERED_AMOUNT_UI)
    );

    let vault_offer_ata_account: TokenAccount =
        get_spl_account(&svm, &vault_offer_ata_pda).unwrap();
    assert_eq!(
        vault_offer_ata_account.amount,
        to_base_units(OFFERED_AMOUNT_UI)
    );

    let offer_account_raw = svm.get_account(&offer_pda).unwrap();
    let offer_account =
        Offer::deserialize(&mut &offer_account_raw.data[8..]).expect("Failed to deserialize Offer");

    assert_eq!(offer_account.id, offer_id);
    assert_eq!(offer_account.maker, maker.pubkey());
    assert_eq!(
        offer_account.offered_amount,
        to_base_units(OFFERED_AMOUNT_UI)
    );
}
