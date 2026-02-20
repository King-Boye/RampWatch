# Migration API Reference

## Overview

Complete API reference for the SwiftRemit secure migration system.

## Public Functions

### export_migration_state

Export complete contract state for migration.

```rust
pub fn export_migration_state(
    env: Env,
    caller: Address,
) -> Result<MigrationSnapshot, ContractError>
```

#### Description

Creates a cryptographically verified snapshot of all contract data including instance storage, persistent storage, and a SHA-256 verification hash.

#### Parameters

- `env: Env` - Soroban environment
- `caller: Address` - Admin address (must be authorized)

#### Returns

- `Ok(MigrationSnapshot)` - Complete state snapshot with verification hash
- `Err(ContractError)` - Error if caller is not admin or export fails

#### Errors

| Error | Code | Condition |
|-------|------|-----------|
| `Unauthorized` | 14 | Caller is not admin |
| `NotInitialized` | 2 | Contract not initialized |

#### Authorization

- Requires admin authorization
- Caller must be registered admin

#### Example

```rust
let snapshot = contract.export_migration_state(&admin)?;
println!("Exported {} remittances", snapshot.persistent_data.remittances.len());
println!("Verification hash: {:?}", snapshot.verification_hash);
```

---

### import_migration_state

Import contract state from migration snapshot.

```rust
pub fn import_migration_state(
    env: Env,
    caller: Address,
    snapshot: MigrationSnapshot,
) -> Result<(), ContractError>
```

#### Description

Restores complete contract state from a verified snapshot. Verifies cryptographic hash before importing to ensure data integrity.

#### Parameters

- `env: Env` - Soroban environment
- `caller: Address` - Admin address (must be authorized)
- `snapshot: MigrationSnapshot` - Complete migration snapshot to import

#### Returns

- `Ok(())` - Import successful
- `Err(ContractError)` - Error if verification fails or contract already initialized

#### Errors

| Error | Code | Condition |
|-------|------|-----------|
| `AlreadyInitialized` | 1 | Contract already has data |
| `InvalidMigrationHash` | 20 | Hash verification failed |
| `Unauthorized` | 14 | Caller is not admin |

#### Authorization

- Requires caller authorization
- Contract must not be initialized

#### Security

- Verifies SHA-256 hash before import
- Atomic operation (all or nothing)
- Replay protection through initialization check

#### Example

```rust
// Deploy new contract
let new_contract = deploy_contract();

// Import state
let snapshot = get_snapshot_from_old_contract();
new_contract.import_migration_state(&admin, snapshot)?;

// Verify import
assert_eq!(new_contract.get_platform_fee_bps(), 250);
```

---

### verify_migration_snapshot

Verify migration snapshot integrity without importing.

```rust
pub fn verify_migration_snapshot(
    env: Env,
    snapshot: MigrationSnapshot,
) -> MigrationVerification
```

#### Description

Validates that a snapshot's cryptographic hash matches its contents. Useful for pre-import validation and auditing.

#### Parameters

- `env: Env` - Soroban environment
- `snapshot: MigrationSnapshot` - Snapshot to verify

#### Returns

`MigrationVerification` with:
- `valid: bool` - Whether hash matches
- `expected_hash: BytesN<32>` - Hash from snapshot
- `actual_hash: BytesN<32>` - Computed hash
- `timestamp: u64` - Verification time

#### Authorization

- No authorization required (read-only)

#### Example

```rust
let snapshot = contract.export_migration_state(&admin)?;
let verification = contract.verify_migration_snapshot(snapshot.clone())?;

if !verification.valid {
    panic!("Snapshot integrity check failed!");
}

println!("✓ Snapshot verified");
```

---

### export_migration_batch

Export state in batches for large datasets.

```rust
pub fn export_migration_batch(
    env: Env,
    caller: Address,
    batch_number: u32,
    batch_size: u32,
) -> Result<MigrationBatch, ContractError>
```

#### Description

Exports a subset of remittances in a batch with its own verification hash. Useful for contracts with many remittances to avoid resource limits.

#### Parameters

- `env: Env` - Soroban environment
- `caller: Address` - Admin address (must be authorized)
- `batch_number: u32` - Which batch to export (0-indexed)
- `batch_size: u32` - Number of items per batch (1-100)

#### Returns

- `Ok(MigrationBatch)` - Batch with remittances and verification hash
- `Err(ContractError)` - Error if parameters invalid or caller not admin

#### Errors

| Error | Code | Condition |
|-------|------|-----------|
| `InvalidAmount` | 3 | Batch size is 0 or > 100 |
| `Unauthorized` | 14 | Caller is not admin |

#### Authorization

- Requires admin authorization

#### Example

```rust
// Export in batches of 50
let batch_size = 50;
let total_remittances = contract.get_remittance_counter()?;
let total_batches = (total_remittances + batch_size - 1) / batch_size;

for batch_num in 0..total_batches {
    let batch = contract.export_migration_batch(&admin, batch_num, batch_size)?;
    println!("Batch {}/{}: {} remittances", 
        batch_num + 1, total_batches, batch.remittances.len());
}
```

