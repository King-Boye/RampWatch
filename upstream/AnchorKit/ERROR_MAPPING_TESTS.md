# Error Mapping Tests

## Overview

This document describes the comprehensive test suite for error mapping in AnchorKit. The error mapping system ensures that anchor error responses are correctly mapped to internal error types, distinguishing between transport-layer errors (HTTP/network) and protocol-layer errors (anchor validation).

## Implementation

The error mapping logic is implemented in `src/error_mapping.rs` with tests in `src/error_mapping_tests.rs`.

### Core Components

1. **Transport Errors**: HTTP and network-level errors
   - `TransportError`: Generic transport/network error (4xx/5xx)
   - `TransportTimeout`: Timeout errors (408, 504)
   - `TransportUnauthorized`: Authentication errors (401, 403)

2. **Protocol Errors**: Anchor validation and business logic errors
   - `ProtocolError`: Generic protocol error
   - `ProtocolInvalidPayload`: Invalid/malformed payload
   - `ProtocolRateLimitExceeded`: Rate limiting (retryable)
   - `ProtocolComplianceViolation`: Compliance/KYC errors (critical)

3. **Error Classification Functions**:
   - `map_http_status_to_error()`: Maps HTTP status codes to errors
   - `map_anchor_error_to_protocol()`: Maps anchor error codes to protocol errors
   - `map_network_error_to_transport()`: Maps network errors to transport errors
   - `is_transport_error()`: Checks if error is transport-layer
   - `is_protocol_error()`: Checks if error is protocol-layer
   - `is_transport_error_retryable()`: Determines if transport error is retryable
   - `is_protocol_error_retryable()`: Determines if protocol error is retryable
   - `get_error_category()`: Returns error category ("transport", "protocol", "application")
   - `get_error_severity()`: Returns severity level (1-4)

## Test Coverage

### Test Goal 1: HTTP errors → TransportError

Tests verify that HTTP status codes are correctly mapped to transport errors:

#### Authentication Errors (Non-Retryable)
- **test_http_401_unauthorized**: Maps 401 to `TransportUnauthorized`
  - High severity (3)
  - Not retryable
  
- **test_http_403_forbidden**: Maps 403 to `TransportUnauthorized`
  - Not retryable

#### Timeout Errors (Retryable)
- **test_http_408_timeout**: Maps 408 to `TransportTimeout`
  - Low severity (1)
  - Retryable
  
- **test_http_504_gateway_timeout**: Maps 504 to `TransportTimeout`
  - Retryable

#### Rate Limiting (Retryable)
- **test_http_429_rate_limit**: Maps 429 to `ProtocolRateLimitExceeded`
  - Protocol error (not transport)
  - Retryable

#### Server Errors (Retryable)
- **test_http_500_server_error**: Maps 500 to `TransportError`
  - Retryable
  
- **test_http_4xx_codes_map_to_transport_error**: Maps 400, 404, 405, 410, 422 to `TransportError`
  
- **test_http_5xx_codes_map_to_transport_error**: Maps 502, 503, 505 to `TransportError`

### Test Goal 2: Anchor validation → ProtocolError

Tests verify that anchor error codes are correctly mapped to protocol errors:

#### Validation Errors (Non-Retryable)
- **test_anchor_invalid_payload_error**: Maps "invalid_payload" to `ProtocolInvalidPayload`
  - Not retryable
  
- **test_anchor_malformed_request_error**: Maps "malformed_request" to `ProtocolInvalidPayload`
  
- **test_anchor_missing_field_error**: Maps "missing_field" to `ProtocolInvalidPayload`

#### Rate Limiting (Retryable)
- **test_anchor_rate_limit_exceeded_error**: Maps "rate_limit_exceeded" to `ProtocolRateLimitExceeded`
  - Low severity (1)
  - Retryable
  
- **test_anchor_too_many_requests_error**: Maps "too_many_requests" to `ProtocolRateLimitExceeded`

#### Compliance Errors (Critical, Non-Retryable)
- **test_anchor_kyc_required_error**: Maps "kyc_required" to `ProtocolComplianceViolation`
  - Critical severity (4)
  - Not retryable
  
- **test_anchor_compliance_violation_error**: Maps "compliance_violation" to `ProtocolComplianceViolation`
  
- **test_anchor_sanctions_check_failed_error**: Maps "sanctions_check_failed" to `ProtocolComplianceViolation`

#### Unknown Errors
- **test_anchor_unknown_error_code**: Maps unknown codes to `ProtocolError`

