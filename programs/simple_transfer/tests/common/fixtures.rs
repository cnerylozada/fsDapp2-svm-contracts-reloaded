use litesvm::LiteSVM;
use solana_sdk::signature::{Keypair, Signer};

use crate::common::models::{DepositPdas, TestEnv};
use crate::common::utils::get_create_deposit_pdas;

pub fn create_svm_and_actors() -> TestEnv {
    let mut svm: LiteSVM = LiteSVM::new();
    let program_id = simple_transfer::ID;
    let program_bytes = include_bytes!("../../../../target/deploy/simple_transfer.so");
    svm.add_program(program_id, program_bytes);

    let signer = Keypair::new();
    svm.airdrop(&signer.pubkey(), 10_000_000_000).unwrap();

    TestEnv { svm, signer }
}

pub fn derive_create_deposit_accounts(signer: &Keypair, goal: &String) -> DepositPdas {
    let (deposit_account, vault_account) = get_create_deposit_pdas(signer.pubkey(), goal);

    DepositPdas {
        deposit_account,
        vault_account,
    }
}
