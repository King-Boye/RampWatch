# Post-Merge Cleanup: Fix Compilation Errors and Runtime Panics

## Summary
Fixes four bugs introduced during contributor PR merges: two hard compile errors, one runtime panic, and one missing input validation.

## Changes

### 1. Fix HealthStatus test compilation (#897)
**File:** `src/transport.rs`

Three test functions (`test_mock_transport_reset`, `test_mock_transport_multiple_requests`, `test_request_timeout_not_exceeded`) constructed `HealthStatus` with a `last_check: 1000` field that no longer exists on the struct. Removed the field from all three literals to match the current definition in `src/types.rs:477-484`.

### 2. Remove duplicate `get_rate_limit_status` definition
**File:** `src/rate_limiter.rs`

`impl RateLimiter` contained two identical `get_rate_limit_status` method definitions (lines 175 and 206), which is a hard compile error (E0592). Removed the first duplicate, keeping the second.

### 3. Prevent overflow panic in `delay_for_attempt` (#900)
**File:** `src/retry.rs`

`delay_for_attempt` computed `jitter_seed % (capped + 1)`. When `max_delay_ms` is `u64::MAX` (a valid "no cap" sentinel, mirroring the `budget_ms` convention), `capped + 1` overflows to 0 and the modulo panics with division-by-zero. Replaced with `capped.checked_add(1)` — on overflow, returns `jitter_seed` directly (the full seed is already within the intended jitter range).

### 4. Validate `fee_percent_bps` in `get_fee_estimate` (#888)
**File:** `src/sep6.rs`

`fee_percent_bps` is documented as basis points (0–10,000), but was never validated. A misconfigured or malicious anchor returning a value above 10,000 would produce `total_fee` larger than `amount`, causing unsigned underflow panics in callers. Added an early return with `ErrorCode::ValidationError` when `fee_percent_bps > 10_000`.

## Closes Issues
- Closes #897
- Closes #898
- Closes #900
- Closes #888
