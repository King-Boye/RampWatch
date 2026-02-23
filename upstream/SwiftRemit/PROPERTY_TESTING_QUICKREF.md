# Property-Based Testing Quick Reference

## Quick Start

```bash
# Run all property tests (50 cases each)
cargo test test_property

# Run with more cases for thorough testing
PROPTEST_CASES=200 cargo test test_property

# Run specific invariant test
cargo test prop_no_balance_creation

# Run with verbose output
cargo test test_property -- --nocapture
```

## Test Categories

### Balance Conservation Tests
```bash
cargo test prop_no_balance_creation_on_create
cargo test prop_no_balance_creation_on_settlement
cargo test prop_no_balance_creation_on_cancel
```

### Non-Negative Invariants
```bash
cargo test prop_no_negative_balances
cargo test prop_payout_amount_non_negative
```

### Determinism Tests
```bash
cargo test prop_netting_order_independence
```

### Fee Correctness Tests
```bash
cargo test prop_fee_calculation_accuracy
cargo test prop_accumulated_fees_correctness
```

### State Machine Tests
```bash
cargo test prop_valid_state_transitions
cargo test prop_cancel_state_transition
```

### Duplicate Prevention Tests
```bash
cargo test prop_no_duplicate_settlement
```

### Net Settlement Tests
```bash
cargo test prop_netting_preserves_fees
```

## Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `PROPTEST_CASES` | Number of test cases per property | `PROPTEST_CASES=1000` |
| `PROPTEST_MAX_SHRINK_ITERS` | Max shrinking iterations | `PROPTEST_MAX_SHRINK_ITERS=10000` |
| `PROPTEST_SEED` | Replay specific test seed | `PROPTEST_SEED=xs16s...` |

## Common Commands

### Development
```bash
# Quick validation (10 cases)
PROPTEST_CASES=10 cargo test test_property

# Standard testing (50 cases - default)
cargo test test_property
```

### CI/CD
```bash
# Thorough testing (200 cases)
PROPTEST_CASES=200 cargo test test_property

# Comprehensive testing (1000 cases)
PROPTEST_CASES=1000 cargo test test_property
```

### Debugging
```bash
# Replay failed test with seed
PROPTEST_SEED=xs16s1234567890abcdef cargo test prop_test_name

# Run single test with verbose output
cargo test prop_no_balance_creation -- --nocapture --test-threads=1
```

## Invariants Tested

| # | Invariant | Description |
|---|-----------|-------------|
| 1 | No Balance Creation | Total balance conserved across all operations |
| 2 | No Negative Settlements | All balances remain non-negative |
| 3 | Deterministic Results | Order-independent net settlement |
| 4 | Fee Calculation | Accurate and consistent fee computation |
| 5 | State Transitions | Valid state machine transitions only |
| 6 | Duplicate Prevention | Idempotent settlement operations |
| 7 | Net Settlement Conservation | Fees preserved during netting |

## Test Case Recommendations

| Environment | Cases | Duration | Use Case |
|-------------|-------|----------|----------|
| Development | 10-20 | ~5s | Quick feedback during coding |
| Pre-commit | 50 | ~15s | Standard validation |
| CI/CD | 200 | ~1m | Thorough automated testing |
| Release | 1000+ | ~5m | Comprehensive validation |

## Interpreting Results

### Success
```
test prop_no_balance_creation_on_create ... ok
```
Property holds for all generated test cases.

### Failure
```
test prop_no_balance_creation_on_create ... FAILED
thread 'prop_no_balance_creation_on_create' panicked at 'Test failed: Balance created during remittance creation'
minimal failing input: amount = 100, fee_bps = 50
```
Property violated. Proptest shows minimal failing case.

### Shrinking
```
Test failed with input: amount = 999999, fee_bps = 999
Shrinking to minimal case...
Minimal failing input: amount = 100, fee_bps = 100
```
Proptest automatically finds simplest failing case.

## Integration Examples

### Makefile
```makefile
test-property:
	cargo test test_property

test-property-thorough:
	PROPTEST_CASES=200 cargo test test_property

test-property-comprehensive:
	PROPTEST_CASES=1000 cargo test test_property
```

### GitHub Actions
```yaml
- name: Property-Based Tests
  run: PROPTEST_CASES=200 cargo test test_property
```

### GitLab CI
```yaml
property-tests:
  script:
    - PROPTEST_CASES=200 cargo test test_property
```

## Troubleshooting

### Test Timeout
```bash
# Reduce test cases
PROPTEST_CASES=20 cargo test test_property

# Increase timeout
cargo test test_property -- --test-threads=1 --nocapture
```

### Memory Issues
```bash
# Run tests sequentially
cargo test test_property -- --test-threads=1
```

### Flaky Tests
```bash
# Run multiple times with different seeds
for i in {1..10}; do cargo test test_property; done
```

## Performance Tips

1. **Parallel Execution**: Tests run in parallel by default
2. **Incremental Testing**: Start with fewer cases during development
3. **Focused Testing**: Run specific tests when debugging
4. **CI Optimization**: Use caching for dependencies

## Coverage Analysis

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Run with coverage
cargo tarpaulin --test test_property --out Html
```

## Next Steps

1. Review [PROPERTY_BASED_TESTING.md](./PROPERTY_BASED_TESTING.md) for detailed documentation
2. Examine [src/test_property.rs](./src/test_property.rs) for implementation
3. Add custom properties for domain-specific invariants
4. Integrate into CI/CD pipeline
5. Monitor for edge cases discovered

## Support

For issues or questions:
- Review test output and shrunk inputs
- Check proptest documentation: https://docs.rs/proptest/
- Examine test implementation in `src/test_property.rs`
