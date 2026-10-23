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
                println!("Retryable error on attempt {}: {:?}", attempt, e);
                Err(e)
            }
            Err(e) => Err(e),
        }
    },
    |e| should_retry(e),
    |delay_ms| std::thread::sleep(std::time::Duration::from_millis(delay_ms)),
);
```

## Backoff Calculation

Delays are calculated as:

```
delay = min(base_delay_ms * (backoff_multiplier ^ (attempt - 1)), max_delay_ms)
```

### Example Progression

With `base_delay_ms = 100`, `backoff_multiplier = 2`, `max_delay_ms = 5000`:

| Attempt | Delay (ms) |
| --- | --- |
| 1 | 100 |
| 2 | 200 |
| 3 | 400 |
| 4 | 800 |
| 5 | 1600 |
| 6 | 3200 |
| 7 | 5000 (capped) |

## Best Practices

1. **Use exponential backoff** for network operations
2. **Set reasonable max attempts** (3-5 for most cases)
3. **Cap maximum delay** to avoid excessive waits
4. **Classify errors correctly** — don't retry permanent failures
5. **Log retry attempts** for debugging
6. **Consider jitter** for distributed systems to avoid thundering herd

## Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retry_succeeds_after_failures() {
        let config = RetryConfig::new(3, 10, 100, 2);
        let mut attempts = 0;

        let result = retry_with_backoff(
            &config,
            |_| {
                attempts += 1;
                if attempts < 3 {
                    Err("transient")
                } else {
                    Ok("success")
                }
            },
            |_| true,
            |_| {},
        );

        assert_eq!(result, Ok("success"));
        assert_eq!(attempts, 3);
    }
}
```
