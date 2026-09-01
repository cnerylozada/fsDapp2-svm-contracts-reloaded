use litesvm::LiteSVM;
use litesvm_token::{spl_token::native_mint::DECIMALS, CreateMint};
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};

#[test]
fn create_deposit() {
    // Initialize the test environment
    let mut svm = LiteSVM::new();
    // Deploy your program to the test environment
    let program_id = Pubkey::from(token101::ID);
    let program_bytes = include_bytes!("../../../target/deploy/token101.so");
    svm.add_program(program_id, program_bytes);
    // Create and fund test accounts
    let main_signer = Keypair::new();
    svm.airdrop(&main_signer.pubkey(), 10_000_000_000).unwrap();

    let mint = CreateMint::new(&mut svm, &main_signer)
        .authority(&main_signer.pubkey())
        .decimals(DECIMALS)
        .send()
        .unwrap();
}
