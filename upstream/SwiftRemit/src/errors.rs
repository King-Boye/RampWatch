//! Error types for the SwiftRemit contract.
//!
//! This module defines all possible error conditions that can occur
//! during contract execution.

use soroban_sdk::contracterror;


#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    /// Contract has not been initialized yet.
    NotInitialized = 1,

    /// Contract has already been initialized.
    AlreadyInitialized = 2,

    /// Remittance with the specified ID does not exist.
    RemittanceNotFound = 3,

    /// Remittance status is invalid for the requested operation.
    InvalidStatus = 4,

    /// Address validation failed.
    InvalidAddress = 5,

    /// Amount is invalid (zero, negative, or exceeds limits).
    InvalidAmount = 6,

    /// Arithmetic overflow occurred during calculation.
    Overflow = 7,

    /// Settlement has expired based on the expiry timestamp.
    SettlementExpired = 8,

    /// Agent is not registered in the system.
    AgentNotRegistered = 9,

    /// Agent is already registered in the system.
    AgentAlreadyRegistered = 10,

    /// Caller is not authorized as admin.
    NotAdmin = 11,

    /// Duplicate settlement detected.
    DuplicateSettlement = 12,

    /// Contract is paused. Settlements are temporarily disabled.
    ContractPaused = 13,
    
    /// Rate limit exceeded. Sender must wait before submitting another settlement.
    RateLimitExceeded = 14,

    /// Caller is not authorized to perform admin operations.
    Unauthorized = 15,

    /// Admin address already exists in the system.
    AdminAlreadyExists = 16,

    /// Admin address does not exist in the system.
    AdminNotFound = 17,

    /// Cannot remove the last admin from the system.
    CannotRemoveLastAdmin = 18,
    
    /// Token is not whitelisted for use in the system.
    TokenNotWhitelisted = 19,
    
    /// Token is already whitelisted in the system.
    TokenAlreadyWhitelisted = 20,
    
    /// Migration hash verification failed.
    InvalidMigrationHash = 21,
    
    /// Migration already in progress or completed.
    MigrationInProgress = 22,
    
    /// Migration batch out of order or invalid.
    InvalidMigrationBatch = 23,
    
    /// Daily send limit exceeded for this user.
    DailySendLimitExceeded = 24,
}
