# SwiftRemit Deployment Checklist

Use this checklist for every mainnet or testnet deployment.  Work through each
section in order; do not proceed to the next section until all items in the
current section are checked.

---

## 1. Pre-Deployment Verification

### Build & Tests
- [ ] `cargo build --target wasm32-unknown-unknown --release` exits 0.
- [ ] `cargo test` exits 0 — all unit and property tests pass.
- [ ] `cargo clippy -- -D warnings` exits 0.
- [ ] The compiled `.wasm` artifact size is within the Soroban limits (≤ 64 KB after `stellar contract optimize`).

### Code Review
- [ ] All changes since the previous deployment have been reviewed by at least one engineer other than the author.
- [ ] No `TODO` / `FIXME` markers remain in security-critical files (`src/validation.rs`, `src/abuse_protection.rs`, `src/rate_limit.rs`, `src/governance.rs`, `src/multisig.rs`).
- [ ] `CHANGELOG.md` has been updated with the release notes for this version.

---

## 2. Proof Validation Verification Steps

These steps must be completed whenever the contract contains proof-validation
logic (i.e., any deployment of `v0.2.0` or later that ships `src/verification.rs`).

### 2a. Check proof-validation feature flags
- [ ] Confirm `src/verification.rs` is present and `compute_payout_commitment` is exported.
- [ ] Confirm `ContractError::InvalidProof` (51), `ContractError::MissingProof` (52), and `ContractError::InvalidOracleAddress` (53) are defined in `src/errors.rs`.
- [ ] Confirm `SettlementConfig` and its `require_proof` / `oracle_address` fields are defined in `src/types.rs`.

### 2b. Smoke-test proof validation on testnet (before mainnet)
Run the following sequence against the freshly deployed testnet contract:

```bash
# 1. Create a remittance WITHOUT proof requirement and confirm payout normally.
stellar contract invoke --id <CONTRACT_ID> --network testnet \
  -- create_remittance \
  --sender <SENDER> --agent <AGENT> --amount 100 \
  --expiry null --token null --idempotency_key null \
  --settlement_config null --recipient_hash null

# Note the returned remittance_id, then:
stellar contract invoke --id <CONTRACT_ID> --network testnet \
  -- confirm_payout --remittance_id <ID> --proof null --recipient_details_hash null
# Expected: success (Completed status).

# 2. Create a remittance WITH proof requirement.
stellar contract invoke --id <CONTRACT_ID> --network testnet \
  -- create_remittance \
  --sender <SENDER> --agent <AGENT> --amount 100 \
  --settlement_config '{"require_proof":true,"oracle_address":"<ORACLE_ADDRESS>"}'

# 3. Attempt confirm_payout WITHOUT proof → must fail.
stellar contract invoke --id <CONTRACT_ID> --network testnet \
  -- confirm_payout --remittance_id <ID> --proof null --recipient_details_hash null
# Expected: ContractError::MissingProof (52).

# 4. Attempt confirm_payout WITH a bad proof (32 zero bytes) → must fail.
stellar contract invoke --id <CONTRACT_ID> --network testnet \
  -- confirm_payout --remittance_id <ID> \
  --proof '{"bytes":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA="}' \
  --recipient_details_hash null
# Expected: ContractError::InvalidProof (51).

# 5. Compute the correct proof off-chain and confirm payout → must succeed.
#    Use examples/proof-validation-example.js for the computation:
node examples/proof-validation-example.js --remittance-id <ID> --rpc-url <RPC_URL> --contract-id <CONTRACT_ID>
# Supply the printed proof bytes to confirm_payout; expected: Completed status.
```

- [ ] Step 1 passes (backward-compatible path works).
- [ ] Step 3 returns `MissingProof` (error code 52).
- [ ] Step 4 returns `InvalidProof` (error code 51).
- [ ] Step 5 settles the remittance successfully.

### 2c. Pause + proof validation interaction
- [ ] Pause the testnet contract (`emergency_pause` or legacy `pause`).
- [ ] Attempt `confirm_payout` with a valid proof against a pending remittance.
  - Expected: `ContractError::ContractPaused` (13).
- [ ] Unpause the contract.
- [ ] Re-attempt `confirm_payout` with the **same** valid proof.
  - Expected: success (Completed status) — the pause must not have invalidated the proof.
- [ ] Verify the remittance is `Completed` via `get_remittance`.

### 2d. Duplicate-settlement guard with proof
- [ ] Attempt `confirm_payout` a second time on an already-`Completed` remittance.
  - Expected: `ContractError::DuplicateSettlement` (12).
- [ ] Attempt the same with a **different** 32-byte proof value.
  - Expected: still `ContractError::DuplicateSettlement` (12) — proof content is irrelevant once settled.

### 2e. Rate-limiting interaction
- [ ] Confirm that a rejected proof call (`InvalidProof`) does **not** consume a rate-limit slot (verify via `get_rate_limit_status` before and after).
- [ ] Confirm that a successful `confirm_payout` with a valid proof increments the rate-limit counter by exactly 1.

---

## 3. Storage TTL Verification
- [ ] Run `get_remittance` on a recently created remittance and confirm the expiry is plausible.
- [ ] Run `extend_storage_ttl` for the freshly deployed contract to ensure instance storage will not expire within the expected operational window.

---

## 4. Agent & Fee Configuration
- [ ] Register at least one agent: `register_agent --agent <ADDRESS>`.
- [ ] Verify the agent appears in `is_agent_registered`.
- [ ] Confirm `get_platform_fee_bps` returns the intended fee.
- [ ] Confirm `get_protocol_fee_bps` returns the intended protocol fee.

---

## 5. Circuit Breaker Sanity Check
- [ ] `is_paused` returns `false` on the freshly deployed contract.
- [ ] `get_circuit_breaker_status` returns expected defaults (timelock 0, quorum 1, no active pause).

---

## 6. Post-Deployment Monitoring
- [ ] Grafana dashboard is pointed at the new contract address.
- [ ] Alerting rules for `ContractPaused`, `InvalidProof`, and `MissingProof` events are active.
- [ ] On-call rotation has been notified of the deployment window.

---

## 7. Rollback Plan
If any of the above checks fail:

1. Do **not** redirect production traffic to the new deployment.
2. If state was already written, run `export_migration_snapshot` on the new contract to preserve in-flight remittances.
3. Redeploy the previous WASM hash:
   ```bash
   stellar contract invoke --id <CONTRACT_ID> \
     -- upgrade --caller <ADMIN> --new_wasm_hash <PREVIOUS_WASM_HASH>
   ```
4. Follow the full rollback procedure in [`docs/ROLLBACK_RUNBOOK.md`](docs/ROLLBACK_RUNBOOK.md).

---

*This checklist is tracked as part of the Off-Chain Proof Validation implementation
(issues #1532, spec: `.kiro/specs/off-chain-verification-proof-validation/`).*
