#![cfg(test)]

extern crate alloc;

use crate::{
    error_mapping::{
        get_error_category, get_error_severity, is_protocol_error, is_protocol_error_retryable,
        is_transport_error, is_transport_error_retryable, map_anchor_error_to_protocol,
        map_http_status_to_error, map_network_error_to_transport,
    },
    errors::Error,
};

/// Test Goal 1: HTTP errors → TransportError

#[test]
fn test_http_401_unauthorized() {
    let error = map_http_status_to_error(401);
    assert_eq!(error, Error::TransportUnauthorized);
    assert!(is_transport_error(&error));
    assert!(!is_transport_error_retryable(&error));
    assert_eq!(get_error_severity(&error), 3); // High severity
}

#[test]
fn test_http_403_forbidden() {
    let error = map_http_status_to_error(403);
    assert_eq!(error, Error::TransportUnauthorized);
    assert!(is_transport_error(&error));
}

#[test]
fn test_http_408_timeout() {
    let error = map_http_status_to_error(408);
    assert_eq!(error, Error::TransportTimeout);
    assert!(is_transport_error(&error));
    assert!(is_transport_error_retryable(&error));
    assert_eq!(get_error_severity(&error), 1); // Low severity (retryable)
}

#[test]
fn test_http_429_rate_limit() {
    let error = map_http_status_to_error(429);
    assert_eq!(error, Error::ProtocolRateLimitExceeded);
    assert!(is_protocol_error(&error));
    assert!(is_protocol_error_retryable(&error));
}

#[test]
fn test_http_500_server_error() {
    let error = map_http_status_to_error(500);
    assert_eq!(error, Error::TransportError);
    assert!(is_transport_error(&error));
    assert!(is_transport_error_retryable(&error));
}

#[test]
fn test_http_504_gateway_timeout() {
    let error = map_http_status_to_error(504);
    assert_eq!(error, Error::TransportTimeout);
    assert!(is_transport_error(&error));
    assert!(is_transport_error_retryable(&error));
}

#[test]
fn test_http_4xx_codes_map_to_transport_error() {
    // Test various 4xx codes
    for code in [400, 404, 405, 410, 422] {
        let error = map_http_status_to_error(code);
        assert_eq!(error, Error::TransportError);
        assert!(is_transport_error(&error));
    }
}

#[test]
fn test_http_5xx_codes_map_to_transport_error() {
    // Test various 5xx codes
    for code in [502, 503, 505] {
        let error = map_http_status_to_error(code);
        assert_eq!(error, Error::TransportError);
        assert!(is_transport_error(&error));
    }
}

/// Test Goal 2: Anchor validation → ProtocolError

#[test]
fn test_anchor_invalid_payload_error() {
    let error = map_anchor_error_to_protocol("invalid_payload");
    assert_eq!(error, Error::ProtocolInvalidPayload);
    assert!(is_protocol_error(&error));
    assert!(!is_protocol_error_retryable(&error));
}

#[test]
fn test_anchor_malformed_request_error() {
    let error = map_anchor_error_to_protocol("malformed_request");
    assert_eq!(error, Error::ProtocolInvalidPayload);
    assert!(is_protocol_error(&error));
}

#[test]
fn test_anchor_missing_field_error() {
    let error = map_anchor_error_to_protocol("missing_field");
    assert_eq!(error, Error::ProtocolInvalidPayload);
    assert!(is_protocol_error(&error));
}

#[test]
fn test_anchor_rate_limit_exceeded_error() {
    let error = map_anchor_error_to_protocol("rate_limit_exceeded");
    assert_eq!(error, Error::ProtocolRateLimitExceeded);
    assert!(is_protocol_error(&error));
    assert!(is_protocol_error_retryable(&error));
    assert_eq!(get_error_severity(&error), 1); // Low severity (retryable)
}

#[test]
fn test_anchor_too_many_requests_error() {
    let error = map_anchor_error_to_protocol("too_many_requests");
    assert_eq!(error, Error::ProtocolRateLimitExceeded);
    assert!(is_protocol_error(&error));
}

