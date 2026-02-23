# Asset Compatibility Validator

## Overview

Validate asset compatibility before initiating flows to reject unsupported assets early with clear errors.

## Features

- ✅ Early rejection of unsupported assets
- ✅ Clear error output
- ✅ Per-anchor asset configuration
- ✅ Asset pair validation

## Usage

### Configure Supported Assets

```rust
let assets = vec![
    &env,
    String::from_str(&env, "USDC"),
    String::from_str(&env, "BTC"),
    String::from_str(&env, "ETH"),
];

client.set_supported_assets(&anchor, &assets);
```

### Check Asset Support

```rust
if client.is_asset_supported(&anchor, &String::from_str(&env, "USDC")) {
    // Asset is supported
}
```

### Validate Asset Pair

```rust
match client.try_validate_asset_pair(&anchor, &base_asset, &quote_asset) {
    Ok(()) => {
        // Both assets supported
    }
    Err(Ok(Error::UnsupportedAsset)) => {
        // One or both assets not supported
    }
    Err(Ok(Error::AssetNotConfigured)) => {
        // No assets configured for anchor
    }
}
```

### Submit Quote with Validation

```rust
// Automatically validates assets before submission
let quote_id = client.submit_quote_validated(
    &anchor,
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
// Configure assets
pub fn set_supported_assets(
    anchor: Address,
    assets: Vec<String>,
) -> Result<(), Error>

// Query support
pub fn get_supported_assets(anchor: Address) -> Option<Vec<String>>
pub fn is_asset_supported(anchor: Address, asset: String) -> bool

// Validate
pub fn validate_asset_pair(
    anchor: Address,
    base_asset: String,
    quote_asset: String,
) -> Result<(), Error>

// Submit with validation
pub fn submit_quote_validated(...) -> Result<u64, Error>
```

## Error Codes

- `Error::UnsupportedAsset` (48) - Asset not in supported list
- `Error::AssetNotConfigured` (49) - No assets configured for anchor

## Configuration Structure

```rust
pub struct AssetConfig {
    pub anchor: Address,
    pub supported_assets: Vec<String>,
}
```

## Example Flow

```rust
// 1. Configure supported assets
let assets = vec![
    &env,
    String::from_str(&env, "USD"),
    String::from_str(&env, "USDC"),
];
client.set_supported_assets(&anchor, &assets);

// 2. Validate before operation
client.validate_asset_pair(
    &anchor,
    &String::from_str(&env, "USD"),
    &String::from_str(&env, "USDC"),
)?;

// 3. Or use automatic validation
let quote_id = client.submit_quote_validated(
    &anchor,
    &String::from_str(&env, "USD"),
    &String::from_str(&env, "USDC"),
    &10000,
    &100,
    &100,
    &10000,
    &valid_until,
);
```

## Benefits

### Early Rejection
Fail fast before expensive operations.

### Clear Errors
Specific error codes for debugging:
- `UnsupportedAsset` - Know exactly what's wrong
- `AssetNotConfigured` - Know configuration is missing

### Prevent Invalid Flows
Stop invalid transactions before they start.

### Better UX
Users get immediate feedback on unsupported assets.

## Storage

Asset configurations stored in persistent storage with 90-day TTL.

## Best Practices

1. **Configure on registration** - Set assets when registering anchor
2. **Validate early** - Check before expensive operations
3. **Use validated methods** - Prefer `submit_quote_validated` over manual checks
4. **Update as needed** - Reconfigure when asset support changes
5. **Handle errors clearly** - Provide user-friendly messages for unsupported assets
