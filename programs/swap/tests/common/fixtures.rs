use crate::common::instructions::{create_and_fund_ata, create_offered_and_wanted_mint};
use crate::common::models::{MakeOfferAccounts, TestEnv};
use crate::common::utils::{create_pda_by_seeds, OFFER_TAG};
use litesvm::LiteSVM;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use spl_associated_token_account::get_associated_token_address;

fn create_svm_and_actors() -> (LiteSVM, Keypair, Keypair, Keypair) {
    let mut svm = LiteSVM::new();
    let program_id = Pubkey::from(swap::ID);
    let program_bytes = include_bytes!("../../../../target/deploy/swap.so");
    svm.add_program(program_id, program_bytes);

    let token_admin = Keypair::new();
    let maker = Keypair::new();
    let taker = Keypair::new();

    let default_funds = 1_000_000_000;

    svm.airdrop(&token_admin.pubkey(), default_funds).unwrap();
    svm.airdrop(&maker.pubkey(), default_funds).unwrap();
    svm.airdrop(&taker.pubkey(), default_funds).unwrap();

    (svm, token_admin, maker, taker)
}

pub fn setup_test_env(maker_offered_ata_initial_balance: u64) -> TestEnv {
    let (mut svm, token_admin, maker, _taker) = create_svm_and_actors();

    let (offered_mint, wanted_mint) = create_offered_and_wanted_mint(&mut svm, &token_admin);

    let maker_offered_ata = create_and_fund_ata(
        &mut svm,
        &token_admin,
        offered_mint,
        maker.pubkey(),
        maker_offered_ata_initial_balance,
    );

    TestEnv {
        svm,
        maker,
        offered_mint,
        maker_offered_ata,
        wanted_mint,
    }
}

pub fn derive_make_offer_accounts(env: &TestEnv, offer_id: [u8; 16]) -> MakeOfferAccounts {
    let (offer_pda, _) =
        create_pda_by_seeds(&[OFFER_TAG, offer_id.as_ref(), env.maker.pubkey().as_ref()]);

    let vault_offer_ata_pda = get_associated_token_address(&offer_pda, &env.offered_mint);

    MakeOfferAccounts {
        offer_pda,
        offered_mint: env.offered_mint,
        maker_offered_ata: env.maker_offered_ata,
        vault_offer_ata_pda,
        wanted_mint: env.wanted_mint,
    }
}
