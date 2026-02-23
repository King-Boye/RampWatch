use soroban_sdk::contracterror;

/// Error codes for AnchorKit contract operations.
/// All error codes are in the range 100-130 for stable API compatibility.
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
    InvalidPublicKey = 9,
    InvalidEndpointFormat = 10,
    EndpointNotFound = 11,
    EndpointAlreadyExists = 12,
    ServicesNotConfigured = 13,
    InvalidServiceType = 14,

    /// Session-related errors
    SessionNotFound = 16,
    InvalidSessionId = 17,
    SessionReplayAttack = 18,

    /// Quote-related errors
    InvalidQuote = 19,
    StaleQuote = 20,
    NoQuotesAvailable = 21,
    QuoteNotFound = 22,

    /// Transaction intent / compliance errors
    InvalidTransactionIntent = 23,
    ComplianceNotMet = 24,

    /// Configuration validation errors
    InvalidConfig = 25,
    DuplicateAttestor = 26,
    NoEnabledAttestors = 27,

    /// Detailed config validation errors
    InvalidConfigName = 28,
    InvalidConfigVersion = 29,
    InvalidConfigNetwork = 30,
    InvalidAttestorName = 31,
    InvalidAttestorAddress = 32,
    InvalidAttestorRole = 33,

    /// Credential errors
    InvalidCredentialFormat = 34,
    CredentialNotFound = 35,
    InsecureCredentialStorage = 36,
    CredentialExpired = 37,

    /// Anchor metadata errors
    InvalidAnchorMetadata = 38,
    AnchorMetadataNotFound = 39,
    NoAnchorsAvailable = 40,

    /// Transport errors (HTTP/Network layer)
    TransportError = 41, // Generic transport/network error
    TransportTimeout = 42,      // Timeout errors (408, 504)
    TransportUnauthorized = 43, // Auth errors (401, 403)

    /// Protocol errors (Anchor validation layer)
    ProtocolError = 44, // Generic protocol error
    ProtocolInvalidPayload = 45,      // Invalid/malformed payload
    ProtocolRateLimitExceeded = 46,   // Rate limiting (retryable)
    ProtocolComplianceViolation = 47, // Compliance/KYC errors

    /// Cache errors
    CacheExpired = 48,
    CacheNotFound = 49,
}
