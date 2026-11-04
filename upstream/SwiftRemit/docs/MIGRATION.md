# SwiftRemit Contract Migration Guide

This document covers two distinct migration scenarios:

1. **In-place WASM upgrade** — the contract address stays the same; only the
   bytecode changes.  Call `migrate()` immediately after the upgrade.
2. **Cross-contract migration** — state is moved from one deployed contract
   instance to a freshly deployed one using `export_migration_snapshot` /
   `import_migration_batch`.

---

## Part 1 — In-Place WASM Upgrade Migration

### Background: Why Agent Keys Are at Risk

Soroban persistent storage keys are XDR-encoded `contracttype` enum variants.
When a contract is upgraded via `env.deployer().update_current_contract_wasm()`,
the new bytecode is live immediately but **all existing storage entries remain
unchanged**.  If the new code changes the discriminant, field order, or type of
any `DataKey` variant, the old entries become unreadable — effectively orphaned.

The following persistent keys are written per-agent and are at risk:

| `DataKey` variant              | Value type        | Risk                                                  |
|-------------------------------|-------------------|-------------------------------------------------------|
| `AgentRegistered(Address)`    | `bool`            | Orphaned if variant discriminant or Address XDR changes |
| `AgentKycHash(Address)`       | `BytesN<32>`      | Orphaned if variant discriminant changes              |
| `AgentStats(Address)`         | `AgentStats`      | Orphaned if `AgentStats` struct layout changes        |
| `AgentDailyCap(Address)`      | `i128`            | Orphaned if variant discriminant changes              |
| `AgentWithdrawals(Address)`   | `Vec<TransferRecord>` | Orphaned if `TransferRecord` layout changes       |
| `RoleAssignment(Addr, Role)`  | `bool`            | Orphaned if `Role` enum repr changes                  |
| `AgentList`                   | `Vec<Address>`    | **Was missing in schema v1** — must be rebuilt        |

### Why Keys Are Not Automatically Migrated

Soroban does not provide a built-in key-rename or schema-migration primitive.
The runtime simply reads raw XDR bytes from the ledger using the key produced
by the current code.  If the key encoding changed, the lookup returns `None`.

Additionally, there is no way to iterate all persistent storage entries from
within a contract — you can only read a key if you already know it.  This is
why the `AgentList` index is critical: it is the only way to enumerate all
registered agents so their keys can be re-written after an upgrade.

### Assumptions

1. The `AgentList` persistent key is kept in sync with `AgentRegistered` by
   `set_agent_registered()` (enforced since schema v2).
2. On a v1 contract (deployed before this fix), `AgentList` may be empty.
   In that case the admin must supply the list of known agent addresses
   out-of-band before calling `migrate()`.
3. `AgentStats`, `AgentDailyCap`, and `AgentWithdrawals` are performance /
   rate-limit data.  Loss of these values degrades analytics but does not
   affect fund safety.  They are **not** re-written by `migrate()` in v2
   because their `DataKey` variants were not changed.  Add a v3 step if
   their layout changes in a future upgrade.

---

## In-Place Upgrade: Step-by-Step

### 1. Verify the current schema version (optional)

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  -- get_schema_version
```

If the output is already `2` (or `CURRENT_SCHEMA_VERSION`), no migration is
needed.

### 2. Upgrade the WASM

```bash
stellar contract install --wasm target/wasm32-unknown-unknown/release/swiftremit.wasm
# Note the new WASM hash printed above, e.g. abc123...

stellar contract invoke \
  --id <CONTRACT_ID> \
  -- upgrade \
  --caller <ADMIN_ADDRESS> \
  --new_wasm_hash <NEW_WASM_HASH>
```

### 3. Call `migrate()` immediately after the upgrade

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  -- migrate \
  --caller <ADMIN_ADDRESS>
```

`migrate()` is **idempotent** — calling it a second time is a no-op.

### 4. Verify agent records are intact

```bash
stellar contract invoke --id <CONTRACT_ID> -- is_agent_registered --agent <AGENT_ADDRESS>
```

Repeat for a representative sample of agents.

### 5. Rollback (if validation failed)

If `migrate()` returned `MigrationValidationFailed`:

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  -- rollback_migration \
  --caller <ADMIN_ADDRESS>
```

This restores all agent registration records from the pre-migration snapshot
that was saved at the start of `migrate()`.

---

## Part 2 — Cross-Contract Migration

Use this path when deploying a new contract address (e.g., after a breaking
change that requires a fresh deployment).

### Prerequisites

- Admin role on both source and destination contracts.
- Destination contract already initialized (`initialize` called).

### Step 1 — Initialize the destination contract

```bash
stellar contract invoke \
  --id <DEST_CONTRACT_ID> \
  -- initialize \
  --admin <ADMIN_ADDRESS> \
  --usdc_token <USDC_TOKEN_ADDRESS> \
  --fee_bps 250 \
  --rate_limit_cooldown 3600 \
  --protocol_fee_bps 0 \
  --treasury <TREASURY_ADDRESS>
