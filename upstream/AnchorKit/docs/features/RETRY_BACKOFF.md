# Retry & Exponential Backoff

AnchorKit includes robust retry logic with exponential backoff for handling transient failures in anchor communications.

## Features

- **Exponential Backoff**: Delays increase exponentially between retries (configurable multiplier)
- **Configurable Strategy**: Customize max attempts, initial delay, max delay, and backoff multiplier
- **Smart Error Classification**: Automatically distinguishes retryable vs non-retryable errors
- **Network Failure Handling**: Retries on transport errors and timeouts
- **Rate Limit Handling**: Backs off when encountering 429 rate limit responses
- **5xx Response Handling**: Retries on server errors

## Retryable Errors

The following errors are automatically retried:

### Network Failures
- `TransportError` - General network/transport failures
- `TransportTimeout` - Request timeouts
- `EndpointNotFound` - Endpoint temporarily unavailable

### Rate Limiting
- `RateLimitExceeded` - Rate limit exceeded (429)
- `ProtocolRateLimitExceeded` - Protocol-level rate limiting

### Transient Failures
- `ServicesNotConfigured` - Service temporarily unavailable
- `AttestationNotFound` - Attestation not yet available
- `SessionNotFound` - Session not yet created
- `StaleQuote` - Quote expired, can fetch fresh
- `NoQuotesAvailable` - No quotes currently available
- `AnchorMetadataNotFound` - Metadata not yet cached
- `CacheExpired` - Cache expired, can refresh
- `CacheNotFound` - Cache miss, can fetch

## Non-Retryable Errors

The following errors are NOT retried (permanent failures):

- `InvalidConfig` - Configuration error
- `UnauthorizedAttestor` - Authentication failure
- `TransportUnauthorized` - Authorization failure
- `InvalidQuote` - Invalid quote data
- `InvalidTimestamp` - Invalid timestamp
- `ReplayAttack` - Replay attack detected
- `ComplianceNotMet` - Compliance check failed
- `CredentialExpired` - Expired credentials
- `ProtocolError` - Protocol violation
- `ProtocolInvalidPayload` - Invalid payload format

## Configuration

### Default Configuration

```rust
use anchorkit::retry::RetryConfig;

let config = RetryConfig::default();
// max_attempts: 3
// initial_delay_ms: 100
// max_delay_ms: 5000
// backoff_multiplier: 2
```

### Custom Configuration

```rust
use anchorkit::retry::RetryConfig;

// Aggressive: many attempts, short delays
let aggressive = RetryConfig::new(
    10,    // max_attempts
    10,    // initial_delay_ms
    1000,  // max_delay_ms
    2      // backoff_multiplier
);

// Conservative: few attempts, long delays
let conservative = RetryConfig::new(
    3,     // max_attempts
    1000,  // initial_delay_ms
    10000, // max_delay_ms
    3      // backoff_multiplier
);

// Custom for rate limiting: longer delays
let rate_limit = RetryConfig::new(
    5,     // max_attempts
    500,   // initial_delay_ms
    30000, // max_delay_ms (30 seconds)
    3      // backoff_multiplier
);
```

## Usage Examples

### Basic Retry

```rust
use anchorkit::retry::{RetryConfig, RetryEngine};

let config = RetryConfig::default();
let engine = RetryEngine::new(config);

let result = engine.execute(|attempt| {
    // Your operation here
    // Returns Ok(value) on success or Err(error) on failure
    make_network_request()
});

if result.is_success() {
    println!("Success after {} attempts", result.attempts);
    println!("Total delay: {}ms", result.total_delay_ms);
    let value = result.value.unwrap();
} else {
    println!("Failed after {} attempts", result.attempts);
    let error = result.error.unwrap();
}
```

### With Transport Layer

```rust
use anchorkit::{
    retry::{RetryConfig, RetryEngine},
    transport::{AnchorTransport, TransportRequest, TransportResponse},
};

let config = RetryConfig::new(5, 100, 5000, 2);
let engine = RetryEngine::new(config);

let result = engine.execute(|attempt| {
    println!("Attempt {}", attempt + 1);
    
    let request = TransportRequest::GetQuote {
        endpoint: endpoint.clone(),
        base_asset: base.clone(),
        quote_asset: quote.clone(),
        amount: 1000,
    };
    
    transport.send_request(&env, request)
});

match result.value {
    Some(TransportResponse::Quote(quote)) => {
        println!("Got quote: rate={}", quote.rate);
    }
    _ => {
        println!("Failed to get quote: {:?}", result.error);
    }
}
```

### Rate Limit Handling

```rust
use anchorkit::retry::{RetryConfig, RetryEngine};

// Configure longer delays for rate limiting
let config = RetryConfig::new(
    5,     // max_attempts
    500,   // initial_delay_ms
    30000, // max_delay_ms (30 seconds)
    3      // backoff_multiplier (aggressive backoff)
);

let engine = RetryEngine::new(config);

let result = engine.execute(|attempt| {
    match make_api_call() {
        Ok(response) => Ok(response),
        Err(Error::RateLimitExceeded) => {
            println!("Rate limited, backing off...");
            Err(Error::RateLimitExceeded)
        }
        Err(e) => Err(e),
    }
});
```

