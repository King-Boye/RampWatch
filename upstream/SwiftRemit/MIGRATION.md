# Secure Contract Migration System

## Overview

The SwiftRemit migration system enables safe state transfer from a non-upgradable contract deployment to a new deployment without introducing trust assumptions. The system uses cryptographic verification to ensure data integrity, deterministic encoding for consistency, and replay protection to prevent duplicate imports.

## Problem Statement

Soroban smart contracts are non-upgradable by design. When bugs are discovered or new features are needed, a new contract must be deployed. However, migrating state (remittances, balances, configuration) from the old contract to the new one is challenging:

- **Trust**: How do users verify the migration was done correctly?
- **Integrity**: How to prevent tampering or partial transfers?
- **Completeness**: How to ensure all data was migrated?
- **Verification**: How to prove the new contract has identical state?

## Solution: Cryptographic Verification

The migration system solves these problems through:

1. **Export Function**: Creates a complete snapshot of contract state
2. **Verification Hash**: SHA-256 hash of all data for integrity checking
3. **Import Function**: Restores state with hash verification
4. **Batch Support**: Handles large datasets through incremental migration
5. **Replay Protection**: Prevents duplicate imports

## Architecture

### Core Components

#### 1. Migration Snapshot

Complete state snapshot with cryptographic verification:

```rust
struct MigrationSnapshot {
    version: u32,              // Schema version
    timestamp: u64,            // Creation time
    ledger_sequence: u32,      // Ledger number
    instance_data: InstanceData,
    persistent_data: PersistentData,
    verification_hash: BytesN<32>, // SHA-256 hash
}
```

#### 2. Instance Data

Contract-level configuration:

```rust
struct InstanceData {
    admin: Address,
    usdc_token: Address,
    platform_fee_bps: u32,
    remittance_counter: u64,
    accumulated_fees: i128,
    paused: bool,
    admin_count: u32,
}
```

#### 3. Persistent Data

Per-entity data:

```rust
struct PersistentData {
    remittances: Vec<Remittance>,
    agents: Vec<Address>,
    admin_roles: Vec<Address>,
    settlement_hashes: Vec<u64>,
    whitelisted_tokens: Vec<Address>,
}
```

#### 4. Migration Batch

For incremental migration:

```rust
struct MigrationBatch {
    batch_number: u32,
    total_batches: u32,
    remittances: Vec<Remittance>,
    batch_hash: BytesN<32>,
}
```

## Migration Process

### Full Migration (Small Datasets)

For contracts with < 100 remittances:

```rust
// 1. Export from old contract
let snapshot = old_contract.export_migration_state(&admin)?;

// 2. Verify integrity (optional but recommended)
let verification = old_contract.verify_migration_snapshot(snapshot.clone())?;
assert!(verification.valid);

// 3. Deploy new contract
let new_contract = deploy_new_contract();

// 4. Import state
new_contract.import_migration_state(&admin, snapshot)?;

// 5. Verify import (optional)
let new_snapshot = new_contract.export_migration_state(&admin)?;
assert_eq!(snapshot.verification_hash, new_snapshot.verification_hash);
```

### Batch Migration (Large Datasets)

For contracts with > 100 remittances:

```rust
// 1. Determine batch size
let batch_size = 50;
let total_remittances = old_contract.get_remittance_counter()?;
let total_batches = (total_remittances + batch_size - 1) / batch_size;

// 2. Export batches
let mut batches = Vec::new();
for batch_num in 0..total_batches {
    let batch = old_contract.export_migration_batch(
        &admin,
        batch_num,
        batch_size
    )?;
    batches.push(batch);
}

// 3. Deploy new contract and initialize
let new_contract = deploy_new_contract();
new_contract.initialize(&admin, &token, &fee_bps)?;

// 4. Import batches in order
for batch in batches {
    new_contract.import_migration_batch(&admin, batch)?;
}

// 5. Verify completeness
assert_eq!(
    new_contract.get_remittance_counter()?,
    old_contract.get_remittance_counter()?
);
```

## Security Features

### 1. Cryptographic Verification

**Hash Algorithm**: SHA-256 of deterministically encoded data

