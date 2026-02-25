use crate::errors::Error;

/// HTTP status code to transport error mapping
#[allow(dead_code)]
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
#[allow(dead_code)]
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
#[allow(dead_code)]
pub fn map_network_error_to_transport(error_type: &str) -> Error {
    match error_type {
        "timeout" | "request_timeout" => Error::TransportTimeout,
        _ => Error::TransportError,
    }
}

/// Determine if an error is a transport layer error
#[allow(dead_code)]
pub fn is_transport_error(error: &Error) -> bool {
    matches!(
        error,
        Error::TransportError | Error::TransportTimeout | Error::TransportUnauthorized
    )
}

/// Determine if an error is a protocol layer error
#[allow(dead_code)]
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
#[allow(dead_code)]
pub fn is_transport_error_retryable(error: &Error) -> bool {
    matches!(error, Error::TransportTimeout | Error::TransportError)
}

/// Determine if a protocol error is retryable
#[allow(dead_code)]
pub fn is_protocol_error_retryable(error: &Error) -> bool {
    matches!(error, Error::ProtocolRateLimitExceeded)
}

/// Get error category as string for logging
#[allow(dead_code)]
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
#[allow(dead_code)]
pub fn get_error_severity(error: &Error) -> u32 {
    match error {
        // Critical errors
        Error::ReplayAttack | Error::ComplianceNotMet => 4,

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

// ========== Rate Limit Detection Functions ==========

use crate::rate_limit_response::RateLimitInfo;

/// Check if HTTP status code indicates rate limiting (429)
#[allow(dead_code)]
pub fn is_rate_limit_status(status_code: u32) -> bool {
    status_code == 429
}

/// Check if HTTP status code indicates a server error (5xx)
#[allow(dead_code)]
pub fn is_server_error(status_code: u32) -> bool {
    status_code >= 500 && status_code < 600
}

/// Check if HTTP status code indicates a client error (4xx, except 429)
#[allow(dead_code)]
pub fn is_client_error(status_code: u32) -> bool {
    status_code >= 400 && status_code < 500 && status_code != 429
}

/// Check if HTTP status code indicates the request can be retried
/// This includes rate limits (429), server errors (5xx), and timeout (408)
#[allow(dead_code)]
pub fn is_retryable_status(status_code: u32) -> bool {
    is_rate_limit_status(status_code) || is_server_error(status_code) || status_code == 408
}

/// Extract rate limit information from HTTP response headers
/// Returns None if no rate limit headers are present
#[allow(dead_code)]
pub fn extract_rate_limit_info(
    _env: &soroban_sdk::Env,
    status_code: u32,
    retry_after_header: Option<u64>,
    rate_limit_remaining: Option<u32>,
    rate_limit_reset: Option<u64>,
    rate_limit_limit: Option<u32>,
    rate_limit_window: Option<u32>,
) -> Option<RateLimitInfo> {
    // If it's a 429, create rate limit info
    if is_rate_limit_status(status_code) {
        return Some(RateLimitInfo::from_headers(
            _env,
            retry_after_header,
            rate_limit_remaining,
            rate_limit_reset,
            rate_limit_limit,
            rate_limit_window,
        ));
    }

    // If we have rate limit headers but not a 429, still track them
    if rate_limit_remaining.is_some() || rate_limit_reset.is_some() {
        return Some(RateLimitInfo::from_headers(
            _env,
            retry_after_header,
            rate_limit_remaining,
            rate_limit_reset,
            rate_limit_limit,
            rate_limit_window,
        ));
    }

    None
}

/// Get recommended retry delay based on HTTP response
#[allow(dead_code)]
pub fn get_retry_delay_from_response(
    status_code: u32,
    rate_limit_info: Option<&RateLimitInfo>,
) -> u64 {
    // If rate limited, use Retry-After if available
    if is_rate_limit_status(status_code) {
        if let Some(info) = rate_limit_info {
            if info.retry_after_ms > 0 {
                return info.retry_after_ms;
            }
        }
        // Default delay for rate limiting
        return 1000;
    }

    // For server errors, use exponential backoff
    if is_server_error(status_code) {
        return 500;
    }

    // For timeouts
    if status_code == 408 {
        return 200;
    }

    0
}