#[test]
fn test_anchor_kyc_required_error() {
    let error = map_anchor_error_to_protocol("kyc_required");
    assert_eq!(error, Error::ComplianceNotMet);
    assert!(is_protocol_error(&error));
    assert!(!is_protocol_error_retryable(&error));
}

#[test]
fn test_anchor_compliance_violation_error() {
    let error = map_anchor_error_to_protocol("compliance_violation");
    assert_eq!(error, Error::ComplianceNotMet);
    assert!(is_protocol_error(&error));
    assert_eq!(get_error_severity(&error), 4); // Critical severity
}

#[test]
fn test_anchor_sanctions_check_failed_error() {
    let error = map_anchor_error_to_protocol("sanctions_check_failed");
    assert_eq!(error, Error::ComplianceNotMet);
    assert!(is_protocol_error(&error));
}

#[test]
fn test_anchor_unknown_error_code() {
    let error = map_anchor_error_to_protocol("completely_unknown_error");
    assert_eq!(error, Error::ProtocolError);
    assert!(is_protocol_error(&error));
}

/// Network error mapping tests

#[test]
fn test_network_timeout_error() {
    let error = map_network_error_to_transport("timeout");
    assert_eq!(error, Error::TransportTimeout);
    assert!(is_transport_error(&error));
    assert!(is_transport_error_retryable(&error));
}

#[test]
fn test_network_connection_failed_error() {
    let error = map_network_error_to_transport("connection_failed");
    assert_eq!(error, Error::TransportError);
    assert!(is_transport_error(&error));
}

#[test]
fn test_network_unknown_error() {
    let error = map_network_error_to_transport("unknown_network_issue");
    assert_eq!(error, Error::TransportError);
    assert!(is_transport_error(&error));
}

/// Error classification tests

#[test]
fn test_all_transport_errors_classified_correctly() {
    let transport_errors = alloc::vec![
        Error::TransportTimeout,
        Error::TransportError,
        Error::TransportUnauthorized,
    ];

    for error in transport_errors {
        assert!(is_transport_error(&error));
        assert!(!is_protocol_error(&error));
        assert_eq!(get_error_category(&error), "transport");
    }
}

#[test]
fn test_all_protocol_errors_classified_correctly() {
    let protocol_errors = alloc::vec![
        Error::ProtocolError,
        Error::ProtocolInvalidPayload,
        Error::ProtocolRateLimitExceeded,
        Error::ComplianceNotMet,
    ];

    for error in protocol_errors {
        assert!(is_protocol_error(&error));
        assert!(!is_transport_error(&error));
        assert_eq!(get_error_category(&error), "protocol");
    }
}

#[test]
fn test_retryable_transport_errors() {
    assert!(is_transport_error_retryable(&Error::TransportTimeout));
    assert!(is_transport_error_retryable(&Error::TransportError));
}

#[test]
fn test_non_retryable_transport_errors() {
    assert!(!is_transport_error_retryable(&Error::TransportUnauthorized));
}

#[test]
fn test_retryable_protocol_errors() {
    assert!(is_protocol_error_retryable(
        &Error::ProtocolRateLimitExceeded
    ));
}

#[test]
fn test_non_retryable_protocol_errors() {
    assert!(!is_protocol_error_retryable(&Error::ProtocolInvalidPayload));
    assert!(!is_protocol_error_retryable(&Error::ProtocolError));
    assert!(!is_protocol_error_retryable(
        &Error::ComplianceNotMet
    ));
}

/// Error severity tests

#[test]
fn test_critical_severity_errors() {
    assert_eq!(get_error_severity(&Error::ReplayAttack), 4);
    assert_eq!(get_error_severity(&Error::ReplayAttack), 4);
    assert_eq!(get_error_severity(&Error::ComplianceNotMet), 4);
}

#[test]
fn test_high_severity_errors() {
    assert_eq!(get_error_severity(&Error::UnauthorizedAttestor), 3);
    assert_eq!(get_error_severity(&Error::TransportUnauthorized), 3);
}

