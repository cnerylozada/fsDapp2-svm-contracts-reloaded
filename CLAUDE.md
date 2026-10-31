# fsDapp2 SVM contracts

Anchor workspace for learning Solana. The author comes from EVM/Solidity, so explain SVM concepts by contrast with the EVM.

## Programs

All live in `programs/` and share one `Cargo.lock`.

| Program | What it is |
|---|---|
| `token101` | SPL token deposits and transfers out of a PDA vault |
| `swap` | Escrow: `make_offer` / `take_offer`, one PDA per offer, keyed by a Supabase UUID (`[u8; 16]`) |
| `simple_transfer` | SOL deposit into a System-owned PDA vault, with `withdraw` |
| `default` | Leftover template program, probably unused |

## Commands

```
anchor build                 # must succeed before any test: tests load target/deploy/<program>.so
cargo test --workspace       # LiteSVM tests (also what `anchor test` runs, see Anchor.toml)
cargo test -p simple_transfer --test create_deposit -- --nocapture
cargo check --workspace --tests
```

A test that behaves strangely after a program change is usually a stale `.so`. Run `anchor build` first.

## Versions

Keep these in sync across every program:
- `anchor-lang`, `anchor-spl` and the `anchor` CLI: all `1.2.1`. Install with `avm install 1.2.1 && avm use 1.2.1` (not `latest`, which is a 2.0 RC).
- Test crates: `litesvm` / `litesvm-token` `0.18.0`, `solana-sdk` `4.0.1`. A crate can only `use` what its own `Cargo.toml` declares, so use `solana_sdk::transaction::Transaction`.
- Rust toolchain is pinned in `rust-toolchain.toml` (`1.97.1`).
- `anchor-lang` needs `features = ["init-if-needed"]` in `swap` and `token101` only.
- In each program's `idl-build` feature, list `anchor-lang/idl-build`, plus `anchor-spl/idl-build` when `anchor-spl` types appear in accounts, args or events. Dev-dependencies never go there.

## Conventions

- No leading underscore on names that are used. `_name` means "intentionally unused". The one exception is `_goal` in `simple_transfer/src/instructions/withdraw.rs`, which exists only so `#[instruction(goal)]` can build the seeds.
- Anchor binds accounts by **position**. If the order of fields in a `#[derive(Accounts)]` struct changes, update the `AccountMeta` lists in `tests/` too.
- Test helpers build instructions by hand with `<program>::instruction::<Ix> { .. }.data()`. When an instruction argument is renamed, update those structs.
- The Supabase UUID in `swap` is passed as `id: [u8; 16]`, not as a `String`. A seed is a `&[u8]` of at most 32 bytes, and `String::as_bytes()` returns the UTF-8 bytes of the text, so the canonical 36-character UUID is 36 bytes and fails with `MaxSeedLengthExceeded`. The same UUID parsed to its 128-bit value is 16 bytes. The fixed size also keeps `Offer::INIT_SPACE` exact (16 bytes, against 4 + 32 for a hex `String`), shrinks the instruction data, and leaves room for more seeds. The client and program must derive the PDA from the same encoding, so the client converts with `Buffer.from(uuid.replaceAll("-", ""), "hex")` (or `Uuid::into_bytes()` in Rust tests). The text form derives a different PDA.
- Prefer `Rent::get()?.minimum_balance(n)` (program) and `svm.minimum_balance_for_rent_exemption(n)` (test) over hardcoded rent values.
- When asserting a transaction succeeded, use `.unwrap()` or `assert!(r.is_ok(), "{:?}", r)` so the logs are visible. `assert_eq!(r.is_ok(), true)` hides the error.

## How a call finds its handler

An instruction has three parts: `programAddress` (which program), `accounts[]` (which accounts, by position) and `data` (which handler, plus its inputs). The accounts list never selects the function, so two handlers can share the same accounts in the same order.

```
data = [ 8-byte instruction discriminator ][ handler inputs, Borsh-encoded ]
         sha256("global:<fn_name>")[..8]      e.g. String = u32 length + UTF-8, u64 = 8 bytes LE
```

