# Contract Pause Mechanism

## Overview

Emergency pause feature that allows the admin to temporarily halt settlement operations.

## Implementation

### Storage
- **Key**: `Paused` (instance storage)
- **Type**: `bool`
- **Default**: `false`

### Functions

#### `pause()`
- **Access**: Admin only
- **Effect**: Sets contract to paused state
- **Event**: Emits `paused` event

#### `unpause()`
- **Access**: Admin only
- **Effect**: Resumes normal operations
- **Event**: Emits `unpaused` event

#### `is_paused()`
- **Access**: Public query
- **Returns**: Current pause status

### Protected Operations

Only `confirm_payout()` is blocked when paused. Other operations remain available:
- ✅ `create_remittance()` - Users can still create remittances
- ❌ `confirm_payout()` - Blocked (returns `ContractPaused` error)
- ✅ `cancel_remittance()` - Users can still cancel
- ✅ `withdraw_fees()` - Admin can still withdraw fees
- ✅ Agent registration/removal - Admin operations continue

### Error Code

- **ContractPaused** = 13 - Returned when attempting settlement while paused

### Events

```rust
// Pause activated
("admin", "paused") -> (schema_version, sequence, timestamp, admin)

// Pause deactivated
("admin", "unpaused") -> (schema_version, sequence, timestamp, admin)
```

## Usage

### Pause Contract
```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source admin \
  --network testnet \
  -- pause
```

### Unpause Contract
```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source admin \
  --network testnet \
  -- unpause
```

### Check Status
```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --network testnet \
  -- is_paused
```

## Test Coverage

- ✅ `test_pause_unpause` - Basic pause/unpause functionality
- ✅ `test_settlement_blocked_when_paused` - Confirms settlements fail when paused
- ✅ `test_settlement_works_after_unpause` - Verifies normal operation after unpause

## Security Considerations

1. **Admin-only control** - Only the admin address can pause/unpause
2. **Non-destructive** - Pause doesn't affect existing data or balances
3. **Selective blocking** - Only settlements are blocked, allowing users to cancel if needed
4. **Event transparency** - All pause state changes are logged on-chain
