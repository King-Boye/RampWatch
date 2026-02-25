use crate::error_mapping::{
    get_error_category, get_error_severity, is_protocol_error, is_protocol_error_retryable,
    is_transport_error, is_transport_error_retryable,
};
/// AnchorKitError: Custom error class with standardized error codes and consistent response format
///
/// This module provides a comprehensive error handling system for AnchorKit that:
/// 1. Wraps the base Error enum with additional context and metadata
/// 2. Provides standardized error codes with semantic meaning
/// 3. Ensures consistent response formatting across all operations
/// 4. Enables rich error context for debugging and logging
/// 5. Supports error classification and recovery strategies
use crate::errors::Error;

/// Standardized error codes with semantic meaning
/// These codes are stable and can be used for API contracts
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ErrorCode {
    // Initialization & State (1000-1099)
    AlreadyInitialized = 1001,
    NotInitialized = 1002,

    // Attestor Management (1100-1199)
    UnauthorizedAttestor = 1101,
    AttestorAlreadyRegistered = 1102,
    AttestorNotRegistered = 1103,

    // Security (1200-1299)
    ReplayAttack = 1201,
    InvalidTimestamp = 1202,

    // Attestation (1300-1399)
    AttestationNotFound = 1301,

    // Endpoint Management (1400-1499)
    InvalidEndpointFormat = 1401,
    EndpointNotFound = 1402,

    // Service Configuration (1500-1599)
    ServicesNotConfigured = 1501,
    InvalidServiceType = 1502,

    // Session Management (1600-1699)
    SessionNotFound = 1601,
    InvalidSessionId = 1602,

    // Quote Management (1700-1799)
    InvalidQuote = 1701,
    StaleQuote = 1702,
    NoQuotesAvailable = 1703,

    // Transaction Intent (1800-1899)
    InvalidTransactionIntent = 1801,
    ComplianceNotMet = 1802,

    // Configuration (1900-1999)
    InvalidConfig = 1901,
    DuplicateAttestor = 1902,

    // Credentials (2000-2099)
    InvalidCredentialFormat = 2001,
    CredentialNotFound = 2002,
    CredentialExpired = 2003,

    // Anchor Metadata (2100-2199)
    InvalidAnchorMetadata = 2101,
    AnchorMetadataNotFound = 2102,

    // Transport Layer (2200-2299)
    TransportError = 2201,
    TransportTimeout = 2202,
    TransportUnauthorized = 2203,

    // Protocol Layer (2300-2399)
    ProtocolError = 2301,
    ProtocolInvalidPayload = 2302,
    ProtocolRateLimitExceeded = 2303,

    // Cache (2400-2499)
    CacheExpired = 2401,
    CacheNotFound = 2402,

    // Rate Limiting (2500-2599)
    RateLimitExceeded = 2501,

    // Asset Validation (2600-2699)
    AssetNotConfigured = 2601,
    UnsupportedAsset = 2602,

    // Webhook Middleware (2700-2799)
    WebhookTimestampExpired = 2701,
    WebhookTimestampInFuture = 2702,
    WebhookPayloadTooLarge = 2703,
    WebhookSignatureInvalid = 2704,
    WebhookValidationFailed = 2705,
}

