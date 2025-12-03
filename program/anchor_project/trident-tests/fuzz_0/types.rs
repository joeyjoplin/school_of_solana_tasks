use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

/// File containing all custom types which can be used
/// in transactions and instructions or invariant checks.
///
/// You can define your own custom types here.

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct Offer {
    pub id: u64,

    pub maker: TridentPubkey,

    pub token_mint_a: TridentPubkey,

    pub token_mint_b: TridentPubkey,

    pub token_b_wanted_amount: u64,

    pub bump: u8,
}