```

### Step 2 — Export the snapshot from the source contract

```bash
stellar contract invoke \
  --id <SOURCE_CONTRACT_ID> \
  -- export_migration_snapshot \
  --caller <ADMIN_ADDRESS>
```

Save the returned `MigrationSnapshot` JSON.  The source contract is now
**locked** — `create_remittance` and `confirm_payout` return `MigrationInProgress`.

The snapshot now includes **full agent records** (`AgentRecord` with `address`,
`registered`, and `kyc_hash`), not just addresses.

### Step 3 — Split into batches and import

Split `MigrationSnapshot.persistent_data.remittances` into chunks of at most
`MAX_MIGRATION_BATCH_SIZE` (100) items.  Batches **must** be submitted in strict
sequential order starting from `batch_number = 0`.  Submitting a batch whose
`batch_number` does not equal the next expected index returns
`InvalidMigrationBatch` (32) and leaves the destination contract in its current
state — no partial write occurs.

For each chunk:

```bash
stellar contract invoke \
  --id <DEST_CONTRACT_ID> \
  -- import_migration_batch \
  --caller <ADMIN_ADDRESS> \
  --batch '{ "batch_number": 0, "total_batches": N, "remittances": [...], "batch_hash": "..." }'
```

After the final batch the destination contract automatically clears the
`MigrationInProgress` flag.

### Step 3a — Abort a failed import (rollback)

If an import fails mid-way (e.g. a batch hash mismatch, out-of-order batch, or
off-chain tooling error), call `abort_migration` on the **destination** contract
to reset the state machine back to Idle:

```bash
stellar contract invoke \
  --id <DEST_CONTRACT_ID> \
  -- abort_migration \
  --caller <ADMIN_ADDRESS>
```

`abort_migration`:
- Clears the `MigrationInProgress` flag (re-enables normal operations).
- Resets the batch ordering counter so a fresh import can start from batch 0.
- Emits a `migration_aborted` event for off-chain indexers.

> **Note:** Any remittances already written by previous `import_migration_batch`
> calls are **not** automatically removed.  If a clean slate is required,
> re-initialize the destination contract before retrying the import.

### Step 4 — Verify

```bash
stellar contract invoke --id <DEST_CONTRACT_ID> -- get_remittance --remittance_id 1
stellar contract invoke --id <DEST_CONTRACT_ID> -- is_agent_registered --agent <AGENT_ADDRESS>
```

### Step 5 — Redirect traffic

Update off-chain services to point to `<DEST_CONTRACT_ID>`.

---

## Error Reference

| Error                       | Code | Meaning                                                    |
|-----------------------------|------|------------------------------------------------------------|
| `MigrationInProgress`       | 31   | Export already called; or normal op blocked during migration |
| `InvalidMigrationHash`      | 30   | Batch hash mismatch — data was tampered or corrupted       |
| `InvalidMigrationBatch`     | 32   | `batch_number != expected_next_batch` or `batch_number >= total_batches` |
| `MigrationValidationFailed` | 56   | One or more agents unreadable after `migrate()`            |
| `NotFound`                  | 57   | No rollback snapshot exists; or `abort_migration` called when not in progress |
| `Unauthorized`              | 20   | Caller does not have Admin role                            |

---

## Security Notes

- The `verification_hash` in `MigrationSnapshot` covers all instance and
  persistent data plus the timestamp and ledger sequence.  Any tampering will
  cause `InvalidMigrationHash` on import.
- Each `MigrationBatch` carries its own `batch_hash` verified independently.
- `migrate()` saves a `RollbackSnapshot` to instance storage before making any
  writes.  The snapshot is cleared only after successful validation.
- The source contract stays locked until you explicitly clear the flag (or
  redeploy), preventing new state from being created after the snapshot.

---

## Post-Upgrade Hash Verification (#846)

After any upgrade, verify that payout commitment hashes are unchanged to confirm
no state corruption occurred during the WASM swap or migration step.

### Automated (unit tests — no network required)

```bash
cargo test test_migrate_preserves_commitment_hashes -- --nocapture
```

This test:
1. Creates remittances and records their commitment hashes.
2. Runs `migration::migrate()` directly.
3. Re-fetches each hash and asserts byte-for-byte equality.

### Manual (testnet)

For every in-flight remittance ID known prior to the upgrade:

```bash
# Before upgrade — save to file
stellar contract invoke --id <CONTRACT_ID> \
  -- get_settlement_hash --remittance_id <ID> > hash_before_$ID.json

# After upgrade + migrate — compare
stellar contract invoke --id <CONTRACT_ID> \
  -- get_settlement_hash --remittance_id <ID> > hash_after_$ID.json

diff hash_before_$ID.json hash_after_$ID.json
```

A non-empty diff means state corruption — initiate rollback immediately.

### Post-Unpause Cooldown

After calling `emergency_unpause`, the contract automatically enters a 1-hour
cooldown window during which per-sender rate limits are halved.  The default
period is configurable:

```bash
# Set cooldown to 30 minutes (admin required)
stellar contract invoke --id <CONTRACT_ID> \
  -- set_cooldown_period \
  --caller <ADMIN_ADDRESS> \
  --seconds 1800