impl ErrorCode {
    /// Convert from base Error to ErrorCode
    pub fn from_error(error: &Error) -> Self {
        match error {
            Error::AlreadyInitialized => ErrorCode::AlreadyInitialized,
            Error::NotInitialized => ErrorCode::NotInitialized,
            Error::UnauthorizedAttestor => ErrorCode::UnauthorizedAttestor,
            Error::AttestorAlreadyRegistered => ErrorCode::AttestorAlreadyRegistered,
            Error::AttestorNotRegistered => ErrorCode::AttestorNotRegistered,
            Error::ReplayAttack => ErrorCode::ReplayAttack,
            Error::InvalidTimestamp => ErrorCode::InvalidTimestamp,
            Error::AttestationNotFound => ErrorCode::AttestationNotFound,
            Error::InvalidEndpointFormat => ErrorCode::InvalidEndpointFormat,
            Error::EndpointNotFound => ErrorCode::EndpointNotFound,
            Error::ServicesNotConfigured => ErrorCode::ServicesNotConfigured,
            Error::InvalidServiceType => ErrorCode::InvalidServiceType,
            Error::SessionNotFound => ErrorCode::SessionNotFound,
            Error::InvalidSessionId => ErrorCode::InvalidSessionId,
            Error::InvalidQuote => ErrorCode::InvalidQuote,
            Error::StaleQuote => ErrorCode::StaleQuote,
            Error::NoQuotesAvailable => ErrorCode::NoQuotesAvailable,
            Error::InvalidTransactionIntent => ErrorCode::InvalidTransactionIntent,
            Error::ComplianceNotMet => ErrorCode::ComplianceNotMet,
            Error::InvalidConfig => ErrorCode::InvalidConfig,
            Error::DuplicateAttestor => ErrorCode::DuplicateAttestor,
            Error::InvalidCredentialFormat => ErrorCode::InvalidCredentialFormat,
            Error::CredentialNotFound => ErrorCode::CredentialNotFound,
            Error::CredentialExpired => ErrorCode::CredentialExpired,
            Error::InvalidAnchorMetadata => ErrorCode::InvalidAnchorMetadata,
            Error::AnchorMetadataNotFound => ErrorCode::AnchorMetadataNotFound,
            Error::TransportError => ErrorCode::TransportError,
            Error::TransportTimeout => ErrorCode::TransportTimeout,
            Error::TransportUnauthorized => ErrorCode::TransportUnauthorized,
            Error::ProtocolError => ErrorCode::ProtocolError,
            Error::ProtocolInvalidPayload => ErrorCode::ProtocolInvalidPayload,
            Error::ProtocolRateLimitExceeded => ErrorCode::ProtocolRateLimitExceeded,
            Error::CacheExpired => ErrorCode::CacheExpired,
            Error::CacheNotFound => ErrorCode::CacheNotFound,
            Error::RateLimitExceeded => ErrorCode::RateLimitExceeded,
            Error::AssetNotConfigured => ErrorCode::AssetNotConfigured,
            Error::UnsupportedAsset => ErrorCode::UnsupportedAsset,
            Error::WebhookTimestampExpired => ErrorCode::WebhookTimestampExpired,
            Error::WebhookTimestampInFuture => ErrorCode::WebhookTimestampInFuture,
            Error::WebhookPayloadTooLarge => ErrorCode::WebhookPayloadTooLarge,
            Error::WebhookSignatureInvalid => ErrorCode::WebhookSignatureInvalid,
            Error::WebhookValidationFailed => ErrorCode::WebhookValidationFailed,
        }
    }