**What's Hashed**:
- All instance data (admin, token, fees, counters)
- All persistent data (remittances, agents, admins)
- Timestamp and ledger sequence
- Deterministic encoding (big-endian, consistent ordering)

**Verification**:
```rust
// Compute hash during export
let hash = sha256(serialize(instance_data) + serialize(persistent_data) + timestamp + ledger);

// Verify hash during import
let computed_hash = sha256(serialize(snapshot.instance_data) + ...);
if computed_hash != snapshot.verification_hash {
    return Err(InvalidMigrationHash);
}
```

### 2. Replay Protection

**Problem**: Prevent importing the same snapshot multiple times

**Solution**: Check if contract is already initialized

```rust
pub fn import_state(env: &Env, snapshot: MigrationSnapshot) -> Result<(), ContractError> {
    // Prevent import if already initialized
    if has_admin(env) {
        return Err(AlreadyInitialized);
    }
    
    // Verify hash
    verify_hash(&snapshot)?;
    
    // Import data
    restore_state(env, snapshot)?;
    
    Ok(())
}
```

### 3. Atomic Operations

**Guarantee**: All data imported or none (no partial state)

**Implementation**: 
- All storage operations in single transaction
- If any operation fails, entire transaction reverts
- No intermediate state visible

### 4. Authorization

**Export**: Only admin can export state
**Import**: Only admin can import state (on new contract)

```rust
pub fn export_migration_state(
    env: Env,
    caller: Address,
) -> Result<MigrationSnapshot, ContractError> {
    require_admin(&env, &caller)?;
    // ... export logic
}
```

### 5. Deterministic Encoding

**Requirement**: Same data always produces same hash

**Implementation**:
- Big-endian byte order for numbers
- Consistent address serialization
- Fixed enum encoding (Pending=0, Completed=1, Cancelled=2)
- Ordered iteration (no random ordering)

## Verification Process

### Pre-Migration Verification

Before migrating, verify the snapshot:

```rust
// 1. Export snapshot
let snapshot = old_contract.export_migration_state(&admin)?;

// 2. Verify hash
let verification = old_contract.verify_migration_snapshot(snapshot.clone())?;

// 3. Check verification result
assert!(verification.valid, "Snapshot integrity check failed");
assert_eq!(verification.expected_hash, verification.actual_hash);

// 4. Inspect data
println!("Remittances: {}", snapshot.persistent_data.remittances.len());
println!("Accumulated fees: {}", snapshot.instance_data.accumulated_fees);
println!("Platform fee: {} bps", snapshot.instance_data.platform_fee_bps);
```

### Post-Migration Verification

After migrating, verify completeness:

```rust
// 1. Export from both contracts
let old_snapshot = old_contract.export_migration_state(&admin)?;
let new_snapshot = new_contract.export_migration_state(&admin)?;

// 2. Compare critical data
assert_eq!(
    old_snapshot.instance_data.remittance_counter,
    new_snapshot.instance_data.remittance_counter
);
assert_eq!(
    old_snapshot.instance_data.accumulated_fees,
    new_snapshot.instance_data.accumulated_fees
);
assert_eq!(
    old_snapshot.persistent_data.remittances.len(),
    new_snapshot.persistent_data.remittances.len()
);

// 3. Spot check individual remittances
for id in 1..=10 {
    let old_rem = old_contract.get_remittance(&id)?;
    let new_rem = new_contract.get_remittance(&id)?;
    assert_eq!(old_rem, new_rem);
}
```

## Data Integrity Guarantees

### What's Preserved

✅ **All remittances** with exact amounts, fees, and status
✅ **All balances** including accumulated fees
✅ **All configuration** (admin, token, fee percentage)
✅ **All counters** (remittance counter, admin count)
✅ **All relationships** (agents, admins, settlement hashes)
✅ **All state** (pause status, whitelisted tokens)

### What's Verified

✅ **Completeness**: All data included in snapshot
✅ **Integrity**: No tampering or corruption
✅ **Authenticity**: Hash matches original export
✅ **Consistency**: Deterministic encoding ensures reproducibility

### What's Protected

✅ **Against tampering**: Hash verification detects any changes
✅ **Against partial transfer**: Atomic operations ensure all-or-nothing
✅ **Against replay**: Initialization check prevents duplicate imports
✅ **Against corruption**: Hash mismatch rejects corrupted data

