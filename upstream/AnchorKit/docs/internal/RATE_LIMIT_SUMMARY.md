# Rate Limit Detection & Backoff Implementation Summary

## Overview

This document describes the comprehensive rate limit detection and backoff mechanism implemented in AnchorKit to ensure:

- Stability under load
- Proper retry behavior
- Reduced risk of temporary bans
- Improved resilience of financial operations

---

## Features Implemented

### 1. Rate Limit Detection

#### 1.1 HTTP Status Code Detection

- **429 Too Many Requests**: Primary rate limit indicator from:
  - Anchor APIs
  - Stellar RPC / Horizon
  - Third-party financial APIs

#### 1.2 Rate Limit Headers Parsing

- `Retry-After`: Seconds to wait or HTTP-date format
- `X-RateLimit-Remaining`: Remaining requests in window
- `X-RateLimit-Reset`: Unix timestamp when limit resets
- `X-RateLimit-Limit`: Total requests allowed in window

#### 1.3 Rate Limit Sources

```
rust
pub enum RateLimitSource {
    AnchorApi,        // Anchor-specific rate limiting
    StellarRpc,       // Stellar RPC node rate limiting
    Horizon,          // Horizon API rate limiting
    ThirdParty,       // External API rate limiting
    Unknown,          // Unidentified source
}
```

### 2. Enhanced Retry Configuration

#### 2.1 Jitter Support

Prevents "thundering herd" problem by adding randomization to retry delays.

```
rust
pub struct RetryConfig {
    // Existing fields
    pub max_attempts: u32,
    pub initial_delay_ms: u64,
    pub max_delay_ms: u64,
    pub backoff_multiplier: u32,

    // New fields
    pub jitter_factor: f64,           // 0.0 to 1.0 (default: 0.1)
    pub use_retry_after: bool,        // Respect Retry-After header (default: true)
    pub rate_limit_initial_delay_ms: u64, // Initial delay for rate limit errors
}
```

#### 2.2 Default Configuration

```
rust
impl RetryConfig {
    pub fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay_ms: 100,
            max_delay_ms: 5000,
            backoff_multiplier: 2,
            jitter_factor: 0.1,              // 10% randomization
            use_retry_after: true,           // Respect server's Retry-After
            rate_limit_initial_delay_ms: 1000, // 1 second for rate limits
        }
    }
}
```

#### 2.3 Delay Calculation with Jitter

```
rust
fn calculate_delay_with_jitter(&self, attempt: u32, is_rate_limit: bool) -> u64 {
    let base_delay = if is_rate_limit {
        self.rate_limit_initial_delay_ms * (self.backoff_multiplier as u64).pow(attempt.saturating_sub(1))
    } else {
        self.calculate_delay(attempt)
    };

    let jitter_range = (base_delay as f64 * self.jitter_factor) as u64;
    let jitter = (rand::random::<u64>() % (jitter_range * 2 + 1)) as i64 - jitter_range as i64;

    (base_delay as i64 + jitter).max(0) as u64
}
```

### 3. Rate Limit Events

Three new events for monitoring rate limit incidents:

#### 3.1 RateLimitEncountered

Emitted when a 429 response is detected.

```
rust
pub struct RateLimitEncountered {
    pub source: RateLimitSource,      // Where the rate limit came from
    pub retry_after_ms: Option<u64>,  // Server-suggested wait time
    pub limit: Option<u32>,            // X-RateLimit-Limit header
    pub remaining: Option<u32>,        // X-RateLimit-Remaining header
    pub reset_timestamp: Option<u64>,   // X-RateLimit-Reset header
}
```

#### 3.2 RateLimitBackoff

Emitted before retrying after a rate limit.

```
rust
pub struct RateLimitBackoff {
    pub attempt: u32,                 // Current retry attempt
    pub delay_ms: u64,                // Calculated backoff delay
    pub uses_retry_after: bool,      // Whether using server's Retry-After
}
```

#### 3.3 RateLimitRecovered

Emitted when requests succeed after rate limiting.

```
rust
pub struct RateLimitRecovered {
    pub total_retries: u32,          // Total attempts before success
    pub total_backoff_ms: u64,       // Total time spent backing off
}
```