### Network Error Mapping Tests

Tests verify that network-level errors are correctly mapped:

- **test_network_timeout_error**: Maps "timeout" to `TransportTimeout`
  - Retryable
  
- **test_network_connection_failed_error**: Maps "connection_failed" to `TransportError`
  - Retryable
  
- **test_network_unknown_error**: Maps unknown network errors to `TransportError`

### Error Classification Tests

Tests verify that errors are correctly classified by category:

- **test_all_transport_errors_classified_correctly**: All transport errors return "transport" category
  
- **test_all_protocol_errors_classified_correctly**: All protocol errors return "protocol" category
  
- **test_retryable_transport_errors**: `TransportTimeout` and `TransportError` are retryable
  
- **test_non_retryable_transport_errors**: `TransportUnauthorized` is not retryable
  
- **test_retryable_protocol_errors**: `ProtocolRateLimitExceeded` is retryable
  
- **test_non_retryable_protocol_errors**: Other protocol errors are not retryable

### Error Severity Tests

Tests verify that errors are assigned correct severity levels:

- **test_critical_severity_errors** (Level 4):
  - `ReplayAttack`
  - `SessionReplayAttack`
  - `ProtocolComplianceViolation`

- **test_high_severity_errors** (Level 3):
  - `UnauthorizedAttestor`
  - `TransportUnauthorized`

- **test_medium_severity_errors** (Level 2):
  - `TransportError`
  - `ProtocolInvalidPayload`
  - `InvalidConfig`

- **test_low_severity_errors** (Level 1):
  - `TransportTimeout`
  - `ProtocolRateLimitExceeded`
  - `StaleQuote`

### Integration Tests - Realistic Scenarios

Tests verify error mapping in realistic scenarios:

- **test_scenario_anchor_returns_400_with_invalid_payload**:
  - HTTP 400 → `TransportError`
  - Anchor "invalid_payload" → `ProtocolInvalidPayload`
  - Both correctly categorized

- **test_scenario_network_timeout_during_quote_request**:
  - Network timeout → `TransportTimeout`
  - Retryable, low severity
  - Transport category

- **test_scenario_anchor_rate_limits_client**:
  - HTTP 429 → `ProtocolRateLimitExceeded`
  - Anchor "rate_limit_exceeded" → `ProtocolRateLimitExceeded`
  - Both retryable

- **test_scenario_anchor_compliance_failure**:
  - Anchor "sanctions_check_failed" → `ProtocolComplianceViolation`
  - Not retryable, critical severity

- **test_scenario_server_temporarily_unavailable**:
  - HTTP 503 → `TransportError`
  - Retryable, transport category

- **test_http_to_transport_error_mapping_comprehensive**:
  - Comprehensive HTTP status code mapping verification

- **test_anchor_to_protocol_error_mapping_comprehensive**:
  - Comprehensive anchor error code mapping verification

## Error Mapping Tables

### HTTP Status Code Mapping

| HTTP Code | Error Type | Retryable | Severity |
|-----------|------------|-----------|----------|
| 401, 403 | TransportUnauthorized | No | High (3) |
| 408, 504 | TransportTimeout | Yes | Low (1) |
| 429 | ProtocolRateLimitExceeded | Yes | Low (1) |
| 400, 404, 405, 410, 422 | TransportError | Yes | Medium (2) |
| 500, 502, 503, 505 | TransportError | Yes | Medium (2) |

### Anchor Error Code Mapping

| Anchor Error Code | Error Type | Retryable | Severity |
|-------------------|------------|-----------|----------|
| invalid_payload, malformed_request, missing_field | ProtocolInvalidPayload | No | Medium (2) |
| rate_limit_exceeded, too_many_requests | ProtocolRateLimitExceeded | Yes | Low (1) |
| kyc_required, compliance_violation, sanctions_check_failed | ProtocolComplianceViolation | No | Critical (4) |
| (unknown) | ProtocolError | No | Medium (2) |

### Network Error Mapping

| Network Error | Error Type | Retryable | Severity |
|---------------|------------|-----------|----------|
| timeout, request_timeout | TransportTimeout | Yes | Low (1) |
| connection_failed, dns_error, ssl_error | TransportError | Yes | Medium (2) |
| (unknown) | TransportError | Yes | Medium (2) |

## Test Results

All 37 error mapping tests pass successfully:

