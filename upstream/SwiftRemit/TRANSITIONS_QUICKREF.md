# Lifecycle Transitions - Quick Reference

## TL;DR

SwiftRemit now enforces strict state transitions. Remittances must go through `Processing` before completion.

## States

- **Pending** → Initial state
- **Processing** → Agent working on it
- **Completed** → Success (terminal)
- **Cancelled** → Sender cancelled (terminal)
- **Failed** → Payout failed (terminal)

## Functions

### `start_processing(remittance_id: u64)`
**Who:** Agent  
**From:** Pending  
**To:** Processing  
**Purpose:** Signal you've started working on the payout

### `confirm_payout(remittance_id: u64)`
**Who:** Agent  
**From:** Processing  
**To:** Completed  
**Purpose:** Confirm successful payout (releases funds)

### `mark_failed(remittance_id: u64)`
**Who:** Agent  
**From:** Processing  
**To:** Failed  
**Purpose:** Report failed payout (refunds sender)

### `cancel_remittance(remittance_id: u64)`
**Who:** Sender  
**From:** Pending  
**To:** Cancelled  
**Purpose:** Cancel before agent starts (refunds sender)

## Common Patterns

### Happy Path
```rust
let id = contract.create_remittance(&sender, &agent, &1000, &None);
contract.start_processing(&id);      // Agent starts
contract.confirm_payout(&id);        // Agent completes
```

### Sender Cancels Early
```rust
let id = contract.create_remittance(&sender, &agent, &1000, &None);
contract.cancel_remittance(&id);     // Sender cancels
```

### Payout Fails
```rust
let id = contract.create_remittance(&sender, &agent, &1000, &None);
contract.start_processing(&id);      // Agent starts
contract.mark_failed(&id);           // Agent reports failure
```

## Common Errors

### ❌ Error #7: InvalidStatus

**Cause:** Invalid state transition

**Examples:**
```rust
// Cannot skip Processing
contract.create_remittance(...);
contract.confirm_payout(id);  // ❌ Must call start_processing() first

// Cannot cancel during processing
contract.start_processing(id);
contract.cancel_remittance(id);  // ❌ Too late to cancel

// Cannot modify terminal states
contract.confirm_payout(id);
contract.start_processing(id);  // ❌ Already completed
```

**Fix:** Follow the correct state flow

## Migration from Old Version

### Before (Old Code)
```rust
let id = contract.create_remittance(&sender, &agent, &1000, &None);
contract.confirm_payout(&id);  // Direct completion
```

### After (New Code)
```rust
let id = contract.create_remittance(&sender, &agent, &1000, &None);
contract.start_processing(&id);  // ← Add this line
contract.confirm_payout(&id);
```

## Events

All transitions emit `("status", "transit")` events:

```javascript
{
  remittance_id: 42,
  from_status: "Pending",
  to_status: "Processing",
  actor: "GXXX...XXX",
  timestamp: 1708545351
}
```

## Rules

1. ✅ Pending → Processing → Completed (normal flow)
2. ✅ Pending → Cancelled (early cancellation)
3. ✅ Processing → Failed (payout failed)
4. ❌ Pending → Completed (must go through Processing)
5. ❌ Processing → Cancelled (too late to cancel)
6. ❌ Any transition from terminal states

## Testing

```bash
# Run all transition tests
cargo test transitions

# Run specific test
cargo test test_lifecycle_pending_to_processing
```

## Need Help?

- See [LIFECYCLE_TRANSITIONS.md](LIFECYCLE_TRANSITIONS.md) for full documentation
- See [STATE_MACHINE.md](STATE_MACHINE.md) for visual diagrams
- Check test files for examples: `src/test_transitions.rs`
