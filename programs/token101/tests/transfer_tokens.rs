use anchor_lang::{system_program, InstructionData};
use litesvm_token::{get_spl_account, spl_token::state::Account as TokenAccount, TOKEN_ID};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    signature::Signer,
};
use solana_transaction::Transaction;
use spl_associated_token_account::get_associated_token_address;

mod utils;
use utils::{
    create_deposit_tx, create_funded_ata, create_mint_account, get_vault_authority_pda, setup_svm,
    to_base_units,
};

const INITIAL_BALANCE_UI: u64 = 10;
const DEPOSIT_AMOUNT_UI: u64 = 3;
const TRANSFER_AMOUNT_UI: u64 = 1;

#[test]
pub fn transfer_tokens() {
    let (mut svm, minter_user, main_user) = setup_svm();

    let mint_account = create_mint_account(&mut svm, &minter_user);

    let amount_to_mint = to_base_units(INITIAL_BALANCE_UI);

    let main_user_sender_ata = create_funded_ata(
        &mut svm,
        &minter_user,
        mint_account,
        &main_user,
        amount_to_mint,
    );

    let vault_authority_pda = get_vault_authority_pda(main_user.pubkey());
    let vault_ata = get_associated_token_address(&vault_authority_pda, &mint_account);

    let create_deposit_tx = create_deposit_tx(
        &mut svm,
        &main_user,
        mint_account,
        main_user_sender_ata,
        vault_authority_pda,
        vault_ata,
        DEPOSIT_AMOUNT_UI,
    );

    let create_deposit_tx_result = svm.send_transaction(create_deposit_tx);
    assert_eq!(create_deposit_tx_result.is_ok(), true);

    let recipient_ata = get_associated_token_address(&minter_user.pubkey(), &mint_account);

    let transfer_tokens_ix = Instruction {
        program_id: token101::ID,
        accounts: vec![
            AccountMeta::new(main_user.pubkey(), true),
            AccountMeta::new(mint_account, false),
            AccountMeta::new(vault_authority_pda, false),
            AccountMeta::new(vault_ata, false),
            AccountMeta::new(minter_user.pubkey(), false),
            AccountMeta::new(recipient_ata, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
            AccountMeta::new_readonly(TOKEN_ID, false),
        ],
        data: token101::instruction::TransferTokens {
            _amount: TRANSFER_AMOUNT_UI,
        }
        .data(),
    };

    let transfer_tokens_tx = Transaction::new_signed_with_payer(
        &[transfer_tokens_ix],
        Some(&main_user.pubkey()),
        &[main_user],
        svm.latest_blockhash(),
    );

    let transfer_tokens_tx_result = svm.send_transaction(transfer_tokens_tx);
    assert_eq!(transfer_tokens_tx_result.is_ok(), true);

    let vault_ata_account: TokenAccount = get_spl_account(&mut svm, &vault_ata).unwrap();
    assert_eq!(
        vault_ata_account.amount,
        to_base_units(DEPOSIT_AMOUNT_UI - TRANSFER_AMOUNT_UI)
    );

    let minter_user_ata_account: TokenAccount = get_spl_account(&mut svm, &recipient_ata).unwrap();
    assert_eq!(
        minter_user_ata_account.amount,
        to_base_units(TRANSFER_AMOUNT_UI)
    );
}
