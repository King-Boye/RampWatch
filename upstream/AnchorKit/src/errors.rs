//! Error types for AnchorKit
//!
//! All errors are represented as [`AnchorKitError`], a unified base error type
//! carrying a [`code`](AnchorKitError::code), [`message`](AnchorKitError::message),
//! and optional [`context`](AnchorKitError::context).

extern crate alloc;

use alloc::string::String;
use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ErrorCode {
    AlreadyInitialized = 1,
    AttestorAlreadyRegistered = 2,
    AttestorNotRegistered = 3,
    UnauthorizedAttestor = 4,
    InvalidTimestamp = 5,
    ReplayAttack = 6,
    InvalidQuote = 7,
    InvalidServiceType = 8,
    InvalidTransactionIntent = 9,
    StaleQuote = 10,
    ComplianceNotMet = 11,
    InvalidEndpointFormat = 12,
    NoQuotesAvailable = 13,
    ServicesNotConfigured = 14,
    ValidationError = 15,
    RateLimitExceeded = 16,
    AttestationNotFound = 17,
    InvalidSep10Token = 18,
    StorageCorrupted = 19,
    CacheExpired = 48,
    CacheNotFound = 49,
    AuditLogMaxSizeInvalid = 51,
    PendingAdminAlreadyExists = 52,
    NoPendingAdmin = 53,
    NotPendingAdmin = 54,
    SessionNotFound = 55,
    SessionExpired = 56,
    MissingSigningKey = 57,
    InvalidStrategy = 58,
    AttestationLimitReached = 59,
    PathTraversalDetected = 60,
    NotInitialized = 101,
}