# Check current cooldown and last-unpause timestamp
stellar contract invoke --id <CONTRACT_ID> -- get_circuit_breaker_status
```

The cooldown period can also be changed via governance proposal
(`UpdateCooldownPeriod` action) without requiring a contract upgrade.

---

## Part 3 — Off-Chain Proof Validation (v0.1.0 → v0.2.0)

### Background

Contract version `0.2.0` introduces optional off-chain proof validation for
`confirm_payout`.  When a remittance is created with
`SettlementConfig { require_proof: true, oracle_address: Some(...) }`, the
agent must supply a valid 32-byte proof at payout time or the call is rejected
with `MissingProof` (52) / `InvalidProof` (51).

This change is **fully backward-compatible** for existing deployments.

---

### Impact on Existing Remittances

Remittances created on a `v0.1.0` contract (before proof validation was deployed)
have **no stored commitment** (`payout_commitment` field is `None` in persistent
storage).

When `confirm_payout` is called on such a remittance:

- If the remittance was created **without** a `SettlementConfig` (or with
  `require_proof = false`), settlement proceeds as it always has — **no change**.
- If the remittance was somehow created with `require_proof = true` on a
  `v0.1.0` binary that did not yet enforce it, the commitment will be absent.
  In this case, `v0.2.0` code skips the proof check (`stored commitment is None`
  → backward-compat bypass) and settles normally.

**In both cases existing remittances are unaffected.**  No data migration is
required.

---

### Storage Changes

| Key / Field | v0.1.0 | v0.2.0 | Notes |
|---|---|---|---|
| `Remittance.settlement_config` | absent | `Option<SettlementConfig>` | New optional field; absent in old records |
| `PayoutCommitment(id)` | absent | `Option<BytesN<32>>` | Written at creation when `require_proof = true` |

The `DataKey` variants for `PayoutCommitment` are new and do not collide with
any existing keys.  Old records remain readable without modification.

---

### No `migrate()` Call Required

Unlike the agent-key migration in Part 1, the proof-validation upgrade does not
change existing `DataKey` discriminants or struct layouts.  **You do not need to
call `migrate()` after upgrading to `v0.2.0`.**

A `migrate()` call on `v0.2.0` is still safe (idempotent) if triggered
accidentally.

---

### New Error Codes Introduced

| Code | Name | When thrown |
|------|------|-------------|
| 51 | `InvalidProof` | Proof supplied but does not match the stored commitment |
| 52 | `MissingProof` | `require_proof = true` but no proof was supplied |
| 53 | `InvalidOracleAddress` | `require_proof = true` but `oracle_address` is absent or invalid |

Off-chain consumers (indexers, backends, SDKs) should add handling for these
three new error codes.

---

### Upgrade Steps for Existing Deployments

1. **Build and optimize** the `v0.2.0` WASM:
   ```bash
   cargo build --target wasm32-unknown-unknown --release
   stellar contract optimize --wasm target/wasm32-unknown-unknown/release/swiftremit.wasm
   ```

2. **Install** the new WASM and note the hash:
   ```bash
   stellar contract install \
     --wasm target/wasm32-unknown-unknown/release/swiftremit.optimized.wasm \
     --network mainnet
   ```

3. **Upgrade** the live contract:
   ```bash
   stellar contract invoke --id <CONTRACT_ID> --network mainnet \
     -- upgrade --caller <ADMIN_ADDRESS> --new_wasm_hash <NEW_WASM_HASH>
   ```

4. **Verify** backward compatibility (see
   [DEPLOYMENT_CHECKLIST.md §2b](../DEPLOYMENT_CHECKLIST.md)):
   - Confirm a pre-existing remittance (without `require_proof`) still settles
     without a proof argument.
   - Create a new remittance with `require_proof = true` and confirm that the
     proof gate is enforced.

5. **Update off-chain services** to handle error codes 51–53 where they invoke
   `confirm_payout`.

---

### Rollback

If the upgrade must be rolled back, reinstall the `v0.1.0` WASM:

```bash
stellar contract invoke --id <CONTRACT_ID> --network mainnet \
  -- upgrade --caller <ADMIN_ADDRESS> --new_wasm_hash <V0_1_0_WASM_HASH>
```

Any remittances created with `require_proof = true` on `v0.2.0` will be
unmigrateable back to `v0.1.0` (the old binary does not know the `settlement_config`
field).  Those remittances can still be cancelled by the sender to recover escrowed
funds; they cannot be settled on the rolled-back binary.

Keep the `v0.1.0` WASM hash on hand before upgrading so this step is possible
without a recompile.

---

### References

- Spec: `kiro/specs/off-chain-verification-proof-validation/`
- Full design: [`docs/PROOF_VALIDATION.md`](PROOF_VALIDATION.md)
- Deployment checklist: [`DEPLOYMENT_CHECKLIST.md`](../DEPLOYMENT_CHECKLIST.md)
- Issues: #1495, #1498, #1499, #1500, #1527, #1528, #1531, #1532, #1533, #1534
