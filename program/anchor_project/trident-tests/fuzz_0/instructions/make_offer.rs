use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("9VdKGKXs5ZJd6Cr9GtJcPP8fdUSmRgvkYScvhi1oPkFc")]
#[discriminator([214u8, 98u8, 97u8, 35u8, 59u8, 12u8, 44u8, 178u8])]
pub struct MakeOfferInstruction {
    pub accounts: MakeOfferInstructionAccounts,
    pub data: MakeOfferInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(MakeOfferInstructionData)]
#[storage(FuzzAccounts)]
pub struct MakeOfferInstructionAccounts {
    #[account(mut, signer)]
    pub maker: TridentAccount,

    pub token_mint_a: TridentAccount,

    pub token_mint_b: TridentAccount,

    #[account(mut)]
    pub maker_token_account_a: TridentAccount,

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
pub struct MakeOfferInstructionData {
    pub id: u64,

    pub token_a_offered_amount: u64,

    pub token_b_wanted_amount: u64,
}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for MakeOfferInstruction {
    type IxAccounts = FuzzAccounts;
}