### 4. HTTP Status Classification

New functions in error_mapping.rs:

```
rust
/// Check if HTTP status indicates rate limiting
pub fn is_rate_limit_status(status_code: u32) -> bool {
    status_code == 429
}

/// Check if HTTP status indicates server error (5xx)
pub fn is_server_error(status_code: u32) -> bool {
    status_code >= 500 && status_code < 600
}

/// Check if HTTP status indicates client error (4xx)
pub fn is_client_error(status_code: u32) -> bool {
    status_code >= 400 && status_code < 500
}

/// Check if HTTP status is retryable
pub fn is_retryable_status(status_code: u32) -> bool {
    is_rate_limit_status(status_code)
        || is_server_error(status_code)
        || status_code == 408 // Request Timeout
        || status_code == 504  // Gateway Timeout
}

/// Extract rate limit information from response headers
pub fn extract_rate_limit_info(headers: &[(String, String)]) -> Option<RateLimitInfo> {
    // Parse Retry-After, X-RateLimit-* headers
}

/// Calculate retry delay from response
pub fn get_retry_delay_from_response(
    status_code: u32,
    headers: &[(String, String)],
    config: &RetryConfig,
) -> Option<u64> {
    // Priority: Retry-After header > X-RateLimit-Reset > exponential backoff
}
```

---

## Usage Examples

### Basic Rate Limit Handling

```
rust
use anchorkit::retry::{RetryConfig, RetryEngine};
use anchorkit::error_mapping::{is_rate_limit_status, get_retry_delay_from_response};

let config = RetryConfig::default();
let engine = RetryEngine::new(config);

let result = engine.execute(|attempt| {
    let response = make_http_request()?;

    if is_rate_limit_status(response.status_code) {
        // Extract rate limit info from headers
        let delay = get_retry_delay_from_response(
            response.status_code,
            &response.headers,
            &config,
        );

        // Emit rate limit event
        RateLimitEncountered::publish(
            &env,
            RateLimitSource::AnchorApi,
            delay,
            None,
            None,
            None,
        );

        return Err(Error::ProtocolRateLimitExceeded);
    }

    Ok(response)
});
```

### Aggressive Rate Limit Configuration

```
rust
let config = RetryConfig {
    max_attempts: 5,
    initial_delay_ms: 200,
    rate_limit_initial_delay_ms: 1000,  // Longer for rate limits
    max_delay_ms: 60000,                // 1 minute max
    backoff_multiplier: 3,              // More aggressive
    jitter_factor: 0.2,                 // 20% randomization
    use_retry_after: true,              // Respect server
};
```

### Monitoring Rate Limit Events

```
rust
// Subscribe to rate limit events
env.events()
    .subscribe((symbol_short!("rate"), symbol_short!("limit")), |event| {
        let data: RateLimitEncountered = event;
        println!("Rate limited by {:?}, retry after {}ms",
            data.source,
            data.retry_after_ms.unwrap_or(0)
        );
    });
```

---

## Files Modified/Created

### New Files

- `src/rate_limit_response.rs` - Rate limit response parsing module

### Modified Files

- `src/retry.rs` - Added jitter, rate-limit-aware configuration
- `src/events.rs` - Added rate limit events
- `src/error_mapping.rs` - Added HTTP status detection functions
- `src/lib.rs` - Added module exports

---

## Testing

Run rate limit related tests:

```
bash
# All retry tests
cargo test retry --lib

# Rate limit specific tests
cargo test rate_limit --lib

# Jitter tests
cargo test jitter --lib
```

---

## Summary

| Feature                    | Status         |
| -------------------------- | -------------- |
| 429 Detection              | ✅ Implemented |
| Retry-After Parsing        | ✅ Implemented |
| X-RateLimit-\* Headers     | ✅ Implemented |
| Exponential Backoff        | ✅ Existing    |
| Jitter Support             | ✅ Implemented |
| Rate Limit Events          | ✅ Implemented |
| HTTP Status Classification | ✅ Implemented |
| Configuration Options      | ✅ Implemented |
