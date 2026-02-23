# Fallback Anchor Selection

## Overview

Automatically reroute to fallback anchors when preferred anchor fails.

## Features

- ✅ Configurable fallback order
- ✅ Failure detection logic
- ✅ Automatic anchor health tracking
- ✅ Retry with max attempts

## Usage

### Configure Fallback Order

```rust
let anchor_order = vec![&env, anchor1, anchor2, anchor3];
client.configure_fallback(
    &anchor_order,
    &3,  // max_retries
    &2   // failure_threshold
);
```

### Record Failures

```rust
// Automatically tracked, or manually:
client.record_anchor_failure(&anchor);
```

### Record Success

```rust
client.record_anchor_success(&anchor);
```

### Select Next Anchor

```rust
// Get next available anchor
let next_anchor = client.select_fallback_anchor(&Some(failed_anchor));

// Or start from beginning
let first_anchor = client.select_fallback_anchor(&None);
```

### Automatic Fallback

```rust
// Automatically tries fallback anchors on failure
let quote_id = client.submit_quote_with_fallback(
    &base_asset,
    &quote_asset,
    &rate,
    &fee_percentage,
    &minimum_amount,
    &maximum_amount,
    &valid_until,
);
```

## API Methods

```rust
// Configure fallback
pub fn configure_fallback(
    anchor_order: Vec<Address>,
    max_retries: u32,
    failure_threshold: u32,
) -> Result<(), Error>

// Get configuration
pub fn get_fallback_config() -> Option<FallbackConfig>

// Record anchor state
pub fn record_anchor_failure(anchor: Address) -> Result<(), Error>
pub fn record_anchor_success(anchor: Address) -> Result<(), Error>

// Get failure state
pub fn get_anchor_failure_state(anchor: Address) -> Option<AnchorFailureState>

// Select fallback
pub fn select_fallback_anchor(failed_anchor: Option<Address>) -> Result<Address, Error>

// Automatic fallback
pub fn submit_quote_with_fallback(...) -> Result<u64, Error>
```

## Configuration

```rust
pub struct FallbackConfig {
    pub anchor_order: Vec<Address>,  // Ordered list to try
    pub max_retries: u32,             // Max retry attempts
    pub failure_threshold: u32,       // Failures before marking down
}
```

## Failure State

```rust
pub struct AnchorFailureState {
    pub anchor: Address,
    pub failure_count: u32,
    pub last_failure: u64,
    pub is_down: bool,  // true when >= threshold
}
```

## How It Works

1. **Configure Order**: Set preferred anchor order
2. **Detect Failure**: Track failures per anchor
3. **Mark Down**: After threshold failures, mark anchor as down
4. **Skip Down Anchors**: Automatically skip unavailable anchors
5. **Retry**: Try next anchor in order
6. **Success Clears**: Success resets failure state

## Example Flow

```rust
// Setup
let order = vec![&env, primary, secondary, tertiary];
client.configure_fallback(&order, &3, &2);

// Primary fails twice - marked down
client.record_anchor_failure(&primary);
client.record_anchor_failure(&primary);

// Next selection skips primary, returns secondary
let next = client.select_fallback_anchor(&None);
assert_eq!(next, secondary);

// Secondary succeeds - clears its failure state
client.record_anchor_success(&secondary);
```

## Storage

- **Config**: Persistent storage (90-day TTL)
- **Failure State**: Temporary storage (1-day TTL)

## Best Practices

1. **Order by preference** - Put most reliable anchors first
2. **Set appropriate threshold** - Balance sensitivity vs false positives
3. **Monitor failure states** - Track which anchors are down
4. **Clear on success** - Automatically done by `record_anchor_success`
5. **Use automatic fallback** - Let system handle retries

## Error Handling

- `Error::NoAnchorsAvailable` - All anchors down or exhausted retries
- `Error::InvalidConfig` - No fallback config set
