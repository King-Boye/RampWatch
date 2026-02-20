//! Type definitions for the SwiftRemit contract.
//!
//! This module defines the core data structures used throughout the contract,
//! including remittance records and status enums.

use soroban_sdk::{contracttype, Address};

/// Status of a remittance transaction.
///
/// Remittances progress through these states:
/// - `Pending`: Initial state after creation, awaiting agent confirmation
/// - `Completed`: Agent has confirmed payout and received funds
/// - `Cancelled`: Sender has cancelled and received refund
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RemittanceStatus {
    /// Remittance is awaiting agent confirmation
    Pending,
    /// Remittance has been paid out to the agent
    Completed,
    /// Remittance has been cancelled and refunded to sender
    Cancelled,
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
