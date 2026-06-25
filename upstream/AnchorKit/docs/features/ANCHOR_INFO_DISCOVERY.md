# Anchor Info Discovery Service

The Anchor Info Discovery Service provides functionality to fetch, parse, and cache Stellar anchor metadata from `.well-known/stellar.toml` files.

## Features

- **Fetch stellar.toml**: Retrieve anchor metadata from standard Stellar TOML endpoints
- **Parse metadata**: Extract supported assets, fees, limits, and service endpoints
- **Cache with TTL**: Store parsed data with configurable time-to-live
- **Query capabilities**: Check asset support, fees, and limits programmatically

## Data Structures

### StellarToml

Complete representation of a stellar.toml file:

```rust
pub struct StellarToml {
    pub version: String,
    pub network_passphrase: String,
    pub accounts: Vec<String>,
    pub signing_key: String,
    pub currencies: Vec<AssetInfo>,
    pub transfer_server: String,
    pub transfer_server_sep0024: String,
    pub kyc_server: String,
    pub web_auth_endpoint: String,
}
```

### AssetInfo

Detailed information about a supported asset:

```rust
pub struct AssetInfo {
    pub code: String,
    pub issuer: String,
    pub deposit_enabled: bool,
    pub withdrawal_enabled: bool,
    pub deposit_fee_fixed: u64,
    pub deposit_fee_percent: u32,
    pub withdrawal_fee_fixed: u64,
    pub withdrawal_fee_percent: u32,
    pub deposit_min_amount: u64,
    pub deposit_max_amount: u64,
    pub withdrawal_min_amount: u64,
    pub withdrawal_max_amount: u64,
}
```

## Contract Methods

### Fetch and Cache

```rust
pub fn fetch_anchor_info(
    env: Env,
    anchor: Address,
    domain: String,
    ttl_seconds: Option<u64>,
) -> Result<StellarToml, Error>
```

Fetches stellar.toml from the specified domain and caches it. Requires admin authorization.

**Parameters:**
- `anchor`: Address of the anchor
- `domain`: Domain to fetch from (e.g., "example.com")
- `ttl_seconds`: Optional cache TTL (default: 3600 seconds)

**Example:**
```rust
let toml = contract.fetch_anchor_info(
    &anchor_addr,
    String::from_str(&env, "anchor.example.com"),
    Some(7200)
)?;
```

### Get Cached TOML

```rust
pub fn get_anchor_toml(
    env: Env,
    anchor: Address
) -> Result<StellarToml, Error>
```

Retrieves cached stellar.toml for an anchor.

**Example:**
```rust
let toml = contract.get_anchor_toml(&anchor_addr)?;
println!("Version: {}", toml.version);
```

### Refresh Cache

```rust
pub fn refresh_anchor_info(
    env: Env,
    anchor: Address,
    domain: String
) -> Result<StellarToml, Error>
```

Manually refreshes cached data. Requires admin authorization.

### Query Supported Assets

```rust
pub fn get_anchor_assets(
    env: Env,
    anchor: Address
) -> Result<Vec<String>, Error>
```

Returns list of asset codes supported by the anchor.

**Example:**
```rust
let assets = contract.get_anchor_assets(&anchor_addr)?;
// Returns: ["USDC", "XLM", "BTC"]
```

### Get Asset Details

```rust
pub fn get_anchor_asset_info(
    env: Env,
    anchor: Address,
    asset_code: String
) -> Result<AssetInfo, Error>
```

Retrieves complete information about a specific asset.

**Example:**
```rust
let usdc = String::from_str(&env, "USDC");
let info = contract.get_anchor_asset_info(&anchor_addr, usdc)?;
println!("Issuer: {}", info.issuer);
println!("Deposit enabled: {}", info.deposit_enabled);
```

### Query Limits

```rust
// Deposit limits
pub fn get_anchor_deposit_limits(
    env: Env,
    anchor: Address,
    asset_code: String
) -> Result<(u64, u64), Error>

// Withdrawal limits
pub fn get_anchor_withdrawal_limits(
    env: Env,
    anchor: Address,
    asset_code: String
) -> Result<(u64, u64), Error>
```

Returns (min, max) limits for deposits or withdrawals.

**Example:**
```rust
let usdc = String::from_str(&env, "USDC");
let (min, max) = contract.get_anchor_deposit_limits(&anchor_addr, usdc)?;
println!("Deposit range: {} - {}", min, max);
```

### Query Fees

```rust
// Deposit fees
pub fn get_anchor_deposit_fees(
    env: Env,
    anchor: Address,
    asset_code: String
) -> Result<(u64, u32), Error>

// Withdrawal fees
pub fn get_anchor_withdrawal_fees(
    env: Env,
    anchor: Address,
    asset_code: String
) -> Result<(u64, u32), Error>
```

