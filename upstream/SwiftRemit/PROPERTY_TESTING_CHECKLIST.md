# Property-Based Testing Checklist

## Pre-Development

- [ ] Review `PROPERTY_TESTS_README.md` for overview
- [ ] Understand the 7 invariants being tested
- [ ] Set up development environment with proptest
- [ ] Run initial test suite to verify setup

## During Development

### Before Making Changes
- [ ] Run quick property tests (10 cases)
  ```bash
  PROPTEST_CASES=10 cargo test test_property --lib
  ```
- [ ] Identify which invariants your changes might affect

### After Making Changes
- [ ] Run standard property tests (50 cases)
  ```bash
  cargo test test_property --lib
  ```
- [ ] Run affected invariant tests specifically
  ```bash
  cargo test prop_<invariant_name> --lib
  ```
- [ ] Fix any failures before committing

### Adding New Features
- [ ] Consider if new properties/invariants are needed
- [ ] Add property tests for new invariants
- [ ] Document new properties in `PROPERTY_BASED_TESTING.md`
- [ ] Update this checklist if needed

## Before Committing

- [ ] Run standard property tests (50 cases)
  ```bash
  cargo test test_property --lib
  ```
- [ ] All tests pass
- [ ] No new warnings introduced
- [ ] Commit message mentions property test coverage

## Before Pull Request

- [ ] Run thorough property tests (200 cases)
  ```bash
  PROPTEST_CASES=200 cargo test test_property --lib
  ```
- [ ] All tests pass consistently
- [ ] Review test output for any warnings
- [ ] Update documentation if invariants changed
- [ ] Add property test results to PR description

## CI/CD Integration

- [ ] Property tests run automatically on push
- [ ] Using 200+ test cases in CI
- [ ] Failures block merge
- [ ] Test results visible in PR checks

## Before Release

- [ ] Run comprehensive property tests (1000 cases)
  ```bash
  PROPTEST_CASES=1000 cargo test test_property --lib
  ```
- [ ] All 7 invariants validated
- [ ] No edge cases discovered
- [ ] Document any new edge cases found
- [ ] Update version notes with property test status

## Debugging Failures

When a property test fails:

- [ ] Note the minimal failing input from proptest output
- [ ] Copy the seed for replay
  ```bash
  PROPTEST_SEED=<seed> cargo test prop_test_name --lib
  ```
- [ ] Understand which invariant was violated
- [ ] Reproduce the issue manually if needed
- [ ] Fix the underlying issue
- [ ] Verify fix with full test suite
- [ ] Document the edge case if significant

## Maintenance

### Weekly
- [ ] Run property tests with current codebase
- [ ] Review any new edge cases discovered
- [ ] Update documentation if needed

### Monthly
- [ ] Review property test coverage
- [ ] Consider adding new properties
- [ ] Update test case counts if needed
- [ ] Review and update documentation

### Quarterly
- [ ] Comprehensive review of all invariants
- [ ] Consider increasing test case counts
- [ ] Review proptest version for updates
- [ ] Audit property test effectiveness

## Documentation Review

- [ ] `PROPERTY_TESTS_README.md` - Up to date
- [ ] `PROPERTY_BASED_TESTING.md` - Reflects current invariants
- [ ] `PROPERTY_TESTING_QUICKREF.md` - Commands are correct
- [ ] `PROPERTY_TESTING_SUMMARY.md` - Accurate summary
- [ ] Code comments in `src/test_property.rs` - Clear and helpful

## Team Onboarding

For new team members:

- [ ] Share `PROPERTY_TESTS_README.md`
- [ ] Explain the 7 invariants
- [ ] Demonstrate running tests
- [ ] Show how to debug failures
- [ ] Review integration with workflow

## Invariant Checklist

Verify each invariant is properly tested:

### 1. No Balance Creation
- [ ] `prop_no_balance_creation_on_create` - Passes
- [ ] `prop_no_balance_creation_on_settlement` - Passes
- [ ] `prop_no_balance_creation_on_cancel` - Passes

### 2. No Negative Settlements
- [ ] `prop_no_negative_balances` - Passes
- [ ] `prop_payout_amount_non_negative` - Passes

### 3. Deterministic Results
- [ ] `prop_netting_order_independence` - Passes

### 4. Fee Calculation Correctness
- [ ] `prop_fee_calculation_accuracy` - Passes
- [ ] `prop_accumulated_fees_correctness` - Passes

### 5. State Transition Validity
- [ ] `prop_valid_state_transitions` - Passes
- [ ] `prop_cancel_state_transition` - Passes

### 6. Idempotency
- [ ] `prop_no_duplicate_settlement` - Passes

### 7. Net Settlement Conservation
- [ ] `prop_netting_preserves_fees` - Passes

## Performance Checklist

- [ ] Tests complete in reasonable time
- [ ] No memory issues with large test counts
- [ ] Parallel execution working properly
- [ ] CI/CD pipeline time acceptable

## Edge Cases Discovered

Document any edge cases found through property testing:

- [ ] Case 1: _____________________
  - Input: _____________________
  - Issue: _____________________
  - Resolution: _____________________

- [ ] Case 2: _____________________
  - Input: _____________________
  - Issue: _____________________
  - Resolution: _____________________

## Quick Commands Reference

```bash
# Quick validation (development)
PROPTEST_CASES=10 cargo test test_property --lib

# Standard testing (pre-commit)
cargo test test_property --lib

# Thorough testing (pre-PR)
PROPTEST_CASES=200 cargo test test_property --lib

# Comprehensive testing (pre-release)
PROPTEST_CASES=1000 cargo test test_property --lib

# Specific invariant
cargo test prop_no_balance_creation --lib

# With verbose output
cargo test test_property --lib -- --nocapture

# Replay failure
PROPTEST_SEED=<seed> cargo test prop_test_name --lib
```

## Success Criteria

Property-based testing is successful when:

- [ ] All 15 property tests pass consistently
- [ ] Tests run in reasonable time
- [ ] Edge cases are discovered and documented
- [ ] Team understands and uses property tests
- [ ] CI/CD integration is working
- [ ] Documentation is up to date
- [ ] No critical bugs slip through

## Notes

Use this space for project-specific notes:

```
_____________________________________________
_____________________________________________
_____________________________________________
```

## Last Review

- Date: _____________________
- Reviewer: _____________________
- Status: _____________________
- Action Items: _____________________

---

**Remember**: Property-based tests are your mathematical proof that critical invariants hold. Don't skip them!
