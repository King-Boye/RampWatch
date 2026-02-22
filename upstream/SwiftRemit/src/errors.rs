//! Error types for the SwiftRemit contract.
//!
//! This module defines all possible error conditions that can occur
//! during contract execution.

use soroban_sdk::contracterror;


#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {

    DuplicateSettlement = 12,

    /// Contract is paused. Settlements are temporarily disabled.
    /// Cause: Attempting confirm_payout() while contract is in paused state.
    ContractPaused = 13,
    
    /// Rate limit exceeded. Sender must wait before submitting another settlement.
    /// Cause: Attempting confirm_payout() before cooldown period has elapsed.
    RateLimitExceeded = 14,
    /// Caller is not authorized to perform admin operations.
    /// Cause: Non-admin attempting to perform admin-only operations.
    Unauthorized = 14,

    /// Admin address already exists in the system.
    /// Cause: Attempting to add an admin that is already registered.
    AdminAlreadyExists = 15,

    /// Admin address does not exist in the system.
    /// Cause: Attempting to remove an admin that is not registered.
    AdminNotFound = 16,

    /// Cannot remove the last admin from the system.
    /// Cause: Attempting to remove the only remaining admin.
    CannotRemoveLastAdmin = 17,

    /// Token is not whitelisted for use in the system.
    /// Cause: Attempting to initialize contract with non-whitelisted token.
    TokenNotWhitelisted = 18,

    /// Token is already whitelisted in the system.
    /// Cause: Attempting to add a token that is already whitelisted.
    TokenAlreadyWhitelisted = 19,

    /// Migration hash verification failed.
    /// Cause: Snapshot hash doesn't match computed hash (data tampering or corruption).
    InvalidMigrationHash = 20,

    /// Migration already in progress or completed.
    /// Cause: Attempting to start migration when one is already active.
    MigrationInProgress = 21,

    /// Migration batch out of order or invalid.
    /// Cause: Importing batches in wrong order or invalid batch number.
    InvalidMigrationBatch = 22,
    
    /// Daily send limit exceeded for this user.
    /// Cause: User's total transfers in the last 24 hours exceed the configured limit.
    DailySendLimitExceeded = 23,
}
