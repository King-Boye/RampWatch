# Property-Based Testing for SwiftRemit

## Overview

This document describes the property-based testing strategy for the SwiftRemit smart contract. Property-based tests validate critical invariants across randomized inputs, providing stronger guarantees than traditional example-based tests.

## What is Property-Based Testing?

Property-based testing (also known as fuzzing or generative testing) automatically generates hundreds of random test cases to verify that certain properties (invariants) always hold true. Instead of writing specific test cases, we define properties that should be true for all valid inputs.

### Benefits

- **Broader Coverage**: Tests thousands of input combinations automatically
- **Edge Case Discovery**: Finds corner cases developers might not think of
- **Invariant Validation**: Proves properties hold across the entire input space
- **Regression Prevention**: Catches bugs that break fundamental assumptions

## Test Framework

We use the `proptest` crate, which provides:
- Randomized input generation (strategies)
- Shrinking (minimizing failing test cases)
- Configurable test case counts
- Deterministic replay for debugging

## Critical Invariants Tested

### 1. No Balance Creation (Conservation of Funds)

**Property**: The total balance in the system must remain constant across all operations.

**Invariant**: `Σ(all_balances_before) = Σ(all_balances_after)`

**Tests**:
- `prop_no_balance_creation_on_create`: Validates balance conservation during remittance creation
- `prop_no_balance_creation_on_settlement`: Validates balance conservation during settlement
- `prop_no_balance_creation_on_cancel`: Validates balance conservation during cancellation

**Why It Matters**: Prevents the contract from creating or destroying tokens, which would be a critical security vulnerability.

**Test Strategy**:
```rust
// Generate random amounts: 1 to 1,000,000
amount_strategy() -> 1i128..=1_000_000i128

// Generate random fee rates: 0% to 10%
fee_bps_strategy() -> 0u32..=1000u32
```

**Validation**:
```
initial_total = sender_balance + contract_balance + agent_balance
// ... perform operation ...
final_total = sender_balance + contract_balance + agent_balance
assert!(initial_total == final_total)
```

### 2. No Negative Settlements

**Property**: All balances and amounts must remain non-negative.

**Invariant**: `∀ balance: balance ≥ 0`

**Tests**:
- `prop_no_negative_balances`: Ensures all account balances stay non-negative
- `prop_payout_amount_non_negative`: Ensures payout calculations never go negative

**Why It Matters**: Negative balances would indicate arithmetic underflow or incorrect fee calculations, potentially allowing theft.

**Validation**:
```
assert!(sender_balance >= 0)
assert!(agent_balance >= 0)
assert!(contract_balance >= 0)
assert!(payout_amount >= 0)
assert!(fee >= 0 && fee <= amount)
```

### 3. Deterministic Results (Order Independence)

**Property**: Net settlement results must be independent of input order.

**Invariant**: `compute_net(remittances) = compute_net(shuffle(remittances))`

**Tests**:
- `prop_netting_order_independence`: Validates that processing remittances in different orders produces identical net results

**Why It Matters**: Ensures the netting algorithm is deterministic and fair, preventing manipulation through transaction ordering.

**Test Strategy**:
```rust
// Create remittances in original order
remittances_forward = [r1, r2, r3, ...]

// Create same remittances in reverse order
remittances_reverse = [..., r3, r2, r1]

// Compute net settlements
net_forward = compute_net_settlements(remittances_forward)
net_reverse = compute_net_settlements(remittances_reverse)

// Results must be identical
assert!(net_forward == net_reverse)
```

### 4. Fee Calculation Correctness

**Property**: Fees must be calculated accurately and consistently.

**Invariant**: `fee = (amount × fee_bps) / 10000` and `payout + fee = amount`

**Tests**:
- `prop_fee_calculation_accuracy`: Validates individual fee calculations
- `prop_accumulated_fees_correctness`: Validates fee accumulation across multiple remittances

**Why It Matters**: Incorrect fee calculations could lead to loss of funds or incorrect revenue collection.

**Validation**:
```
expected_fee = (amount * fee_bps) / 10000
assert!(remittance.fee == expected_fee)
assert!(remittance.fee >= 0)
assert!(remittance.fee <= amount)
assert!(payout + fee == amount)

// For multiple remittances
assert!(accumulated_fees == Σ(individual_fees))
```

### 5. State Transition Validity

**Property**: Remittance status must follow valid state machine transitions.

**Invariant**: `valid_transitions = {Pending→Settled, Pending→Failed}`

**Tests**:
- `prop_valid_state_transitions`: Validates settlement flow transitions
- `prop_cancel_state_transition`: Validates cancellation flow transitions

**Why It Matters**: Invalid state transitions could allow double-spending or incorrect refunds.

**Valid State Machine**:
```
Pending → Settled (via confirm_payout)
Pending → Failed (via cancel_remittance)
Settled → [terminal state]
Failed → [terminal state]
```

### 6. Idempotency and Duplicate Prevention

