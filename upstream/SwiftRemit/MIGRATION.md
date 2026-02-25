# Migration Guide: SwiftRemit Contract

This guide outlines the key changes and migration steps for upgrading to the latest version of the SwiftRemit contract, focusing on deterministic settlement hashes and batch settlement features.

## New Features

### 1. Deterministic Settlement Hashing
The contract now utilizes a deterministic hashing mechanism for all settlements. This allows off-chain systems to pre-calculate settlement IDs and verify on-chain state with cryptographic certainty.

- **Internal Logic**: Uses SHA-256 over canonicalized fields (sender, agent, amount, fee, expiry).
- **Public API**: `compute_settlement_hash(env, remittance_id)` allows external callers to retrieve the expected hash for any pending remittance.

### 2. Batch Settlement with Netting
To optimize gas costs and reduce token transfer overhead, the contract now supports batch settlement of multiple remittances with netting logic.

- **Net Settlement**: Offsets opposing flows between the same two parties within a single batch.
- **Max Batch Size**: 50 remittances per transaction.
- **Function**: `batch_settle_with_netting(env, entries)`.

## Breaking Changes

### Function Signatures
The `create_remittance` method has been simplified. The `default_currency` and `default_country` arguments have been removed in favor of a simpler 4-argument signature (plus `Env`).

**Old Signature:**
```rust
pub fn create_remittance(
    env: Env,
    sender: Address,
    agent: Address,
    amount: i128,
    currency: String,
    country: String,
    expiry: Option<u64>,
) -> Result<u64, ContractError>
```

**New Signature:**
```rust
pub fn create_remittance(
    env: Env,
    sender: Address,
    agent: Address,
    amount: i128,
    expiry: Option<u64>,
) -> Result<u64, ContractError>
```

### Authorization Model
The `authorize_remittance` function has been removed. Payout confirmation is now handled directly via `confirm_payout`, which requires `require_auth` from the agent and the `Settler` role.

## Migration Steps

1. **Update Clients**: Update all off-chain clients to use the new `create_remittance` signature.
2. **Assign Roles**: Ensure all authorized agents are assigned the `Role::Settler` using the `assign_role` function.
3. **Verify Hashes**: Use `compute_settlement_hash` to reconcile existing pending transactions if necessary.