## Error Handling

### Error Codes

| Code | Error | Description |
|------|-------|-------------|
| 1 | AlreadyInitialized | Contract already has data (replay protection) |
| 3 | InvalidAmount | Batch size invalid (0 or > 100) |
| 14 | Unauthorized | Caller is not admin |
| 20 | InvalidMigrationHash | Hash verification failed (tampering detected) |
| 21 | MigrationInProgress | Migration already active |
| 22 | InvalidMigrationBatch | Batch number or order invalid |

### Common Errors

#### Error: InvalidMigrationHash

**Cause**: Snapshot data doesn't match verification hash

**Possible Reasons**:
- Data was tampered with
- Snapshot was corrupted during transfer
- Incorrect serialization

**Solution**:
```rust
// Re-export snapshot
let snapshot = old_contract.export_migration_state(&admin)?;

// Verify before using
let verification = old_contract.verify_migration_snapshot(snapshot.clone())?;
if !verification.valid {
    panic!("Snapshot corrupted - re-export required");
}
```

#### Error: AlreadyInitialized

**Cause**: Trying to import into already-initialized contract

**Solution**:
```rust
// Deploy fresh contract (don't call initialize)
let new_contract = deploy_new_contract();

// Import directly (don't initialize first)
new_contract.import_migration_state(&admin, snapshot)?;
```

## Best Practices

### 1. Verify Before Import

Always verify snapshot integrity before importing:

```rust
let snapshot = old_contract.export_migration_state(&admin)?;
let verification = old_contract.verify_migration_snapshot(snapshot.clone())?;

if !verification.valid {
    return Err("Snapshot verification failed");
}

new_contract.import_migration_state(&admin, snapshot)?;
```

### 2. Pause Old Contract

Pause the old contract before migration to prevent new transactions:

```rust
// 1. Pause old contract
old_contract.pause(&admin)?;

// 2. Export state
let snapshot = old_contract.export_migration_state(&admin)?;

// 3. Import to new contract
new_contract.import_migration_state(&admin, snapshot)?;

// 4. Verify new contract
verify_migration_success(&old_contract, &new_contract)?;

// 5. Announce new contract address to users
```

### 3. Use Batch Migration for Large Datasets

For > 100 remittances, use batch migration:

```rust
let batch_size = 50; // Optimal size
let total_batches = calculate_total_batches(remittance_count, batch_size);

for batch_num in 0..total_batches {
    let batch = old_contract.export_migration_batch(&admin, batch_num, batch_size)?;
    new_contract.import_migration_batch(&admin, batch)?;
}
```

### 4. Test Migration on Testnet

Always test the migration process on testnet first:

```rust
// 1. Deploy test contracts
let old_test = deploy_to_testnet(old_wasm);
let new_test = deploy_to_testnet(new_wasm);

// 2. Populate with test data
populate_test_data(&old_test);

// 3. Perform migration
let snapshot = old_test.export_migration_state(&admin)?;
new_test.import_migration_state(&admin, snapshot)?;

// 4. Verify results
verify_migration_success(&old_test, &new_test)?;

// 5. If successful, proceed to mainnet
```

### 5. Document Migration

Keep detailed records of the migration:

```rust
struct MigrationRecord {
    old_contract_id: String,
    new_contract_id: String,
    migration_timestamp: u64,
    snapshot_hash: BytesN<32>,
    remittances_migrated: u64,
    fees_migrated: i128,
    verification_passed: bool,
}
```

## Performance Considerations

### Gas Costs

| Operation | Estimated Gas | Notes |
|-----------|--------------|-------|
| Export snapshot (100 remittances) | ~500,000 | Scales with data size |
| Import snapshot (100 remittances) | ~800,000 | Includes hash verification |
| Verify snapshot | ~100,000 | Read-only operation |
| Export batch (50 remittances) | ~250,000 | Per batch |
| Import batch (50 remittances) | ~400,000 | Per batch |

### Optimization Tips

1. **Batch Size**: Use 50-100 items per batch for optimal gas efficiency
2. **Parallel Export**: Export batches in parallel (read-only)
3. **Sequential Import**: Import batches sequentially (write operations)
4. **Verification**: Verify once before importing all batches