#[test]
fn test_medium_severity_errors() {
    assert_eq!(get_error_severity(&Error::TransportError), 2);
    assert_eq!(get_error_severity(&Error::ProtocolInvalidPayload), 2);
    assert_eq!(get_error_severity(&Error::InvalidConfig), 2);
}

#[test]
fn test_low_severity_errors() {
    assert_eq!(get_error_severity(&Error::TransportTimeout), 1);
    assert_eq!(get_error_severity(&Error::ProtocolRateLimitExceeded), 1);
    assert_eq!(get_error_severity(&Error::StaleQuote), 1);
}

/// Integration tests - realistic error mapping scenarios

#[test]
fn test_scenario_anchor_returns_400_with_invalid_payload() {
    // HTTP 400 maps to transport error
    let http_error = map_http_status_to_error(400);
    assert_eq!(http_error, Error::TransportError);

    // Anchor error code maps to protocol error
    let protocol_error = map_anchor_error_to_protocol("invalid_payload");
    assert_eq!(protocol_error, Error::ProtocolInvalidPayload);

    // Both are correctly categorized
    assert!(is_transport_error(&http_error));
    assert!(is_protocol_error(&protocol_error));
}

#[test]
fn test_scenario_network_timeout_during_quote_request() {
    let error = map_network_error_to_transport("timeout");
    assert_eq!(error, Error::TransportTimeout);
    assert!(is_transport_error_retryable(&error));
    assert_eq!(get_error_severity(&error), 1);
    assert_eq!(get_error_category(&error), "transport");
}

#[test]
fn test_scenario_anchor_rate_limits_client() {
    // HTTP 429 maps to protocol rate limit
    let http_error = map_http_status_to_error(429);
    assert_eq!(http_error, Error::ProtocolRateLimitExceeded);

    // Anchor error also maps to rate limit
    let anchor_error = map_anchor_error_to_protocol("rate_limit_exceeded");
    assert_eq!(anchor_error, Error::ProtocolRateLimitExceeded);

    // Both should be retryable
    assert!(is_protocol_error_retryable(&http_error));
    assert!(is_protocol_error_retryable(&anchor_error));
}

#[test]
fn test_scenario_anchor_compliance_failure() {
    let error = map_anchor_error_to_protocol("sanctions_check_failed");
    assert_eq!(error, Error::ComplianceNotMet);
    assert!(!is_protocol_error_retryable(&error));
    assert_eq!(get_error_severity(&error), 4); // Critical
}

#[test]
fn test_scenario_server_temporarily_unavailable() {
    let error = map_http_status_to_error(503);
    assert_eq!(error, Error::TransportError);
    assert!(is_transport_error_retryable(&error));
    assert_eq!(get_error_category(&error), "transport");
}

#[test]
fn test_http_to_transport_error_mapping_comprehensive() {
    // Test comprehensive HTTP status code mapping
    assert_eq!(map_http_status_to_error(400), Error::TransportError);
    assert_eq!(map_http_status_to_error(401), Error::TransportUnauthorized);
    assert_eq!(map_http_status_to_error(408), Error::TransportTimeout);
    assert_eq!(
        map_http_status_to_error(429),
        Error::ProtocolRateLimitExceeded
    );
    assert_eq!(map_http_status_to_error(500), Error::TransportError);
    assert_eq!(map_http_status_to_error(503), Error::TransportError);
    assert_eq!(map_http_status_to_error(504), Error::TransportTimeout);
}

#[test]
fn test_anchor_to_protocol_error_mapping_comprehensive() {
    // Test comprehensive anchor error code mapping
    assert_eq!(
        map_anchor_error_to_protocol("invalid_payload"),
        Error::ProtocolInvalidPayload
    );
    assert_eq!(
        map_anchor_error_to_protocol("rate_limit_exceeded"),
        Error::ProtocolRateLimitExceeded
    );
    assert_eq!(
        map_anchor_error_to_protocol("kyc_required"),
        Error::ComplianceNotMet
    );
    assert_eq!(
        map_anchor_error_to_protocol("unknown_error"),
        Error::ProtocolError
    );
}
