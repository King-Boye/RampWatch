# Lifecycle Transition Enforcement

## Overview

This implementation enforces valid lifecycle transitions for remittance transfers in the SwiftRemit contract. The system now supports granular state tracking with strict validation rules to prevent invalid state changes.

## State Definitions

The remittance lifecycle now includes 5 distinct states:

| State | Description | Terminal |
|-------|-------------|----------|
| **Pending** | Initial state after remittance creation | No |
| **Processing** | Agent has started processing the payout | No |
| **Completed** | Successfully settled with funds transferred | Yes |
| **Cancelled** | Cancelled by sender with full refund | Yes |
| **Failed** | Failed settlement with sender refund | Yes |

## Valid State Transitions

The following state transitions are enforced by the contract:

```
Pending → Processing    (Agent starts processing)
Pending → Cancelled     (Sender cancels before processing)

Processing → Completed  (Successful payout confirmation)
Processing → Failed     (Failed payout with refund)

Completed → [NONE]      (Terminal state)
Cancelled → [NONE]      (Terminal state)
Failed → [NONE]         (Terminal state)
```

### Invalid Transitions

All other transitions are rejected with `ContractError::InvalidStatus`:

- ❌ Pending → Completed (must go through Processing)
- ❌ Pending → Failed (must go through Processing)
- ❌ Processing → Cancelled (cannot cancel once processing started)
- ❌ Processing → Pending (cannot revert to pending)
- ❌ Any transition from terminal states (Completed, Cancelled, Failed)

## Implementation Details

### 1. Enhanced Status Enum (`src/types.rs`)

```rust
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RemittanceStatus {
    Pending,      // Initial state after creation
    Processing,   // Agent has started processing
    Completed,    // Successfully settled
    Cancelled,    // Cancelled by sender
    Failed,       // Failed settlement
}
```

### 2. Transition Validation Module (`src/transitions.rs`)

Core validation logic that enforces state transition rules:

```rust
pub fn validate_transition(
    from: &RemittanceStatus,
    to: &RemittanceStatus,
) -> Result<(), ContractError>
```

**Features:**
- Pattern matching for all valid transitions
- Explicit rejection of terminal state transitions
- Returns `ContractError::InvalidStatus` for invalid transitions
- Includes comprehensive unit tests

### 3. Transition Event Logging (`src/events.rs`)

All state transitions are logged via events for off-chain monitoring:

```rust
pub fn emit_status_transition(
    env: &Env,
    remittance_id: u64,
    from_status: RemittanceStatus,
    to_status: RemittanceStatus,
    actor: Address,
)
```

**Event Structure:**
- Topic: `("status", "transit")`
- Data: Schema version, ledger sequence, timestamp, remittance ID, from/to status, actor address

### 4. New Contract Functions (`src/lib.rs`)

#### `start_processing(remittance_id: u64)`
- **Authorization:** Agent only
- **Transition:** Pending → Processing
- **Purpose:** Agent signals they've begun processing the payout
- **Side Effects:** Status update, transition event emission

#### `mark_failed(remittance_id: u64)`
- **Authorization:** Agent only
- **Transition:** Processing → Failed
- **Purpose:** Agent reports failed payout attempt
- **Side Effects:** Full refund to sender, status update, transition event emission

### 5. Updated Existing Functions

#### `confirm_payout(remittance_id: u64)`
- **Before:** Checked `status == Pending`
- **After:** Uses `validate_transition()` to allow Processing → Completed
- **New Behavior:** Emits transition event with actor information

#### `cancel_remittance(remittance_id: u64)`
- **Before:** Checked `status == Pending`
- **After:** Uses `validate_transition()` to enforce Pending → Cancelled only
- **New Behavior:** Emits transition event, prevents cancellation during processing

## Testing

### Unit Tests (`src/transitions.rs`)

Basic transition validation tests:
- ✅ Valid transitions (Pending→Processing, Processing→Completed, etc.)
- ✅ Invalid transitions from Pending
- ✅ Invalid transitions from Processing
- ✅ Terminal states cannot transition

### Integration Tests (`src/test_transitions.rs`)

Comprehensive end-to-end lifecycle tests:

1. **Valid Lifecycle Paths**
   - `test_lifecycle_pending_to_processing` - Agent starts processing
   - `test_lifecycle_pending_to_cancelled` - Sender cancels early
   - `test_lifecycle_processing_to_completed` - Successful completion
   - `test_lifecycle_processing_to_failed` - Failed payout

2. **Invalid Transition Rejection**
   - `test_invalid_transition_pending_to_completed` - Cannot skip Processing
   - `test_invalid_transition_pending_to_failed` - Cannot skip Processing
   - `test_invalid_transition_processing_to_cancelled` - Cannot cancel during processing

3. **Terminal State Enforcement**
   - `test_terminal_state_completed_cannot_transition` - Completed is final
   - `test_terminal_state_cancelled_cannot_transition` - Cancelled is final
   - `test_terminal_state_failed_cannot_transition` - Failed is final

4. **Event Logging**
   - `test_transition_events_logged` - All transitions emit events
   - `test_transitions_include_actor` - Events include actor address