## Migration Checklist

### Pre-Migration

- [ ] Test migration on testnet
- [ ] Verify all tests pass on new contract
- [ ] Announce migration to users
- [ ] Pause old contract
- [ ] Export state snapshot
- [ ] Verify snapshot hash
- [ ] Save snapshot securely

### Migration

- [ ] Deploy new contract
- [ ] Import state snapshot
- [ ] Verify import success
- [ ] Spot check critical data
- [ ] Test new contract functions
- [ ] Verify balances match

### Post-Migration

- [ ] Announce new contract address
- [ ] Update documentation
- [ ] Update client applications
- [ ] Monitor new contract
- [ ] Keep old contract paused
- [ ] Archive migration records

## Example: Complete Migration

```rust
use soroban_sdk::{Env, Address};

fn migrate_contract(
    env: &Env,
    old_contract: &SwiftRemitContractClient,
    new_contract_wasm: &[u8],
    admin: &Address,
) -> Result<Address, Error> {
    // 1. Pause old contract
    old_contract.pause(admin)?;
    println!("✓ Old contract paused");
    
    // 2. Export state
    let snapshot = old_contract.export_migration_state(admin)?;
    println!("✓ State exported: {} remittances", 
        snapshot.persistent_data.remittances.len());
    
    // 3. Verify snapshot
    let verification = old_contract.verify_migration_snapshot(snapshot.clone())?;
    if !verification.valid {
        return Err(Error::InvalidSnapshot);
    }
    println!("✓ Snapshot verified");
    
    // 4. Deploy new contract
    let new_contract_id = env.deployer().deploy(new_contract_wasm);
    let new_contract = SwiftRemitContractClient::new(env, &new_contract_id);
    println!("✓ New contract deployed: {}", new_contract_id);
    
    // 5. Import state
    new_contract.import_migration_state(admin, snapshot)?;
    println!("✓ State imported");
    
    // 6. Verify import
    let new_snapshot = new_contract.export_migration_state(admin)?;
    assert_eq!(
        snapshot.instance_data.remittance_counter,
        new_snapshot.instance_data.remittance_counter
    );
    println!("✓ Import verified");
    
    // 7. Test new contract
    let fee = new_contract.get_platform_fee_bps()?;
    assert_eq!(fee, snapshot.instance_data.platform_fee_bps);
    println!("✓ New contract functional");
    
    Ok(new_contract_id)
}
```

## Troubleshooting

### Issue: Hash Verification Fails

**Symptoms**: `InvalidMigrationHash` error during import

**Diagnosis**:
```rust
let verification = contract.verify_migration_snapshot(snapshot.clone())?;
println!("Expected: {:?}", verification.expected_hash);
println!("Actual: {:?}", verification.actual_hash);
```

**Solutions**:
1. Re-export snapshot from old contract
2. Check for data corruption during transfer
3. Verify serialization is deterministic

### Issue: Import Fails with AlreadyInitialized

**Symptoms**: Cannot import into new contract

**Diagnosis**:
```rust
let has_admin = new_contract.is_admin(admin.clone());
println!("Contract initialized: {}", has_admin);
```

**Solutions**:
1. Deploy fresh contract (don't call initialize)
2. Use different contract instance
3. Clear test environment between attempts

### Issue: Incomplete Migration

**Symptoms**: Some data missing after import

**Diagnosis**:
```rust
let old_count = old_contract.get_remittance_counter()?;
let new_count = new_contract.get_remittance_counter()?;
println!("Old: {}, New: {}", old_count, new_count);
```

**Solutions**:
1. Use batch migration for large datasets
2. Verify all batches imported
3. Check for errors during batch import

## Conclusion

The SwiftRemit migration system provides:

✅ **Trustless Migration**: Cryptographic verification eliminates trust assumptions
✅ **Data Integrity**: SHA-256 hashing ensures no tampering
✅ **Completeness**: All state preserved exactly
✅ **Replay Protection**: Prevents duplicate imports
✅ **Deterministic**: Same data always produces same hash
✅ **Auditable**: Full verification before and after migration
✅ **Scalable**: Batch support for large datasets

The system enables safe contract upgrades while maintaining complete data integrity and user trust.
