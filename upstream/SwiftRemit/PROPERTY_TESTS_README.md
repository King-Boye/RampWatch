# Property-Based Tests for SwiftRemit

## What Are Property-Based Tests?

Property-based tests validate that certain properties (invariants) hold true across thousands of randomly generated test cases. Unlike traditional unit tests that check specific examples, property tests prove that fundamental rules are never violated.

## Why Property-Based Testing?

Traditional test:
```rust
#[test]
fn test_fee_calculation() {
    assert_eq!(calculate_fee(1000, 250), 25); // One specific case
}
```

Property test:
```rust
#[test]
fn prop_fee_calculation(amount: i128, fee_bps: u32) {
    let fee = calculate_fee(amount, fee_bps);
    assert!(fee >= 0);           // Always non-negative
    assert!(fee <= amount);      // Never exceeds amount
    assert_eq!(fee, (amount * fee_bps) / 10000); // Correct formula
}
// Automatically tests 50+ random combinations!
```

## Quick Start

### Run All Property Tests
```bash
cargo test test_property --lib
```

### Run with More Test Cases
```bash
PROPTEST_CASES=200 cargo test test_property --lib
```

### Run Specific Test
```bash
cargo test prop_no_balance_creation --lib
```

## What's Being Tested?

### 1. No Balance Creation ✅
**Property**: Money can't be created or destroyed

```
Total balance before = Total balance after
```

**Why it matters**: Prevents the contract from creating tokens out of thin air.

### 2. No Negative Settlements ✅
**Property**: All balances stay non-negative

```
∀ balance: balance ≥ 0
```

**Why it matters**: Negative balances would indicate arithmetic errors or potential exploits.

### 3. Deterministic Results ✅
**Property**: Order doesn't matter

```
compute_net([A, B, C]) = compute_net([C, B, A])
```

**Why it matters**: Ensures fair and predictable netting regardless of transaction order.

### 4. Fee Calculation Correctness ✅
**Property**: Fees are calculated accurately

```
fee = (amount × fee_bps) / 10000
payout + fee = amount
```

**Why it matters**: Incorrect fees could lead to loss of funds.

### 5. State Transitions ✅
**Property**: Only valid state changes allowed

```
Pending → Settled ✓
Pending → Failed ✓
Settled → Pending ✗
```

**Why it matters**: Invalid transitions could allow double-spending.

### 6. Duplicate Prevention ✅
**Property**: Can't settle twice

```
settle(id); settle(id) → ERROR
```

**Why it matters**: Prevents agents from receiving multiple payouts.

### 7. Net Settlement Conservation ✅
**Property**: Netting preserves fees

```
Σ(original_fees) = Σ(net_fees)
```

**Why it matters**: Ensures accounting integrity during optimization.

## Test Output Examples

### ✅ Success
```
test prop_no_balance_creation_on_create ... ok
test prop_no_negative_balances ... ok
test prop_fee_calculation_accuracy ... ok

test result: ok. 15 passed; 0 failed
```

### ❌ Failure (with automatic shrinking)
```
test prop_no_balance_creation ... FAILED

Test failed with input:
  amount = 999999
  fee_bps = 999

Shrinking to minimal case...

Minimal failing input:
  amount = 100
  fee_bps = 50

Error: Balance created during remittance creation
  Expected: 10000
  Got: 10001
```

Proptest automatically finds the simplest case that fails!

## Configuration

### Test Case Counts

| Environment | Cases | Duration | Use Case |
|-------------|-------|----------|----------|
| Development | 10 | ~5s | Quick feedback |
| Default | 50 | ~15s | Standard testing |
| CI/CD | 200 | ~1m | Thorough validation |
| Release | 1000+ | ~5m | Comprehensive check |

### Environment Variables

