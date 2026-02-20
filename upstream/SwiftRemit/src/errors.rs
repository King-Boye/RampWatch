//! Error types for the SwiftRemit contract.
//!
//! This module defines all possible error conditions that can occur
//! during contract execution.

use soroban_sdk::contracterror;

/// Errors that can occur during contract operations.
///
/// Each error has a unique numeric code for identification in transaction results.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    /// Contract has already been initialized and cannot be initialized again
    AlreadyInitialized = 1,
    /// Contract has not been initialized yet
    NotInitialized = 2,
    /// Amount is zero, negative, or otherwise invalid
    InvalidAmount = 3,
    /// Fee in basis points exceeds maximum allowed (10000 bps = 100%)
    InvalidFeeBps = 4,
    /// Specified agent is not registered in the system
    AgentNotRegistered = 5,
    /// Remittance with the specified ID does not exist
    RemittanceNotFound = 6,
    /// Operation not allowed for remittance in current status
    InvalidStatus = 7,
    /// Arithmetic operation resulted in overflow
    Overflow = 8,
    /// No fees available to withdraw (balance is zero or negative)
    NoFeesToWithdraw = 9,
    /// Address validation failed
    InvalidAddress = 10,
    /// Settlement cannot be executed because expiry time has passed
    SettlementExpired = 11,
    /// Settlement has already been executed for this remittance
    DuplicateSettlement = 12,
}
