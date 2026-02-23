//! Error types for the SwiftRemit contract.
//!
//! This module defines all possible error conditions that can occur
//! during contract execution. All errors are explicitly defined with
//! unique error codes to ensure deterministic error handling.

use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    // ═══════════════════════════════════════════════════════════════════════════
    // Initialization Errors (1-2)
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// Contract has already been initialized.
    /// Cause: Attempting to call initialize() on an already initialized contract.
    AlreadyInitialized = 1,
    
    /// Contract has not been initialized yet.
    /// Cause: Attempting operations before calling initialize().
    NotInitialized = 2,
    
    // ═══════════════════════════════════════════════════════════════════════════
    // Validation Errors (3-10)
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// Amount must be greater than zero.
    /// Cause: Providing zero or negative amount in remittance creation.
    InvalidAmount = 3,
    
    /// Fee must be between 0 and 10000 basis points (0-100%).
    /// Cause: Setting platform fee outside valid range.
    InvalidFeeBps = 4,
    
    /// Agent is not registered in the system.
    /// Cause: Attempting to create remittance with unregistered agent.
    AgentNotRegistered = 5,
    
    /// Remittance not found.
    /// Cause: Querying or operating on non-existent remittance ID.
    RemittanceNotFound = 6,
    
    /// Invalid remittance status for this operation.
    /// Cause: Attempting operation on remittance in wrong status (e.g., settling completed remittance).
    InvalidStatus = 7,
    
    /// Invalid state transition attempted.
    /// Cause: Attempting to transition remittance to invalid state.
    InvalidStateTransition = 8,
    
    /// No fees available to withdraw.
    /// Cause: Attempting to withdraw fees when accumulated fees is zero or negative.
    NoFeesToWithdraw = 9,
    
    /// Invalid address format or validation failed.
    /// Cause: Address does not meet validation requirements.
    InvalidAddress = 10,
    
    // ═══════════════════════════════════════════════════════════════════════════
    // Settlement Errors (11-14)
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// Settlement window has expired.
    /// Cause: Attempting to settle remittance after expiry timestamp.
    SettlementExpired = 11,
    
    /// Settlement has already been executed.
    /// Cause: Attempting to settle the same remittance twice (duplicate prevention).
    DuplicateSettlement = 12,
    
    /// Contract is paused. Settlements are temporarily disabled.
    /// Cause: Attempting confirm_payout() while contract is in paused state.
    ContractPaused = 13,
    
    RateLimitExceeded = 14,
    
    // ═══════════════════════════════════════════════════════════════════════════
    // Authorization Errors (15-18)
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// Caller is not authorized to perform admin operations.
    /// Cause: Non-admin attempting to perform admin-only operations.
    Unauthorized = 15,
    
    /// Admin address already exists in the system.
    /// Cause: Attempting to add an admin that is already registered.
    AdminAlreadyExists = 16,
    
    /// Admin address does not exist in the system.
    /// Cause: Attempting to remove an admin that is not registered.
    AdminNotFound = 17,
    
    /// Cannot remove the last admin from the system.
    /// Cause: Attempting to remove the only remaining admin.
    CannotRemoveLastAdmin = 18,
    
    // ═══════════════════════════════════════════════════════════════════════════
    // Token Whitelist Errors (19-20)
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// Token is not whitelisted for use in the system.
    /// Cause: Attempting to initialize contract with non-whitelisted token.
    TokenNotWhitelisted = 19,
    
    /// Token is already whitelisted in the system.
    /// Cause: Attempting to add a token that is already whitelisted.
    TokenAlreadyWhitelisted = 20,
    
    // ═══════════════════════════════════════════════════════════════════════════
    // Migration Errors (21-23)
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// Migration hash verification failed.
    /// Cause: Snapshot hash doesn't match computed hash (data tampering or corruption).
    InvalidMigrationHash = 21,
    
    /// Migration already in progress or completed.
    /// Cause: Attempting to start migration when one is already active.
    MigrationInProgress = 22,
    
    /// Migration batch out of order or invalid.
    /// Cause: Importing batches in wrong order or invalid batch number.
    InvalidMigrationBatch = 23,
    
    // ═══════════════════════════════════════════════════════════════════════════
    // Rate Limiting Errors (24)
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// Daily send limit exceeded for this user.
    /// Cause: User's total transfers in the last 24 hours exceed the configured limit.
    DailySendLimitExceeded = 24,
    
    // ═══════════════════════════════════════════════════════════════════════════
    // Arithmetic Errors (25-26)
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// Arithmetic overflow occurred in calculation.
    /// Cause: Result of arithmetic operation exceeds maximum value for type.
    Overflow = 25,
    
    /// Arithmetic underflow occurred in calculation.
    /// Cause: Result of arithmetic operation is less than minimum value for type.
    Underflow = 26,
    
    // ═══════════════════════════════════════════════════════════════════════════
    // Data Integrity Errors (27-30)
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// Net settlement validation failed.
    /// Cause: Net settlement calculations don't preserve fees or amounts correctly.
    NetSettlementValidationFailed = 27,
    
    /// Settlement counter overflow.
    /// Cause: Settlement counter would exceed u64::MAX (extremely unlikely).
    SettlementCounterOverflow = 28,
    
    /// Invalid batch size.
    /// Cause: Batch size is zero or exceeds maximum allowed.
    InvalidBatchSize = 29,
    
    /// Data corruption detected.
    /// Cause: Storage data is corrupted or inconsistent.
    DataCorruption = 30,
    
    // ═══════════════════════════════════════════════════════════════════════════
    // Collection Errors (31-33)
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// Index out of bounds.
    /// Cause: Attempting to access collection element at invalid index.
    IndexOutOfBounds = 31,
    
    /// Collection is empty.
    /// Cause: Attempting operation on empty collection that requires elements.
    EmptyCollection = 32,
    
    /// Key not found in map.
    /// Cause: Attempting to access map value with non-existent key.
    KeyNotFound = 33,
    
    // ═══════════════════════════════════════════════════════════════════════════
    // String/Symbol Errors (34-35)
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// String conversion failed.
    /// Cause: Unable to convert between string types or invalid string format.
    StringConversionFailed = 34,
    
    /// Symbol is invalid or malformed.
    /// Cause: Symbol contains invalid characters or exceeds length limits.
    InvalidSymbol = 35,
    
    // ═══════════════════════════════════════════════════════════════════════════
    // Escrow Errors (36-37)
    // ═══════════════════════════════════════════════════════════════════════════
    
    /// Escrow not found.
    /// Cause: Querying or operating on non-existent escrow ID.
    EscrowNotFound = 36,
    
    /// Invalid escrow status for this operation.
    /// Cause: Attempting operation on escrow in wrong status.
    InvalidEscrowStatus = 37,
}
