use soroban_sdk::{contracttype, Address, Bytes, BytesN, String, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Attestation {
    pub id: u64,
    pub issuer: Address,
    pub subject: Address,
    pub timestamp: u64,
    pub payload_hash: BytesN<32>,
    pub signature: Bytes,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Endpoint {
    pub url: String,
    pub attestor: Address,
    pub is_active: bool,
}


/// Supported service types for anchors
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ServiceType {
    Deposits = 1,
    Withdrawals = 2,
    Quotes = 3,
    KYC = 4,
}

/// Configuration of supported services for an anchor
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnchorServices {
    pub anchor: Address,
    pub services: Vec<ServiceType>,
}

/// Quote data structure for rate comparison
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuoteData {
    pub anchor: Address,
    pub base_asset: String,
    pub quote_asset: String,
    pub rate: u64, // Rate in basis points (10000 = 1.0)
    pub fee_percentage: u32, // Fee in basis points
    pub minimum_amount: u64,
    pub maximum_amount: u64,
    pub valid_until: u64, // Unix timestamp
    pub quote_id: u64,
}

/// Rate comparison result
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RateComparison {
    pub best_quote: QuoteData,
    pub all_quotes: Vec<QuoteData>,
    pub comparison_timestamp: u64,
}

/// Quote request parameters
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuoteRequest {
    pub base_asset: String,
    pub quote_asset: String,
    pub amount: u64,
    pub operation_type: ServiceType, // Deposits or Withdrawals
}

/// Represents a reproducible interaction session.
/// Each session is uniquely identified and tracks all operations within it.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InteractionSession {
    /// Unique session identifier
    pub session_id: u64,
    /// Address that initiated the session
    pub initiator: Address,
    /// Unix timestamp when session was created
    pub created_at: u64,
    /// Total number of operations in this session
    pub operation_count: u64,
    /// Session nonce for replay protection
    pub nonce: u64,
}

/// Context for each operation within a session.
/// Enables full traceability of all contract interactions.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationContext {
    /// Session ID this operation belongs to
    pub session_id: u64,
    /// Sequential operation index within the session (0-based)
    pub operation_index: u64,
    /// Type of operation (e.g., "init", "register", "attest", "endpoint")
    pub operation_type: String,
    /// Timestamp of operation execution
    pub timestamp: u64,
    /// Result status: "success" or error code
    pub status: String,
    /// Optional result data (e.g., attestation ID, session ID)
    pub result_data: u64,
}

/// Full audit log entry for reproducibility
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuditLog {
    /// Log entry ID (incremental)
    pub log_id: u64,
    /// Associated session ID
    pub session_id: u64,
    /// The operation context
    pub operation: OperationContext,
    /// Actor performing the operation
    pub actor: Address,

}