    /// Get human-readable error name
    pub fn name(&self) -> &'static str {
        match self {
            ErrorCode::AlreadyInitialized => "AlreadyInitialized",
            ErrorCode::NotInitialized => "NotInitialized",
            ErrorCode::UnauthorizedAttestor => "UnauthorizedAttestor",
            ErrorCode::AttestorAlreadyRegistered => "AttestorAlreadyRegistered",
            ErrorCode::AttestorNotRegistered => "AttestorNotRegistered",
            ErrorCode::ReplayAttack => "ReplayAttack",
            ErrorCode::InvalidTimestamp => "InvalidTimestamp",
            ErrorCode::AttestationNotFound => "AttestationNotFound",
            ErrorCode::InvalidEndpointFormat => "InvalidEndpointFormat",
            ErrorCode::EndpointNotFound => "EndpointNotFound",
            ErrorCode::ServicesNotConfigured => "ServicesNotConfigured",
            ErrorCode::InvalidServiceType => "InvalidServiceType",
            ErrorCode::SessionNotFound => "SessionNotFound",
            ErrorCode::InvalidSessionId => "InvalidSessionId",
            ErrorCode::InvalidQuote => "InvalidQuote",
            ErrorCode::StaleQuote => "StaleQuote",
            ErrorCode::NoQuotesAvailable => "NoQuotesAvailable",
            ErrorCode::InvalidTransactionIntent => "InvalidTransactionIntent",
            ErrorCode::ComplianceNotMet => "ComplianceNotMet",
            ErrorCode::InvalidConfig => "InvalidConfig",
            ErrorCode::DuplicateAttestor => "DuplicateAttestor",
            ErrorCode::InvalidCredentialFormat => "InvalidCredentialFormat",
            ErrorCode::CredentialNotFound => "CredentialNotFound",
            ErrorCode::CredentialExpired => "CredentialExpired",
            ErrorCode::InvalidAnchorMetadata => "InvalidAnchorMetadata",
            ErrorCode::AnchorMetadataNotFound => "AnchorMetadataNotFound",
            ErrorCode::TransportError => "TransportError",
            ErrorCode::TransportTimeout => "TransportTimeout",
            ErrorCode::TransportUnauthorized => "TransportUnauthorized",
            ErrorCode::ProtocolError => "ProtocolError",
            ErrorCode::ProtocolInvalidPayload => "ProtocolInvalidPayload",
            ErrorCode::ProtocolRateLimitExceeded => "ProtocolRateLimitExceeded",
            ErrorCode::CacheExpired => "CacheExpired",
            ErrorCode::CacheNotFound => "CacheNotFound",
            ErrorCode::RateLimitExceeded => "RateLimitExceeded",
            ErrorCode::AssetNotConfigured => "AssetNotConfigured",
            ErrorCode::UnsupportedAsset => "UnsupportedAsset",
            ErrorCode::WebhookTimestampExpired => "WebhookTimestampExpired",
            ErrorCode::WebhookTimestampInFuture => "WebhookTimestampInFuture",
            ErrorCode::WebhookPayloadTooLarge => "WebhookPayloadTooLarge",
            ErrorCode::WebhookSignatureInvalid => "WebhookSignatureInvalid",
            ErrorCode::WebhookValidationFailed => "WebhookValidationFailed",
        }
    }

    /// Get error category (transport, protocol, application)
    pub fn category(&self) -> ErrorCategory {
        let error = self.to_base_error();
        if is_transport_error(&error) {
            ErrorCategory::Transport
        } else if is_protocol_error(&error) {
            ErrorCategory::Protocol
        } else {
            ErrorCategory::Application
        }
    }

    /// Get error severity level
    pub fn severity(&self) -> ErrorSeverity {
        let error = self.to_base_error();
        match get_error_severity(&error) {
            4 => ErrorSeverity::Critical,
            3 => ErrorSeverity::High,
            2 => ErrorSeverity::Medium,
            _ => ErrorSeverity::Low,
        }
    }

    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        let error = self.to_base_error();
        if is_transport_error(&error) {
            is_transport_error_retryable(&error)
        } else if is_protocol_error(&error) {
            is_protocol_error_retryable(&error)
        } else {
            // Application-level retryable errors
            matches!(
                error,
                Error::AttestationNotFound
                    | Error::SessionNotFound
                    | Error::StaleQuote
                    | Error::NoQuotesAvailable
                    | Error::AnchorMetadataNotFound
                    | Error::CacheExpired
                    | Error::CacheNotFound
                    | Error::ServicesNotConfigured
                    | Error::EndpointNotFound
            )
        }
    }

    /// Convert ErrorCode back to base Error
    fn to_base_error(self) -> Error {
        match self {
            ErrorCode::AlreadyInitialized => Error::AlreadyInitialized,
            ErrorCode::NotInitialized => Error::NotInitialized,
            ErrorCode::UnauthorizedAttestor => Error::UnauthorizedAttestor,
            ErrorCode::AttestorAlreadyRegistered => Error::AttestorAlreadyRegistered,
            ErrorCode::AttestorNotRegistered => Error::AttestorNotRegistered,
            ErrorCode::ReplayAttack => Error::ReplayAttack,
            ErrorCode::InvalidTimestamp => Error::InvalidTimestamp,
            ErrorCode::AttestationNotFound => Error::AttestationNotFound,
            ErrorCode::InvalidEndpointFormat => Error::InvalidEndpointFormat,
            ErrorCode::EndpointNotFound => Error::EndpointNotFound,
            ErrorCode::ServicesNotConfigured => Error::ServicesNotConfigured,
            ErrorCode::InvalidServiceType => Error::InvalidServiceType,
            ErrorCode::SessionNotFound => Error::SessionNotFound,
            ErrorCode::InvalidSessionId => Error::InvalidSessionId,
            ErrorCode::InvalidQuote => Error::InvalidQuote,
            ErrorCode::StaleQuote => Error::StaleQuote,
            ErrorCode::NoQuotesAvailable => Error::NoQuotesAvailable,
            ErrorCode::InvalidTransactionIntent => Error::InvalidTransactionIntent,
            ErrorCode::ComplianceNotMet => Error::ComplianceNotMet,
            ErrorCode::InvalidConfig => Error::InvalidConfig,
            ErrorCode::DuplicateAttestor => Error::DuplicateAttestor,
            ErrorCode::InvalidCredentialFormat => Error::InvalidCredentialFormat,
            ErrorCode::CredentialNotFound => Error::CredentialNotFound,
            ErrorCode::CredentialExpired => Error::CredentialExpired,
            ErrorCode::InvalidAnchorMetadata => Error::InvalidAnchorMetadata,
            ErrorCode::AnchorMetadataNotFound => Error::AnchorMetadataNotFound,
            ErrorCode::TransportError => Error::TransportError,
            ErrorCode::TransportTimeout => Error::TransportTimeout,
            ErrorCode::TransportUnauthorized => Error::TransportUnauthorized,
            ErrorCode::ProtocolError => Error::ProtocolError,
            ErrorCode::ProtocolInvalidPayload => Error::ProtocolInvalidPayload,
            ErrorCode::ProtocolRateLimitExceeded => Error::ProtocolRateLimitExceeded,
            ErrorCode::CacheExpired => Error::CacheExpired,
            ErrorCode::CacheNotFound => Error::CacheNotFound,
            ErrorCode::RateLimitExceeded => Error::RateLimitExceeded,
            ErrorCode::AssetNotConfigured => Error::AssetNotConfigured,
            ErrorCode::UnsupportedAsset => Error::UnsupportedAsset,
            ErrorCode::WebhookTimestampExpired => Error::WebhookTimestampExpired,
            ErrorCode::WebhookTimestampInFuture => Error::WebhookTimestampInFuture,
            ErrorCode::WebhookPayloadTooLarge => Error::WebhookPayloadTooLarge,
            ErrorCode::WebhookSignatureInvalid => Error::WebhookSignatureInvalid,
            ErrorCode::WebhookValidationFailed => Error::WebhookValidationFailed,
        }
    }
}

