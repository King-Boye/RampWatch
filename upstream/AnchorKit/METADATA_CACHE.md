# Metadata Cache

## Overview

Cache anchor metadata and TOML capabilities to avoid repeated discovery calls.

## Features

- ✅ TTL-based cache expiration
- ✅ Manual refresh (invalidation)
- ✅ Separate caches for metadata and capabilities
- ✅ Automatic cleanup via TTL

## Usage

### Cache Metadata

```rust
let metadata = AnchorMetadata {
    anchor: anchor.clone(),
    reputation_score: 9000,
    average_settlement_time: 300,
    liquidity_score: 8500,
    uptime_percentage: 9900,
    total_volume: 1000000,
    is_active: true,
};

// Cache for 1 hour (3600 seconds)
client.cache_metadata(&anchor, &metadata, &3600);
```

### Retrieve Cached Metadata

```rust
match client.try_get_cached_metadata(&anchor) {
    Ok(metadata) => {
        // Use cached metadata
    }
    Err(Ok(Error::CacheExpired)) => {
        // Fetch fresh data
    }
    Err(Ok(Error::CacheNotFound)) => {
        // No cache exists
    }
}
```

### Manual Refresh

```rust
// Invalidate cache to force fresh fetch
client.refresh_metadata_cache(&anchor);
```

### Cache Capabilities

```rust
let toml_url = String::from_str(&env, "https://anchor.example/.well-known/stellar.toml");
let capabilities = String::from_str(&env, r#"{"deposits":true,"withdrawals":true}"#);

// Cache for 30 minutes (1800 seconds)
client.cache_capabilities(&anchor, &toml_url, &capabilities, &1800);
```

### Retrieve Cached Capabilities

```rust
let cached = client.get_cached_capabilities(&anchor);
println!("TOML URL: {}", cached.toml_url);
println!("Capabilities: {}", cached.capabilities);
```

## API Methods

```rust
// Metadata cache
pub fn cache_metadata(anchor: Address, metadata: AnchorMetadata, ttl_seconds: u64) -> Result<(), Error>
pub fn get_cached_metadata(anchor: Address) -> Result<AnchorMetadata, Error>
pub fn refresh_metadata_cache(anchor: Address) -> Result<(), Error>

// Capabilities cache
pub fn cache_capabilities(anchor: Address, toml_url: String, capabilities: String, ttl_seconds: u64) -> Result<(), Error>
pub fn get_cached_capabilities(anchor: Address) -> Result<CachedCapabilities, Error>
pub fn refresh_capabilities_cache(anchor: Address) -> Result<(), Error>
```

## TTL Recommendations

- **Metadata**: 1-24 hours (3600-86400 seconds)
- **Capabilities**: 30 minutes - 6 hours (1800-21600 seconds)
- **High-frequency updates**: 5-15 minutes (300-900 seconds)

## Error Handling

- `Error::CacheExpired` (48) - Cache exists but TTL expired
- `Error::CacheNotFound` (49) - No cache entry exists

## Storage

Uses temporary storage with automatic TTL management:
- Lightweight and efficient
- No manual cleanup needed
- Per-anchor isolation

## Best Practices

1. **Set appropriate TTLs** based on data volatility
2. **Handle cache misses** gracefully
3. **Refresh on critical updates** using manual refresh
4. **Monitor cache hit rates** to optimize TTLs
