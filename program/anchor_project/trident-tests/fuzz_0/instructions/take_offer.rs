use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("9VdKGKXs5ZJd6Cr9GtJcPP8fdUSmRgvkYScvhi1oPkFc")]
#[discriminator([128u8, 156u8, 242u8, 207u8, 237u8, 192u8, 103u8, 240u8])]
pub struct TakeOfferInstruction {
    pub accounts: TakeOfferInstructionAccounts,
    pub data: TakeOfferInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(TakeOfferInstructionData)]
#[storage(FuzzAccounts)]
pub struct TakeOfferInstructionAccounts {
    #[account(mut, signer)]
    pub taker: TridentAccount,

    #[account(mut)]
    pub maker: TridentAccount,

    pub token_mint_a: TridentAccount,

    pub token_mint_b: TridentAccount,

    #[account(mut)]
    pub taker_token_account_a: TridentAccount,

    #[account(mut)]
    pub taker_token_account_b: TridentAccount,

    #[account(mut)]
    pub maker_token_account_b: TridentAccount,

    #[account(mut)]
    pub offer: TridentAccount,

    #[account(mut)]
    pub vault: TridentAccount,

    #[account(address = "11111111111111111111111111111111")]
    pub system_program: TridentAccount,

    pub token_program: TridentAccount,

    #[account(address = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL")]
    pub associated_token_program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct TakeOfferInstructionData {}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for TakeOfferInstruction {
    type IxAccounts = FuzzAccounts;
}
