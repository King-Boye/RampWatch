# Migration Quick Reference

## One-Line Summary

Secure, trustless contract state migration with cryptographic verification and zero trust assumptions.

## Quick Start

### Full Migration (< 100 remittances)

```rust
// 1. Export
let snapshot = old_contract.export_migration_state(&admin)?;

// 2. Verify
let verification = old_contract.verify_migration_snapshot(snapshot.clone())?;
assert!(verification.valid);

// 3. Import
new_contract.import_migration_state(&admin, snapshot)?;
```

### Batch Migration (> 100 remittances)

```rust
// 1. Export batches
let batch_size = 50;
for batch_num in 0..total_batches {
    let batch = old_contract.export_migration_batch(&admin, batch_num, batch_size)?;
    batches.push(batch);
}

// 2. Import batches
for batch in batches {
    new_contract.import_migration_batch(&admin, batch)?;
}
```

## Functions

| Function | Purpose | Auth Required |
|----------|---------|---------------|
| `export_migration_state` | Export complete state | Admin |
| `import_migration_state` | Import complete state | Admin |
| `verify_migration_snapshot` | Verify integrity | None |
| `export_migration_batch` | Export batch | Admin |
| `import_migration_batch` | Import batch | Admin |

## Data Types

```rust
MigrationSnapshot {
    version: u32,
    timestamp: u64,
    ledger_sequence: u32,
    instance_data: InstanceData,
    persistent_data: PersistentData,
    verification_hash: BytesN<32>,
}

InstanceData {
    admin, usdc_token, platform_fee_bps,
    remittance_counter, accumulated_fees,
    paused, admin_count
}

PersistentData {
    remittances, agents, admin_roles,
    settlement_hashes, whitelisted_tokens
}
```

## Error Codes

| Code | Error | Fix |
|------|-------|-----|
| 1 | AlreadyInitialized | Deploy fresh contract |
| 3 | InvalidAmount | Use 1-100 batch size |
| 14 | Unauthorized | Use admin address |
| 20 | InvalidMigrationHash | Re-export snapshot |

## Security Features

✅ **SHA-256 Hash** - Cryptographic verification
✅ **Deterministic** - Same data = same hash
✅ **Atomic** - All or nothing
✅ **Replay Protection** - Prevents duplicate imports
✅ **No Trust** - Cryptographically verified

## Verification

```rust
// Before import
let verification = contract.verify_migration_snapshot(snapshot.clone())?;
if !verification.valid {
    panic!("Verification failed!");
}

// After import
assert_eq!(
    old_contract.get_remittance_counter()?,
    new_contract.get_remittance_counter()?
);
```

## Best Practices

1. **Always verify** before importing
2. **Pause old contract** during migration
3. **Test on testnet** first
4. **Use batches** for > 100 remittances
5. **Document migration** with records

## Common Patterns

### Pattern 1: Safe Migration

```rust
// Pause
old_contract.pause(&admin)?;

// Export & verify
let snapshot = old_contract.export_migration_state(&admin)?;
let verification = old_contract.verify_migration_snapshot(snapshot.clone())?;
assert!(verification.valid);

// Import
new_contract.import_migration_state(&admin, snapshot)?;

// Verify
verify_migration_success(&old_contract, &new_contract)?;
```

### Pattern 2: Batch Migration

```rust
let batch_size = 50;
let total = (remittance_count + batch_size - 1) / batch_size;

for i in 0..total {
    let batch = old_contract.export_migration_batch(&admin, i, batch_size)?;
    new_contract.import_migration_batch(&admin, batch)?;
}
```

### Pattern 3: Verification

```rust
fn verify_migration_success(
    old: &Contract,
    new: &Contract,
) -> Result<(), Error> {
    assert_eq!(old.get_remittance_counter()?, new.get_remittance_counter()?);
    assert_eq!(old.get_accumulated_fees()?, new.get_accumulated_fees()?);
    assert_eq!(old.get_platform_fee_bps()?, new.get_platform_fee_bps()?);
    Ok(())
}
```

## JavaScript Example

```javascript
// Export
const snapshot = await oldContract.export_migration_state({ caller: admin });

// Verify
const verification = await oldContract.verify_migration_snapshot({ snapshot });
if (!verification.valid) throw new Error('Verification failed');

// Import
await newContract.import_migration_state({ caller: admin, snapshot });
```

## Troubleshooting

| Issue | Cause | Solution |
|-------|-------|----------|
| Hash mismatch | Data tampering | Re-export snapshot |
| Already initialized | Contract has data | Deploy fresh contract |
| Unauthorized | Not admin | Use admin address |
| Batch too large | Size > 100 | Use smaller batch size |

## Performance

| Operation | Gas Cost | Notes |
|-----------|----------|-------|
| Export (100) | ~500k | Scales with data |
| Import (100) | ~800k | Includes verification |
| Verify | ~100k | Read-only |
| Batch export (50) | ~250k | Per batch |
| Batch import (50) | ~400k | Per batch |

## Checklist

### Pre-Migration
- [ ] Test on testnet
- [ ] Pause old contract
- [ ] Export state
- [ ] Verify snapshot

### Migration
- [ ] Deploy new contract
- [ ] Import state
- [ ] Verify import
- [ ] Test functions

### Post-Migration
- [ ] Announce new address
- [ ] Update docs
- [ ] Monitor new contract
- [ ] Archive records

## Constants

```rust
MAX_MIGRATION_BATCH_SIZE = 100
```

## What's Preserved

✅ All remittances (amounts, fees, status)
✅ All balances (accumulated fees)
✅ All configuration (admin, token, fee %)
✅ All counters (remittance, admin)
✅ All relationships (agents, admins)
✅ All state (pause, whitelisted tokens)

## What's Verified

✅ Completeness (all data included)
✅ Integrity (no tampering)
✅ Authenticity (hash matches)
✅ Consistency (deterministic encoding)

## Documentation

- Full guide: [MIGRATION.md](MIGRATION.md)
- API reference: [MIGRATION_API.md](MIGRATION_API.md)
- Quick reference: This document

---

**Version**: 1.0.0
**Last Updated**: 2026-02-20
