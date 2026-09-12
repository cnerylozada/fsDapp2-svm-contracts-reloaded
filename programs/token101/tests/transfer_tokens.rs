use solana_sdk::signature::Signer;
use spl_associated_token_account::get_associated_token_address;

mod utils;
use utils::{
    create_deposit_tx, create_funded_ata, create_mint_account, get_recipient_authority_pda,
    setup_svm, to_base_units,
};

const INITIAL_BALANCE_UI: u64 = 10;
const DEPOSIT_AMOUNT_UI: u64 = 3;

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

    let vault_authority_pda = get_recipient_authority_pda(main_user.pubkey());
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
}
