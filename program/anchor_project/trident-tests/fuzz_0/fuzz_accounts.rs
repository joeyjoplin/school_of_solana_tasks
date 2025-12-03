use trident_fuzz::fuzzing::*;

/// FuzzAccounts contains all available accounts
///
/// You can create your own accounts by adding new fields to the struct.
///
/// Docs: https://ackee.xyz/trident/docs/latest/trident-api-macro/trident-types/fuzz-accounts/
#[derive(Default)]
pub struct FuzzAccounts {
    pub vault: AccountsStorage,

    pub maker_token_account_b: AccountsStorage,

    pub maker_token_account_a: AccountsStorage,

    pub taker_token_account_b: AccountsStorage,

    pub token_mint_b: AccountsStorage,

    pub maker: AccountsStorage,

    pub offer: AccountsStorage,

    pub taker: AccountsStorage,

    pub token_program: AccountsStorage,

    pub associated_token_program: AccountsStorage,

    pub token_mint_a: AccountsStorage,

    pub taker_token_account_a: AccountsStorage,

    pub system_program: AccountsStorage,
}
