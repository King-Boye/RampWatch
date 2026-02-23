# Daily Send Limit Implementation

## Overview
This document describes the daily send-limit enforcement feature for transfers in the SwiftRemit contract. The feature tracks and aggregates each user's transfer amounts within a rolling 24-hour window and enforces configurable limits per currency and country.

## Implementation Details

### 1. New Error Type
- **Error Code**: `DailySendLimitExceeded` (Error #14)
- **Description**: Returned when a user's total transfers in the last 24 hours would exceed the configured limit
- **Location**: `src/errors.rs`

### 2. New Data Types

#### DailyLimit
```rust
pub struct DailyLimit {
    pub currency: String,
    pub country: String,
    pub limit: i128,
}
```
Stores the configured daily limit for a specific currency-country combination.

#### TransferRecord
```rust
pub struct TransferRecord {
    pub timestamp: u64,
    pub amount: i128,
}
```
Tracks individual transfer amounts with timestamps for rolling window calculation.

**Location**: `src/types.rs`

### 3. Storage Functions

#### Daily Limit Configuration
- `set_daily_limit(env, currency, country, limit)` - Admin function to configure limits
- `get_daily_limit(env, currency, country)` - Retrieves configured limit

#### Transfer Tracking
- `get_user_transfers(env, user)` - Retrieves user's transfer history
- `set_user_transfers(env, user, transfers)` - Updates user's transfer history

**Storage Keys**:
- `DailyLimit(String, String)` - Persistent storage for limits by currency and country
- `UserTransfers(Address)` - Persistent storage for user transfer records

**Location**: `src/storage.rs`

### 4. Validation Logic

#### validate_daily_send_limit()
**Location**: `src/validation.rs`

**Algorithm**:
1. Retrieve configured daily limit for the currency-country pair
2. If no limit is configured, allow the transfer (no enforcement)
3. Calculate cutoff time (current time - 24 hours)
4. Filter user's transfer history to only include transfers within the rolling window
5. Calculate total amount sent in the last 24 hours
6. Check if new transfer would exceed the limit
7. If within limit:
   - Record the new transfer with current timestamp
   - Clean up old transfers outside the window
   - Update storage
8. If exceeds limit, return `DailySendLimitExceeded` error

**Rolling Window**: Uses a 24-hour (86,400 seconds) rolling window, automatically cleaning up old transfer records.

### 5. Integration Points

#### create_remittance()
**Location**: `src/lib.rs`

The daily limit validation is called early in the `create_remittance` function, before any token transfers occur:

```rust
pub fn create_remittance(
    env: Env,
    sender: Address,
    agent: Address,
    amount: i128,
    currency: String,  // NEW PARAMETER
    country: String,   // NEW PARAMETER
    expiry: Option<u64>,
) -> Result<u64, ContractError>
```

**Validation Order**:
1. Sender authentication
2. Amount validation (> 0)
3. Agent registration check
4. **Daily send limit validation** ← NEW
5. Fee calculation
6. Token transfer
7. Remittance creation

#### Admin Functions

**set_daily_limit()**
```rust
pub fn set_daily_limit(
    env: Env,
    currency: String,
    country: String,
    limit: i128,
) -> Result<(), ContractError>
```
- Requires admin authentication
- Validates limit is non-negative
- Configures daily limit for specific currency-country pair

**get_daily_limit()**
```rust
pub fn get_daily_limit(
    env: Env,
    currency: String,
    country: String,
) -> Option<DailyLimit>
```
- Public read function
- Returns configured limit or None if not set

## Usage Examples

### Setting Daily Limits
```rust
// Set $10,000 daily limit for USD transfers to US
contract.set_daily_limit(
    &String::from_str(&env, "USD"),
    &String::from_str(&env, "US"),
    &10000_0000000  // Amount in stroops (7 decimals)
);

// Set €15,000 daily limit for EUR transfers to UK
contract.set_daily_limit(
    &String::from_str(&env, "EUR"),
    &String::from_str(&env, "UK"),
    &15000_0000000
);
```

### Creating Transfers with Limits
```rust
// This will check against the USD-US daily limit
let remittance_id = contract.create_remittance(
    &sender,
    &agent,
    &5000_0000000,  // $5,000
    &String::from_str(&env, "USD"),
    &String::from_str(&env, "US"),
    &None
);
```

## Key Features

### 1. Rolling 24-Hour Window
- Transfers are tracked with timestamps
- Only transfers within the last 24 hours count toward the limit
- Old transfers automatically expire and don't affect future transfers

### 2. Per-User Enforcement
- Each user has their own independent limit
- User A's transfers don't affect User B's available limit

### 3. Configurable by Currency and Country
- Different limits can be set for different currency-country combinations
- Example: USD-US can have a $10,000 limit while USD-UK has a $15,000 limit

### 4. Optional Enforcement
- If no limit is configured for a currency-country pair, transfers are not restricted
- Allows gradual rollout of limits

### 5. Automatic Cleanup
- Transfer records outside the 24-hour window are automatically removed
- Prevents unbounded storage growth

## Testing

Comprehensive test coverage includes:

1. **Basic Functionality**
   - Setting and retrieving daily limits
   - Transfers within limits succeed
   - Transfers exceeding limits fail

2. **Rolling Window**
   - Old transfers expire after 24 hours
   - New transfers can be made after old ones expire

3. **Multi-Currency/Country**
   - Different limits for different currencies
   - Different limits for different countries
   - Limits are independent

4. **Edge Cases**
   - No limit configured (unlimited transfers)
   - Multiple users with same limits
   - Exact limit amount transfers
   - Negative limit validation

**Test Location**: `src/test.rs` (tests prefixed with `test_daily_limit_*`)

## Security Considerations

1. **Admin-Only Configuration**: Only the contract admin can set daily limits
2. **Overflow Protection**: All arithmetic operations use checked math
3. **Atomic Updates**: Transfer records are updated atomically with validation
4. **No Retroactive Changes**: Changing limits doesn't affect already-recorded transfers

## Performance Considerations

1. **Storage Efficiency**: Only transfers within 24 hours are stored
2. **Automatic Cleanup**: Old records are removed during validation
3. **Per-User Storage**: Transfer records are isolated per user
4. **Optional Feature**: No overhead when limits are not configured

## Migration Notes

### Breaking Changes
The `create_remittance` function signature has changed:

**Before**:
```rust
pub fn create_remittance(
    env: Env,
    sender: Address,
    agent: Address,
    amount: i128,
    expiry: Option<u64>,
) -> Result<u64, ContractError>
```

**After**:
```rust
pub fn create_remittance(
    env: Env,
    sender: Address,
    agent: Address,
    amount: i128,
    currency: String,  // NEW
    country: String,   // NEW
    expiry: Option<u64>,
) -> Result<u64, ContractError>
```

### Client Updates Required
All clients calling `create_remittance` must be updated to provide `currency` and `country` parameters.

### Backward Compatibility
- Existing remittances are not affected
- No limits are enforced until explicitly configured by admin
- Other contract functions remain unchanged

## Future Enhancements

Potential improvements for future versions:

1. **Receive Limits**: Track and limit received amounts
2. **Time-Based Limits**: Different limits for different times of day
3. **Velocity Limits**: Limit number of transactions per time period
4. **Tiered Limits**: Different limits based on user verification level
5. **Limit History**: Track limit changes over time
6. **Notifications**: Event emission when approaching limits
