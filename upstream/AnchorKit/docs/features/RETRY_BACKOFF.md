# Retry & Exponential Backoff

> **Status: Implemented** — `src/retry.rs` (`retry_with_backoff`, `RetryConfig`, `is_retryable`)

AnchorKit includes robust retry logic with exponential backoff for handling transient failures in anchor communications.

## Implementation

```rust
use anchorkit::retry::{RetryConfig, retry_with_backoff, is_retryable};

let config = RetryConfig::default(); // max_attempts=3, base_delay_ms=100, max_delay_ms=5000, backoff_multiplier=2

let result = retry_with_backoff(
    &config,
    |attempt| fetch_stellar_toml(attempt),
    |e| is_retryable(e.code as u32),
    |delay_ms| std::thread::sleep(std::time::Duration::from_millis(delay_ms)),
);
```

## How `is_retryable` works

The helper `is_retryable(code)` classifies only transient, availability, and cache-related failure codes as retryable. It returns `true` when the numeric error code matches one of the following `ErrorCode` variants:

- `ServicesNotConfigured`
- `AttestationNotFound`
- `StaleQuote`
- `NoQuotesAvailable`
- `CacheExpired`
- `CacheNotFound`

**Note**: `RateLimitExceeded` is intentionally **NOT** retryable. The rate window only clears after `window_length` ledgers, so retrying in a tight backoff loop would burn through every attempt and still fail. Callers that need to recover from a rate limit should wait for the window to reset (or call the admin reset path) before issuing the next request.

All other `ErrorCode` values are considered non-retryable and should stop immediately.

> Note: `is_retryable(code)` only classifies the numeric error codes above. The lists below describe the broader retry categories used by AnchorKit, while the helper itself is implemented on a concise set of transient codes.

### Retryability summary

| Error code | Retryable? | Reason |
| --- | --- | --- |
| `AlreadyInitialized` | No | Permanent contract state error |
| `AttestorAlreadyRegistered` | No | Duplicate registration |
| `AttestorNotRegistered` | No | Invalid anchor state |
| `UnauthorizedAttestor` | No | Auth failure |
| `InvalidTimestamp` | No | Bad request data |
| `ReplayAttack` | No | Security violation |
| `InvalidQuote` | No | Bad quote data |
| `InvalidServiceType` | No | Invalid request parameter |
| `InvalidTransactionIntent` | No | Invalid transaction shape |
| `StaleQuote` | Yes | Quote expired; retry after refresh |
| `ComplianceNotMet` | No | Permanent policy failure |
| `InvalidEndpointFormat` | No | Bad endpoint configuration |
| `NoQuotesAvailable` | Yes | Transient quote availability |
| `ServicesNotConfigured` | Yes | Anchor not ready yet |
| `ValidationError` | No | Invalid response payload |
| `RateLimitExceeded` | No | Rate window only clears after window_length ledgers; tight retry loops will fail |
| `NotInitialized` | No | Contract not ready |
| `AttestationNotFound` | Yes | Data may become available soon |
| `InvalidSep10Token` | No | Auth failure |
| `StorageCorrupted` | No | Persistent on-chain corruption |
| `CacheExpired` | Yes | Refreshable cache state |
| `CacheNotFound` | Yes | Cache miss; can refresh |

## Features

- **Exponential Backoff**: Delays increase exponentially between retries (configurable multiplier)
- **Configurable Strategy**: Customize max attempts, initial delay, max delay, and backoff multiplier
- **Smart Error Classification**: Automatically distinguishes retryable vs non-retryable errors
- **Network Failure Handling**: Retries on transport errors and timeouts
- **Rate Limit Handling**: Backs off when encountering 429 rate limit responses
- **5xx Response Handling**: Retries on server errors

## Retryable Errors

The following errors are automatically retried by `is_retryable`:

### Transient Failures
- `ServicesNotConfigured` - Service temporarily unavailable
- `AttestationNotFound` - Attestation not yet available
- `StaleQuote` - Quote expired, can fetch fresh
- `NoQuotesAvailable` - No quotes currently available
- `CacheExpired` - Cache expired, can refresh
- `CacheNotFound` - Cache miss, can fetch

## Non-Retryable Errors

The following errors are NOT retried (permanent failures):