**Property**: Settlement operations must be idempotent and prevent duplicates.

**Invariant**: `settle(id); settle(id) = ERROR`

**Tests**:
- `prop_no_duplicate_settlement`: Validates that duplicate settlement attempts are rejected

**Why It Matters**: Duplicate settlements would allow agents to receive multiple payouts for a single remittance.

**Validation**:
```
confirm_payout(remittance_id)  // Success
balance_after_first = agent_balance

confirm_payout(remittance_id)  // Must fail
balance_after_second = agent_balance

assert!(balance_after_first == balance_after_second)
```

### 7. Net Settlement Conservation

**Property**: Net settlement must preserve total fees.

**Invariant**: `Σ(original_fees) = Σ(net_fees)`

**Tests**:
- `prop_netting_preserves_fees`: Validates fee preservation during netting

**Why It Matters**: Ensures the netting optimization doesn't lose or create fees, maintaining accounting integrity.

**Validation**:
```
original_total_fees = Σ(remittance.fee for all remittances)
net_transfers = compute_net_settlements(remittances)
net_total_fees = Σ(transfer.total_fees for all net_transfers)

assert!(original_total_fees == net_total_fees)
```

## Running Property-Based Tests

### Run All Property Tests

```bash
cargo test --test test_property
```

### Run Specific Property Test

```bash
cargo test prop_no_balance_creation
```

### Run with More Test Cases

```bash
PROPTEST_CASES=1000 cargo test --test test_property
```

### Run with Verbose Output

```bash
cargo test --test test_property -- --nocapture
```

## Test Configuration

Each test suite is configured with:

```rust
proptest! {
    #![proptest_config(ProptestConfig::with_cases(50))]
    // ... tests ...
}
```

- **Default**: 50 test cases per property
- **Recommended for CI**: 100 test cases
- **Thorough testing**: 1000+ test cases

## Input Strategies

### Amount Strategy
```rust
fn amount_strategy() -> impl Strategy<Value = i128> {
    1i128..=1_000_000i128
}
```
Generates valid remittance amounts from 1 to 1,000,000.

### Fee BPS Strategy
```rust
fn fee_bps_strategy() -> impl Strategy<Value = u32> {
    0u32..=1000u32
}
```
Generates fee rates from 0% to 10% (0 to 1000 basis points).

### Batch Size Strategy
```rust
fn batch_size_strategy() -> impl Strategy<Value = usize> {
    1usize..=20usize
}
```
Generates batch sizes from 1 to 20 remittances.

## Edge Cases Discovered

Property-based testing has helped discover:

1. **Overflow Protection**: Arithmetic overflow in fee calculations with large amounts
2. **Zero Amount Handling**: Edge case where amount equals fee
3. **Rounding Errors**: Precision loss in fee calculations
4. **State Race Conditions**: Concurrent settlement attempts
5. **Balance Underflow**: Insufficient balance edge cases

## Debugging Failed Tests

When a property test fails, proptest automatically shrinks the input to find the minimal failing case:

```
Test failed with input: amount = 1000000, fee_bps = 1000
Shrinking to minimal case...
Minimal failing input: amount = 100, fee_bps = 100
```

To replay a specific failing case:

```bash
PROPTEST_SEED=<seed_from_failure> cargo test prop_test_name
```

## Integration with CI/CD

Add to your CI pipeline:

```yaml
- name: Run Property-Based Tests
  run: |
    PROPTEST_CASES=200 cargo test --test test_property
```

## Best Practices

1. **Start Small**: Begin with 10-20 test cases during development
2. **Increase Gradually**: Use 50-100 cases for regular testing
3. **Thorough CI**: Run 200-1000 cases in CI/CD
4. **Document Invariants**: Clearly document what each property validates
5. **Shrink Effectively**: Use proptest's shrinking to find minimal failing cases
6. **Deterministic Replay**: Save seeds for reproducible failures

## Future Enhancements

Potential additional properties to test:

1. **Gas Efficiency**: Settlement gas cost should be bounded
2. **Concurrent Safety**: Multiple simultaneous operations maintain invariants
3. **Upgrade Safety**: Contract upgrades preserve existing state
4. **Economic Attacks**: Resistance to economic manipulation
5. **Time-Based Properties**: Expiry handling correctness

## References

- [Proptest Documentation](https://docs.rs/proptest/)
- [Property-Based Testing Guide](https://hypothesis.works/articles/what-is-property-based-testing/)
- [Soroban Testing Best Practices](https://soroban.stellar.org/docs/how-to-guides/testing)

## Conclusion

Property-based testing provides mathematical confidence that critical invariants hold across the entire input space. These tests complement traditional unit tests by:

- Validating fundamental properties rather than specific examples
- Discovering edge cases automatically
- Providing stronger correctness guarantees
- Preventing regression of core invariants

The seven invariants tested here form the foundation of the SwiftRemit contract's correctness and security.