```
test error_mapping_tests::test_all_protocol_errors_classified_correctly ... ok
test error_mapping_tests::test_all_transport_errors_classified_correctly ... ok
test error_mapping_tests::test_anchor_compliance_violation_error ... ok
test error_mapping_tests::test_anchor_invalid_payload_error ... ok
test error_mapping_tests::test_anchor_kyc_required_error ... ok
test error_mapping_tests::test_anchor_malformed_request_error ... ok
test error_mapping_tests::test_anchor_missing_field_error ... ok
test error_mapping_tests::test_anchor_rate_limit_exceeded_error ... ok
test error_mapping_tests::test_anchor_sanctions_check_failed_error ... ok
test error_mapping_tests::test_anchor_to_protocol_error_mapping_comprehensive ... ok
test error_mapping_tests::test_anchor_too_many_requests_error ... ok
test error_mapping_tests::test_anchor_unknown_error_code ... ok
test error_mapping_tests::test_critical_severity_errors ... ok
test error_mapping_tests::test_high_severity_errors ... ok
test error_mapping_tests::test_http_401_unauthorized ... ok
test error_mapping_tests::test_http_403_forbidden ... ok
test error_mapping_tests::test_http_408_timeout ... ok
test error_mapping_tests::test_http_429_rate_limit ... ok
test error_mapping_tests::test_http_4xx_codes_map_to_transport_error ... ok
test error_mapping_tests::test_http_500_server_error ... ok
test error_mapping_tests::test_http_504_gateway_timeout ... ok
test error_mapping_tests::test_http_5xx_codes_map_to_transport_error ... ok
test error_mapping_tests::test_http_to_transport_error_mapping_comprehensive ... ok
test error_mapping_tests::test_low_severity_errors ... ok
test error_mapping_tests::test_medium_severity_errors ... ok
test error_mapping_tests::test_network_connection_failed_error ... ok
test error_mapping_tests::test_network_timeout_error ... ok
test error_mapping_tests::test_network_unknown_error ... ok
test error_mapping_tests::test_non_retryable_protocol_errors ... ok
test error_mapping_tests::test_non_retryable_transport_errors ... ok
test error_mapping_tests::test_retryable_protocol_errors ... ok
test error_mapping_tests::test_retryable_transport_errors ... ok
test error_mapping_tests::test_scenario_anchor_compliance_failure ... ok
test error_mapping_tests::test_scenario_anchor_rate_limits_client ... ok
test error_mapping_tests::test_scenario_anchor_returns_400_with_invalid_payload ... ok
test error_mapping_tests::test_scenario_network_timeout_during_quote_request ... ok
test error_mapping_tests::test_scenario_server_temporarily_unavailable ... ok
```

## Usage Example

```rust
use crate::error_mapping::{
    map_http_status_to_error,
    map_anchor_error_to_protocol,
    is_transport_error_retryable,
    get_error_severity,
};

// Map HTTP status code
let error = map_http_status_to_error(503);
assert_eq!(error, Error::TransportError);
assert!(is_transport_error_retryable(&error));

// Map anchor error code
let error = map_anchor_error_to_protocol("rate_limit_exceeded");
assert_eq!(error, Error::ProtocolRateLimitExceeded);
assert_eq!(get_error_severity(&error), 1); // Low severity

// Handle compliance violation
let error = map_anchor_error_to_protocol("sanctions_check_failed");
assert_eq!(error, Error::ProtocolComplianceViolation);
assert_eq!(get_error_severity(&error), 4); // Critical
```

## Integration

The error mapping system integrates with the AnchorKit contract to provide:

- Clear separation between transport and protocol errors
- Intelligent retry logic based on error type
- Severity-based error handling and logging
- Consistent error categorization across the system

## Benefits

1. **Clear Error Boundaries**: Distinguishes between network issues and business logic errors
2. **Intelligent Retries**: Only retries errors that are likely to succeed on retry
3. **Severity Awareness**: Enables appropriate alerting and logging based on error severity
4. **Extensibility**: Easy to add new error mappings as anchor protocols evolve
5. **Testability**: Comprehensive test coverage ensures correct error handling

## Conclusion

The error mapping test suite provides comprehensive coverage of HTTP-to-transport and anchor-to-protocol error mapping. All 37 tests pass, validating that the error mapping system correctly categorizes errors, determines retryability, and assigns appropriate severity levels.

Total test count: 130 tests passing (11 config + 12 capability + 13 transport + 16 serialization + 18 retry internal + 23 retry + 37 error mapping)