### Network Failure Recovery

```rust
use anchorkit::retry::{RetryConfig, RetryEngine};

let config = RetryConfig::new(4, 100, 5000, 2);
let engine = RetryEngine::new(config);

let result = engine.execute(|attempt| {
    match fetch_anchor_data() {
        Ok(data) => Ok(data),
        Err(Error::TransportTimeout) => {
            println!("Timeout on attempt {}, retrying...", attempt + 1);
            Err(Error::TransportTimeout)
        }
        Err(Error::TransportError) => {
            println!("Network error on attempt {}, retrying...", attempt + 1);
            Err(Error::TransportError)
        }
        Err(e) => Err(e),
    }
});
```

## Backoff Timing

The delay between retries follows an exponential pattern:

```
Attempt 0: 0ms (immediate)
Attempt 1: initial_delay_ms
Attempt 2: initial_delay_ms * multiplier^1
Attempt 3: initial_delay_ms * multiplier^2
Attempt 4: initial_delay_ms * multiplier^3
...
```

### Example with Default Config

```
max_attempts: 3
initial_delay_ms: 100
backoff_multiplier: 2

Attempt 0: 0ms
Attempt 1: 100ms
Attempt 2: 200ms
Total: 300ms
```

### Example with Aggressive Config

```
max_attempts: 5
initial_delay_ms: 50
backoff_multiplier: 3

Attempt 0: 0ms
Attempt 1: 50ms
Attempt 2: 150ms
Attempt 3: 450ms
Attempt 4: 1350ms
Total: 2000ms
```

### Example with Rate Limit Config

```
max_attempts: 5
initial_delay_ms: 500
backoff_multiplier: 3
max_delay_ms: 30000

Attempt 0: 0ms
Attempt 1: 500ms
Attempt 2: 1500ms
Attempt 3: 4500ms
Attempt 4: 13500ms
Total: 20000ms
```

## Best Practices

### 1. Choose Appropriate Strategy

- **Network failures**: Moderate attempts (3-5), short delays (100-500ms)
- **Rate limiting**: Fewer attempts (3-4), longer delays (500-1000ms), higher multiplier (3-4)
- **Server errors (5xx)**: Moderate attempts (3-5), moderate delays (200-500ms)

### 2. Set Reasonable Max Delays

```rust
// For user-facing operations: keep max delay low
let user_facing = RetryConfig::new(3, 100, 2000, 2);

// For background operations: can use longer delays
let background = RetryConfig::new(5, 500, 30000, 3);
```

### 3. Monitor Retry Metrics

```rust
let result = engine.execute(|attempt| {
    // Your operation
});

// Log retry metrics
println!("Attempts: {}", result.attempts);
println!("Total delay: {}ms", result.total_delay_ms);
println!("Success: {}", result.is_success());
```

### 4. Handle Non-Retryable Errors

```rust
let result = engine.execute(|attempt| {
    match operation() {
        Err(Error::InvalidConfig) => {
            // Non-retryable, fail fast
            return Err(Error::InvalidConfig);
        }
        Err(Error::TransportTimeout) => {
            // Retryable, will retry
            return Err(Error::TransportTimeout);
        }
        Ok(value) => Ok(value),
    }
});
```

## Testing

The retry logic includes comprehensive tests:

```bash
# Run all retry tests
cargo test retry --lib

# Run specific test categories
cargo test test_network_failure --lib
cargo test test_rate_limit --lib
cargo test test_exponential_backoff --lib
```

## Integration with Transport

The retry logic is designed to work seamlessly with the transport layer:

```rust
use anchorkit::{
    retry::{RetryConfig, RetryEngine},
    transport::{AnchorTransport, MockTransport, TransportRequest},
};

let mut transport = MockTransport::new();
let config = RetryConfig::default();
let engine = RetryEngine::new(config);

let result = engine.execute(|_| {
    transport.send_request(&env, request.clone())
});
```

## Error Classification

To check if an error is retryable:

```rust
use anchorkit::retry::is_retryable_error;
use anchorkit::errors::Error;

if is_retryable_error(&Error::TransportTimeout) {
    println!("This error will be retried");
}

if !is_retryable_error(&Error::InvalidConfig) {
    println!("This error will NOT be retried");
}
```

## Summary

- ✅ Exponential backoff with configurable parameters
- ✅ Smart error classification (retryable vs non-retryable)
- ✅ Network failure handling (timeouts, transport errors)
- ✅ Rate limit handling (429 responses)
- ✅ 5xx server error handling
- ✅ Configurable retry strategies
- ✅ Comprehensive test coverage
- ✅ Integration with transport layer
