# SwiftRemit Lifecycle State Machine

## State Transition Diagram

```
                    ┌─────────────────────────────────────┐
                    │                                     │
                    │         REMITTANCE CREATED          │
                    │                                     │
                    └──────────────┬──────────────────────┘
                                   │
                                   ▼
                    ┌──────────────────────────┐
                    │                          │
                    │        PENDING           │
                    │   (Initial State)        │
                    │                          │
                    └────┬──────────────┬──────┘
                         │              │
                         │              │
        ┌────────────────┘              └────────────────┐
        │                                                │
        │ start_processing()                             │ cancel_remittance()
        │ (Agent)                                        │ (Sender)
        │                                                │
        ▼                                                ▼
┌──────────────────┐                          ┌──────────────────┐
│                  │                          │                  │
│   PROCESSING     │                          │    CANCELLED     │
│                  │                          │   (Terminal)     │
│                  │                          │                  │
└────┬────────┬────┘                          └──────────────────┘
     │        │                                         │
     │        │                                         │
     │        │                                         ▼
     │        │                                  Full Refund
     │        │                                  to Sender
     │        │
     │        └──────────────────┐
     │                           │
     │ confirm_payout()          │ mark_failed()
     │ (Agent)                   │ (Agent)
     │                           │
     ▼                           ▼
┌──────────────────┐    ┌──────────────────┐
│                  │    │                  │
│    COMPLETED     │    │     FAILED       │
│   (Terminal)     │    │   (Terminal)     │
│                  │    │                  │
└──────────────────┘    └──────────────────┘
     │                           │
     │                           │
     ▼                           ▼
Funds to Agent              Full Refund
(minus fee)                 to Sender
```

## State Details

### 🟡 PENDING (Initial State)
- **Entry:** Remittance created via `create_remittance()`
- **Characteristics:**
  - Funds locked in escrow
  - Sender can cancel
  - Agent has not started processing
- **Valid Transitions:**
  - → PROCESSING (via `start_processing()`)
  - → CANCELLED (via `cancel_remittance()`)

### 🔵 PROCESSING (Active State)
- **Entry:** Agent calls `start_processing()`
- **Characteristics:**
  - Agent actively working on payout
  - Sender cannot cancel
  - Funds still in escrow
- **Valid Transitions:**
  - → COMPLETED (via `confirm_payout()`)
  - → FAILED (via `mark_failed()`)

### 🟢 COMPLETED (Terminal State)
- **Entry:** Agent calls `confirm_payout()`
- **Characteristics:**
  - Payout successful
  - Funds transferred to agent (minus fee)
  - Platform fee accumulated
  - No further transitions allowed
- **Valid Transitions:** None (terminal)

### 🔴 CANCELLED (Terminal State)
- **Entry:** Sender calls `cancel_remittance()`
- **Characteristics:**
  - Cancelled before processing started
  - Full refund to sender
  - No fee charged
  - No further transitions allowed
- **Valid Transitions:** None (terminal)

### 🟠 FAILED (Terminal State)
- **Entry:** Agent calls `mark_failed()`
- **Characteristics:**
  - Payout attempt failed
  - Full refund to sender
  - No fee charged
  - No further transitions allowed
- **Valid Transitions:** None (terminal)

## Transition Rules

### ✅ Valid Transitions

| From | To | Function | Actor | Conditions |
|------|-----|----------|-------|------------|
| Pending | Processing | `start_processing()` | Agent | Agent is registered |
| Pending | Cancelled | `cancel_remittance()` | Sender | Sender owns remittance |
| Processing | Completed | `confirm_payout()` | Agent | Not expired, not duplicate |
| Processing | Failed | `mark_failed()` | Agent | Agent is registered |

### ❌ Invalid Transitions

| From | To | Reason |
|------|-----|--------|
| Pending | Completed | Must go through Processing |
| Pending | Failed | Must go through Processing |
| Processing | Pending | Cannot revert to pending |
| Processing | Cancelled | Cannot cancel once processing |
| Completed | Any | Terminal state |
| Cancelled | Any | Terminal state |
| Failed | Any | Terminal state |

## Authorization Matrix

| Function | Pending | Processing | Completed | Cancelled | Failed |
|----------|---------|------------|-----------|-----------|--------|
| `start_processing()` | ✅ Agent | ❌ | ❌ | ❌ | ❌ |
| `cancel_remittance()` | ✅ Sender | ❌ | ❌ | ❌ | ❌ |
| `confirm_payout()` | ❌ | ✅ Agent | ❌ | ❌ | ❌ |
| `mark_failed()` | ❌ | ✅ Agent | ❌ | ❌ | ❌ |

## Event Flow

Every state transition emits a `status_transition` event:

```rust
Event {
  topics: ["status", "transit"],
  data: {
    schema_version: 1,
    ledger_sequence: u32,
    timestamp: u64,
    remittance_id: u64,
    from_status: RemittanceStatus,
    to_status: RemittanceStatus,
    actor: Address,
  }
}
```

## Example Flows

### Flow 1: Successful Remittance
```
1. create_remittance()     → PENDING
2. start_processing()      → PROCESSING
3. confirm_payout()        → COMPLETED ✓
```

### Flow 2: Early Cancellation
```
1. create_remittance()     → PENDING
2. cancel_remittance()     → CANCELLED ✓
```

### Flow 3: Failed Payout
```
1. create_remittance()     → PENDING
2. start_processing()      → PROCESSING
3. mark_failed()           → FAILED ✓
```

### Flow 4: Invalid - Skip Processing
```
1. create_remittance()     → PENDING
2. confirm_payout()        → ❌ InvalidStatus Error
```

### Flow 5: Invalid - Cancel During Processing
```
1. create_remittance()     → PENDING
2. start_processing()      → PROCESSING
3. cancel_remittance()     → ❌ InvalidStatus Error
```

### Flow 6: Invalid - Modify Terminal State
```
1. create_remittance()     → PENDING
2. cancel_remittance()     → CANCELLED
3. start_processing()      → ❌ InvalidStatus Error
```

## Implementation Notes

### Validation Function
```rust
pub fn validate_transition(
    from: &RemittanceStatus,
    to: &RemittanceStatus,
) -> Result<(), ContractError>
```

### Usage in Contract
```rust
// Before changing state
validate_transition(&remittance.status, &RemittanceStatus::Processing)?;

// Update state
let old_status = remittance.status.clone();
remittance.status = RemittanceStatus::Processing;

// Emit event
emit_status_transition(&env, remittance_id, old_status, 
                      RemittanceStatus::Processing, actor);
```

## Security Properties

1. **Monotonic Progression**: States generally move forward, no backwards transitions
2. **Terminal State Immutability**: Completed, Cancelled, and Failed cannot be changed
3. **Authorization Enforcement**: Each transition requires specific actor authorization
4. **Audit Trail**: All transitions logged with actor and timestamp
5. **Atomic Updates**: State changes are atomic with storage updates

## Testing Coverage

- ✅ All valid transitions
- ✅ All invalid transitions
- ✅ Terminal state enforcement
- ✅ Authorization checks
- ✅ Event emission
- ✅ Refund logic
- ✅ Multiple concurrent remittances
- ✅ Edge cases (expired, duplicate, etc.)