/// Error categories for classification
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ErrorCategory {
    /// Transport layer errors (HTTP, network)
    Transport,
    /// Protocol layer errors (Anchor validation)
    Protocol,
    /// Application layer errors (business logic)
    Application,
}

impl ErrorCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorCategory::Transport => "transport",
            ErrorCategory::Protocol => "protocol",
            ErrorCategory::Application => "application",
        }
    }
}

/// Error severity levels
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum ErrorSeverity {
    /// Low severity - transient, likely recoverable
    Low = 1,
    /// Medium severity - requires attention but not critical
    Medium = 2,
    /// High severity - significant issue
    High = 3,
    /// Critical severity - must be addressed immediately
    Critical = 4,
}

impl ErrorSeverity {
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorSeverity::Low => "low",
            ErrorSeverity::Medium => "medium",
            ErrorSeverity::High => "high",
            ErrorSeverity::Critical => "critical",
        }
    }
}

/// Standardized error response format
/// This ensures consistent error responses across all operations
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ErrorResponse {
    /// Standardized error code (e.g., 1001, 2201)
    pub code: u32,
    /// Human-readable error name
    pub name: &'static str,
    /// Error category (transport, protocol, application)
    pub category: &'static str,
    /// Severity level (low, medium, high, critical)
    pub severity: &'static str,
    /// Whether the operation can be retried
    pub retryable: bool,
}

