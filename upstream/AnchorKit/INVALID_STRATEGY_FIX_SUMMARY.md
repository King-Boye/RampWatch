# Invalid Routing Strategy Fix — Summary

## Problem
The `route_transaction` function in `src/contract.rs` had a silent fallback behavior when an unrecognized strategy symbol was passed. Instead of erroring, it would fall through all strategy branches and return the first candidate in storage iteration order, which is non-deterministic and makes it impossible to detect misconfigured strategies at call time.

**Documentation stated:** "An unrecognised symbol falls through all branches and returns the first candidate in iteration order (no explicit sort)."

This violated contract semantics and created security risks:
- Silent failure — caller doesn't know strategy wasn't applied
- Non-deterministic results — same input produces different outputs on different ledgers
- Violates caller expectations — strategy field should be validated, not silently ignored

## Solution
Implemented explicit validation of the routing strategy symbol before applying any selection logic. If the symbol doesn't match one of the four valid strategies, the function now panics with a new `InvalidStrategy` error code.

## Changes Made

### 1. **src/errors.rs** — Added new error code
- Added `InvalidStrategy = 55` to the `ErrorCode` enum
- Added default message: `"Routing strategy symbol is not recognized"`
- Added named constructor: `pub fn invalid_strategy() -> Self`
- Updated test to include the new error code in validation

### 2. **src/contract.rs** — Added validation logic
Added explicit strategy symbol validation in `route_transaction` (lines 1563-1568):
```rust
// Validate that the strategy symbol is recognized
if strategy_sym != lowest_fee_sym 
    && strategy_sym != fastest_sym 
    && strategy_sym != reputation_sym 
    && strategy_sym != balanced_sym {
    panic_with_error!(&env, ErrorCode::InvalidStrategy);
}
```

This check occurs immediately after extracting the strategy symbol and before any selection logic, ensuring:
- Early detection of misconfigured strategies
- Deterministic behavior — all calls with invalid strategies fail consistently
- Clear error feedback to callers

### 3. **src/types.rs** — Updated documentation
Updated `RoutingOptions` struct documentation to reflect the new behavior:
- Changed from: "An unrecognised symbol falls through all branches and returns the first candidate in iteration order"
- Changed to: "An unrecognised symbol causes the call to panic with `InvalidStrategy`"
- Added `"Balanced"` strategy to the documentation table

### 4. **docs/features/ROUTING_STRATEGY.md** — Updated guide
Updated the "Default Strategy" section to document the new validation:
- Removed: "An unrecognised symbol string does not error — it falls through all strategy branches..."
- Added: "An unrecognised symbol string causes the call to panic with `InvalidStrategy`"

## Valid Strategy Symbols
The four recognized strategy symbols are:
1. `"LowestFee"` — Selects anchor with lowest fee_percentage
2. `"FastestSettlement"` — Selects anchor with lowest average_settlement_time
3. `"HighestReputation"` — Selects anchor with highest reputation_score
4. `"Balanced"` — Composite scoring: (40_000/fee) + (30_000/time) + (reputation*3000/10000)

## Error Behavior
- **Empty strategy vec** → Panics with `NoQuotesAvailable` (unchanged)
- **Unrecognized symbol** → Panics with `InvalidStrategy` (new behavior)
- **Valid symbol** → Proceeds with selection logic (unchanged)

## Testing
All diagnostics pass:
- `src/errors.rs` — No diagnostics
- `src/contract.rs` — No diagnostics
- `src/types.rs` — No diagnostics

The implementation follows the existing error handling patterns in the codebase and maintains backward compatibility for all valid use cases.

## Impact
- **Breaking Change:** Code passing invalid strategy symbols will now panic instead of silently returning the first candidate
- **Benefit:** Immediate feedback on misconfigured strategies, deterministic behavior, and improved security
- **Migration:** Callers must ensure they pass one of the four valid strategy symbols

