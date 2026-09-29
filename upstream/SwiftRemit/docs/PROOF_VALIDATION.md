# Proof Validation

Off-chain proof validation for SwiftRemit settlements. Ensures that `confirm_payout` cannot execute without cryptographic proof that off-chain or oracle conditions were met.

## Overview

Proof validation closes the gap between on-chain settlement execution and off-chain reality. Without it, an agent could call `confirm_payout` and receive funds without actually completing the fiat payout to the recipient. With proof validation enabled, the agent must supply a valid 32-byte proof that matches the commitment computed at remittance creation time.

## How It Works

### Settlement Configuration

When creating a remittance, the sender can optionally attach a `SettlementConfig`:

```rust
pub struct SettlementConfig {
    /// Whether proof validation is required for this settlement
    pub require_proof: bool,
    /// Oracle/signer address for proof validation (required if require_proof is true)
    pub oracle_address: Option<Address>,
}
```

- `require_proof = false` (or no config): `confirm_payout` works as before, no proof needed.
- `require_proof = true`: `confirm_payout` requires a valid proof. The `oracle_address` must be set at creation time or the call fails with `InvalidOracleAddress`.

### Commitment Computation

At remittance creation, the contract computes a **payout commitment** — a SHA-256 hash of the canonical serialization of the remittance fields:

1. `remittance.id` (u64, big-endian)
2. `remittance.sender` (Address XDR bytes)
3. `remittance.agent` (Address XDR bytes)
4. `remittance.amount` (i128, big-endian)
5. `remittance.fee` (i128, big-endian)
6. `remittance.expiry` (u64, big-endian, 0 if None)

This commitment is stored in persistent storage and serves as the expected proof value.

### Proof Verification at Payout

When `confirm_payout` is called on a remittance with `require_proof = true`:

1. If no proof is supplied, the call fails with `MissingProof` (error 52).
2. If a proof is supplied, it is compared against the stored commitment using **constant-time comparison**.
3. If the proof does not match, the call fails with `InvalidProof` (error 51).
4. If the proof matches (or no commitment is stored for backward compatibility), settlement proceeds.

## Security Properties

### Timing-Attack Resistance (#1527)

Proof comparison uses a constant-time byte comparison (`subtle::ConstantTimeEq` pattern) rather than a naive `==` check. This prevents attackers from learning how many bytes of their proof match the expected value through timing side-channels.

### Replay Prevention (#1528)

The commitment is bound to the specific remittance ID and all remittance fields. A proof that is valid for remittance N will not be accepted for remittance M, because the commitments differ. This prevents replay attacks where a valid proof from one settlement is reused for another.

### Backward Compatibility

Remittances created before proof validation was introduced have no stored commitment. When `confirm_payout` is called on such a remittance with a proof supplied, the proof check is skipped (the stored commitment is `None`), maintaining backward compatibility.

## Error Codes

| Code | Name | Description |
|------|------|-------------|
| 51 | `InvalidProof` | The submitted proof does not match the expected commitment. |
| 52 | `MissingProof` | Proof is required (`require_proof = true`) but was not provided. |
| 53 | `InvalidOracleAddress` | `require_proof = true` but no `oracle_address` was set at creation. |

## Contract Functions

### `create_remittance` with proof validation

```rust
let settlement_config = SettlementConfig {
    require_proof: true,
    oracle_address: Some(oracle_address),
};

let remittance_id = contract.create_remittance(
    &sender,
    &agent,
    amount,
    expiry,
    token,
    None,          // idempotency_key
    Some(settlement_config),
    None,          // recipient_hash
    None,          // integrator
);
```

### `confirm_payout` with proof

```rust
// Compute the expected proof (off-chain, using the same algorithm)
let proof = compute_payout_commitment(&env, &remittance);

// Submit with proof
contract.confirm_payout(&agent, &remittance_id, &Some(proof), &None);
```

## Off-Chain Proof Generation

The proof is the SHA-256 hash of the canonical serialization of the remittance fields. Off-chain services (oracles, agents) must compute this hash using the same algorithm:

```
proof = SHA-256(
    remittance.id (u64, big-endian) ||
    remittance.sender (Address XDR bytes) ||
    remittance.agent (Address XDR bytes) ||
    remittance.amount (i128, big-endian) ||
    remittance.fee (i128, big-endian) ||
    remittance.expiry (u64, big-endian, 0 if None)
)
```

The contract exposes `compute_settlement_hash(remittance_id)` as a public query that returns the deterministic settlement hash for a remittance, which can be used to verify proof computation off-chain.

## Related Documentation

- [Transaction State Machine](TRANSACTION_STATE_MACHINE.md) — Remittance lifecycle
- [Events](EVENTS.md) — Contract event catalogue
- [API Reference](../API.md) — Full contract API
