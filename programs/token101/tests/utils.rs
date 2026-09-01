use anchor_lang::{system_program, InstructionData};
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

pub fn setup_svm() -> (LiteSVM, Keypair, Keypair) {
    let mut svm = LiteSVM::new();
    let program_id = Pubkey::from(token101::ID);
    let program_bytes = include_bytes!("../../../target/deploy/token101.so");
    svm.add_program(program_id, program_bytes);

    let minter_user = Keypair::new();
    svm.airdrop(&minter_user.pubkey(), 10_000_000_000).unwrap();

    let main_user = Keypair::new();
    svm.airdrop(&main_user.pubkey(), 10_000_000_000).unwrap();

    (svm, minter_user, main_user)
}

pub fn create_mint_account(svm: &mut LiteSVM, mint_authority: &Keypair) -> Pubkey {
    CreateMint::new(svm, &mint_authority)
        .authority(&mint_authority.pubkey())
        .decimals(DECIMALS)
        .send()
        .unwrap()
}

pub fn mint_to(
    svm: &mut LiteSVM,
    mint_authority: &Keypair,
    mint_account: Pubkey,
    recipient: &Keypair,
    amount: u64,
) -> Pubkey {
    let token_account = CreateAssociatedTokenAccount::new(svm, recipient, &mint_account)
        .owner(&recipient.pubkey())
        .send()
        .unwrap();

    MintTo::new(svm, mint_authority, &mint_account, &token_account, amount)
        .owner(mint_authority)
        .send()
        .unwrap();

    token_account
}

pub fn create_deposit_tx(
    svm: &mut LiteSVM,
    main_user: &Keypair,
    mint_account: Pubkey,
    ata_vault: Pubkey,
) -> Transaction {
    let create_deposit_ix = Instruction {
        program_id: token101::ID,
        accounts: vec![
            AccountMeta::new(main_user.pubkey(), true),
            AccountMeta::new(mint_account, false),
            AccountMeta::new(ata_vault, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::id(), false),
            AccountMeta::new_readonly(TOKEN_ID, false),
        ],
        data: token101::instruction::CreateDeposit { _amount: 120 }.data(),
    };

    Transaction::new_signed_with_payer(
        &[create_deposit_ix],
        Some(&main_user.pubkey()),
        &[main_user],
        svm.latest_blockhash(),
    )
}