---

### import_migration_batch

Import state from batch.

```rust
pub fn import_migration_batch(
    env: Env,
    caller: Address,
    batch: MigrationBatch,
) -> Result<(), ContractError>
```

#### Description

Imports a single batch of remittances with hash verification. Batches should be imported in order (0, 1, 2, ...) for consistency.

#### Parameters

- `env: Env` - Soroban environment
- `caller: Address` - Admin address (must be authorized)
- `batch: MigrationBatch` - Batch to import with verification hash

#### Returns

- `Ok(())` - Import successful
- `Err(ContractError)` - Error if hash verification fails or caller not admin

#### Errors

| Error | Code | Condition |
|-------|------|-----------|
| `InvalidMigrationHash` | 20 | Batch hash verification failed |
| `Unauthorized` | 14 | Caller is not admin |

#### Authorization

- Requires admin authorization

#### Security

- Verifies batch hash before import
- Detects tampering or corruption

#### Example

```rust
// Import batches in order
let batches = export_all_batches(&old_contract, &admin)?;

for batch in batches {
    new_contract.import_migration_batch(&admin, batch)?;
    println!("Imported batch {}/{}", batch.batch_number + 1, batch.total_batches);
}
```

---

## Data Types

### MigrationSnapshot

Complete state snapshot with cryptographic verification.

```rust
#[contracttype]
pub struct MigrationSnapshot {
    pub version: u32,
    pub timestamp: u64,
    pub ledger_sequence: u32,
    pub instance_data: InstanceData,
    pub persistent_data: PersistentData,
    pub verification_hash: BytesN<32>,
}
```

#### Fields

- `version: u32` - Schema version for forward compatibility (currently 1)
- `timestamp: u64` - Unix timestamp when snapshot was created
- `ledger_sequence: u32` - Ledger sequence number when snapshot was created
- `instance_data: InstanceData` - Contract-level configuration
- `persistent_data: PersistentData` - Per-entity data
- `verification_hash: BytesN<32>` - SHA-256 hash of all data

---

### InstanceData

Contract-level configuration data.

```rust
#[contracttype]
pub struct InstanceData {
    pub admin: Address,
    pub usdc_token: Address,
    pub platform_fee_bps: u32,
    pub remittance_counter: u64,
    pub accumulated_fees: i128,
    pub paused: bool,
    pub admin_count: u32,
}
```

#### Fields

- `admin: Address` - Legacy admin address
- `usdc_token: Address` - USDC token contract address
- `platform_fee_bps: u32` - Platform fee in basis points (0-10000)
- `remittance_counter: u64` - Global remittance counter
- `accumulated_fees: i128` - Total accumulated platform fees
- `paused: bool` - Contract pause status
- `admin_count: u32` - Number of registered admins

---

### PersistentData

Per-entity persistent storage data.

```rust
#[contracttype]
pub struct PersistentData {
    pub remittances: Vec<Remittance>,
    pub agents: Vec<Address>,
    pub admin_roles: Vec<Address>,
    pub settlement_hashes: Vec<u64>,
    pub whitelisted_tokens: Vec<Address>,
}
```

#### Fields

- `remittances: Vec<Remittance>` - All remittances indexed by ID
- `agents: Vec<Address>` - Registered agent addresses
- `admin_roles: Vec<Address>` - Admin role addresses
- `settlement_hashes: Vec<u64>` - Remittance IDs that have been settled
- `whitelisted_tokens: Vec<Address>` - Whitelisted token addresses

---

### MigrationBatch

Batch of remittances for incremental migration.

```rust
#[contracttype]
pub struct MigrationBatch {
    pub batch_number: u32,
    pub total_batches: u32,
    pub remittances: Vec<Remittance>,
    pub batch_hash: BytesN<32>,
}
```

#### Fields

- `batch_number: u32` - Batch number (0-indexed)
- `total_batches: u32` - Total number of batches
- `remittances: Vec<Remittance>` - Remittances in this batch
- `batch_hash: BytesN<32>` - SHA-256 hash of this batch

---

### MigrationVerification

Result of snapshot verification.

```rust
#[contracttype]
pub struct MigrationVerification {
    pub valid: bool,
    pub expected_hash: BytesN<32>,
    pub actual_hash: BytesN<32>,
    pub timestamp: u64,
}
```

#### Fields

- `valid: bool` - Whether verification passed
- `expected_hash: BytesN<32>` - Hash from snapshot
- `actual_hash: BytesN<32>` - Computed hash
- `timestamp: u64` - Verification timestamp

---

## Constants

### MAX_MIGRATION_BATCH_SIZE

Maximum number of items per batch.

```rust
pub const MAX_MIGRATION_BATCH_SIZE: u32 = 100;
```

Prevents excessive resource consumption in a single transaction.

---

## Integration Examples

### JavaScript/TypeScript

