use litesvm_token::{get_spl_account, spl_token::state::Account as TokenAccount, TOKEN_ID};
use solana_sdk::signature::Signer;
use spl_associated_token_account::get_associated_token_address;

mod utils;
use utils::{create_mint_account, mint_to, setup_svm};

use crate::utils::create_deposit_tx;

#[test]
fn create_deposit() {
    let (mut svm, minter_user, main_user) = setup_svm();

    let mint_account = create_mint_account(&mut svm, &minter_user);

    let amount_to_mint = 100;
    let main_user_ata = mint_to(
        &mut svm,
        &minter_user,
        mint_account,
        &main_user,
        amount_to_mint,
    );
    let main_user_ata_account: TokenAccount = get_spl_account(&mut svm, &main_user_ata).unwrap();
    assert_eq!(main_user_ata_account.amount, amount_to_mint);

    let ata_vault = get_associated_token_address(&main_user.pubkey(), &mint_account);

    let create_deposit_tx = create_deposit_tx(&mut svm, &main_user, mint_account, ata_vault);

    let create_deposit_tx_result = svm.send_transaction(create_deposit_tx);
    assert_eq!(create_deposit_tx_result.is_ok(), true);

    let ata_vault_account_raw = svm.get_account(&ata_vault).unwrap();
    assert_eq!(ata_vault_account_raw.owner, TOKEN_ID);
}
