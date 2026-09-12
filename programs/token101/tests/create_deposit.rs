use litesvm_token::{get_spl_account, spl_token::state::Account as TokenAccount, TOKEN_ID};
use solana_sdk::{pubkey::Pubkey, signature::Signer};
use spl_associated_token_account::get_associated_token_address;

mod utils;
use utils::{create_deposit_tx, create_funded_ata, create_mint_account, setup_svm, to_base_units};

const INITIAL_BALANCE_UI: u64 = 10;
const DEPOSIT_AMOUNT_UI: u64 = 3;

#[test]
fn create_deposit() {
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
    let main_user_initial_ata_account: TokenAccount =
        get_spl_account(&mut svm, &main_user_sender_ata).unwrap();
    assert_eq!(main_user_initial_ata_account.owner, main_user.pubkey());
    assert_eq!(main_user_initial_ata_account.amount, amount_to_mint);

    let (recipient_authority_pda, _) =
        Pubkey::find_program_address(&[b"vault", main_user.pubkey().as_ref()], &token101::ID);
    let recipient_ata = get_associated_token_address(&recipient_authority_pda, &mint_account);

    let create_deposit_tx = create_deposit_tx(
        &mut svm,
        &main_user,
        mint_account,
        main_user_sender_ata,
        recipient_authority_pda,
        recipient_ata,
        DEPOSIT_AMOUNT_UI,
    );

    let create_deposit_tx_result = svm.send_transaction(create_deposit_tx);
    assert_eq!(create_deposit_tx_result.is_ok(), true);

    let recipient_ata_raw = svm.get_account(&recipient_ata).unwrap();
    assert_eq!(recipient_ata_raw.owner, TOKEN_ID);
    let recipient_ata_account: TokenAccount = get_spl_account(&mut svm, &recipient_ata).unwrap();
    assert_eq!(recipient_ata_account.owner, recipient_authority_pda);
    assert_eq!(
        recipient_ata_account.amount,
        to_base_units(DEPOSIT_AMOUNT_UI)
    );

    let main_user_final_ata_account: TokenAccount =
        get_spl_account(&mut svm, &main_user_sender_ata).unwrap();
    assert_eq!(
        main_user_final_ata_account.amount,
        to_base_units(INITIAL_BALANCE_UI - DEPOSIT_AMOUNT_UI)
    );
}
