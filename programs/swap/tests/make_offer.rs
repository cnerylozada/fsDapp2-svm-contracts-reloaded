use anchor_lang::{prelude::system_program, InstructionData};
use litesvm_token::{get_spl_account, spl_token::state::Account as TokenAccount, TOKEN_ID};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
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

    const OFFER_TAG: &[u8; 5] = b"offer";
    let (offer_pda, _) =
        create_pda_by_seeds(&[OFFER_TAG, offer_id.as_ref(), main_user.pubkey().as_ref()]);

    const VAULT_OFFER_TAG: &[u8; 11] = b"vault_offer";
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
            AccountMeta::new(vault_offer_ata_authority_pda, false),
            AccountMeta::new(vault_offer_ata_pda, false),
            AccountMeta::new_readonly(wanted_mint, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
            AccountMeta::new_readonly(TOKEN_ID, false),
        ],
        data: swap::instruction::MakeOffer {
            _id: offer_id,
            _deposited_amount: 2,
            _wanted_amount: 3,
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
}
