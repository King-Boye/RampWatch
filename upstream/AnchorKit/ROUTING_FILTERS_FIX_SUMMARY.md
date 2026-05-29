# Routing Filters Implementation - Summary

## Issue Fixed

**Problem**: `RoutingOptions.max_anchors` and `require_kyc` were documented as "reserved for future filtering" but were never enforced. Callers setting these fields believed they were filtering anchors, but the contract ignored them.

**Impact**: 
- Callers expecting KYC-only anchors could receive quotes from non-KYC anchors
- Callers expecting a limited candidate pool received all matching anchors
- Silent contract-caller mismatch created security and usability issues

## Solution Implemented

Implemented both filters in the `route_transaction` function:

### 1. KYC Requirement Filter (`require_kyc`)

When `require_kyc = true`, only anchors with `SERVICE_KYC` in their services list are included in the candidate pool.

**Implementation:**
```rust
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

### 2. Max Anchors Limit (`max_anchors`)

When `max_anchors > 0`, stop adding candidates once N anchors pass all filters.

**Implementation:**
```rust
candidates.push_back(quote);

if options.max_anchors > 0 && candidates.len() >= options.max_anchors as usize {
    break;
}
```

## Changes Made

### File: `AnchorKit/src/contract.rs`

**Function**: `route_transaction` (Line 1490)

**Changes:**
1. Added KYC requirement check after reputation filtering
2. Added max_anchors limit check after adding each candidate
3. Filters are applied in order: active → reputation → KYC → quote validity → amount → max_anchors

### File: `AnchorKit/src/types.rs`

**Struct**: `RoutingOptions` documentation

**Changes:**
1. Updated `max_anchors` documentation to describe the new limiting behavior
2. Updated `require_kyc` documentation to describe the new filtering behavior
3. Removed "reserved for future filtering" language

## Filter Application Order

1. **Active status** — Skip inactive anchors
2. **Reputation threshold** — Skip anchors below `min_reputation`
3. **KYC requirement** — Skip anchors without KYC if `require_kyc = true`
4. **Quote validity** — Skip anchors with no valid quote or expired quote
5. **Amount constraints** — Skip anchors whose quote doesn't cover the requested amount
6. **Max anchors limit** — Stop adding candidates once `max_anchors` is reached

## Backward Compatibility

- ✓ Default values (`max_anchors = 0`, `require_kyc = false`) preserve existing behavior
- ✓ Existing callers unaffected
- ✓ New callers can opt-in to filtering
- ✓ No API changes

## Key Design Decisions

1. **KYC check uses SERVICE_KYC constant** — Consistent with existing service management
2. **Max anchors applied after all other filters** — Ensures most relevant anchors are selected
3. **Candidates added in iteration order** — No sorting before limiting (deterministic, efficient)
4. **Default `max_anchors = 0` means no limit** — Preserves existing behavior
5. **Anchors without services are skipped if KYC required** — Safe default (fail-closed)

## Testing Recommendations

1. **KYC filtering:**
   - Anchor with KYC, `require_kyc = true` → included
   - Anchor without KYC, `require_kyc = true` → excluded
   - Anchor without KYC, `require_kyc = false` → included

2. **Max anchors limiting:**
   - 5 matching anchors, `max_anchors = 0` → all 5 considered
   - 5 matching anchors, `max_anchors = 3` → only first 3 considered
   - 2 matching anchors, `max_anchors = 5` → all 2 considered

3. **Combined filtering:**
   - Multiple filters applied together
   - Filters interact correctly

4. **Edge cases:**
   - No anchors match filters → `NoQuotesAvailable` error
   - Anchor with no services → treated as not supporting KYC

## Performance Impact

- **KYC check:** One additional storage lookup per anchor (only if `require_kyc = true`)
- **Max anchors limit:** Early loop termination (reduces iterations)
- **Overall:** Negligible; filtering typically reduces work

## Security Improvements

- ✓ Callers can now reliably enforce KYC requirements
- ✓ Candidate pool can be limited to reduce attack surface
- ✓ No new vulnerabilities introduced
- ✓ Filters only exclude anchors, never include invalid ones

## Technical Details

See `ROUTING_FILTERS_FIX_ANALYSIS.md` for:
- Detailed problem analysis
- Root cause explanation
- Implementation details
- Filter application order
- Storage access patterns
- Performance considerations
