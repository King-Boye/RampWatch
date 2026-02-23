# Settlement Timestamp - Quick Reference

## What Was Added

A ledger timestamp is now captured and stored when a settlement is created (confirmed) to improve traceability and support audit requirements.

## Key Changes

### Storage (`src/storage.rs`)
- **New DataKey**: `SettlementTimestamp(u64)` - stores timestamp per remittance ID
- **New Functions**:
  - `set_settlement_timestamp(env, remittance_id, timestamp)` - stores timestamp
  - `get_settlement_timestamp(env, remittance_id) -> Option<u64>` - retrieves timestamp
  - `has_settlement_timestamp(env, remittance_id) -> bool` - checks existence

### Contract Interface (`src/lib.rs`)
- **Modified**: `confirm_payout` now captures and stores ledger timestamp
- **New Public Function**: `get_settlement_timestamp(env, remittance_id) -> Option<u64>`

## Usage

### Query Settlement Timestamp
```rust
// Returns Some(timestamp) if settlement exists, None otherwise
let timestamp = contract.get_settlement_timestamp(&env, remittance_id);
```

### When Timestamp is Captured
Timestamp is automatically captured during `confirm_payout` after:
1. Settlement is marked as executed
2. Status is updated to Settled
3. Before events are emitted

## Benefits

- **Audit Trails**: Exact timestamp for compliance and forensic analysis
- **Analytics**: Time-based settlement metrics and reporting
- **Traceability**: Immutable record of settlement creation time
- **Compliance**: Supports regulatory reporting requirements

## Backward Compatibility

- Old settlements without timestamps return `None`
- No migration required
- No breaking changes to existing APIs
- Fully backward compatible

## Storage Details

- **Type**: Persistent storage (long-term retention)
- **Size**: 8 bytes per settlement (u64 timestamp)
- **Indexed by**: Remittance ID
- **Immutable**: Once set, cannot be changed

## Example Integration

```rust
// After settlement confirmation
let remittance_id = 123;
let timestamp = contract.get_settlement_timestamp(&env, remittance_id);

match timestamp {
    Some(ts) => {
        // Settlement exists, use timestamp
        println!("Settlement created at: {}", ts);
        // Convert to human-readable format if needed
        // Use for audit logs, reports, analytics
    }
    None => {
        // Settlement not yet created or old data
        println!("No settlement timestamp available");
    }
}
```

## Testing

Verify the feature works by:
1. Creating a remittance
2. Confirming payout (settlement)
3. Querying the timestamp
4. Verifying it matches the ledger time

## Notes

- Timestamp is Unix seconds (u64)
- Captured from `env.ledger().timestamp()` (trusted source)
- Stored in persistent storage for durability
- No performance impact on existing operations
