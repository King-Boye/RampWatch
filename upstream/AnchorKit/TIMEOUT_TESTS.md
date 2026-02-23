# Timeout Handling Tests

## Overview
Tests ensuring slow anchors trigger proper timeout behavior with exponential backoff and retry logic.

## Test Implementation

### File: `src/timeout_tests.rs`

### Test Cases

#### 1. `test_timeout_enforced_on_slow_anchor`
**Purpose**: Verify timeout is enforced when anchor is slow/unresponsive.

**Setup**:
- 3 max attempts
- Simulates continuous timeout errors

**Assertions**:
- ✅ All 3 retry attempts executed
- ✅ Final result is failure
- ✅ Attempt counter matches expected

#### 2. `test_retries_triggered_correctly`
**Purpose**: Verify retries trigger correctly and succeed eventually.

**Setup**:
- Fails on first 2 attempts
- Succeeds on 3rd attempt

**Assertions**:
- ✅ 3 attempts made
- ✅ Final result is success
- ✅ Correct value returned

#### 3. `test_timeout_with_exponential_backoff`
**Purpose**: Validate exponential backoff delay calculation.

**Setup**:
- 4 attempts with 2x multiplier
- Initial delay: 100ms

**Assertions**:
- ✅ Total delay: 700ms (0 + 100 + 200 + 400)
- ✅ Exponential growth applied
- ✅ All attempts exhausted

#### 4. `test_non_retryable_error_stops_immediately`
**Purpose**: Non-retryable errors stop retry loop immediately.

**Setup**:
- InvalidConfig error (non-retryable)
- 5 max attempts configured

**Assertions**:
- ✅ Only 1 attempt made
- ✅ No retries triggered
- ✅ Immediate failure

#### 5. `test_success_on_first_attempt_no_retry`
**Purpose**: Successful operations don't trigger retries.

**Setup**:
- Operation succeeds immediately

**Assertions**:
- ✅ Only 1 attempt
- ✅ Zero delay
- ✅ Success result

#### 6. `test_max_delay_cap_enforced`
**Purpose**: Maximum delay cap is enforced.

**Setup**:
- Max delay: 3000ms
- High attempt numbers

**Assertions**:
- ✅ Delay never exceeds 3000ms
- ✅ Cap applies to all high attempts

#### 7. `test_retryable_errors_classification`
**Purpose**: Verify error classification logic.

**Assertions**:
- ✅ Transient errors are retryable
- ✅ Permanent errors are not retryable
- ✅ Correct classification for all error types

#### 8. `test_retry_with_alternating_errors`
**Purpose**: Handle different error types across retries.

**Setup**:
- Attempt 1: QuoteNotFound
- Attempt 2: EndpointNotFound
- Attempt 3: Success

**Assertions**:
- ✅ Retries continue through different errors
- ✅ Success on 3rd attempt
- ✅ Correct attempt count

#### 9. `test_all_retries_exhausted`
**Purpose**: Verify behavior when all retries fail.

**Setup**:
- 2 max attempts
- All attempts fail

**Assertions**:
- ✅ Both attempts executed
- ✅ Final result is failure
- ✅ Correct error returned

#### 10. `test_timeout_delay_calculation`
**Purpose**: Validate delay calculation with different multiplier.

**Setup**:
- 3x backoff multiplier
- Initial delay: 200ms

**Assertions**:
- ✅ Correct exponential growth (200, 600, 1800, 5400)
- ✅ First attempt has zero delay

#### 11. `test_custom_retry_config`
**Purpose**: Verify custom configuration is applied.

**Setup**:
- Custom retry parameters

**Assertions**:
- ✅ Config values correctly set
- ✅ Engine uses custom config

## Key Features

✅ **Timeout Enforcement**: Slow anchors trigger timeouts  
✅ **Retry Logic**: Automatic retries with exponential backoff  
✅ **Backoff Calculation**: Exponential delay growth  
✅ **Max Delay Cap**: Upper limit on retry delays  
✅ **Error Classification**: Retryable vs non-retryable  
✅ **Immediate Stop**: Non-retryable errors stop retries  
✅ **Success Handling**: No unnecessary retries on success

## Retry Configuration

```rust
RetryConfig {
    max_attempts: 3,        // Maximum retry attempts
    initial_delay_ms: 100,  // Starting delay
    max_delay_ms: 5000,     // Maximum delay cap
    backoff_multiplier: 2,  // Exponential multiplier
}
```

## Exponential Backoff Formula

```
delay(n) = min(initial_delay * multiplier^(n-1), max_delay)
```

Example with initial=100ms, multiplier=2:
- Attempt 0: 0ms (no delay)
- Attempt 1: 100ms
- Attempt 2: 200ms
- Attempt 3: 400ms
- Attempt 4: 800ms

## Retryable Errors

- `EndpointNotFound` - Transient network issue
- `ServicesNotConfigured` - Temporary unavailability
- `QuoteNotFound` - Data not yet available
- `StaleQuote` - Can fetch fresh data
- `NoQuotesAvailable` - Temporary condition
- `NoAnchorsAvailable` - Temporary condition

## Non-Retryable Errors

- `InvalidConfig` - Configuration error
- `UnauthorizedAttestor` - Permission issue
- `ReplayAttack` - Security violation
- `ComplianceNotMet` - Business rule violation
- `InvalidQuote` - Data validation error

## Running Tests

```bash
cargo test timeout_tests --lib
```

## Test Results
```
running 11 tests
test timeout_tests::test_all_retries_exhausted ... ok
test timeout_tests::test_custom_retry_config ... ok
test timeout_tests::test_max_delay_cap_enforced ... ok
test timeout_tests::test_non_retryable_error_stops_immediately ... ok
test timeout_tests::test_retries_triggered_correctly ... ok
test timeout_tests::test_retry_with_alternating_errors ... ok
test timeout_tests::test_retryable_errors_classification ... ok
test timeout_tests::test_success_on_first_attempt_no_retry ... ok
test timeout_tests::test_timeout_delay_calculation ... ok
test timeout_tests::test_timeout_enforced_on_slow_anchor ... ok
test timeout_tests::test_timeout_with_exponential_backoff ... ok

test result: ok. 11 passed; 0 failed
```