impl ErrorResponse {
    /// Create an ErrorResponse from a base Error
    pub fn from_error(error: &Error) -> Self {
        let error_code = ErrorCode::from_error(error);
        Self {
            code: error_code as u32,
            name: error_code.name(),
            category: error_code.category().as_str(),
            severity: error_code.severity().as_str(),
            retryable: error_code.is_retryable(),
        }
    }

    /// Create an ErrorResponse from an ErrorCode
    pub fn from_code(code: ErrorCode) -> Self {
        Self {
            code: code as u32,
            name: code.name(),
            category: code.category().as_str(),
            severity: code.severity().as_str(),
            retryable: code.is_retryable(),
        }
    }
}

/// AnchorKitError: Main error wrapper with full context
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnchorKitError {
    /// The underlying base error
    pub error: Error,
    /// Standardized error code
    pub code: ErrorCode,
    /// Error response with metadata
    pub response: ErrorResponse,
}

impl AnchorKitError {
    /// Create a new AnchorKitError from a base Error
    pub fn new(error: Error) -> Self {
        let code = ErrorCode::from_error(&error);
        let response = ErrorResponse::from_code(code);
        Self {
            error,
            code,
            response,
        }
    }

    /// Get the error code as u32
    pub fn code_u32(&self) -> u32 {
        self.code as u32
    }

    /// Get the error name
    pub fn name(&self) -> &'static str {
        self.code.name()
    }

    /// Get the error category
    pub fn category(&self) -> ErrorCategory {
        self.code.category()
    }

    /// Get the error severity
    pub fn severity(&self) -> ErrorSeverity {
        self.code.severity()
    }

    /// Check if the error is retryable
    pub fn is_retryable(&self) -> bool {
        self.code.is_retryable()
    }

    /// Check if this is a transport error
    pub fn is_transport_error(&self) -> bool {
        is_transport_error(&self.error)
    }

    /// Check if this is a protocol error
    pub fn is_protocol_error(&self) -> bool {
        is_protocol_error(&self.error)
    }

    /// Get the base error
    pub fn base_error(&self) -> Error {
        self.error
    }
}

/// Conversion from base Error to AnchorKitError
impl From<Error> for AnchorKitError {
    fn from(error: Error) -> Self {
        AnchorKitError::new(error)
    }
}

