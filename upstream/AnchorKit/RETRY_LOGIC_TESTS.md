# Retry Logic Behavior Tests

## Overview

This document describes the comprehensive test suite for the retry logic behavior in AnchorKit. The retry engine implements exponential backoff with configurable parameters and intelligent error classification to determine which errors should be retried.

## Implementation

The retry logic is implemented in `src/retry.rs` with tests in `src/retry_tests.rs`.

### Core Components

1. **RetryConfig**: Configuration for retry behavior
   - `max_attempts`: Maximum number of retry attempts
   - `initial_delay_ms`: Initial delay in milliseconds
   - `max_delay_ms`: Maximum delay cap in milliseconds
   - `backoff_multiplier`: Exponential growth multiplier

2. **RetryEngine**: Executes operations with retry logic
   - Applies exponential backoff between attempts
   - Classifies errors as retryable or non-retryable
   - Tracks total delay and attempt count

3. **RetryResult**: Result wrapper with retry metadata
   - Contains success value or error
   - Tracks number of attempts made
   - Records total delay accumulated

## Test Coverage

### Test Goal 1: Exponential Retry Timing

Tests verify that delays grow exponentially according to the configured multiplier:

- **test_exponential_backoff_timing**: Validates basic exponential growth with multiplier 2
  - Delays: 0ms, 100ms, 200ms, 400ms, 800ms

- **test_exponential_backoff_with_multiplier_3**: Tests with multiplier 3
  - Delays: 0ms, 50ms, 150ms, 450ms, 1350ms

- **test_exponential_backoff_respects_max_delay**: Ensures delays cap at max_delay_ms
  - Verifies delays never exceed the configured maximum

- **test_total_delay_accumulation**: Confirms total delay is correctly summed
  - Validates the cumulative delay tracking across all attempts

- **test_delay_calculation_with_zero_initial_delay**: Edge case with 0ms initial delay
  - All delays should remain 0

- **test_delay_calculation_with_multiplier_1**: No exponential growth (constant delay)
  - Delays stay constant when multiplier is 1

- **test_delay_calculation_large_multiplier**: Tests with multiplier 10
  - Verifies rapid exponential growth and max cap

### Test Goal 2: Stops After Max Attempts

Tests verify that retry logic respects the maximum attempt limit:

- **test_stops_after_max_attempts**: Basic test with 3 max attempts
  - Confirms exactly 3 attempts are made with retryable errors

- **test_stops_after_max_attempts_with_different_configs**: Tests multiple configurations
  - max_attempts = 1: Single attempt only
  - max_attempts = 5: Exactly 5 attempts
  - max_attempts = 10: Exactly 10 attempts

- **test_succeeds_before_max_attempts**: Success on 3rd attempt
  - Verifies early termination on success

- **test_zero_max_attempts**: Edge case with 0 max attempts
  - Should not execute at all

### Test Goal 3: Does Not Retry Non-Recoverable Errors

Tests verify that non-retryable errors fail immediately without retries:

- **test_does_not_retry_invalid_config**: Configuration errors are not retryable
  - Only 1 attempt made

- **test_does_not_retry_unauthorized_attestor**: Authorization errors are not retryable
  - Only 1 attempt made

- **test_does_not_retry_replay_attack**: Security errors are not retryable
  - Only 1 attempt made

- **test_does_not_retry_compliance_not_met**: Compliance errors are not retryable
  - Only 1 attempt made

- **test_retryable_vs_non_retryable_classification**: Comprehensive error classification test
  - Validates all error types are correctly classified

### Additional Test Scenarios

- **test_first_attempt_success**: No retries needed when first attempt succeeds
  - Verifies 0ms total delay on immediate success

- **test_stops_on_non_retryable_after_retries**: Mixed error scenario
  - Retries on retryable errors, stops immediately on non-retryable

- **test_all_attempts_fail_retryable**: All attempts exhausted with retryable errors
  - Returns the last error encountered

- **test_retry_engine_default_config**: Validates default configuration values
  - max_attempts: 3
  - initial_delay_ms: 100
  - max_delay_ms: 5000
  - backoff_multiplier: 2

- **test_attempt_parameter_increments**: Verifies attempt counter increments correctly
  - Attempts are 0-indexed: 0, 1, 2, 3, 4

- **test_complex_retry_scenario**: Multi-error scenario with eventual success
  - Tests realistic retry flow with different error types

- **test_delay_accumulation_correctness**: Validates delay calculation accuracy
  - Confirms delays sum correctly: 0 + 100 + 200 + 400 = 700ms

## Error Classification

### Retryable Errors (Transient/Temporary)

