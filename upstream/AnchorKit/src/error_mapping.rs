use crate::errors::Error;

/// HTTP status code to transport error mapping
pub fn map_http_status_to_error(status_code: u32) -> Error {
    match status_code {
        // Auth errors
        401 | 403 => Error::TransportUnauthorized,

        // Timeout errors
        408 | 504 => Error::TransportTimeout,

        // Rate limiting
        429 => Error::ProtocolRateLimitExceeded,

        // All other 4xx and 5xx errors map to generic TransportError
        _ if status_code >= 400 => Error::TransportError,

        // Success/redirect codes shouldn't be errors
        _ => Error::TransportError,
    }
}

/// Anchor error code to protocol error mapping
pub fn map_anchor_error_to_protocol(anchor_error_code: &str) -> Error {
    match anchor_error_code {
        // Validation errors
        "invalid_payload" | "malformed_request" | "missing_field" | "required_field_missing" => {
            Error::ProtocolInvalidPayload
        }

        // Rate limiting (retryable)
        "rate_limit_exceeded" | "too_many_requests" => Error::ProtocolRateLimitExceeded,

        // Compliance errors (critical)
        "kyc_required" | "kyc_not_verified" | "compliance_violation" | "sanctions_check_failed" => {
            Error::ComplianceNotMet
        }

        // All other anchor errors
        _ => Error::ProtocolError,
    }
}

/// Network error to transport error mapping
pub fn map_network_error_to_transport(error_type: &str) -> Error {
    match error_type {
        "timeout" | "request_timeout" => Error::TransportTimeout,
        _ => Error::TransportError,
    }
}

/// Determine if an error is a transport layer error
pub fn is_transport_error(error: &Error) -> bool {
    matches!(
        error,
        Error::TransportError | Error::TransportTimeout | Error::TransportUnauthorized
    )
}

/// Determine if an error is a protocol layer error
pub fn is_protocol_error(error: &Error) -> bool {
    matches!(
        error,
        Error::ProtocolError
            | Error::ProtocolInvalidPayload
            | Error::ProtocolRateLimitExceeded
            | Error::ComplianceNotMet
    )
}

/// Determine if a transport error is retryable
pub fn is_transport_error_retryable(error: &Error) -> bool {
    matches!(error, Error::TransportTimeout | Error::TransportError)
}

/// Determine if a protocol error is retryable
pub fn is_protocol_error_retryable(error: &Error) -> bool {
    matches!(error, Error::ProtocolRateLimitExceeded)
}

/// Get error category as string for logging
pub fn get_error_category(error: &Error) -> &'static str {
    if is_transport_error(error) {
        "transport"
    } else if is_protocol_error(error) {
        "protocol"
    } else {
        "application"
    }
}

/// Get error severity level (1=low, 2=medium, 3=high, 4=critical)
pub fn get_error_severity(error: &Error) -> u32 {
    match error {
        // Critical errors
        Error::ReplayAttack | Error::ReplayAttack | Error::ComplianceNotMet => 4,

        // High severity
        Error::UnauthorizedAttestor | Error::TransportUnauthorized => 3,

        // Medium severity
        Error::TransportError
        | Error::ProtocolError
        | Error::ProtocolInvalidPayload
        | Error::InvalidConfig => 2,

        // Low severity (transient/recoverable)
        Error::TransportTimeout
        | Error::ProtocolRateLimitExceeded
        | Error::StaleQuote
        | Error::NoQuotesAvailable => 1,

        // Default medium severity
        _ => 2,
    }
}