/// Conversion from AnchorKitError to base Error (for contract compatibility)
impl From<AnchorKitError> for Error {
    fn from(error: AnchorKitError) -> Self {
        error.error
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_code_conversion() {
        let error = Error::NotInitialized;
        let code = ErrorCode::from_error(&error);
        assert_eq!(code, ErrorCode::NotInitialized);
        assert_eq!(code as u32, 1002);
    }

    #[test]
    fn test_error_code_name() {
        let code = ErrorCode::TransportTimeout;
        assert_eq!(code.name(), "TransportTimeout");
    }

    #[test]
    fn test_error_category_classification() {
        let transport_error = ErrorCode::TransportError;
        assert_eq!(transport_error.category(), ErrorCategory::Transport);

        let protocol_error = ErrorCode::ProtocolInvalidPayload;
        assert_eq!(protocol_error.category(), ErrorCategory::Protocol);

        let app_error = ErrorCode::InvalidConfig;
        assert_eq!(app_error.category(), ErrorCategory::Application);
    }

    #[test]
    fn test_error_severity_levels() {
        let critical = ErrorCode::ReplayAttack;
        assert_eq!(critical.severity(), ErrorSeverity::Critical);

        let high = ErrorCode::UnauthorizedAttestor;
        assert_eq!(high.severity(), ErrorSeverity::High);

        let medium = ErrorCode::TransportError;
        assert_eq!(medium.severity(), ErrorSeverity::Medium);

        let low = ErrorCode::TransportTimeout;
        assert_eq!(low.severity(), ErrorSeverity::Low);
    }

    #[test]
    fn test_retryable_classification() {
        // Retryable errors
        assert!(ErrorCode::TransportTimeout.is_retryable());
        assert!(ErrorCode::ProtocolRateLimitExceeded.is_retryable());
        assert!(ErrorCode::StaleQuote.is_retryable());

        // Non-retryable errors
        assert!(!ErrorCode::ReplayAttack.is_retryable());
        assert!(!ErrorCode::ComplianceNotMet.is_retryable());
        assert!(!ErrorCode::UnauthorizedAttestor.is_retryable());
    }

    #[test]
    fn test_error_response_format() {
        let error = Error::InvalidConfig;
        let response = ErrorResponse::from_error(&error);

        assert_eq!(response.code, 1901);
        assert_eq!(response.name, "InvalidConfig");
        assert_eq!(response.category, "application");
        assert_eq!(response.severity, "medium");
        assert!(!response.retryable);
    }

    #[test]
    fn test_anchor_kit_error_wrapper() {
        let error = Error::SessionNotFound;
        let kit_error = AnchorKitError::new(error);

        assert_eq!(kit_error.code_u32(), 1601);
        assert_eq!(kit_error.name(), "SessionNotFound");
        assert_eq!(kit_error.category(), ErrorCategory::Application);
        assert!(kit_error.is_retryable());
    }

    #[test]
    fn test_error_conversion_roundtrip() {
        let original_error = Error::TransportUnauthorized;
        let kit_error = AnchorKitError::from(original_error);
        let converted_back: Error = kit_error.into();

        assert_eq!(converted_back, original_error);
    }

    #[test]
    fn test_all_error_codes_mapped() {
        // Verify all base errors have corresponding error codes
        let errors = [
            Error::AlreadyInitialized,
            Error::NotInitialized,
            Error::UnauthorizedAttestor,
            Error::AttestorAlreadyRegistered,
            Error::AttestorNotRegistered,
            Error::ReplayAttack,
            Error::InvalidTimestamp,
            Error::AttestationNotFound,
            Error::InvalidEndpointFormat,
            Error::EndpointNotFound,
            Error::ServicesNotConfigured,
            Error::InvalidServiceType,
            Error::SessionNotFound,
            Error::InvalidSessionId,
            Error::InvalidQuote,
            Error::StaleQuote,
            Error::NoQuotesAvailable,
            Error::InvalidTransactionIntent,
            Error::ComplianceNotMet,
            Error::InvalidConfig,
            Error::DuplicateAttestor,
            Error::InvalidCredentialFormat,
            Error::CredentialNotFound,
            Error::CredentialExpired,
            Error::InvalidAnchorMetadata,
            Error::AnchorMetadataNotFound,
            Error::TransportError,
            Error::TransportTimeout,
            Error::TransportUnauthorized,
            Error::ProtocolError,
            Error::ProtocolInvalidPayload,
            Error::ProtocolRateLimitExceeded,
            Error::CacheExpired,
            Error::CacheNotFound,
            Error::RateLimitExceeded,
            Error::AssetNotConfigured,
            Error::UnsupportedAsset,
        ];

        for error in &errors {
            let code = ErrorCode::from_error(error);
            let name = code.name();
            assert!(!name.is_empty(), "Error code should have a name");
        }
    }
}
