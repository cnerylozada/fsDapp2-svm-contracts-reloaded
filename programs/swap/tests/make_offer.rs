use anchor_lang::{prelude::system_program, InstructionData};
use borsh::BorshDeserialize;
use litesvm_token::{get_spl_account, spl_token::state::Account as TokenAccount, TOKEN_ID};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signer::Signer,
};
use solana_transaction::Transaction;
use spl_associated_token_account::get_associated_token_address;

mod utils;
use utils::{
    create_and_fund_ata, create_deposited_and_wanted_mint, create_pda_by_seeds, setup_svm,
    supabase_id_to_bytes, to_base_units,
};

const INITIAL_TOKEN_BALANCE_UI: u64 = 15;
const OFFER_DEPOSITED_AMOUNT_UI: u64 = 2;
const OFFER_WANTED_AMOUNT_UI: u64 = 3;
const OFFER_TAG: &[u8; 5] = b"offer";
const VAULT_OFFER_TAG: &[u8; 11] = b"vault_offer";

#[derive(Debug, BorshDeserialize)]
struct Offer {
    pub id: [u8; 16],
    pub user: Pubkey,
    pub deposited_mint: Pubkey,
    pub deposited_amount: u64,
    pub wanted_mint: Pubkey,
    pub wanted_amount: u64,
    pub bump: u8,
}

#[test]
fn make_offer() {
    let (mut svm, token_admin, main_user, other_user) = setup_svm();

    let (deposited_mint, wanted_mint) = create_deposited_and_wanted_mint(&mut svm, &token_admin);

    let main_user_deposited_ata_initial_balance = to_base_units(INITIAL_TOKEN_BALANCE_UI);
    let main_user_deposited_ata = create_and_fund_ata(
        &mut svm,
        &token_admin,
        &deposited_mint,
        &main_user.pubkey(),
        main_user_deposited_ata_initial_balance,
    );

    let main_user_deposited_ata_account: TokenAccount =
        get_spl_account(&svm, &main_user_deposited_ata).unwrap();
    assert_eq!(main_user_deposited_ata_account.mint, deposited_mint);
    assert_eq!(main_user_deposited_ata_account.owner, main_user.pubkey());
    assert_eq!(
        main_user_deposited_ata_account.amount,
        main_user_deposited_ata_initial_balance
    );

    let offer_supabse_id = String::from("c29c3e20-931d-4b75-9de3-103c48ac2976");
    let offer_id = supabase_id_to_bytes(&offer_supabse_id);

    let (offer_pda, _) =
        create_pda_by_seeds(&[OFFER_TAG, offer_id.as_ref(), main_user.pubkey().as_ref()]);

    let (vault_offer_ata_authority_pda, _) = create_pda_by_seeds(&[
        VAULT_OFFER_TAG,
        offer_id.as_ref(),
        main_user.pubkey().as_ref(),
    ]);

    let vault_offer_ata_pda =
        get_associated_token_address(&vault_offer_ata_authority_pda, &deposited_mint);

    let make_offer_ix = Instruction {
        program_id: swap::ID,
        accounts: vec![
            AccountMeta::new(main_user.pubkey(), true),
            AccountMeta::new(offer_pda, false),
            AccountMeta::new_readonly(deposited_mint, false),
            AccountMeta::new(main_user_deposited_ata, false),
            AccountMeta::new_readonly(vault_offer_ata_authority_pda, false),
            AccountMeta::new(vault_offer_ata_pda, false),
            AccountMeta::new_readonly(wanted_mint, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
            AccountMeta::new_readonly(TOKEN_ID, false),
        ],
        data: swap::instruction::MakeOffer {
            _id: offer_id,
            _deposited_amount: to_base_units(OFFER_DEPOSITED_AMOUNT_UI),
            _wanted_amount: to_base_units(OFFER_WANTED_AMOUNT_UI),
        }
        .data(),
    };

    let make_offer_tx = Transaction::new_signed_with_payer(
        &[make_offer_ix],
        Some(&main_user.pubkey()),
        &[&main_user],
        svm.latest_blockhash(),
    );

    let make_offer_tx_result = svm.send_transaction(make_offer_tx);
    assert_eq!(make_offer_tx_result.is_ok(), true);

    let main_user_deposited_ata_account_after_offer: TokenAccount =
        get_spl_account(&svm, &main_user_deposited_ata).unwrap();
    assert_eq!(
        main_user_deposited_ata_account_after_offer.amount,
        to_base_units(INITIAL_TOKEN_BALANCE_UI - OFFER_DEPOSITED_AMOUNT_UI)
    );

    let vault_offer_ata_account: TokenAccount =
        get_spl_account(&svm, &vault_offer_ata_pda).unwrap();
    assert_eq!(
        vault_offer_ata_account.amount,
        to_base_units(OFFER_DEPOSITED_AMOUNT_UI)
    );

    let offer_account_raw = svm.get_account(&offer_pda).unwrap();
    let offer_account = Offer::deserialize(&mut &offer_account_raw.data[8..])
        .expect("Failed to deserialize DataStore");

    assert_eq!(offer_account.id, offer_id);
    assert_eq!(offer_account.user, main_user.pubkey());
    assert_eq!(offer_account.deposited_mint, deposited_mint);
    assert_eq!(
        offer_account.deposited_amount,
        to_base_units(OFFER_DEPOSITED_AMOUNT_UI)
    );
    assert_eq!(offer_account.wanted_mint, wanted_mint);
    assert_eq!(
        offer_account.wanted_amount,
        to_base_units(OFFER_WANTED_AMOUNT_UI)
    );
}
