use crate::common::models::{MakeOfferAccounts, MakeOfferArgs};
use anchor_lang::{prelude::system_program, InstructionData};
use litesvm::LiteSVM;
use litesvm_token::{
    spl_token::native_mint::DECIMALS, CreateAssociatedTokenAccount, CreateMint, MintTo, TOKEN_ID,
};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use solana_transaction::Transaction;

pub fn create_offered_and_wanted_mint(
    svm: &mut LiteSVM,
    token_admin: &Keypair,
) -> (Pubkey, Pubkey) {
    let offered_mint = CreateMint::new(svm, token_admin)
        .authority(&token_admin.pubkey())
        .decimals(DECIMALS)
        .send()
        .unwrap();

    let wanted_mint = CreateMint::new(svm, token_admin)
        .authority(&token_admin.pubkey())
        .decimals(DECIMALS)
        .send()
        .unwrap();

    (offered_mint, wanted_mint)
}

pub fn create_and_fund_ata(
    svm: &mut LiteSVM,
    token_admin: &Keypair,
    mint: &Pubkey,
    owner: &Pubkey,
    amount: u64,
) -> Pubkey {
    let ata = CreateAssociatedTokenAccount::new(svm, token_admin, mint)
        .owner(owner)
        .send()
        .unwrap();

    MintTo::new(svm, token_admin, mint, &ata, amount)
        .owner(token_admin)
        .send()
        .unwrap();

    ata
}

pub fn build_make_offer_tx(
    svm: &mut LiteSVM,
    maker: &Keypair,
    accounts: MakeOfferAccounts,
    args: MakeOfferArgs,
) -> Transaction {
    let MakeOfferAccounts {
        offer_pda,
        offered_mint,
        maker_offered_ata,
        vault_offer_ata_pda,
        wanted_mint,
    } = accounts;
    let MakeOfferArgs {
        id,
        offered_amount,
        wanted_amount,
    } = args;

    let make_offer_ix = Instruction {
        program_id: swap::ID,
        accounts: vec![
            AccountMeta::new(maker.pubkey(), true),
            AccountMeta::new(offer_pda, false),
            AccountMeta::new_readonly(offered_mint, false),
            AccountMeta::new(maker_offered_ata, false),
            AccountMeta::new(vault_offer_ata_pda, false),
            AccountMeta::new_readonly(wanted_mint, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
            AccountMeta::new_readonly(TOKEN_ID, false),
        ],
        data: swap::instruction::MakeOffer {
            _id: id,
            _offered_amount: offered_amount,
            _wanted_amount: wanted_amount,
        }
        .data(),
    };

    Transaction::new_signed_with_payer(
        &[make_offer_ix],
        Some(&maker.pubkey()),
        &[maker],
        svm.latest_blockhash(),
    )
}
