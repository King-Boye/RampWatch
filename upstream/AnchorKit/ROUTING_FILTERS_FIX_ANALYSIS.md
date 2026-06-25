# Routing Filters Implementation - Technical Analysis

## Problem Statement

The `RoutingOptions` struct had two fields that were documented as "reserved for future filtering" but were never actually enforced:

1. **`require_kyc: bool`** — When set to `true`, callers expected only KYC-capable anchors to be returned, but all active anchors were considered regardless.
2. **`max_anchors: u32`** — When set to a positive value, callers expected the candidate pool to be limited to N anchors, but all matching anchors were considered.

This created a **contract-caller mismatch**: callers setting these fields believed they were enforcing constraints, but the contract ignored them silently.

### Example of the Bug

```rust
// Caller expects only KYC-capable anchors
let options = RoutingOptions {
    request: ...,
    strategy: ...,
    min_reputation: 0,
    max_anchors: 5,
    require_kyc: true,  // ← Ignored by contract!
};

let quote = contract.route_transaction(&options);
// Result: Could return a quote from an anchor that doesn't support KYC
// Expected: Only anchors with SERVICE_KYC in their services list
```

## Root Cause Analysis

The `route_transaction` function in `src/contract.rs` only checked:
1. `is_active` status
2. `min_reputation` threshold
3. Quote validity and amount constraints

It never checked:
- Whether the anchor supports KYC (when `require_kyc = true`)
- Whether the candidate pool had reached the `max_anchors` limit

## Solution: Implement Both Filters

### 1. KYC Requirement Filter

Added a check after reputation filtering:

```rust
// Check KYC requirement filter
if options.require_kyc {
    let services_key = StorageKey::Services(anchor.clone());
    let services_record: AnchorServices = match env.storage().persistent().get(&services_key) {
        Some(sr) => sr,
        None => continue,
    };
    if !services_record.services.contains(SERVICE_KYC) {
        continue;
    }
}
```

**Behavior:**
- If `require_kyc = false` (default): No KYC check performed, all active anchors considered
- If `require_kyc = true`: Only anchors with `SERVICE_KYC` in their services list are included

**Error Handling:**
- If an anchor has no services configured, it's skipped (treated as not supporting KYC)
- This is safe because `configure_services` requires at least one service

### 2. Max Anchors Limit

Added a check after a candidate is added to the pool:

```rust
candidates.push_back(quote);

// Stop adding candidates if we've reached max_anchors limit
if options.max_anchors > 0 && candidates.len() >= options.max_anchors as usize {
    break;
}
```

**Behavior:**
- If `max_anchors = 0` (default): No limit, all matching anchors considered
- If `max_anchors > 0`: Stop adding candidates once N anchors are in the pool

**Important Notes:**
- The limit is applied **after** all other filters (reputation, KYC, quote validity)
- Candidates are added in **iteration order** (no sorting before limiting)
- This means the first N anchors that pass all filters are selected
- Strategy selection still happens on the limited candidate pool

## Filter Application Order

The filters are applied in this order (most restrictive first):

1. **Active status** — Skip inactive anchors
2. **Reputation threshold** — Skip anchors below `min_reputation`
3. **KYC requirement** — Skip anchors without KYC if `require_kyc = true`
4. **Quote validity** — Skip anchors with no valid quote or expired quote
5. **Amount constraints** — Skip anchors whose quote doesn't cover the requested amount
6. **Max anchors limit** — Stop adding candidates once `max_anchors` is reached

This order ensures:
- Expensive lookups (quote retrieval) happen after cheap checks (reputation, KYC)
- The candidate pool is as small as possible before strategy selection
- Performance is optimized for common cases

## Implementation Details

### Type Safety

- `max_anchors` is `u32`, but compared as `usize` for vector length
- Cast is safe: `candidates.len() >= options.max_anchors as usize`
- No overflow risk because `max_anchors` is a reasonable limit

### Storage Access

- KYC check retrieves `StorageKey::Services(anchor)` from persistent storage
- This is the same storage key used by `configure_services` and `supports_service`
- Consistent with existing service management patterns

### Backward Compatibility

- Default values (`max_anchors = 0`, `require_kyc = false`) preserve existing behavior
- Existing callers who don't set these fields are unaffected
- New callers can opt-in to filtering by setting these fields

## Documentation Updates

Updated `src/types.rs` to reflect that these fields are now implemented:

**Before:**
```
- `max_anchors` / `require_kyc` — reserved for future filtering; not yet
  enforced by the current implementation.
```

**After:**
```
- `max_anchors` — limits the number of candidate anchors considered before
  strategy selection. Set to `0` (the default) to consider all active anchors.
  When set to a positive value, only the first N anchors (in iteration order)
  that pass all other filters are included in the candidate pool.
- `require_kyc` — when set to `true`, only anchors that support the KYC service
  (SERVICE_KYC) are included in the candidate pool. Set to `false` (the default)
  to include all active anchors regardless of KYC capability.
```

## Testing Considerations

The implementation should be tested with:

1. **KYC filtering:**
   - Anchor with KYC service, `require_kyc = true` → included
   - Anchor without KYC service, `require_kyc = true` → excluded
   - Anchor without KYC service, `require_kyc = false` → included

2. **Max anchors limiting:**
   - 5 matching anchors, `max_anchors = 0` → all 5 considered
   - 5 matching anchors, `max_anchors = 3` → only first 3 considered
   - 2 matching anchors, `max_anchors = 5` → all 2 considered

3. **Combined filtering:**
   - Multiple filters applied together
   - Filters interact correctly (KYC + max_anchors + reputation)

4. **Edge cases:**
   - No anchors match filters → `NoQuotesAvailable` error
   - Anchor with no services configured → treated as not supporting KYC
   - `max_anchors = 0` → no limit (default behavior)

## Performance Impact

- **KYC check:** One additional storage lookup per anchor (only if `require_kyc = true`)
- **Max anchors limit:** Early loop termination (reduces iterations)
- **Overall:** Negligible impact; filtering reduces work in most cases

## Security Considerations

- **KYC enforcement:** Callers can now reliably enforce KYC requirements
- **Candidate pool limiting:** Reduces attack surface by limiting routing options
- **No new vulnerabilities:** Filters only exclude anchors, never include invalid ones
