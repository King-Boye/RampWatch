# SwiftRemit Contract Migration Guide

This document describes how to migrate all contract state from one deployed instance
to another using `export_migration_snapshot` and `import_migration_batch`.

---

## Overview

The migration system works in two phases:

1. **Export** — call `export_migration_snapshot` on the *source* contract.  
   This locks the source contract (blocks `create_remittance` and `confirm_payout`)
   and returns a `MigrationSnapshot` containing all state plus a SHA-256
   verification hash.

2. **Import** — call `import_migration_batch` on the *destination* contract one
   batch at a time.  
   Each batch is hash-verified before any data is written. After the final batch
   the destination contract is unlocked and ready for normal use.

---

## Prerequisites

- You must hold the **Admin** role on both the source and destination contracts.
- The destination contract must already be **initialized** (call `initialize` first).
- Keep the source contract locked (do not call `unpause` or clear the migration flag
  manually) until the import is fully verified.

---

## Step-by-Step Instructions

### 1. Initialize the destination contract

Deploy a new contract and call `initialize` with the same parameters as the source:

```bash
soroban contract invoke \
  --id <DEST_CONTRACT_ID> \
  -- initialize \
  --admin <ADMIN_ADDRESS> \
  --usdc_token <USDC_TOKEN_ADDRESS> \
  --fee_bps 250 \
  --rate_limit_cooldown 3600 \
  --protocol_fee_bps 0 \
  --treasury <TREASURY_ADDRESS>
```

### 2. Export the snapshot from the source contract

```bash
soroban contract invoke \
  --id <SOURCE_CONTRACT_ID> \
  -- export_migration_snapshot \
  --caller <ADMIN_ADDRESS>
```

Save the returned `MigrationSnapshot` JSON. The source contract is now **locked** —
`create_remittance` and `confirm_payout` will return `MigrationInProgress` (error 30).

### 3. Split the snapshot into batches (off-chain)

Use the `MigrationSnapshot.persistent_data.remittances` array. Split it into chunks
of at most `MAX_MIGRATION_BATCH_SIZE` (100) items. For each chunk compute the
`batch_hash` using the same algorithm as `compute_batch_hash` in `src/migration.rs`:

```
SHA-256( batch_number_be32 || for each remittance { id_be64 || sender_xdr || agent_xdr || amount_be128 || fee_be128 || status_u8 || expiry_be64? } )
```

### 4. Import each batch into the destination contract

Call `import_migration_batch` for batch 0, then 1, then 2, … in order:

```bash
soroban contract invoke \
  --id <DEST_CONTRACT_ID> \
  -- import_migration_batch \
  --caller <ADMIN_ADDRESS> \
  --batch '{ "batch_number": 0, "total_batches": N, "remittances": [...], "batch_hash": "..." }'
```

After the **final** batch (`batch_number == total_batches - 1`) the destination
contract automatically clears the `MigrationInProgress` flag and resumes normal
operations.

### 5. Verify the migration

Query a sample of remittances on the destination contract and compare with the source:

```bash
soroban contract invoke --id <DEST_CONTRACT_ID> -- get_remittance --remittance_id 1
soroban contract invoke --id <DEST_CONTRACT_ID> -- get_remittance --remittance_id 2
```

Also confirm the counters match:

```bash
soroban contract invoke --id <DEST_CONTRACT_ID> -- get_platform_fee_bps
```

### Rust Contract Code

The Rust contract code remains largely unchanged. Constants like `MAX_FEE_BPS` and `FEE_DIVISOR` are still hardcoded in the contract for on-chain consistency, but they are now documented with comments explaining their purpose.

## Breaking Changes

### None for Normal Usage

If you were using the system normally, there are no breaking changes. The refactoring maintains backward compatibility:

- All existing functionality works the same way
- Default values match previous hardcoded values
- Tests continue to pass

### If You Modified Hardcoded Values

If you previously modified hardcoded values in the code, you now need to set them via environment variables instead:

1. Identify the values you changed
2. Add them to your `.env` file
3. Remove your code modifications

## Troubleshooting

### "Missing required environment variable" Error

**Problem**: You're missing a required configuration value

**Solution**: Add the variable to your `.env` file. Check `.env.example` for the complete list of variables.

