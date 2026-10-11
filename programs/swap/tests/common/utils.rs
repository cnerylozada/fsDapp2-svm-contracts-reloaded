use litesvm::LiteSVM;
use litesvm_token::spl_token::native_mint::DECIMALS;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use uuid::Uuid;

pub fn to_base_units(amount: u64) -> u64 {
    amount * 10u64.pow(DECIMALS as u32)
}

pub fn create_pda_by_seeds(seeds: &[&[u8]]) -> (Pubkey, u8) {
    Pubkey::find_program_address(seeds, &swap::ID)
}

pub fn supabase_id_to_bytes(supabase_id: &str) -> [u8; 16] {
    Uuid::parse_str(supabase_id)
        .expect("supabase id must be a valid uuid")
        .into_bytes()
}

pub fn setup_svm() -> (LiteSVM, Keypair, Keypair, Keypair) {
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