- The name is the Rust function name in the `#[program]` module (snake_case). Only the 8-byte hash prefix is sent, never the text. The EVM equivalent is the 4-byte selector, and the ABI's equivalent is the IDL (`target/idl/<program>.json`, which lists each instruction's discriminator).
- In Rust tests the handler is chosen by the struct name: `<program>::instruction::Withdraw { goal, amount }.data()`. Anchor generates one struct per `pub fn`, and `.data()` prepends the discriminator. The fields are the handler's inputs, not accounts.
- On the validator, the runtime only starts the program. Anchor's generated entrypoint reads `data[0..8]`, matches it to a handler (no match is error 101), runs `try_accounts` (position, owner, account discriminator, seeds, mut/signer), decodes `data[8..]`, and only then calls the handler. A failed check means the handler never runs.
- Client side: the Anchor TS client (`program.methods.withdraw(...)`) or a Codama-generated Kit client (`getWithdrawInstruction(...)`) builds the discriminator from the name. A hand-written Kit client copies it from the IDL.
- The instruction discriminator (`"global:withdraw"`) is a different tag from the account discriminator (`"account:DepositAccount"`, the first 8 bytes of an account's data, checked on `Account<'info, T>`). Anchor hashes only the name, so handlers cannot be overloaded.

## Rent and the vault's minimum deposit

The vault in `simple_transfer` is a System account with **no data**, and it still needs a minimum balance. Anyone can send it lamports, so the first deposit has to clear that floor.

- **What rent is.** Every account must hold a minimum balance to exist: `(128 + data_len) × 6,960` lamports (6,960 = 3,480 lamports per byte-year × 2 years). The 128 is the fixed per-account overhead (address, owner, lamports, flags), so even `data_len = 0` has a minimum. On mainnet constants that is 890,880 lamports for the vault. The LiteSVM value is unverified, so print `svm.minimum_balance_for_rent_exemption(0)` instead of assuming.
- **It is a deposit, not a fee.** Nobody is paid. The lamports stay inside the account, and they only come back when the account is emptied to 0. Analogy: a security deposit that stays on the account while it exists. To keep 2 in the vault when the minimum is 1, the account must end up holding 3, and the 1 is locked, not spent.
- **The rule the runtime enforces.** At the end of a transaction an account must hold either 0 lamports or at least its minimum. A new account cannot start in between. So a first transfer below the minimum fails (the System program's "insufficient funds for rent" error), and a withdrawal that leaves the vault between 0 and the minimum fails too.
- **Reading the value, never hardcoding it.** Program: `Rent::get()?.minimum_balance(n)`. LiteSVM: `svm.minimum_balance_for_rent_exemption(n)`. Client: `getMinimumBalanceForRentExemption`. CLI: `solana rent 0`.
- **Two separate floors in `create_deposit`.**
  - Rent: `Rent::get()?.minimum_balance(0)`. The network's rule, so the vault can exist.
  - `DepositAccount::MIN_BUSINESS_DEPOSIT` (1_000_000): our own rule, the least we consider a real deposit.
  - The check is `amount < minimum_balance(0) + MIN_BUSINESS_DEPOSIT`, with both parts summed, so the business amount is what remains after rent. Example: rent 1 and business minimum 2 means the minimum deposit is 3.
  - The check uses `<`, so a deposit of exactly the minimum passes.
- **Why check it in the program.** Without it the transfer would still fail below the rent floor, but with a generic System error. Checking first gives our own error (`InsufficientDeposit`) and lets the business rule sit on top.
- **Withdraw.** `withdraw` today only rejects `amount` greater than the vault balance (`checked_sub`, `InsufficientDeposit`). Nothing in the program blocks leaving the vault under rent + business, so a partial withdrawal that lands below the rent floor is rejected by the runtime, not by our code. Decide whether a withdrawal may only leave 0 (a full exit) or at least rent + business (a permanent floor). `VaultBelowMinimumBalance` exists for the second option and is unused now.
- **Draining the vault.** A full withdrawal that takes the vault to 0 is allowed by the runtime, and `get_balance` then returns `None` in LiteSVM (the account is gone), so tests must not `.unwrap()` it.
- **The `DepositAccount` pays rent too.** `init` charges `signer` the rent for `8 + INIT_SPACE` bytes, which is a different, larger minimum than the vault's.
- **Tests.** Use amounts above `minimum_balance(0) + MIN_BUSINESS_DEPOSIT`, plus boundary tests at `min - 1` (must fail) and `min` (must pass).

## Account discriminator

Every account created with `#[account]` starts with 8 bytes that tag its type. This is separate from the instruction discriminator above.

```
account data = [ 8-byte discriminator ][ the struct's fields, Borsh-encoded ]
                 sha256("account:DepositAccount")[..8]
```

- **Written once, by Anchor.** `init` writes it when the account is created. You never set it by hand.
- **Space.** `INIT_SPACE` only counts your fields, so `space = ACCOUNT_DISCRIMINATOR + DepositAccount::INIT_SPACE` (`ACCOUNT_DISCRIMINATOR` is the constant `8`). Forgetting the `+ 8` makes the account too small and serialization fails at the end of the instruction. Rent is computed on the total.
- **Checked on every use.** For each `Account<'info, T>`, Anchor's generated `try_accounts` runs these in order: owner is this program (`AccountOwnedByWrongProgram`, 3007), data has at least 8 bytes (`AccountDiscriminatorNotFound`, 3001), the first 8 bytes equal `T::DISCRIMINATOR` (`AccountDiscriminatorMismatch`, 3002), then the fields are deserialized, and only then `seeds`/`bump`/`has_one` constraints run (`bump = deposit_account.bump` needs the deserialized account).
- **Its role is type safety.** Without it, someone could pass a different account type owned by the same program whose bytes happen to fit the expected struct (type confusion). It is compared against the type's name, not against the address or the seeds.
- **It is not the whole check.** Three separate checks cover three questions, and all are needed:
  - owner: does my program own this account?
  - discriminator: is its data a `DepositAccount`?
  - seeds: is it at the address this signer and goal would derive? (this stops a real account that belongs to someone else)
- **The runtime ignores it.** Solana only enforces who may write to an account. The discriminator is an Anchor convention.
- **Reading accounts off-chain.** Skip the first 8 bytes before decoding the fields (`data[8..]` in tests), or use `<Type>::try_deserialize(&mut data.as_slice())`, which also checks the discriminator. That needs `pub mod models` in `lib.rs`, and it is private today.
- **Renaming a struct changes its discriminator.** Existing accounts of the old name stop deserializing, so rename before anything is deployed, or migrate.
- **Test helpers drift.** Hand-written copies of the account struct in `tests/utils.rs` must match the program's fields exactly, otherwise Borsh fails with `Unexpected length of input`.

## Test layout: `tests/common/`

`swap` and `simple_transfer` share a pattern (`token101` still uses a single `tests/utils.rs`). Every file in `tests/` is its own crate, so shared helpers live in `tests/common/` (a folder, so Cargo doesn't compile it as a test). Each test file declares `mod common;` and imports from it. Inside `common/`, files refer to each other with `crate::common::...`. `common/mod.rs` only lists the modules (`pub mod fixtures; pub mod instructions; pub mod models; pub mod utils;`).

The rule that splits the files: **a test reads as arrange, act, assert, and each file serves one of those steps.** Dependencies point one way: `fixtures` uses `instructions`, `models` and `utils`; `instructions` uses `models`; `models` and `utils` use nothing in `common`.

| File | Purpose | Contains | Does not contain |
|---|---|---|---|
| `models.rs` | Data shapes only | Input structs (`MakeOfferArgs`, `CreateDepositArgs`), account bundles (`MakeOfferAccounts`, `TakeOfferAccounts`), the environment struct (`TestEnv`), and hand-written copies of on-chain accounts (`Offer`, `DepositAccount`) that derive `BorshDeserialize` | Logic |
| `utils.rs` | Pure helpers with no SVM state | Constants (`OFFER_TAG`, amounts), unit conversion (`to_base_units`), PDA derivation (`create_pda_by_seeds`, `get_create_deposit_pdas`), ID conversion (`supabase_id_to_bytes`) | Anything that needs a `LiteSVM` |
| `instructions.rs` | One function per on-chain action: it builds the `Instruction` and the `Transaction`, or runs a setup action | `build_make_offer_tx`, `build_take_offer_tx`, `create_deposit_tx`, `create_offered_and_wanted_mint`, `create_and_fund_ata` | Assertions, and decisions about which scenario to run |
| `fixtures.rs` | Ready-made starting states (swap only) | `setup_test_env` (a loaded program, funded actors, mints, funded ATAs) and `derive_*_accounts` (the account bundle for a given `TestEnv`) | The transaction itself |

How each file works:

- **`models.rs`.** Bundling accounts into a struct (`MakeOfferAccounts`) and destructuring it where the `AccountMeta` list is built avoids positional mistakes and keeps call sites short. The account structs here mirror the program's on-chain fields exactly, without the 8-byte discriminator, so decode `data[8..]`. If a program struct changes, update the copy here, or Borsh fails with `Unexpected length of input`. They use the test's own `Pubkey`, so the file needs `use solana_sdk::pubkey::Pubkey`.
- **`utils.rs`.** Seeds must match the program byte for byte. `OFFER_TAG` and the `b"vault"` / `b"deposit_account"` seeds here mirror the program's constants. A mismatch derives a different address, and the account fails with a seeds constraint error rather than a clear message. A hand-copied constant can drift, so a `pub` constant exported from the program is safer.
- **`instructions.rs`.** This is the test-side equivalent of a client SDK. Account order in the `AccountMeta` list has to equal the `#[derive(Accounts)]` field order, and the handler is chosen by the `<program>::instruction::<Ix> { .. }.data()` struct. Builders that only create a `Transaction` (`build_*_tx`) let the test choose when to send it and whether to `.unwrap()` it. Builders that send (`create_deposit_tx`) return a `TransactionResult` so the test can assert on the error.
- **`fixtures.rs`.** `setup_test_env` is the equivalent of a Hardhat/Foundry `setUp()`, and it returns one `TestEnv`. `derive_*_accounts(&env, ..)` turns the environment plus a few inputs into the account bundle an instruction needs. `simple_transfer` has no fixtures file because its setup is just `setup_svm()`, which currently sits in `instructions.rs`.
- **`setup_svm` loads `target/deploy/<program>.so`** with `include_bytes!`, so the path is relative to the file that contains it (`../../../target/...` from `simple_transfer/tests/common/`, one `../` more than `tests/utils.rs` had). Run `anchor build` first.

What a test looks like:

```rust
mod common;
use common::fixtures::{setup_test_env, derive_make_offer_accounts};
use common::instructions::build_make_offer_tx;
use common::models::MakeOfferArgs;

let mut env = setup_test_env(initial_balance);                  // fixtures: arrange
let accounts = derive_make_offer_accounts(&env, offer_id);      // fixtures + utils: derive addresses
let tx = build_make_offer_tx(&env.svm, &env.maker, accounts, args); // instructions: act
env.svm.send_transaction(tx).unwrap();
// assertions stay in the test file, decoding accounts with models::Offer
```

Rule of thumb for where new code goes: needs a `LiteSVM` and sends or builds a transaction, so `instructions`; needs no `LiteSVM`, so `utils`; is a struct, so `models`; is a reusable starting state, so `fixtures`. Assertions never go in `common/`, so each test file shows what it expects.

## `AccountMeta` in tests: `new` vs `new_readonly`, `true` vs `false`

Every account in an instruction is declared up front with two flags: **can it change?** (writable) and **did it sign?** (signer). In the EVM nothing is declared, because a call can touch any storage. On Solana the declaration lets the runtime lock accounts, so transactions that touch different accounts run in parallel. The program never sets these flags. The test (or client) must set them to match the `#[derive(Accounts)]` struct.

```rust
AccountMeta::new(pubkey, is_signer)            // writable
AccountMeta::new_readonly(pubkey, is_signer)   // read-only
```

The two choices are independent:
- **`new` or `new_readonly` answers "can it change?"** It is the name of the constructor, not the bool.
- **The bool answers "did it sign?"** `true` means this account's keypair signs the transaction. It is never about writable.

**1. Writable (`new`) when the instruction changes the account's lamports or data.** That includes receiving lamports. In the struct this is `mut`, `init`, `init_if_needed`, `close`, or being the `payer`. Otherwise use `new_readonly`.

**2. Signer (`true`) when the struct field is `Signer<'info>`.** That is a wallet approving the call. Everything else is `false`:
- PDAs are always `false`. A PDA has no private key, so it cannot sign in the transaction. The program signs for it inside the instruction with `with_signer` / `invoke_signed` and the seeds (the vault's transfer in `withdraw`).
- Programs (`system_program`, the token program) are always `false`.
- An account being created by `init` at a PDA is `false`. If `init` creates a plain keypair account (no seeds), that new keypair must sign, so it is `true`.

Quick check for each field in the struct:

| Field in the `Accounts` struct | Constructor | Signer flag |
|---|---|---|
| `Signer<'info>` with `#[account(mut)]` (pays, or receives lamports) | `new` | `true` |
| `Signer<'info>` without `mut` | `new_readonly` | `true` |
| `Account<'info, T>` with `mut` or `init` (PDA) | `new` | `false` |
| `Account<'info, T>` without `mut` (only read) | `new_readonly` | `false` |
| `SystemAccount` with `mut` (the vault) | `new` | `false` |
| `Program<'info, System>`, token program, mint read only | `new_readonly` | `false` |

Examples in `simple_transfer`:

```rust
// create_deposit
AccountMeta::new(signer.pubkey(), true),             // Signer + mut: signs and pays rent and the deposit
AccountMeta::new(deposit_account, false),            // init: its data is written; PDA, so no signature
AccountMeta::new(vault_account, false),              // mut: receives lamports; PDA, so no signature
AccountMeta::new_readonly(system_program::ID, false),// program: never written, never signs

// withdraw
AccountMeta::new(signer.pubkey(), true),             // mut: receives the withdrawn lamports, and signs
AccountMeta::new_readonly(deposit_account, false),   // only read to check the seeds, no mut in the struct
AccountMeta::new(vault_account, false),              // mut: loses lamports; the program signs for it
AccountMeta::new_readonly(system_program::ID, false),
```

What happens when they are wrong:

| Mistake | Result |
|---|---|
| `new_readonly` for a `mut` field | Anchor `ConstraintMut` (2000), or a runtime error that the program changed a read-only account |
| `false` for a `Signer` field | Anchor `AccountNotSigner` (3010). The handler never runs |
| `true` for an account whose keypair is not passed to `Transaction::new_signed_with_payer(.., &[signer], ..)` | The transaction is rejected for a missing signature. The flag says who must sign, and the keypairs in the list are the actual signatures |
| `new` for an account that is only read | Works, but takes a write lock and blocks parallel transactions on that account. Avoid it |
| Wrong order | The accounts bind by position, so you get a seeds, owner or discriminator error on the wrong account |

The fee payer is the first signer in the transaction and is always writable. `Transaction::new_signed_with_payer(.., Some(&signer.pubkey()), ..)` adds that for you, and the account is also `new(.., true)` in the list because the program uses it.

## Working with the author

- Diagnose in text first and offer the fix. Don't edit or run builds unless asked. The author usually runs `anchor build` and the tests themselves.
- When editing, use the Edit tool, never shell commands such as `sed`.
- Use they/them for anyone whose pronouns haven't been stated.
