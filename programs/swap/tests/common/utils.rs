use litesvm_token::spl_token::native_mint::DECIMALS;
use solana_sdk::pubkey::Pubkey;
use uuid::Uuid;

pub const INITIAL_TOKEN_BALANCE_UI: u64 = 15;
pub const OFFERED_AMOUNT_UI: u64 = 2;
pub const WANTED_AMOUNT_UI: u64 = 3;
pub const OFFER_TAG: &[u8; 5] = b"offer";

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
