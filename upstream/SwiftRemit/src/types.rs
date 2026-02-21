use soroban_sdk::{contracttype, Address, String, Vec};

/// Maximum number of settlements that can be processed in a single batch.
/// This limit prevents excessive resource consumption in a single transaction.
pub const MAX_BATCH_SIZE: u32 = 50;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RemittanceStatus {
    Pending,
    Authorized,
    Settled,
    Finalized,
    Failed,
}

impl RemittanceStatus {
    pub fn can_transition_to(&self, next: &RemittanceStatus) -> bool {
        match (self, next) {
            (RemittanceStatus::Pending, RemittanceStatus::Authorized) => true,
            (RemittanceStatus::Pending, RemittanceStatus::Failed) => true,

            (RemittanceStatus::Authorized, RemittanceStatus::Settled) => true,
            (RemittanceStatus::Authorized, RemittanceStatus::Failed) => true,

            (RemittanceStatus::Settled, RemittanceStatus::Finalized) => true,

            // Allow transitions to Failed from any non-terminal state

            _ => false,
        }
    }
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

/// Result of a settlement simulation.
/// Predicts the outcome without executing state changes.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SettlementSimulation {
    /// Whether the settlement would succeed
    pub would_succeed: bool,
    /// The payout amount the agent would receive (amount - fee)
    pub payout_amount: i128,
    /// The platform fee that would be collected
    pub fee: i128,
    /// Error message if would_succeed is false
    pub error_message: Option<u32>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DailyLimit {
    pub currency: String,
    pub country: String,
    pub limit: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransferRecord {
    pub timestamp: u64,
    pub amount: i128,
}