### "Configuration validation failed" Error

**Problem**: A configuration value is invalid (wrong type, out of range, etc.)

**Solution**: Check the error message for details. Common issues:
- Fee values must be 0-10000
- URLs must be HTTPS
- Network must be 'testnet' or 'mainnet'
- Numeric values must be valid numbers

### Client Code Not Finding Configuration

**Problem**: Client code can't load configuration

**Solution**: 
1. Ensure `.env` file exists in project root
2. Ensure you're running from the correct directory
3. Check that `dotenv` package is installed: `npm install`

### Deployment Script Not Using Environment Variables

**Problem**: Deployment script uses defaults instead of your values

**Solution**:
1. Export variables before running script: `export NETWORK=testnet`
2. Or set them in `.env` file
3. Or use CLI overrides: `./deploy.sh testnet`

## Getting Help

If you encounter issues during migration:

1. Check the [Configuration Guide](CONFIGURATION.md) for detailed documentation
2. Review error messages carefully - they indicate which variable is problematic
3. Verify your `.env` file against `.env.example`
4. Ensure all required variables are set
5. Check that values are within valid ranges

## Benefits of the New System

After migration, you'll benefit from:

1. **Easier Environment Management**: Switch between testnet and mainnet by changing one variable
2. **Better Security**: Secrets are in `.env` (gitignored) instead of code
3. **Simplified Deployment**: Deploy to multiple environments without code changes
4. **Centralized Configuration**: All settings in one place
5. **Validation**: Configuration errors caught at startup, not runtime
6. **Documentation**: Clear documentation of all configuration options

## Next Steps

After completing migration:

1. Delete any local modifications to hardcoded values
2. Commit your updated code (but not `.env`!)
3. Share `.env.example` with your team
4. Update your deployment documentation
5. Consider setting up environment-specific `.env` files (`.env.testnet`, `.env.mainnet`)

## Hash Schema Upgrades

Settlement IDs are derived deterministically from remittance fields using the schema
defined in `src/hashing.rs`. The `HASH_SCHEMA_VERSION` constant tracks which version
of that schema is active.

### When is a version bump required?

Increment `HASH_SCHEMA_VERSION` whenever a code change would produce a **different
hash for the same logical inputs**. Concrete triggers:

- Adding, removing, or reordering fields passed to `compute_settlement_id`
- Changing the byte encoding of any field (e.g. endianness, XDR format)
- Changing how `None` optional values are serialized (currently 8 zero bytes)
- Replacing the hash algorithm (currently SHA-256)

Purely internal refactors that leave byte output identical do **not** require a bump.

### Steps to perform a version bump

1. Update the field ordering / encoding in `compute_settlement_id` (`src/hashing.rs`).
2. Increment `HASH_SCHEMA_VERSION` (e.g. `1` → `2`).
3. Add a row to the version history table in the `HASH_SCHEMA_VERSION` doc comment.
4. Communicate the new version to all external integrators before deploying.

### How external systems must handle a mismatch

External systems (banks, anchors, off-chain indexers) **must** persist the schema
version alongside every settlement ID they store. On detecting a version mismatch:

1. **Do not** use the stored ID as-is — it was computed under a different schema.
2. Re-derive the settlement ID for the affected remittances by calling
   `compute_settlement_hash(env, remittance_id)` on-chain, or by re-implementing
   the new field ordering documented in `src/hashing.rs`.
3. Overwrite the stored settlement ID with the newly derived value.
4. Update the stored schema version to match `HASH_SCHEMA_VERSION`.

### Migration steps for existing settlement IDs

If a version bump is deployed to a live contract that already has settled remittances:

1. **Identify affected records** — query all settlement IDs stored with the old
   schema version.
2. **Re-derive in batches** — use `compute_settlement_hash` for each `remittance_id`
   to obtain the new ID. Batch size should respect the `MAX_MIGRATION_BATCH_SIZE`
   limit defined in `src/migration.rs` (currently 100).
3. **Atomic swap** — update the stored ID and schema version atomically in your
   off-chain database to avoid a partial-migration window.
4. **Verify** — after migration, assert that no records remain with the old schema
   version.
5. **Coordinate** — if multiple services share the same settlement ID store,
   coordinate the cutover so all services switch at the same ledger sequence.
