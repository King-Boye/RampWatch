use soroban_sdk::{contracttype, Address, Vec};

/// Maximum number of settlements that can be processed in a single batch.
/// This limit prevents excessive resource consumption in a single transaction.
pub const MAX_BATCH_SIZE: u32 = 50;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RemittanceStatus {
    Pending,
    Completed,
    Cancelled,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Remittance {
    pub id: u64,
    pub sender: Address,
    pub agent: Address,
    pub amount: i128,
    pub fee: i128,
    pub status: RemittanceStatus,
    pub expiry: Option<u64>,
}

/// Entry for batch settlement processing.
/// Each entry represents a single remittance to be settled.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BatchSettlementEntry {
    /// The unique ID of the remittance to settle
    pub remittance_id: u64,
}

/// Result of a batch settlement operation.
/// Contains the IDs of successfully settled remittances.
#[contracttype]
#[derive(Clone, Debug)]
pub struct BatchSettlementResult {
    /// List of successfully settled remittance IDs
    pub settled_ids: Vec<u64>,
}
