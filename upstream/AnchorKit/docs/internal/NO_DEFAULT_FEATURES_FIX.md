# No-Default-Features Build Fix

## Problem

The CI pipeline was failing on the "Feature Flag Build Matrix / Test (no-default-features)" check. This check runs:
```bash
cargo build --no-default-features
cargo test --no-default-features
```

## Root Cause

The skeleton loaders implementation was using `Env::default()` in several places:

```rust
// ❌ WRONG - Env::default() only available with testutils feature
validation_steps: Vec::new(&soroban_sdk::Env::default())
```

The `Env::default()` method is only available when the `testutils` feature is enabled (in tests), but not in production builds or when building with `--no-default-features`.

## Solution

Changed all methods that were creating empty `Vec` instances to accept an `&Env` parameter:

### Before (Broken)
```rust
impl AuthValidationSkeleton {
    pub fn validating(attestor: Address) -> Self {
        Self {
            attestor,
            is_validating: true,
            is_valid: false,
            has_error: false,
            error_message: None,
            validation_steps: Vec::new(&soroban_sdk::Env::default()), // ❌ FAILS
        }
    }
}
```

### After (Fixed)
```rust
impl AuthValidationSkeleton {
    pub fn validating(env: &soroban_sdk::Env, attestor: Address) -> Self {
        Self {
            attestor,
            is_validating: true,
            is_valid: false,
            has_error: false,
            error_message: None,
            validation_steps: Vec::new(env), // ✅ WORKS
        }
    }
}
```

## Changes Made

### 1. `src/skeleton_loaders.rs`
Updated `AuthValidationSkeleton` methods to accept `&Env` parameter:
- `validating(env: &Env, attestor: Address)`
- `validated(env: &Env, attestor: Address)`
- `error(env: &Env, attestor: Address, message: String)`

### 2. `src/lib.rs`
Updated contract method to pass `&env` to skeleton constructors:
- `get_auth_validation_skeleton()` now passes `&env` to `AuthValidationSkeleton::error()` and `AuthValidationSkeleton::validated()`

### 3. `src/skeleton_loader_tests.rs`
Updated tests to pass `&env` parameter:
- `test_auth_validation_skeleton_validating()`
- `test_auth_validation_skeleton_validated()`

## Why This Matters

The Soroban SDK has different feature flags for different build scenarios:

1. **Default build** (`cargo build`): Includes `std` feature
2. **No-default-features** (`cargo build --no-default-features`): Minimal build, no std
3. **WASM build** (`--features wasm`): For WebAssembly targets
4. **Test build** (`cargo test`): Includes `testutils` feature with `Env::default()`

The CI runs all these configurations to ensure the contract works in all environments. Our code must work without relying on test-only features like `Env::default()`.

## Verification

After the fix:
```bash
# All these should now pass:
cargo build --no-default-features
cargo test --no-default-features
cargo build --no-default-features --features wasm
cargo build --no-default-features --features mock-only
```

## Best Practices

When writing Soroban smart contracts:

1. ✅ **DO**: Accept `Env` as a parameter when you need to create SDK types
2. ❌ **DON'T**: Use `Env::default()` in production code
3. ✅ **DO**: Test with `--no-default-features` to catch these issues early
4. ✅ **DO**: Use `#[cfg(test)]` for test-only code that uses `Env::default()`

## Example Pattern

```rust
// ✅ CORRECT: Accept Env parameter
pub fn create_empty_list(env: &Env) -> Vec<String> {
    Vec::new(env)
}

// ❌ WRONG: Don't use Env::default() in production
pub fn create_empty_list() -> Vec<String> {
    Vec::new(&Env::default()) // Only works in tests!
}

// ✅ CORRECT: Use Env::default() only in tests
#[cfg(test)]
fn test_something() {
    let env = Env::default(); // OK in tests
    let list = create_empty_list(&env);
}
```

## Impact

This fix ensures the skeleton loaders work in all build configurations:
- ✅ Production builds
- ✅ WASM builds
- ✅ No-default-features builds
- ✅ Test builds
- ✅ All feature flag combinations

## Related Files

- `src/skeleton_loaders.rs` - Core implementation
- `src/lib.rs` - Contract methods
- `src/skeleton_loader_tests.rs` - Test suite
- `.github/workflows/feature-flag-matrix.yml` - CI configuration
- `Cargo.toml` - Feature flag definitions

## Commit

```
fix: remove Env::default() usage to support no-default-features build

- Updated AuthValidationSkeleton methods to accept &Env parameter
- Fixed get_auth_validation_skeleton to pass env reference
- Updated tests to pass env parameter
- Ensures compatibility with --no-default-features builds
```