- `AlreadyInitialized` - Permanent contract state error
- `AttestorAlreadyRegistered` - Duplicate registration
- `AttestorNotRegistered` - Invalid anchor state
- `UnauthorizedAttestor` - Authentication failure
- `InvalidTimestamp` - Bad request data
- `ReplayAttack` - Security violation
- `InvalidQuote` - Invalid quote data
- `InvalidServiceType` - Invalid request parameter
- `InvalidTransactionIntent` - Invalid transaction shape
- `ComplianceNotMet` - Permanent policy failure
- `InvalidEndpointFormat` - Bad endpoint configuration
- `ValidationError` - Invalid response payload
- `RateLimitExceeded` - Rate window requires ledger-based reset
- `NotInitialized` - Contract not ready
- `InvalidSep10Token` - Authentication failure
- `StorageCorrupted` - Persistent on-chain corruption

## Configuration

### Default Configuration

```rust
use anchorkit::retry::RetryConfig;

let config = RetryConfig::default();
// max_attempts: 3
// base_delay_ms: 100
// max_delay_ms: 5000
// backoff_multiplier: 2
```

### Custom Configuration

```rust
use anchorkit::retry::RetryConfig;

// Aggressive: many attempts, short delays
let aggressive = RetryConfig::new(
    10,    // max_attempts
    10,    // base_delay_ms
    1000,  // max_delay_ms
    2      // backoff_multiplier
);

// Conservative: few attempts, long delays
let conservative = RetryConfig::new(
    3,     // max_attempts
    1000,  // base_delay_ms
    10000, // max_delay_ms
    3      // backoff_multiplier
);

// Custom for rate limiting: longer delays
let rate_limit = RetryConfig::new(
    5,     // max_attempts
    500,   // base_delay_ms
    30000, // max_delay_ms (30 seconds)
    3      // backoff_multiplier
);
```

## Usage Examples

### Non-retryable errors stop immediately

```rust
use anchorkit::retry::{retry_with_backoff, RetryConfig, is_retryable};
use anchorkit::{AnchorKitError, ErrorCode};

let config = RetryConfig::default();
let mut attempts = 0;

let result = retry_with_backoff(
    &config,
    |_| {
        attempts += 1;
        Err::<(), _>(AnchorKitError::invalid_quote())
    },
    |e: &AnchorKitError| is_retryable(e.code as u32),
    |_| {},
);

assert_eq!(attempts, 1);
assert!(matches!(result, Err(err) if err.code == ErrorCode::InvalidQuote));
```

### Basic Retry with Custom Sleep

```rust
use anchorkit::retry::{RetryConfig, retry_with_backoff};

let config = RetryConfig::default();

let result = retry_with_backoff(
    &config,
    |attempt| {
        println!("Attempt {}", attempt);
        // Your operation here
        make_network_request()
    },
    |e| is_retryable_error(e),
    |delay_ms| std::thread::sleep(std::time::Duration::from_millis(delay_ms)),
);

match result {
    Ok(value) => println!("Success: {:?}", value),
    Err(error) => println!("Failed: {:?}", error),
}
```

### Retryable Error Handling

```rust
use anchorkit::retry::{retry_with_backoff, RetryConfig, is_retryable};

let config = RetryConfig::new(5, 100, 5000, 2);

let result = retry_with_backoff(
    &config,
    |attempt| {
        println!("Attempt {}", attempt + 1);
        fetch_anchor_data()
    },
    |e| is_retryable(e.code as u32),
    |delay_ms| std::thread::sleep(std::time::Duration::from_millis(delay_ms)),
);

match result {
    Ok(data) => println!("Got data: {:?}", data),
    Err(error) => println!("Failed after retries: {:?}", error),
}
```

### Custom Retry Logic

```rust
use anchorkit::retry::{RetryConfig, retry_with_backoff};

// Configure longer delays for specific scenarios
let config = RetryConfig::new(
    5,     // max_attempts
    500,   // base_delay_ms
    30000, // max_delay_ms (30 seconds)
    3      // backoff_multiplier (aggressive backoff)
);

let result = retry_with_backoff(
    &config,
    |attempt| {
        match make_api_call() {
            Ok(response) => Ok(response),
            Err(e) if should_retry(&e) => {
                println!("Retryable error on attempt {}, backing off...", attempt + 1);
                Err(e)
            }
            Err(e) => Err(e),
        }
    },
    |e| should_retry(e),
    |delay_ms| std::thread::sleep(std::time::Duration::from_millis(delay_ms)),
);
```

### Transient Failure Recovery

```rust
use anchorkit::retry::{RetryConfig, retry_with_backoff, is_retryable};

let config = RetryConfig::new(4, 100, 5000, 2);

let result = retry_with_backoff(
    &config,
    |attempt| {
        match fetch_anchor_data() {
            Ok(data) => Ok(data),
            Err(e) if is_retryable(e.code as u32) => {
                println!("Transient error on attempt {}, retrying...", attempt + 1);
                Err(e)
            }
            Err(e) => Err(e),
        }
    },
    |e| is_retryable(e.code as u32),
    |delay_ms| std::thread::sleep(std::time::Duration::from_millis(delay_ms)),
);
```

