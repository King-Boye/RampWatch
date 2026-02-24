use soroban_sdk::contracterror;

/// Error codes for AnchorKit contract operations.
/// Consolidated to stay within Soroban's contracterror limit.
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
    SessionNotFound = 16,
    InvalidSessionId = 17,

    /// Quote-related errors
    InvalidQuote = 19,
    StaleQuote = 20,
    NoQuotesAvailable = 21,

    /// Transaction intent / compliance errors
    InvalidTransactionIntent = 19,
    ComplianceNotMet = 20,

    /// Configuration validation errors
    InvalidConfig = 25,
    DuplicateAttestor = 26,

    /// Credential errors
    InvalidCredentialFormat = 34,
    CredentialNotFound = 35,
    CredentialExpired = 37,

    /// Anchor metadata errors
    InvalidAnchorMetadata = 38,
    AnchorMetadataNotFound = 39,

    /// Transport errors (HTTP/Network layer)
    TransportError = 41,
    TransportTimeout = 42,
    TransportUnauthorized = 43,

    /// Protocol errors (Anchor validation layer)
    ProtocolError = 44,
    ProtocolInvalidPayload = 45,
    ProtocolRateLimitExceeded = 46,

    /// Cache errors
    CacheExpired = 48,
    CacheNotFound = 49,
    
    /// Rate limiter errors
    RateLimitExceeded = 50,
    
    /// Asset validation errors
    AssetNotConfigured = 51,
    UnsupportedAsset = 52,
}
