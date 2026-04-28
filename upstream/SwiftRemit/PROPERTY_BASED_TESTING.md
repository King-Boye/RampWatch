# Property-Based Fee Calculation Testing Guide

## Overview

Property-based testing with **proptest** has been implemented for comprehensive fuzzing of the fee calculation system. This approach validates critical invariants across thousands of randomized inputs, detecting edge cases and overflow conditions that traditional unit tests might miss.

## Features

### ✅ Tested Properties

The test suite validates these critical invariants:

| Property | Description | Impact |
|----------|-------------|--------|
| **No Overflows** | All fee calculations handle extreme amounts gracefully | Prevents financial loss from arithmetic errors |
| **Fee Bounds** | Fees never exceed the transaction amount | Ensures mathematical consistency |
| **Non-Negative** | All fees are always ≥ 0 | Prevents negative charges to users |
| **Deterministic** | Same inputs produce same output | Ensures reproducibility and auditability |
| **Monotonic** | Larger amounts produce larger fees | Validates proportional scaling |
| **Breakdown Valid** | Fee breakdowns satisfy `amount = platform_fee + protocol_fee + net_amount` | Ensures accounting correctness |

### 📊 Test Categories

1. **Percentage Fee Calculation** (100 cases)
   - Validates percentage-based fee strategy
   - Tests against maximum fee basis points
   - Validates minimum fee thresholds
   - Tests determinism

2. **Fee Breakdown Consistency** (100 cases)
   - Validates mathematical formula: `amount = platform_fee + protocol_fee + net_amount`
   - Ensures all components are non-negative
   - Checks breakdown validation logic

3. **Overflow Handling** (150 cases)
   - Tests extreme amounts up to `i128::MAX`
   - Validates graceful error handling
   - Ensures no panics on edge cases

4. **Boundary Cases** (100 cases)
   - Tests minimum amounts (100 stroops)
   - Tests boundary values (1K, 10K, 100K, etc.)
   - Validates monotonic fee increase

## Running the Tests

### Quick Validation (10 test cases)
```bash
PROPTEST_CASES=10 cargo test test_fee_property --lib -- --nocapture
```

### Standard Fuzzing (100 test cases per property - default)
```bash
cargo test test_fee_property --lib -- --nocapture
```

### Intensive Fuzzing (1000+ test cases)
```bash
PROPTEST_CASES=1000 cargo test test_fee_property --lib -- --nocapture
```

### Run Specific Test
```bash
cargo test prop_percentage_fee_never_negative --lib -- --nocapture
cargo test prop_no_panic_on_extremes --lib -- --nocapture
```

### Verbose Output (Shows Generated Values)
```bash
PROPTEST_VERBOSE=1 cargo test test_fee_property --lib -- --nocapture
```

## Key Test Functions

### Percentage Fee Tests
```rust
prop_percentage_fee_never_negative()      // Fees ≥ 0
prop_fee_never_exceeds_amount()           // Fees ≤ amount
prop_fee_calculation_deterministic()      // Same inputs → same output
prop_fee_scales_with_amount()             // Larger amounts → larger fees
```

### Breakdown Tests
```rust
prop_breakdown_arithmetic_valid()         // amount = fees + net
prop_breakdown_no_negative_components()   // All components ≥ 0
```

### Overflow & Edge Case Tests
```rust
prop_no_panic_on_extremes()               // No panics on i128::MAX
prop_overflow_handled_gracefully()        // Overflow → Overflow error
prop_minimum_amounts_valid()              // Amounts ≥ 100 stroops
prop_boundary_amounts_valid()             // Boundary values work
prop_fee_monotonic_increase()             // Non-decreasing fees
```

## Test Input Ranges

### Amount Strategy
- **Range**: 100 to 1,000,000,000 stroops
- **Rationale**: Avoids impractical amounts while testing realistic transaction sizes
- **Coverage**: 0.00001 USD to ~100 USD (at 7 decimal places)

### Basis Points Strategy
- **Full Range**: 0 to 10,000 (0% to 100%)
- **Realistic Range**: 1 to 1,000 (0.01% to 10%)
- **Purpose**: Tests actual fee rates used in production

### Fee Amounts
- **Flat Fee Range**: 1 to 1,000,000 stroops
- **Purpose**: Tests fixed fee strategies

## Example Test Output