These errors indicate temporary conditions that may resolve on retry:

- `EndpointNotFound`: Endpoint may become available
- `ServicesNotConfigured`: Configuration may be updated
- `AttestationNotFound`: Data may become available
- `QuoteNotFound`: Quote may be submitted
- `SessionNotFound`: Session may be created
- `StaleQuote`: Fresh quote may be available
- `NoQuotesAvailable`: Quotes may become available
- `NoAnchorsAvailable`: Anchors may come online

### Non-Retryable Errors (Permanent/Logic Errors)

These errors indicate permanent conditions that won't change on retry:

- `InvalidConfig`: Configuration is malformed
- `InvalidEndpointFormat`: URL format is invalid
- `UnauthorizedAttestor`: Authorization will not change
- `AttestorNotRegistered`: Registration required
- `AttestorAlreadyRegistered`: Already registered
- `ReplayAttack`: Security violation
- `SessionReplayAttack`: Security violation
- `InvalidQuote`: Quote data is malformed
- `InvalidTimestamp`: Timestamp is invalid
- `ComplianceNotMet`: Compliance requirements not satisfied
- `CredentialExpired`: Credential needs renewal
- `AlreadyInitialized`: Contract already initialized

## Test Results

All 23 retry logic tests pass successfully:

```
test retry_tests::test_all_attempts_fail_retryable ... ok
test retry_tests::test_attempt_parameter_increments ... ok
test retry_tests::test_complex_retry_scenario ... ok
test retry_tests::test_delay_accumulation_correctness ... ok
test retry_tests::test_delay_calculation_large_multiplier ... ok
test retry_tests::test_delay_calculation_with_multiplier_1 ... ok
test retry_tests::test_delay_calculation_with_zero_initial_delay ... ok
test retry_tests::test_does_not_retry_compliance_not_met ... ok
test retry_tests::test_does_not_retry_invalid_config ... ok
test retry_tests::test_does_not_retry_replay_attack ... ok
test retry_tests::test_does_not_retry_unauthorized_attestor ... ok
test retry_tests::test_exponential_backoff_respects_max_delay ... ok
test retry_tests::test_exponential_backoff_timing ... ok
test retry_tests::test_exponential_backoff_with_multiplier_3 ... ok
test retry_tests::test_first_attempt_success ... ok
test retry_tests::test_retry_engine_default_config ... ok
test retry_tests::test_retryable_vs_non_retryable_classification ... ok
test retry_tests::test_stops_after_max_attempts ... ok
test retry_tests::test_stops_after_max_attempts_with_different_configs ... ok
test retry_tests::test_stops_on_non_retryable_after_retries ... ok
test retry_tests::test_succeeds_before_max_attempts ... ok
test retry_tests::test_total_delay_accumulation ... ok
test retry_tests::test_zero_max_attempts ... ok
```

## Usage Example

```rust
use crate::retry::{RetryConfig, RetryEngine};

// Create a retry configuration
let config = RetryConfig::new(
    5,      // max_attempts
    100,    // initial_delay_ms
    5000,   // max_delay_ms
    2       // backoff_multiplier
);

// Create retry engine
let engine = RetryEngine::new(config);

// Execute operation with retry logic
let result = engine.execute(|attempt| {
    // Your operation here
    // Return Ok(value) on success
    // Return Err(error) on failure
    perform_operation(attempt)
});

// Check result
if result.is_success() {
    println!("Success after {} attempts", result.attempts);
    println!("Total delay: {}ms", result.total_delay_ms);
} else {
    println!("Failed after {} attempts", result.attempts);
    println!("Error: {:?}", result.error);
}
```

## Integration

The retry logic integrates with the AnchorKit contract to provide resilient operations:

- Anchor endpoint calls can be retried on transient failures
- Quote fetching can retry when quotes are temporarily unavailable
- Service discovery can retry when services are being configured

## Future Enhancements

Potential improvements for the retry logic:

1. **Jitter**: Add random jitter to prevent thundering herd
2. **Circuit Breaker**: Stop retrying after sustained failures
3. **Adaptive Backoff**: Adjust multiplier based on success rate
4. **Per-Error Configuration**: Different retry policies per error type
5. **Async Support**: Add async/await support for real delays

## Conclusion

The retry logic test suite provides comprehensive coverage of exponential backoff behavior, attempt limiting, and error classification. All 23 tests pass, validating that the retry engine correctly handles transient failures while avoiding unnecessary retries for permanent errors.

Total test count: 93 tests passing (11 config + 12 capability + 13 transport + 16 serialization + 18 retry internal + 23 retry tests)
