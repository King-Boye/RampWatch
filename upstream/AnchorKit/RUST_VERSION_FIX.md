# Rust Version Requirement Fix

## Problem
The doctor command was validating the Rust toolchain version against `MIN_RUST_MINOR=56`, but Soroban SDK 21.7.0 requires Rust 1.74+. This meant the validation would pass for Rust versions 1.56-1.73, which cannot compile the project.

## Solution
Updated the minimum Rust version requirement to match Soroban SDK 21.7.0's actual requirements.

## Changes Made

### 1. Updated Minimum Version Constants
**File:** `src/bin/anchorkit.rs`

Changed:
```rust
const MIN_RUST_MAJOR: u32 = 1;
const MIN_RUST_MINOR: u32 = 56;
```

To:
```rust
const MIN_RUST_MAJOR: u32 = 1;
const MIN_RUST_MINOR: u32 = 74;
```

### 2. Added Comprehensive Tests
**File:** `tests/doctor_tests.rs` (new file)

Created a new test suite with three test cases:

1. **`test_doctor_rust_version_check`**: Integration test that runs the actual doctor command and verifies it correctly detects the Rust version
2. **`test_parse_rustc_version`**: Unit test for the version parsing logic with various valid and invalid inputs
3. **`test_version_comparison`**: Unit test for the version comparison logic to ensure versions 1.74+ pass and versions below 1.74 fail

## Verification

The tests verify:
- ✅ Rust 1.74+ passes the version check
- ✅ Rust 1.73 and below fail the version check
- ✅ Version string parsing works correctly
- ✅ Version comparison logic is accurate

## Impact

Users with Rust versions 1.56-1.73 will now correctly see a failure message from the doctor command, preventing compilation errors later. The error message guides them to update:

```
✖ Rust 1.73 detected but 1.74+ is required (edition 2021)
  → Run: rustup update stable
```

## Testing

Run the tests with:
```bash
cargo test --test doctor_tests
```

Or run all tests:
```bash
cargo test
```