```bash
# Number of test cases per property
PROPTEST_CASES=100 cargo test test_property --lib

# Replay specific failure
PROPTEST_SEED=xs16s1234567890abcdef cargo test prop_test_name --lib

# Maximum shrinking iterations
PROPTEST_MAX_SHRINK_ITERS=10000 cargo test test_property --lib
```

## Integration Examples

### Makefile
```makefile
.PHONY: test-property test-property-quick test-property-thorough

test-property:
	cargo test test_property --lib

test-property-quick:
	PROPTEST_CASES=10 cargo test test_property --lib

test-property-thorough:
	PROPTEST_CASES=200 cargo test test_property --lib
```

### GitHub Actions
```yaml
name: Property Tests

on: [push, pull_request]

jobs:
  property-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run property tests
        run: PROPTEST_CASES=200 cargo test test_property --lib
```

### Pre-commit Hook
```bash
#!/bin/bash
# .git/hooks/pre-commit

echo "Running property-based tests..."
PROPTEST_CASES=50 cargo test test_property --lib

if [ $? -ne 0 ]; then
    echo "Property tests failed. Commit aborted."
    exit 1
fi
```

## Debugging Failed Tests

### Step 1: Note the Failure
```
Minimal failing input: amount = 100, fee_bps = 50
```

### Step 2: Replay with Seed
```bash
PROPTEST_SEED=xs16s1234567890abcdef cargo test prop_test_name --lib
```

### Step 3: Add Logging
```rust
prop_assert_eq!(
    expected, actual,
    "Expected: {}, Got: {}, Amount: {}, Fee BPS: {}",
    expected, actual, amount, fee_bps
);
```

### Step 4: Fix and Verify
```bash
# Run the specific test
cargo test prop_test_name --lib

# Run all property tests
cargo test test_property --lib
```

## Best Practices

### ✅ Do
- Run property tests regularly during development
- Use 50+ cases for standard testing
- Use 200+ cases in CI/CD
- Document discovered edge cases
- Keep test execution time reasonable

### ❌ Don't
- Skip property tests (they catch real bugs!)
- Use too few test cases (< 10)
- Ignore shrunk minimal cases
- Test implementation details (test properties!)

## Common Questions

### Q: How many test cases should I use?
**A**: Start with 10 during development, use 50 for regular testing, and 200+ for CI/CD.

### Q: Why did my test fail randomly?
**A**: Property tests explore random inputs. Use the seed to replay the exact failure.

### Q: How long should tests take?
**A**: 10 cases: ~5s, 50 cases: ~15s, 200 cases: ~1m. Adjust based on your needs.

### Q: Can I test specific values?
**A**: Yes! Use traditional unit tests for specific cases, property tests for general invariants.

### Q: What if tests are too slow?
**A**: Reduce test cases during development, use more in CI/CD. Run specific tests when debugging.

## Files and Documentation

| File | Purpose |
|------|---------|
| `src/test_property.rs` | Test implementation |
| `PROPERTY_BASED_TESTING.md` | Detailed documentation |
| `PROPERTY_TESTING_QUICKREF.md` | Command reference |
| `PROPERTY_TESTING_SUMMARY.md` | Implementation summary |
| `examples/run_property_tests.sh` | Demo script |

## Next Steps

1. **Run the tests**: `cargo test test_property --lib`
2. **Read the docs**: See `PROPERTY_BASED_TESTING.md`
3. **Integrate into CI**: Add to your pipeline
4. **Add custom properties**: Test domain-specific invariants

## Support

For detailed information:
- **Quick commands**: See `PROPERTY_TESTING_QUICKREF.md`
- **Deep dive**: See `PROPERTY_BASED_TESTING.md`
- **Implementation**: See `src/test_property.rs`
- **Proptest docs**: https://docs.rs/proptest/

## Summary

Property-based testing provides mathematical confidence that critical invariants hold across the entire input space. The 15 property tests in this suite validate 7 fundamental properties, automatically generating thousands of test cases to ensure contract correctness and security.

**Run the tests now**: `cargo test test_property --lib`