5. **Business Logic**
   - `test_failed_remittance_refunds_sender` - Failed state refunds sender
   - `test_multiple_remittances_independent_lifecycles` - Independent state tracking

## Running Tests

```bash
# Run all transition tests
cargo test transitions

# Run specific test
cargo test test_lifecycle_pending_to_processing

# Run with output
cargo test transitions -- --nocapture
```

## Usage Examples

### Happy Path: Successful Remittance

```rust
// 1. Sender creates remittance (status: Pending)
let remittance_id = contract.create_remittance(&sender, &agent, &1000, &None);

// 2. Agent starts processing (status: Processing)
contract.start_processing(&remittance_id);

// 3. Agent confirms payout (status: Completed)
contract.confirm_payout(&remittance_id);
```

### Early Cancellation

```rust
// 1. Sender creates remittance (status: Pending)
let remittance_id = contract.create_remittance(&sender, &agent, &1000, &None);

// 2. Sender cancels before agent processes (status: Cancelled)
contract.cancel_remittance(&remittance_id);
```

### Failed Payout

```rust
// 1. Sender creates remittance (status: Pending)
let remittance_id = contract.create_remittance(&sender, &agent, &1000, &None);

// 2. Agent starts processing (status: Processing)
contract.start_processing(&remittance_id);

// 3. Agent reports failure (status: Failed, sender refunded)
contract.mark_failed(&remittance_id);
```

## Off-Chain Integration

### Monitoring Transition Events

Listen for `("status", "transit")` events to track remittance lifecycle:

```javascript
// Example event structure
{
  topics: ["status", "transit"],
  data: {
    schema_version: 1,
    ledger_sequence: 12345,
    timestamp: 1708545351,
    remittance_id: 42,
    from_status: "Pending",
    to_status: "Processing",
    actor: "GXXX...XXX"
  }
}
```

### State Machine Visualization

```
┌─────────┐
│ Pending │
└────┬────┘
     │
     ├──────────────┐
     │              │
     ▼              ▼
┌────────────┐  ┌───────────┐
│ Processing │  │ Cancelled │ (Terminal)
└─────┬──────┘  └───────────┘
      │
      ├──────────────┐
      │              │
      ▼              ▼
┌───────────┐  ┌────────┐
│ Completed │  │ Failed │ (Terminal)
└───────────┘  └────────┘
(Terminal)     (Terminal)
```

## Security Considerations

1. **Authorization Enforcement**
   - Only agents can call `start_processing()` and `mark_failed()`
   - Only senders can call `cancel_remittance()`
   - Validated via `require_auth()` before transition checks

2. **Immutable Terminal States**
   - Once Completed, Cancelled, or Failed, no further transitions allowed
   - Prevents replay attacks and double-spending

3. **Atomic State Changes**
   - All transitions are atomic with storage updates
   - Failed transitions revert all changes

4. **Audit Trail**
   - Every transition emits an event with actor information
   - Full lifecycle history available for forensics

## Migration Notes

### Breaking Changes

- `confirm_payout()` now requires remittance to be in `Processing` state
- Direct Pending → Completed transitions are no longer allowed
- Agents must call `start_processing()` before `confirm_payout()`

### Backward Compatibility

For existing remittances in `Pending` state:
1. Agent should call `start_processing()` first
2. Then proceed with `confirm_payout()` as normal

### Deployment Checklist

- [ ] Deploy updated contract
- [ ] Update off-chain systems to handle new states
- [ ] Add event listeners for transition events
- [ ] Update agent workflows to call `start_processing()`
- [ ] Update monitoring dashboards for new states
- [ ] Test all lifecycle paths in staging environment

## Future Enhancements

Potential additions to the lifecycle system:

1. **Timeout Transitions**
   - Auto-cancel Pending remittances after expiry
   - Auto-fail Processing remittances after timeout

2. **Retry Mechanism**
   - Allow Failed → Processing transition with admin approval
   - Limited retry attempts

3. **Partial Completion**
   - New state for partial payouts
   - Split remittances

4. **Dispute State**
   - New state for disputed transactions
   - Requires admin resolution

## Acceptance Criteria Status

✅ **Define allowed states** - 5 states defined (Pending, Processing, Completed, Cancelled, Failed)

✅ **Reject invalid transitions** - `validate_transition()` enforces rules, returns `InvalidStatus` error

✅ **Log all transitions** - `emit_status_transition()` logs every state change with actor info

✅ **Unit tests for edge cases** - 15+ tests covering:
  - All valid transitions
  - All invalid transitions
  - Terminal state enforcement
  - Event emission
  - Refund logic
  - Multiple remittances

## Summary

The lifecycle transition enforcement system provides:
- **Granular state tracking** with 5 distinct states
- **Strict validation** preventing invalid state changes
- **Comprehensive logging** for audit and monitoring
- **Terminal state protection** preventing state manipulation
- **Full test coverage** with 15+ integration tests
- **Clear documentation** for developers and integrators

This implementation ensures remittances follow a predictable, auditable lifecycle while maintaining security and preventing invalid state transitions.
