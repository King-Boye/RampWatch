# API Changes - Daily Send Limits

## Summary
This document outlines the API changes introduced by the daily send-limit feature.

## Modified Functions

### create_remittance()

**Status**: BREAKING CHANGE

**Old Signature**:
```rust
pub fn create_remittance(
    env: Env,
    sender: Address,
    agent: Address,
    amount: i128,
    expiry: Option<u64>,
) -> Result<u64, ContractError>
```

**New Signature**:
```rust
pub fn create_remittance(
    env: Env,
    sender: Address,
    agent: Address,
    amount: i128,
    currency: String,  // NEW REQUIRED PARAMETER
    country: String,   // NEW REQUIRED PARAMETER
    expiry: Option<u64>,
) -> Result<u64, ContractError>
```

**New Parameters**:
- `currency`: String - Currency code (e.g., "USD", "EUR", "GBP")
- `country`: String - Country code (e.g., "US", "UK", "CA")

**New Error**:
- `DailySendLimitExceeded` (Error #14) - Returned when transfer would exceed daily limit

**Example Usage**:
```rust
// JavaScript/TypeScript client
const remittanceId = await contract.create_remittance({
    sender: senderAddress,
    agent: agentAddress,
    amount: 1000_0000000n,  // $1,000 in stroops
    currency: "USD",
    country: "US",
    expiry: null
});
```

## New Functions

### set_daily_limit()

**Status**: NEW

**Signature**:
```rust
pub fn set_daily_limit(
    env: Env,
    currency: String,
    country: String,
    limit: i128,
) -> Result<(), ContractError>
```

**Description**: Admin-only function to configure daily send limits for a currency-country pair.

**Parameters**:
- `currency`: String - Currency code
- `country`: String - Country code
- `limit`: i128 - Maximum amount that can be sent in 24 hours (in stroops)

**Authorization**: Requires admin authentication

**Errors**:
- `InvalidAmount` (Error #3) - If limit is negative
- `NotInitialized` (Error #2) - If contract not initialized

**Example Usage**:
```rust
// Set $10,000 daily limit for USD-US
await contract.set_daily_limit({
    currency: "USD",
    country: "US",
    limit: 10000_0000000n  // $10,000 in stroops
});
```

### get_daily_limit()

**Status**: NEW

**Signature**:
```rust
pub fn get_daily_limit(
    env: Env,
    currency: String,
    country: String,
) -> Option<DailyLimit>
```

**Description**: Public function to query configured daily limits.

**Parameters**:
- `currency`: String - Currency code
- `country`: String - Country code

**Returns**:
- `Some(DailyLimit)` - If limit is configured
- `None` - If no limit is configured (unlimited)

**DailyLimit Structure**:
```rust
pub struct DailyLimit {
    pub currency: String,
    pub country: String,
    pub limit: i128,
}
```

**Example Usage**:
```rust
// Query daily limit
const limit = await contract.get_daily_limit({
    currency: "USD",
    country: "US"
});

if (limit) {
    console.log(`Daily limit: ${limit.limit}`);
} else {
    console.log("No limit configured");
}
```

## Error Codes

### New Error
- **Code**: 14
- **Name**: `DailySendLimitExceeded`
- **Description**: User's total transfers in the last 24 hours would exceed the configured limit
- **When**: Thrown during `create_remittance` if limit would be exceeded

### Existing Errors (Unchanged)
All other error codes remain the same.

## Client Migration Guide

### Step 1: Update create_remittance Calls

**Before**:
```javascript
const remittanceId = await contract.create_remittance({
    sender: senderAddress,
    agent: agentAddress,
    amount: 1000_0000000n,
    expiry: null
});
```

**After**:
```javascript
const remittanceId = await contract.create_remittance({
    sender: senderAddress,
    agent: agentAddress,
    amount: 1000_0000000n,
    currency: "USD",  // ADD THIS
    country: "US",    // ADD THIS
    expiry: null
});
```

### Step 2: Handle New Error

```javascript
try {
    const remittanceId = await contract.create_remittance({
        sender: senderAddress,
        agent: agentAddress,
        amount: 5000_0000000n,
        currency: "USD",
        country: "US",
        expiry: null
    });
} catch (error) {
    if (error.code === 14) {
        // Daily send limit exceeded
        console.error("You have exceeded your daily send limit. Please try again tomorrow.");
    } else {
        // Handle other errors
        console.error("Transfer failed:", error);
    }
}
```

### Step 3: (Optional) Query Limits

```javascript
// Check available limit before transfer
const limit = await contract.get_daily_limit({
    currency: "USD",
    country: "US"
});

if (limit) {
    console.log(`Your daily limit is: $${limit.limit / 10000000}`);
    // You could also track user's current usage and show remaining limit
}
```

## Admin Operations

### Setting Up Limits

```javascript
// Admin sets up daily limits for different currency-country pairs
await contract.set_daily_limit({
    currency: "USD",
    country: "US",
    limit: 10000_0000000n  // $10,000
});

await contract.set_daily_limit({
    currency: "EUR",
    country: "UK",
    limit: 15000_0000000n  // €15,000
});

await contract.set_daily_limit({
    currency: "GBP",
    country: "UK",
    limit: 12000_0000000n  // £12,000
});
```

### Updating Limits

```javascript
// Update existing limit
await contract.set_daily_limit({
    currency: "USD",
    country: "US",
    limit: 20000_0000000n  // Increase to $20,000
});
```

### Removing Limits

To effectively remove a limit, set it to a very high value:

```javascript
await contract.set_daily_limit({
    currency: "USD",
    country: "US",
    limit: 9223372036854775807n  // Max i128 value (effectively unlimited)
});
```

## Testing Considerations

### Unit Tests
Update all unit tests that call `create_remittance` to include currency and country parameters.

### Integration Tests
1. Test limit enforcement with multiple transfers
2. Test rolling window behavior (transfers after 24 hours)
3. Test different currency-country combinations
4. Test error handling for exceeded limits

### Example Test
```javascript
describe('Daily Send Limits', () => {
    it('should enforce daily limits', async () => {
        // Set $10,000 limit
        await contract.set_daily_limit({
            currency: "USD",
            country: "US",
            limit: 10000_0000000n
        });

        // First transfer of $6,000 should succeed
        await contract.create_remittance({
            sender: user,
            agent: agent,
            amount: 6000_0000000n,
            currency: "USD",
            country: "US",
            expiry: null
        });

        // Second transfer of $5,000 should fail (total $11,000 > $10,000)
        await expect(
            contract.create_remittance({
                sender: user,
                agent: agent,
                amount: 5000_0000000n,
                currency: "USD",
                country: "US",
                expiry: null
            })
        ).rejects.toThrow('Error(Contract, #14)');
    });
});
```

## Backward Compatibility

### Breaking Changes
- `create_remittance` function signature changed (requires currency and country)
- All existing clients must be updated

### Non-Breaking Changes
- New functions (`set_daily_limit`, `get_daily_limit`) are additive
- Existing remittances are not affected
- Other contract functions remain unchanged

### Migration Timeline
1. Deploy updated contract
2. Update all client applications
3. Configure daily limits as needed
4. Monitor for any issues

## Support

For questions or issues related to this API change, please refer to:
- `DAILY_SEND_LIMITS.md` - Detailed implementation documentation
- `ERROR_REFERENCE.md` - Error code reference
- `API.md` - Complete API documentation
