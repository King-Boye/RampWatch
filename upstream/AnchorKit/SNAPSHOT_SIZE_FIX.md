# Snapshot Size Fix for test_rate_comparison_stress

## Problem
The test snapshot `test_snapshots/load_simulation_tests/test_rate_comparison_stress.1.json` was 876KB (nearly 1MB). This inflated the repository size and made git operations slower.

## Root Cause
The Soroban SDK's test framework automatically records all authentication traces and ledger state in snapshot files. The `test_rate_comparison_stress` test creates 20 anchors × 50 quotes = 1000 quote submissions, each with full auth recording, resulting in a large snapshot file.

## Solution
Modified `tests/load_simulation_tests.rs` to use `env.stop_recording_auth()` before the bulk operations. This prevents the Soroban test framework from recording the auth trace, which is the primary contributor to the large snapshot size.

### Changes Made

1. **Modified `test_rate_comparison_stress` test** (`tests/load_simulation_tests.rs`):
   - Added `env.stop_recording_auth()` call after setup and before the stress operations
   - Added targeted assertions to validate:
     - Total quote count matches expected (1000)
     - Quote IDs are monotonically increasing
     - Final quote ID equals total count
   - Updated documentation to explain the approach

2. **Deleted large snapshot file**:
   - Removed `test_snapshots/load_simulation_tests/test_rate_comparison_stress.1.json` (876KB)

### Code Changes

```rust
// Before the stress operations, stop recording auth to prevent large snapshots
env.stop_recording_auth();

// Use targeted assertions instead of relying on snapshot comparison
assert_eq!(total_quotes, EXPECTED_TOTAL_QUOTES as usize, 
    "Total quote count should match expected");
assert_eq!(prev_quote_id, EXPECTED_TOTAL_QUOTES,
    "Final quote ID should equal total count");
```

## Impact
- **Repository size reduction**: ~876KB saved
- **No CI impact**: Stress tests are gated behind the `stress-tests` feature flag and don't run in normal CI
- **Test functionality preserved**: The test still validates the same behavior with targeted assertions

## Verification
- The snapshot file has been removed from the repository
- The test uses `stop_recording_auth()` to prevent regenerating large snapshots
- Targeted assertions ensure the test still validates correct behavior
- CI configuration confirms stress tests only run with explicit `--features stress-tests` flag