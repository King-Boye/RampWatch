use soroban_sdk::contracterror;

/// Error codes for AnchorKit contract operations.
/// Consolidated to stay within Soroban's 32 error variant limit.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    UnauthorizedAttestor = 3,
    AttestorAlreadyRegistered = 4,
    AttestorNotRegistered = 5,
    ReplayAttack = 6,
    InvalidTimestamp = 7,
    AttestationNotFound = 8,
    InvalidEndpointFormat = 9,
    EndpointNotFound = 10,
    ServicesNotConfigured = 11,
    InvalidServiceType = 12,

    /// Session-related errors
    SessionNotFound = 13,
    InvalidSessionId = 14,

    /// Quote-related errors
    InvalidQuote = 15,
    StaleQuote = 16,
    NoQuotesAvailable = 17,
    QuoteNotFound = 18,

    /// Transaction intent / compliance errors
    InvalidTransactionIntent = 19,
    ComplianceNotMet = 20,

    /// Configuration validation errors
    InvalidConfig = 21,

    /// Credential errors
    InvalidCredentialFormat = 22,
    CredentialNotFound = 23,
    InsecureCredentialStorage = 24,
    CredentialExpired = 25,

    /// Anchor metadata errors
    InvalidAnchorMetadata = 26,
    AnchorMetadataNotFound = 27,
    NoAnchorsAvailable = 28,

    /// Rate limiter errors
    RateLimitExceeded = 29,
    
    /// Asset validator errors
    AssetNotConfigured = 30,
    UnsupportedAsset = 31,
}