Returns (fixed_fee, percent_fee) for deposits or withdrawals.

**Example:**
```rust
let usdc = String::from_str(&env, "USDC");
let (fixed, percent) = contract.get_anchor_deposit_fees(&anchor_addr, usdc)?;
println!("Fee: {} + {}%", fixed, percent);
```

### Check Service Support

```rust
// Check deposit support
pub fn anchor_supports_deposits(
    env: Env,
    anchor: Address,
    asset_code: String
) -> Result<bool, Error>

// Check withdrawal support
pub fn anchor_supports_withdrawals(
    env: Env,
    anchor: Address,
    asset_code: String
) -> Result<bool, Error>
```

**Example:**
```rust
let usdc = String::from_str(&env, "USDC");
if contract.anchor_supports_deposits(&anchor_addr, usdc.clone())? {
    println!("Deposits supported for USDC");
}
```

## Usage Examples

### Complete Workflow

```rust
use soroban_sdk::{Env, String};

// 1. Fetch and cache anchor info
let domain = String::from_str(&env, "anchor.example.com");
let toml = contract.fetch_anchor_info(&anchor, domain, None)?;

// 2. List supported assets
let assets = contract.get_anchor_assets(&anchor)?;
for asset in assets.iter() {
    println!("Supported: {}", asset);
}

// 3. Check specific asset details
let usdc = String::from_str(&env, "USDC");
let info = contract.get_anchor_asset_info(&anchor, usdc.clone())?;

// 4. Validate transaction parameters
let (min, max) = contract.get_anchor_deposit_limits(&anchor, usdc.clone())?;
let amount = 5000;
if amount >= min && amount <= max {
    // Proceed with deposit
}

// 5. Calculate fees
let (fixed, percent) = contract.get_anchor_deposit_fees(&anchor, usdc)?;
let total_fee = fixed + (amount * percent / 10000);
```

### Integration with Existing Features

```rust
// Combine with service configuration
let services = contract.get_supported_services(&anchor)?;
if services.contains(&ServiceType::Deposits) {
    let assets = contract.get_anchor_assets(&anchor)?;
    // Process deposits for supported assets
}

// Use with rate comparison
let usdc = String::from_str(&env, "USDC");
let (fee_fixed, fee_percent) = contract.get_anchor_deposit_fees(&anchor, usdc)?;
// Compare fees across multiple anchors
```

## Cache Management

### Default TTL

The default cache TTL is 3600 seconds (1 hour). This can be customized per fetch:

```rust
// Cache for 2 hours
contract.fetch_anchor_info(&anchor, domain, Some(7200))?;

// Cache for 30 minutes
contract.fetch_anchor_info(&anchor, domain, Some(1800))?;
```

### Manual Refresh

Force a cache refresh when anchor metadata changes:

```rust
let domain = String::from_str(&env, "anchor.example.com");
contract.refresh_anchor_info(&anchor, domain)?;
```

### Cache Expiration

When cache expires, queries return `Error::CacheExpired`. Handle this by refreshing:

```rust
match contract.get_anchor_toml(&anchor) {
    Ok(toml) => {
        // Use cached data
    }
    Err(Error::CacheExpired) => {
        // Refresh cache
        let domain = String::from_str(&env, "anchor.example.com");
        let toml = contract.refresh_anchor_info(&anchor, domain)?;
    }
    Err(e) => return Err(e),
}
```

## Error Handling

### Common Errors

- `Error::CacheNotFound`: No cached data for anchor (call `fetch_anchor_info` first)
- `Error::CacheExpired`: Cached data expired (call `refresh_anchor_info`)
- `Error::UnsupportedAsset`: Asset not found in anchor's supported list
- `Error::NotInitialized`: Contract not initialized
- `Error::UnauthorizedAttestor`: Caller not authorized (for admin-only methods)

### Error Handling Pattern

```rust
use crate::errors::Error;

fn process_anchor_asset(
    contract: &AnchorKitContract,
    anchor: &Address,
    asset_code: String,
) -> Result<(), Error> {
    // Try to get asset info
    let info = match contract.get_anchor_asset_info(anchor, asset_code.clone()) {
        Ok(info) => info,
        Err(Error::CacheNotFound) => {
            // Fetch and cache
            let domain = String::from_str(&env, "anchor.example.com");
            contract.fetch_anchor_info(anchor, domain, None)?;
            contract.get_anchor_asset_info(anchor, asset_code)?
        }
        Err(Error::CacheExpired) => {
            // Refresh cache
            let domain = String::from_str(&env, "anchor.example.com");
            contract.refresh_anchor_info(anchor, domain)?;
            contract.get_anchor_asset_info(anchor, asset_code)?
        }
        Err(e) => return Err(e),
    };

    // Process asset info
    Ok(())
}
```

