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
    create_and_fund_ata, create_offered_and_wanted_mint, create_pda_by_seeds, setup_svm,
    supabase_id_to_bytes, to_base_units,
};

const INITIAL_TOKEN_BALANCE_UI: u64 = 15;
const OFFERED_AMOUNT_UI: u64 = 2;
const WANTED_AMOUNT_UI: u64 = 3;
const OFFER_TAG: &[u8; 5] = b"offer";

#[derive(Debug, BorshDeserialize)]
struct Offer {
    pub id: [u8; 16],
    pub maker: Pubkey,
    pub offered_mint: Pubkey,
    pub offered_amount: u64,
    pub wanted_mint: Pubkey,
    pub wanted_amount: u64,
    pub bump: u8,
}

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

    let maker_offered_ata_account: TokenAccount =
        get_spl_account(&svm, &maker_offered_ata).unwrap();
    assert_eq!(maker_offered_ata_account.mint, offered_mint);
    assert_eq!(maker_offered_ata_account.owner, maker.pubkey());
    assert_eq!(
        maker_offered_ata_account.amount,
        maker_offered_ata_initial_balance
    );

    let offer_supabse_id = String::from("c29c3e20-931d-4b75-9de3-103c48ac2976");
    let offer_id = supabase_id_to_bytes(&offer_supabse_id);

    let (offer_pda, _) =
        create_pda_by_seeds(&[OFFER_TAG, offer_id.as_ref(), maker.pubkey().as_ref()]);

    let vault_offer_ata_pda = get_associated_token_address(&offer_pda, &offered_mint);

    let make_offer_ix = Instruction {
        program_id: swap::ID,
        accounts: vec![
            AccountMeta::new(maker.pubkey(), true),
            AccountMeta::new(offer_pda, false),
            AccountMeta::new_readonly(offered_mint, false),
            AccountMeta::new(maker_offered_ata, false),
            AccountMeta::new(vault_offer_ata_pda, false),
            AccountMeta::new_readonly(wanted_mint, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
            AccountMeta::new_readonly(TOKEN_ID, false),
        ],
        data: swap::instruction::MakeOffer {
            _id: offer_id,
            _offered_amount: to_base_units(OFFERED_AMOUNT_UI),
            _wanted_amount: to_base_units(WANTED_AMOUNT_UI),
        }
        .data(),
    };

    let make_offer_tx = Transaction::new_signed_with_payer(
        &[make_offer_ix],
        Some(&maker.pubkey()),
        &[&maker],
        svm.latest_blockhash(),
    );

    let make_offer_tx_result = svm.send_transaction(make_offer_tx);
    assert_eq!(make_offer_tx_result.is_ok(), true);

    let maker_offered_ata_account_after_offer: TokenAccount =
        get_spl_account(&svm, &maker_offered_ata).unwrap();
    assert_eq!(
        maker_offered_ata_account_after_offer.amount,
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
    assert_eq!(offer_account.offered_mint, offered_mint);
    assert_eq!(
        offer_account.offered_amount,
        to_base_units(OFFERED_AMOUNT_UI)
    );
    assert_eq!(offer_account.wanted_mint, wanted_mint);
    assert_eq!(offer_account.wanted_amount, to_base_units(WANTED_AMOUNT_UI));
}