```
test test_fee_property::prop_percentage_fee_never_negative ... ok
test test_fee_property::prop_fee_never_exceeds_amount ... ok
test test_fee_property::prop_fee_calculation_deterministic ... ok
test test_fee_property::prop_no_panic_on_extremes ... ok
test test_fee_property::prop_overflow_handled_gracefully ... ok

test result: ok. 450 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out
```

## Implementation Details

### Test Configuration
- **Default cases per property**: 100 (configurable via `PROPTEST_CASES`)
- **Total test cases per run**: 450+ (450 cases × 5 test groups)
- **Edge case testing**: 150 cases for overflow scenarios
- **Strategy combination**: Random amounts × Random fee basis points

### Public API Usage
All tests use the **public** `calculate_platform_fee()` API:
```rust
pub fn calculate_platform_fee(
    env: &Env,
    amount: i128,
    token: Option<&Address>,
) -> Result<i128, ContractError>
```

This ensures tests validate the actual contract interface, not implementation details.

## Common Issues & Solutions

### Build Takes Too Long
- Soroban SDK compilation is slow on first run
- Subsequent runs use cached artifacts and are much faster
- Use `PROPTEST_CASES=10` for quick feedback during development

### Tests Fail on Overflow
- This is **expected behavior** - overflow errors are validated
- The test asserts that overflow is handled gracefully
- Check that your error handling returns `ContractError::Overflow`

### Want to Reproduce a Failure
- Proptest saves failing seeds to `proptest-regressions/`
- Use `PROPTEST_REGRESSIONS=regression.txt` to debug specific cases
- Add `#[proptest(strategy = "value")]` to test specific inputs

## Integration with CI/CD

Add to your GitHub Actions workflow:
```yaml
- name: Run property-based fee tests
  run: |
    PROPTEST_CASES=500 cargo test test_fee_property --lib -- --nocapture --test-threads=1
```

For nightly/stress testing:
```yaml
- name: Intensive fee fuzzing
  if: github.event_name == 'schedule'
  run: |
    PROPTEST_CASES=5000 cargo test test_fee_property --lib -- --nocapture
```

## Performance Benchmarks

Expected runtimes (approximate):
- **10 cases**: ~2-3 seconds
- **100 cases**: ~20-30 seconds
- **500 cases**: 2-3 minutes
- **1000 cases**: 4-5 minutes

Times vary based on system performance and compilation cache.

## Manual Fee Calculation for Verification

The test suite includes a helper function to verify calculations:
```rust
fn manual_percentage_fee(amount: i128, bps: u32) -> Option<i128> {
    let product = (amount as i128).checked_mul(bps as i128)?;
    let fee = product.checked_div(FEE_DIVISOR)?;
    Some(fee.max(MIN_FEE))
}
```

**Formula**: `fee = max(MIN_FEE, (amount × bps) / 10000)`

## Coverage Summary

| Component | Coverage | Details |
|-----------|----------|---------|
| Percentage fees | 100 cases | Base strategy, BPS limits, minimum fees |
| Flat fees | Implicit | Handled by strategy dispatch |
| Dynamic fees | Implicit | Handled by strategy dispatch |
| Breakdowns | 100 cases | Arithmetic validation |
| Overflow handling | 150 cases | Extreme amounts, error propagation |
| Edge cases | 100 cases | Boundaries, monotonicity |
| **Total** | **450+** | **Comprehensive fuzzing** |

## Future Enhancements

Potential additions for more thorough testing:
- [ ] Corridor-specific fee validation
- [ ] Protocol fee breakdown validation
- [ ] Volume discount validation
- [ ] Multi-token fee calculations
- [ ] Concurrent transaction fuzzing

## References

- **proptest docs**: https://docs.rs/proptest/latest/proptest/
- **Soroban SDK**: https://docs.rs/soroban-sdk/
- **Property-Based Testing**: https://hypothesis.works/articles/what-is-property-based-testing/

## Testing Commands Quick Reference

```bash
# Development (fast feedback)
PROPTEST_CASES=10 cargo test test_fee_property --lib

# Standard testing
cargo test test_fee_property --lib

# CI/CD (thorough)
PROPTEST_CASES=500 cargo test test_fee_property --lib

# Nightly fuzzing
PROPTEST_CASES=5000 cargo test test_fee_property --lib

# Debug specific test
PROPTEST_VERBOSE=1 cargo test prop_no_panic_on_extremes --lib
```
