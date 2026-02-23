# Rate Limiter Feature

## Overview

AnchorKit now includes a pluggable rate limiter to prevent accidental overload of anchor APIs. The rate limiter is configurable per-anchor and supports two strategies:

1. **Fixed Window** - Limits requests within a fixed time window
2. **Token Bucket** - Allows burst traffic with token-based refill

## Features

- ✅ Per-anchor configuration
- ✅ Two rate limiting strategies (Fixed Window, Token Bucket)
- ✅ Configurable limits and time windows
- ✅ Automatic enforcement on anchor requests
- ✅ Optional (no rate limit by default)

## Usage

### Configure Rate Limit

Only the admin can configure rate limits for anchors:

```rust
use anchorkit::{RateLimitConfig, RateLimitStrategy};

// Fixed Window: 100 requests per 60 seconds
let config = RateLimitConfig {
    strategy: RateLimitStrategy::FixedWindow,
    max_requests: 100,
    window_seconds: 60,
    refill_rate: 0, // Not used for fixed window
};

client.configure_rate_limit(&anchor, &config);
```

### Token Bucket Strategy

```rust
// Token Bucket: 50 tokens, refill 1 token per second
let config = RateLimitConfig {
    strategy: RateLimitStrategy::TokenBucket,
    max_requests: 50,      // Maximum tokens
    window_seconds: 0,     // Not used for token bucket
    refill_rate: 1,        // Tokens per second
};

client.configure_rate_limit(&anchor, &config);
```

### Query Rate Limit Configuration

```rust
let config = client.get_rate_limit_config(&anchor);
if let Some(cfg) = config {
    println!("Max requests: {}", cfg.max_requests);
    println!("Strategy: {:?}", cfg.strategy);
}
```

## How It Works

### Fixed Window

- Tracks number of requests within a time window
- Resets counter when window expires
- Simple and predictable
- May allow burst at window boundaries

Example: 100 requests per 60 seconds
- Request 1-100: ✅ Allowed
- Request 101: ❌ Rate limit exceeded
- After 60 seconds: Counter resets to 0

### Token Bucket

- Maintains a bucket of tokens
- Each request consumes 1 token
- Tokens refill at configured rate
- Allows controlled bursts

Example: 50 tokens, refill 1/second
- Start with 50 tokens
- Request 1-50: ✅ Allowed (consumes all tokens)
- Request 51: ❌ Rate limit exceeded
- After 10 seconds: 10 tokens refilled
- Request 52-61: ✅ Allowed (10 tokens available)

## Automatic Enforcement

Rate limiting is automatically enforced on:
- `submit_quote()` - Quote submissions from anchors

When rate limit is exceeded, the operation returns `Error::RateLimitExceeded`.

## Configuration Examples

### Conservative Anchor (Low Traffic)
```rust
RateLimitConfig {
    strategy: RateLimitStrategy::FixedWindow,
    max_requests: 10,
    window_seconds: 60,
    refill_rate: 0,
}
```

### High-Volume Anchor
```rust
RateLimitConfig {
    strategy: RateLimitStrategy::TokenBucket,
    max_requests: 1000,
    window_seconds: 0,
    refill_rate: 10, // 10 tokens/second = 600/minute
}
```

### Burst-Tolerant Configuration
```rust
RateLimitConfig {
    strategy: RateLimitStrategy::TokenBucket,
    max_requests: 100,  // Allow burst of 100
    window_seconds: 0,
    refill_rate: 5,     // Steady rate of 5/second
}
```

## Error Handling

```rust
match client.try_submit_quote(&anchor, ...) {
    Ok(quote_id) => {
        println!("Quote submitted: {}", quote_id);
    }
    Err(Ok(Error::RateLimitExceeded)) => {
        println!("Rate limit exceeded, retry later");
    }
    Err(e) => {
        println!("Other error: {:?}", e);
    }
}
```

## Storage

Rate limit state is stored in temporary storage with 1-day TTL:
- Lightweight and efficient
- Automatically expires
- Per-anchor isolation

Configuration is stored in persistent storage (90-day TTL).

## Best Practices

1. **Start Conservative** - Begin with lower limits and increase as needed
2. **Monitor Usage** - Track rate limit errors to tune configuration
3. **Per-Anchor Tuning** - Different anchors may need different limits
4. **Token Bucket for APIs** - Better for real-world API usage patterns
5. **Fixed Window for Simplicity** - Easier to reason about and predict

## API Reference

### Types

```rust
pub enum RateLimitStrategy {
    FixedWindow,
    TokenBucket,
}

pub struct RateLimitConfig {
    pub strategy: RateLimitStrategy,
    pub max_requests: u32,
    pub window_seconds: u64,
    pub refill_rate: u32,
}
```

### Contract Methods

```rust
// Configure rate limit (admin only)
pub fn configure_rate_limit(
    env: Env,
    anchor: Address,
    config: RateLimitConfig,
) -> Result<(), Error>

// Get rate limit configuration
pub fn get_rate_limit_config(
    env: Env,
    anchor: Address,
) -> Option<RateLimitConfig>
```

### Errors

- `Error::RateLimitExceeded` (48) - Rate limit exceeded for anchor
- `Error::InvalidConfig` (25) - Invalid rate limit configuration
- `Error::AttestorNotRegistered` (5) - Anchor not registered

## Implementation Details

- Rate limiter uses temporary storage for state tracking
- State includes request count, window start, tokens, and last refill time
- Automatic cleanup via TTL (no manual maintenance needed)
- Zero overhead when no rate limit is configured
- Thread-safe (Soroban contract execution model)

## Testing

The rate limiter includes comprehensive tests:
- Fixed window enforcement
- Token bucket refill logic
- Per-anchor isolation
- No rate limit (unlimited) behavior

Run tests:
```bash
cargo test rate_limiter_tests
```

## Future Enhancements

Potential future improvements:
- Sliding window algorithm
- Distributed rate limiting
- Rate limit metrics and monitoring
- Dynamic rate adjustment based on load
- Per-operation rate limits (not just per-anchor)
