/// The clock account data
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Clock {
    #[prost(uint64, tag = "1")]
    pub slot: u64,
    #[prost(uint64, tag = "2")]
    pub epoch_start_timestamp: u64,
    #[prost(uint64, tag = "3")]
    pub epoch: u64,
    #[prost(uint64, tag = "4")]
    pub leader_schedule_epoch: u64,
    #[prost(uint64, tag = "5")]
    pub unix_timestamp: u64,
}
/// The data for the Rent account
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rent {
    #[prost(uint64, tag = "1")]
    pub lamports_per_byte_year: u64,
    #[prost(double, tag = "2")]
    pub exemption_threshold: f64,
    #[prost(uint64, tag = "3")]
    pub burn_percent: u64,
}
/// The recent slot hash vector contents
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SlotHash {
    #[prost(uint64, tag = "1")]
    pub slot: u64,
    #[prost(bytes = "vec", tag = "2")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
/// The sysvar cache for a transaction execution
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SysvarCache {
    #[prost(message, optional, tag = "1")]
    pub clock: ::core::option::Option<Clock>,
    #[prost(message, optional, tag = "2")]
    pub rent: ::core::option::Option<Rent>,
    /// Slot hashes sysvar: SysvarS1otHashes111111111111111111111111111
    #[prost(message, repeated, tag = "3")]
    pub slot_hash: ::prost::alloc::vec::Vec<SlotHash>,
}
/// A set of feature flags.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeatureSet {
    /// Every item in this list marks an enabled feature.  The value of
    /// each item is the first 8 bytes of the feature ID as a little-
    /// endian integer.
    #[prost(fixed64, repeated, tag = "1")]
    pub features: ::prost::alloc::vec::Vec<u64>,
}
/// A seed address.  This is not a PDA.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeedAddress {
    /// The seed address base.  (32 bytes)
    #[prost(bytes = "vec", tag = "1")]
    pub base: ::prost::alloc::vec::Vec<u8>,
    /// The seed path  (<= 32 bytes)
    #[prost(bytes = "vec", tag = "2")]
    pub seed: ::prost::alloc::vec::Vec<u8>,
    /// The seed address owner.  (32 bytes)
    #[prost(bytes = "vec", tag = "3")]
    pub owner: ::prost::alloc::vec::Vec<u8>,
}
/// The complete state of an account excluding its public key.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcctState {
    /// The account address.  (32 bytes)
    #[prost(bytes = "vec", tag = "1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "2")]
    pub lamports: u64,
    /// Account data is limited to 10 MiB on Solana mainnet as of 2024-Feb.
    #[prost(bytes = "vec", tag = "3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "4")]
    pub executable: bool,
    /// The rent epoch is deprecated on Solana mainnet as of 2024-Feb.
    /// If ommitted, implies a value of UINT64_MAX.
    #[prost(uint64, tag = "5")]
    pub rent_epoch: u64,
    /// Address of the program that owns this account.  (32 bytes)
    #[prost(bytes = "vec", tag = "6")]
    pub owner: ::prost::alloc::vec::Vec<u8>,
    /// The account address, but derived as a seed address.  Overrides
    /// `address` if present.
    /// TODO: This is a solfuzz specific extension and is not compliant
    /// with the org.solana.sealevel.v1 API.
    #[prost(message, optional, tag = "7")]
    pub seed_addr: ::core::option::Option<SeedAddress>,
}
/// EpochContext includes context scoped to an epoch.
/// On "real" ledgers, it is created during the epoch boundary.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EpochContext {
    #[prost(message, optional, tag = "1")]
    pub features: ::core::option::Option<FeatureSet>,
}
/// SlotContext includes context scoped to a block.
/// On "real" ledgers, it is created during the slot boundary.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SlotContext {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub recent_block_hashes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// public key for the leader
    #[prost(bytes = "vec", tag = "2")]
    pub leader: ::prost::alloc::vec::Vec<u8>,
    /// Slot number
    #[prost(fixed64, tag = "3")]
    pub slot: u64,
    #[prost(message, optional, tag = "4")]
    pub sysvar_cache: ::core::option::Option<SysvarCache>,
}
/// Message header contains the counts of required readonly and signatures
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageHeader {
    #[prost(uint32, tag = "1")]
    pub num_required_signatures: u32,
    #[prost(uint32, tag = "2")]
    pub num_readonly_signed_accounts: u32,
    #[prost(uint32, tag = "3")]
    pub num_readonly_unsigned_accounts: u32,
}
/// The instruction a transaction executes
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompiledInstruction {
    /// Index into the message pubkey array
    #[prost(uint32, tag = "1")]
    pub program_id_index: u32,
    /// Indexes into the message pubkey array
    #[prost(uint32, repeated, tag = "2")]
    pub accounts: ::prost::alloc::vec::Vec<u32>,
    #[prost(bytes = "vec", tag = "3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// List of address table lookups used to load additional accounts for a transaction
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageAddressTableLookup {
    #[prost(bytes = "vec", tag = "1")]
    pub account_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, repeated, tag = "2")]
    pub writable_indexes: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, tag = "3")]
    pub readonly_indexes: ::prost::alloc::vec::Vec<u32>,
}
/// Addresses loaded with on-chain lookup tables
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadedAddresses {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub writable: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub readonly: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// Message contains the transaction data
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionMessage {
    /// Whether this is a legacy message or not
    #[prost(bool, tag = "1")]
    pub is_legacy: bool,
    #[prost(message, optional, tag = "2")]
    pub header: ::core::option::Option<MessageHeader>,
    /// Vector of pubkeys
    #[prost(bytes = "vec", repeated, tag = "3")]
    pub account_keys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// Data associated with the accounts referred above. Not all accounts need to be here.
    #[prost(message, repeated, tag = "4")]
    pub account_shared_data: ::prost::alloc::vec::Vec<AcctState>,
    /// The block hash contains 32-bytes
    #[prost(bytes = "vec", tag = "5")]
    pub recent_blockhash: ::prost::alloc::vec::Vec<u8>,
    /// The instructions this transaction executes
    #[prost(message, repeated, tag = "6")]
    pub instructions: ::prost::alloc::vec::Vec<CompiledInstruction>,
    /// Not available in legacy message
    #[prost(message, repeated, tag = "7")]
    pub address_table_lookups: ::prost::alloc::vec::Vec<MessageAddressTableLookup>,
    /// Not available in legacy messages
    #[prost(message, optional, tag = "8")]
    pub loaded_addresses: ::core::option::Option<LoadedAddresses>,
}
/// A valid verified transaction
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SanitizedTransaction {
    /// The transaction information
    #[prost(message, optional, tag = "1")]
    pub message: ::core::option::Option<TransactionMessage>,
    /// The message hash
    #[prost(bytes = "vec", tag = "2")]
    pub message_hash: ::prost::alloc::vec::Vec<u8>,
    /// Is this a voting transaction?
    #[prost(bool, tag = "3")]
    pub is_simple_vote_tx: bool,
    /// The signatures needed in the transaction
    #[prost(bytes = "vec", repeated, tag = "4")]
    pub signatures: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// This Transaction context be used to fuzz either `load_execute_and_commit_transactions`,
/// `load_and_execute_transactions` in `bank.rs` or `load_and_execute_sanitized_transactions`
/// in `svm/transaction_processor.rs`
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxnContext {
    /// The transaction data
    #[prost(message, optional, tag = "1")]
    pub tx: ::core::option::Option<SanitizedTransaction>,
    /// The maximum age allowed for this transaction
    #[prost(uint64, tag = "2")]
    pub max_age: u64,
    /// The limit of bytes allowed for this transaction to load
    #[prost(uint64, tag = "3")]
    pub log_messages_byte_limit: u64,
    #[prost(message, optional, tag = "4")]
    pub epoch_ctx: ::core::option::Option<EpochContext>,
    #[prost(message, optional, tag = "5")]
    pub slot_ctx: ::core::option::Option<SlotContext>,
}
/// The resulting state of an account after a transaction
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResultingState {
    #[prost(message, optional, tag = "1")]
    pub state: ::core::option::Option<AcctState>,
    #[prost(uint64, tag = "2")]
    pub transaction_rent: u64,
    #[prost(message, optional, tag = "3")]
    pub rent_debit: ::core::option::Option<RentDebits>,
}
/// The rent state for an account after a transaction
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RentDebits {
    #[prost(uint64, tag = "1")]
    pub rent_collected: u64,
    #[prost(uint64, tag = "2")]
    pub post_balance: u64,
}
/// The execution results for a transaction
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxnResult {
    /// Whether this transaction was executed
    #[prost(bool, tag = "1")]
    pub executed: bool,
    /// The state of each account after the transaction
    #[prost(message, repeated, tag = "2")]
    pub states: ::prost::alloc::vec::Vec<ResultingState>,
    #[prost(uint64, tag = "3")]
    pub rent: u64,
    /// If an executed transaction has no error
    #[prost(bool, tag = "4")]
    pub is_ok: bool,
    /// The transaction status (error code)
    #[prost(uint32, tag = "5")]
    pub status: u32,
    /// The return data from this transaction, if any
    #[prost(bytes = "vec", tag = "6")]
    pub return_data: ::prost::alloc::vec::Vec<u8>,
    /// Number of executed compute units
    #[prost(uint64, tag = "7")]
    pub executed_units: u64,
    /// The change in accounts data len for this transaction
    #[prost(uint64, tag = "8")]
    pub accounts_data_len_delta: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstrAcct {
    /// Selects an account in an external list
    #[prost(uint32, tag = "1")]
    pub index: u32,
    #[prost(bool, tag = "2")]
    pub is_writable: bool,
    #[prost(bool, tag = "3")]
    pub is_signer: bool,
}
/// The execution context of a program invocation (aka instruction).
/// Contains all required information to independently replay an instruction.
/// Also includes partial transaction context.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstrContext {
    /// The address of the program invoked.  (32 bytes)
    #[prost(bytes = "vec", tag = "1")]
    pub program_id: ::prost::alloc::vec::Vec<u8>,
    /// Account state accessed by the instruction.  This may include
    /// indirect accesses like sysvars.
    #[prost(message, repeated, tag = "3")]
    pub accounts: ::prost::alloc::vec::Vec<AcctState>,
    /// Account access list for this instruction (refers to above accounts list)
    #[prost(message, repeated, tag = "4")]
    pub instr_accounts: ::prost::alloc::vec::Vec<InstrAcct>,
    /// The input data passed to program execution.
    #[prost(bytes = "vec", tag = "5")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "6")]
    pub cu_avail: u64,
    #[prost(message, optional, tag = "7")]
    pub txn_context: ::core::option::Option<TxnContext>,
    #[prost(message, optional, tag = "8")]
    pub slot_context: ::core::option::Option<SlotContext>,
    #[prost(message, optional, tag = "9")]
    pub epoch_context: ::core::option::Option<EpochContext>,
}
/// The results of executing an InstrContext.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstrEffects {
    /// result is zero if the instruction executed succesfully.
    /// Otherwise, a non-zero error code.  Error codes are not relevant to
    /// consensus.
    #[prost(int32, tag = "1")]
    pub result: i32,
    /// Some error cases additionally have a custom error code.  Unlike
    /// the expected_result, this is stable across clients.
    #[prost(uint32, tag = "2")]
    pub custom_err: u32,
    /// Copies of accounts that were changed.  May be in an arbitrary
    /// order.  The pubkey of each account is unique in this list.  Each
    /// account address modified here must also be in the
    /// InstrContext.
    #[prost(message, repeated, tag = "3")]
    pub modified_accounts: ::prost::alloc::vec::Vec<AcctState>,
    #[prost(uint64, tag = "4")]
    pub cu_avail: u64,
    /// Instruction return data.
    #[prost(bytes = "vec", tag = "5")]
    pub return_data: ::prost::alloc::vec::Vec<u8>,
}
/// An instruction processing test fixture.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstrFixture {
    #[prost(message, optional, tag = "1")]
    pub input: ::core::option::Option<InstrContext>,
    #[prost(message, optional, tag = "2")]
    pub output: ::core::option::Option<InstrEffects>,
}
