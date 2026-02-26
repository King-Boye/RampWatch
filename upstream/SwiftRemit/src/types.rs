//! Type definitions for the SwiftRemit contract.
//!
//! This module defines the core data structures used throughout the contract,
//! including remittance records and status enums.

use soroban_sdk::{contracttype, Address, String, Vec};

/// Status of a remittance transaction following a structured state machine.
///
/// State Transitions:
/// ```
/// INITIATED → SUBMITTED → PENDING_ANCHOR → COMPLETED
///                                        ↘ FAILED
/// ```
///
/// Terminal States: COMPLETED, FAILED
/// - Once a remittance reaches a terminal state, no further transitions are allowed
/// - This ensures data integrity and prevents inconsistent transfer statuses
///
/// State Descriptions:
/// - `Initiated`: Initial state when remittance is created, funds locked
/// - `Submitted`: Remittance submitted for processing by agent
/// - `PendingAnchor`: Awaiting anchor/external confirmation
/// - `Completed`: Successfully completed, agent received payout
/// - `Failed`: Failed at any stage, funds refunded to sender
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RemittanceStatus {
    /// Initial state: Remittance created, funds locked in contract
    Initiated,
    /// Submitted for processing by agent
    Submitted,
    /// Awaiting anchor/external confirmation
    PendingAnchor,
    /// Terminal state: Successfully completed
    Completed,
    /// Terminal state: Failed, funds refunded
    Failed,
}

impl RemittanceStatus {
    /// Checks if this status is a terminal state.
    ///
    /// Terminal states (COMPLETED, FAILED) cannot transition to any other state.
    ///
    /// # Returns
    ///
    /// * `true` - Status is terminal (COMPLETED or FAILED)
    /// * `false` - Status is non-terminal and can transition
    pub fn is_terminal(&self) -> bool {
        matches!(self, RemittanceStatus::Completed | RemittanceStatus::Failed)
    }

    /// Checks if a transition to the target status is valid from this status.
    ///
    /// # Arguments
    ///
    /// * `to` - The target status to transition to
    ///
    /// # Returns
    ///
    /// * `true` - Transition is valid
    /// * `false` - Transition is invalid
    pub fn can_transition_to(&self, to: &RemittanceStatus) -> bool {
        match (self, to) {
            // From Initiated
            (RemittanceStatus::Initiated, RemittanceStatus::Submitted) => true,
            (RemittanceStatus::Initiated, RemittanceStatus::Failed) => true,
            
            // From Submitted
            (RemittanceStatus::Submitted, RemittanceStatus::PendingAnchor) => true,
            (RemittanceStatus::Submitted, RemittanceStatus::Failed) => true,
            
            // From PendingAnchor
            (RemittanceStatus::PendingAnchor, RemittanceStatus::Completed) => true,
            (RemittanceStatus::PendingAnchor, RemittanceStatus::Failed) => true,
            
            // Terminal states cannot transition
            (RemittanceStatus::Completed, _) => false,
            (RemittanceStatus::Failed, _) => false,
            
            // All other transitions are invalid
            _ => false,
        }
    }

    /// Returns the next valid states that can be transitioned to from this status.
    ///
    /// # Returns
    ///
    /// Vector of valid next states
    pub fn next_valid_states(&self) -> Vec<RemittanceStatus> {
        match self {
            RemittanceStatus::Initiated => {
                vec![RemittanceStatus::Submitted, RemittanceStatus::Failed]
            }
            RemittanceStatus::Submitted => {
                vec![RemittanceStatus::PendingAnchor, RemittanceStatus::Failed]
            }
            RemittanceStatus::PendingAnchor => {
                vec![RemittanceStatus::Completed, RemittanceStatus::Failed]
            }
            RemittanceStatus::Completed | RemittanceStatus::Failed => {
                vec![] // Terminal states have no valid transitions
            }
        }
    }
}

/// A remittance transaction record.
///
/// Contains all information about a cross-border remittance including
/// parties involved, amounts, fees, status, and optional expiry.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Remittance {
    /// Unique identifier for this remittance
    pub id: u64,
    /// Address of the sender who initiated the remittance
    pub sender: Address,
    /// Address of the agent who will receive the payout
    pub agent: Address,
    /// Total amount sent by the sender (in USDC)
    pub amount: i128,
    /// Platform fee deducted from the amount (in USDC)
    pub fee: i128,
    /// Current status of the remittance
    pub status: RemittanceStatus,
    /// Optional expiry timestamp (seconds since epoch) for settlement
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