impl ErrorCode {
    pub fn default_message(&self) -> &'static str {
        match self {
            ErrorCode::AlreadyInitialized => "Contract is already initialized",
            ErrorCode::AttestorAlreadyRegistered => "Attestor is already registered",
            ErrorCode::AttestorNotRegistered => "Attestor is not registered",
            ErrorCode::UnauthorizedAttestor => "Attestor is not authorized",
            ErrorCode::InvalidTimestamp => "Timestamp is invalid",
            ErrorCode::ReplayAttack => "Replay attack detected",
            ErrorCode::InvalidQuote => "Quote is invalid",
            ErrorCode::InvalidServiceType => "Service type is invalid",
            ErrorCode::InvalidTransactionIntent => "Transaction intent is invalid",
            ErrorCode::StaleQuote => "Quote has expired",
            ErrorCode::ComplianceNotMet => "Compliance requirements not met",
            ErrorCode::InvalidEndpointFormat => "Endpoint format is invalid",
            ErrorCode::NoQuotesAvailable => "No quotes are available",
            ErrorCode::ServicesNotConfigured => "Services are not configured",
            ErrorCode::ValidationError => "Response schema validation failed",
            ErrorCode::RateLimitExceeded => "Rate limit exceeded",
            ErrorCode::AttestationNotFound => "Attestation not found",
            ErrorCode::InvalidSep10Token => "SEP-10 JWT is missing, expired, or invalid",
            ErrorCode::StorageCorrupted => "On-chain storage entry is corrupted or unreadable",
            ErrorCode::CacheExpired => "Cache entry has expired",
            ErrorCode::CacheNotFound => "Cache entry not found",
            ErrorCode::AuditLogMaxSizeInvalid => "max_audit_log_size must be at least 1",
            ErrorCode::PendingAdminAlreadyExists => "An admin transfer is already pending",
            ErrorCode::NoPendingAdmin => "No pending admin transfer found",
            ErrorCode::NotPendingAdmin => "Caller is not the pending admin",
            ErrorCode::SessionNotFound => "Session not found",
            ErrorCode::SessionExpired => "Session has expired",
            ErrorCode::MissingSigningKey => "Anchor TOML does not publish a signing key",
            ErrorCode::InvalidStrategy => "Routing strategy symbol is not recognized",
            ErrorCode::AttestationLimitReached => "Attestation ID counter has reached u64::MAX",
            ErrorCode::PathTraversalDetected => "Path traversal sequence detected in URL",
            ErrorCode::NotInitialized => "Contract is not initialized",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnchorKitError {
    pub code: ErrorCode,
    pub message: String,
    pub context: Option<String>,
}

impl AnchorKitError {
    pub fn new(code: ErrorCode, message: &str) -> Self {
        AnchorKitError {
            code,
            message: String::from(message),
            context: None,
        }
    }

    pub fn with_context(code: ErrorCode, message: &str, context: &str) -> Self {
        AnchorKitError {
            code,
            message: String::from(message),
            context: Some(String::from(context)),
        }
    }

    pub fn from_code(code: ErrorCode) -> Self {
        AnchorKitError::new(code, code.default_message())
    }

    pub fn already_initialized() -> Self { Self::from_code(ErrorCode::AlreadyInitialized) }
    pub fn attestor_already_registered() -> Self { Self::from_code(ErrorCode::AttestorAlreadyRegistered) }
    pub fn attestor_not_registered() -> Self { Self::from_code(ErrorCode::AttestorNotRegistered) }
    pub fn unauthorized_attestor() -> Self { Self::from_code(ErrorCode::UnauthorizedAttestor) }
    pub fn invalid_timestamp() -> Self { Self::from_code(ErrorCode::InvalidTimestamp) }
    pub fn replay_attack() -> Self { Self::from_code(ErrorCode::ReplayAttack) }
    pub fn invalid_quote() -> Self { Self::from_code(ErrorCode::InvalidQuote) }
    pub fn invalid_service_type() -> Self { Self::from_code(ErrorCode::InvalidServiceType) }
    pub fn invalid_transaction_intent() -> Self { Self::from_code(ErrorCode::InvalidTransactionIntent) }
    pub fn stale_quote() -> Self { Self::from_code(ErrorCode::StaleQuote) }
    pub fn compliance_not_met() -> Self { Self::from_code(ErrorCode::ComplianceNotMet) }
    pub fn invalid_endpoint_format() -> Self { Self::from_code(ErrorCode::InvalidEndpointFormat) }
    pub fn no_quotes_available() -> Self { Self::from_code(ErrorCode::NoQuotesAvailable) }
    pub fn services_not_configured() -> Self { Self::from_code(ErrorCode::ServicesNotConfigured) }
    pub fn not_initialized() -> Self { Self::from_code(ErrorCode::NotInitialized) }
    pub fn attestation_not_found() -> Self { Self::from_code(ErrorCode::AttestationNotFound) }
    pub fn invalid_sep10_token() -> Self { Self::from_code(ErrorCode::InvalidSep10Token) }
    pub fn rate_limit_exceeded() -> Self { Self::from_code(ErrorCode::RateLimitExceeded) }
    pub fn storage_corrupted() -> Self { Self::from_code(ErrorCode::StorageCorrupted) }
    pub fn cache_expired() -> Self { Self::from_code(ErrorCode::CacheExpired) }
    pub fn cache_not_found() -> Self { Self::from_code(ErrorCode::CacheNotFound) }
    pub fn audit_log_max_size_invalid() -> Self { Self::from_code(ErrorCode::AuditLogMaxSizeInvalid) }
    pub fn pending_admin_already_exists() -> Self { Self::from_code(ErrorCode::PendingAdminAlreadyExists) }
    pub fn no_pending_admin() -> Self { Self::from_code(ErrorCode::NoPendingAdmin) }
    pub fn not_pending_admin() -> Self { Self::from_code(ErrorCode::NotPendingAdmin) }
    pub fn session_not_found() -> Self { Self::from_code(ErrorCode::SessionNotFound) }
    pub fn session_expired() -> Self { Self::from_code(ErrorCode::SessionExpired) }
    pub fn missing_signing_key() -> Self { Self::from_code(ErrorCode::MissingSigningKey) }
    pub fn invalid_strategy() -> Self { Self::from_code(ErrorCode::InvalidStrategy) }
    pub fn attestation_limit_reached() -> Self { Self::from_code(ErrorCode::AttestationLimitReached) }
    pub fn path_traversal_detected() -> Self { Self::from_code(ErrorCode::PathTraversalDetected) }

    pub fn validation_error(context: &str) -> Self {
        Self::with_context(
            ErrorCode::ValidationError,
            ErrorCode::ValidationError.default_message(),
            context,
        )
    }
}

impl core::fmt::Display for AnchorKitError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.context {
            Some(context) => write!(f, "[E{}] {} ({})", self.code as u32, self.message, context),
            None => write!(f, "[E{}] {}", self.code as u32, self.message),
        }
    }
}

pub type Error = AnchorKitError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_code_sets_message() {
        let err = AnchorKitError::from_code(ErrorCode::AlreadyInitialized);
        assert_eq!(err.code, ErrorCode::AlreadyInitialized);
        assert_eq!(err.message, "Contract is already initialized");
        assert!(err.context.is_none());
    }

    #[test]
    fn test_validation_error_has_context() {
        let err = AnchorKitError::validation_error("missing field: status");
        assert_eq!(err.code, ErrorCode::ValidationError);
        assert_eq!(err.context, Some(String::from("missing field: status")));
    }

    #[test]
    fn test_error_code_default_messages_are_non_empty() {
        let codes = [
            ErrorCode::AlreadyInitialized,
            ErrorCode::PendingAdminAlreadyExists,
            ErrorCode::NoPendingAdmin,
            ErrorCode::NotPendingAdmin,
            ErrorCode::InvalidStrategy,
            ErrorCode::AttestationLimitReached,
            ErrorCode::PathTraversalDetected,
            ErrorCode::SessionNotFound,
            ErrorCode::SessionExpired,
            ErrorCode::MissingSigningKey,
            ErrorCode::NotInitialized,
            ErrorCode::AuditLogMaxSizeInvalid,
        ];
        for code in codes {
            assert!(!code.default_message().is_empty());
        }
    }
}