## Testing

The service includes comprehensive tests covering:

- ✅ Fetch and cache operations
- ✅ Cache retrieval and expiration
- ✅ Asset queries and filtering
- ✅ Limit and fee queries
- ✅ Service support checks
- ✅ Multiple anchor support
- ✅ Custom TTL handling
- ✅ Error conditions

Run tests:

```bash
cargo test anchor_info_discovery
```

## Production Considerations

### HTTP Integration

The current implementation uses mock data for testing. In production, replace `mock_fetch_toml` with actual HTTP client:

```rust
fn fetch_toml(env: &Env, domain: &String) -> Result<StellarToml, Error> {
    let url = format!("https://{}/.well-known/stellar.toml", domain);
    // Use HTTP client to fetch and parse TOML
    // Parse TOML content into StellarToml struct
}
```

### TOML Parsing

Integrate a TOML parser compatible with Soroban:

```rust
use toml_parser::parse;

fn parse_stellar_toml(content: &str) -> Result<StellarToml, Error> {
    let parsed = parse(content)?;
    // Map TOML fields to StellarToml struct
}
```

### Rate Limiting

Consider rate limiting TOML fetches to avoid overwhelming anchor servers:

```rust
// Check last fetch time
if last_fetch_time + MIN_FETCH_INTERVAL > current_time {
    return Err(Error::RateLimitExceeded);
}
```

### Validation

Add validation for fetched data:

```rust
fn validate_toml(toml: &StellarToml) -> Result<(), Error> {
    // Validate version
    if toml.version.is_empty() {
        return Err(Error::InvalidAnchorMetadata);
    }
    
    // Validate URLs
    for url in [&toml.transfer_server, &toml.kyc_server] {
        if !is_valid_url(url) {
            return Err(Error::InvalidEndpointFormat);
        }
    }
    
    Ok(())
}
```

## Integration with Other Features

### With Health Monitoring

```rust
// Check if anchor is healthy before fetching
let health = contract.get_health_status(&anchor)?;
if health.is_active {
    let toml = contract.fetch_anchor_info(&anchor, domain, None)?;
}
```

### With Asset Validator

```rust
// Sync supported assets with asset validator
let assets = contract.get_anchor_assets(&anchor)?;
contract.set_supported_assets(&anchor, assets)?;
```

### With Rate Comparison

```rust
// Use fee data for rate comparison
let usdc = String::from_str(&env, "USDC");
let (fixed, percent) = contract.get_anchor_deposit_fees(&anchor, usdc)?;
// Factor fees into rate comparison logic
```

## API Summary

| Method | Auth Required | Returns | Purpose |
|--------|---------------|---------|---------|
| `fetch_anchor_info` | Admin | `StellarToml` | Fetch and cache TOML |
| `get_anchor_toml` | None | `StellarToml` | Get cached TOML |
| `refresh_anchor_info` | Admin | `StellarToml` | Refresh cache |
| `get_anchor_assets` | None | `Vec<String>` | List assets |
| `get_anchor_asset_info` | None | `AssetInfo` | Asset details |
| `get_anchor_deposit_limits` | None | `(u64, u64)` | Deposit min/max |
| `get_anchor_withdrawal_limits` | None | `(u64, u64)` | Withdrawal min/max |
| `get_anchor_deposit_fees` | None | `(u64, u32)` | Deposit fees |
| `get_anchor_withdrawal_fees` | None | `(u64, u32)` | Withdrawal fees |
| `anchor_supports_deposits` | None | `bool` | Check deposit support |
| `anchor_supports_withdrawals` | None | `bool` | Check withdrawal support |

## Performance

- **Cache storage**: Uses Soroban temporary storage with TTL
- **Query complexity**: O(1) for cache lookups, O(n) for asset searches
- **Memory usage**: Proportional to number of supported assets
- **Network calls**: Only on initial fetch and manual refresh

## Security

- **Admin-only operations**: Fetch and refresh require admin authorization
- **Input validation**: Domain and asset codes validated
- **Cache isolation**: Each anchor has separate cache entry
- **TTL enforcement**: Automatic expiration prevents stale data

## Future Enhancements

- [ ] Real HTTP client integration
- [ ] TOML parser integration
- [ ] Signature verification for TOML files
- [ ] Multi-domain fallback support
- [ ] Automatic cache refresh on expiration
- [ ] Event emission for cache updates
- [ ] Batch asset queries
- [ ] Asset search and filtering