## Backoff Timing

The delay between retries follows an exponential pattern:

```
Attempt 0: base_delay_ms * multiplier^0 + jitter
Attempt 1: base_delay_ms * multiplier^1 + jitter
Attempt 2: base_delay_ms * multiplier^2 + jitter
Attempt 3: base_delay_ms * multiplier^3 + jitter
...
(capped at max_delay_ms)
```

The jitter is derived deterministically from the attempt number and adds randomness up to `base_delay_ms / 2` to help desynchronize retry storms across clients.

### Example with Default Config

```
max_attempts: 3
base_delay_ms: 100
backoff_multiplier: 2

Attempt 0: ~100ms (100 * 2^0 + jitter)
Attempt 1: ~200ms (100 * 2^1 + jitter)
Attempt 2: ~400ms (100 * 2^2 + jitter)
```

### Example with Aggressive Config

```
max_attempts: 5
base_delay_ms: 50
backoff_multiplier: 3

Attempt 0: ~50ms (50 * 3^0 + jitter)
Attempt 1: ~150ms (50 * 3^1 + jitter)
Attempt 2: ~450ms (50 * 3^2 + jitter)
Attempt 3: ~1350ms (50 * 3^3 + jitter)
Attempt 4: ~4050ms (50 * 3^4 + jitter)
```

### Example with Rate Limit Config

```
max_attempts: 5
base_delay_ms: 500
backoff_multiplier: 3
max_delay_ms: 30000

Attempt 0: ~500ms (500 * 3^0 + jitter)
Attempt 1: ~1500ms (500 * 3^1 + jitter)
Attempt 2: ~4500ms (500 * 3^2 + jitter)
Attempt 3: ~13500ms (500 * 3^3 + jitter)
Attempt 4: ~30000ms (capped at max_delay_ms)
```

## Best Practices

### 1. Choose Appropriate Strategy

- **Transient failures**: Moderate attempts (3-5), short delays (100-500ms)
- **Service unavailability**: Fewer attempts (3-4), longer delays (500-1000ms), higher multiplier (3)
- **Cache misses**: Moderate attempts (3-5), moderate delays (200-500ms)

### 2. Set Reasonable Max Delays

```rust
// For user-facing operations: keep max delay low
let user_facing = RetryConfig::new(3, 100, 2000, 2);

// For background operations: can use longer delays
let background = RetryConfig::new(5, 500, 30000, 3);
```

### 3. Provide Custom Sleep Function

```rust
// For synchronous code
let result = retry_with_backoff(
    &config,
    |attempt| operation(attempt),
    |e| is_retryable(e.code as u32),
    |delay_ms| std::thread::sleep(std::time::Duration::from_millis(delay_ms)),
);

// For testing (no actual sleep)
let result = retry_with_backoff(
    &config,
    |attempt| operation(attempt),
    |e| is_retryable(e.code as u32),
    |_| {}, // no-op sleep for tests
);
```

### 4. Handle Non-Retryable Errors

```rust
let result = retry_with_backoff(
    &config,
    |attempt| {
        match operation() {
            Err(e) if !is_retryable(e.code as u32) => {
                // Non-retryable, fail fast
                return Err(e);
            }
            other => other,
        }
    },
    |e| is_retryable(e.code as u32),
    |delay_ms| std::thread::sleep(std::time::Duration::from_millis(delay_ms)),
);
```

## Testing

The retry logic includes comprehensive tests:

```bash
# Run all retry tests
cargo test retry --lib

# Run specific test categories
cargo test test_success_on_first_try --lib
cargo test test_exhausted_retries --lib
cargo test test_delay_increases_exponentially --lib
```

## Integration with Application Code

The retry logic is designed to work with any operation that returns a `Result`:

```rust
use anchorkit::retry::{RetryConfig, retry_with_backoff, is_retryable};

let config = RetryConfig::default();

let result = retry_with_backoff(
    &config,
    |attempt| {
        // Your operation that might fail
        perform_operation(attempt)
    },
    |e| is_retryable(e.code as u32),
    |delay_ms| std::thread::sleep(std::time::Duration::from_millis(delay_ms)),
);
```

## Summary

- ✅ Exponential backoff with configurable parameters
- ✅ Smart error classification (retryable vs non-retryable)
- ✅ Transient failure handling (service unavailability, cache misses)
- ✅ Configurable retry strategies
- ✅ Jitter to prevent retry storms
- ✅ Comprehensive test coverage
- ✅ Simple function-based API with `retry_with_backoff`