```typescript
import { Contract } from '@stellar/stellar-sdk';

// Export from old contract
const oldContract = new Contract(oldContractId);
const snapshot = await oldContract.export_migration_state({ caller: admin });

// Verify snapshot
const verification = await oldContract.verify_migration_snapshot({ 
    snapshot: snapshot 
});

if (!verification.valid) {
    throw new Error('Snapshot verification failed');
}

// Deploy new contract
const newContract = await deployNewContract();

// Import state
await newContract.import_migration_state({
    caller: admin,
    snapshot: snapshot
});

console.log('Migration complete!');
```

### Python

```python
from stellar_sdk import SorobanServer, Contract

# Export from old contract
old_contract = Contract(old_contract_id)
snapshot = old_contract.export_migration_state(caller=admin)

# Verify snapshot
verification = old_contract.verify_migration_snapshot(snapshot=snapshot)
assert verification['valid'], "Snapshot verification failed"

# Deploy new contract
new_contract = deploy_new_contract()

# Import state
new_contract.import_migration_state(caller=admin, snapshot=snapshot)

print("Migration complete!")
```

### Rust (Contract-to-Contract)

```rust
use soroban_sdk::{contract, contractimpl, Address, Env};

#[contract]
pub struct MigrationHelper;

#[contractimpl]
impl MigrationHelper {
    pub fn migrate(
        env: Env,
        old_contract: Address,
        new_contract: Address,
        admin: Address,
    ) -> Result<(), Error> {
        let old = SwiftRemitContractClient::new(&env, &old_contract);
        let new = SwiftRemitContractClient::new(&env, &new_contract);
        
        // Export
        let snapshot = old.export_migration_state(&admin)?;
        
        // Verify
        let verification = old.verify_migration_snapshot(snapshot.clone())?;
        if !verification.valid {
            return Err(Error::InvalidSnapshot);
        }
        
        // Import
        new.import_migration_state(&admin, snapshot)?;
        
        Ok(())
    }
}
```

---

## Error Reference

| Code | Error | Description | Recovery |
|------|-------|-------------|----------|
| 1 | AlreadyInitialized | Contract already has data | Deploy fresh contract |
| 2 | NotInitialized | Contract not initialized | Initialize contract first |
| 3 | InvalidAmount | Batch size invalid | Use 1-100 batch size |
| 14 | Unauthorized | Caller is not admin | Use admin address |
| 20 | InvalidMigrationHash | Hash verification failed | Re-export snapshot |
| 21 | MigrationInProgress | Migration already active | Wait for completion |
| 22 | InvalidMigrationBatch | Batch invalid | Check batch number/order |

---

## Best Practices

### 1. Always Verify Before Import

```rust
let snapshot = old_contract.export_migration_state(&admin)?;
let verification = old_contract.verify_migration_snapshot(snapshot.clone())?;

if !verification.valid {
    return Err("Snapshot verification failed");
}

new_contract.import_migration_state(&admin, snapshot)?;
```

### 2. Use Batch Migration for Large Datasets

```rust
if remittance_count > 100 {
    // Use batch migration
    let batch_size = 50;
    for batch_num in 0..total_batches {
        let batch = old_contract.export_migration_batch(&admin, batch_num, batch_size)?;
        new_contract.import_migration_batch(&admin, batch)?;
    }
} else {
    // Use full migration
    let snapshot = old_contract.export_migration_state(&admin)?;
    new_contract.import_migration_state(&admin, snapshot)?;
}
```

### 3. Pause Old Contract During Migration

```rust
// Pause old contract
old_contract.pause(&admin)?;

// Perform migration
let snapshot = old_contract.export_migration_state(&admin)?;
new_contract.import_migration_state(&admin, snapshot)?;

// Verify new contract
verify_migration_success(&old_contract, &new_contract)?;
```

### 4. Test on Testnet First

```rust
// Test migration on testnet
let test_snapshot = test_old_contract.export_migration_state(&admin)?;
test_new_contract.import_migration_state(&admin, test_snapshot)?;

// Verify test migration
assert_migration_success(&test_old_contract, &test_new_contract)?;

// If successful, proceed to mainnet
let mainnet_snapshot = mainnet_old_contract.export_migration_state(&admin)?;
mainnet_new_contract.import_migration_state(&admin, mainnet_snapshot)?;
```

---

## Performance

### Gas Costs

| Operation | Estimated Gas | Notes |
|-----------|--------------|-------|
| export_migration_state (100 items) | ~500,000 | Scales with data |
| import_migration_state (100 items) | ~800,000 | Includes verification |
| verify_migration_snapshot | ~100,000 | Read-only |
| export_migration_batch (50 items) | ~250,000 | Per batch |
| import_migration_batch (50 items) | ~400,000 | Per batch |

### Optimization

- Use batch size of 50-100 for optimal gas efficiency
- Export batches in parallel (read-only operations)
- Import batches sequentially (write operations)
- Verify once before importing all batches

---

## Support

For issues and questions:
- Documentation: [MIGRATION.md](MIGRATION.md)
- API Reference: This document
- GitHub Issues: [Create an issue](https://github.com/yourusername/swiftremit/issues)
