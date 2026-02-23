# On-Chain Transfer State Registry (#100)

## Overview

Minimal on-chain state registry for transfer auditability and external indexing with validated state transitions.

## Implementation

### 1. TransferState Enum (`src/types.rs`)

```rust
#[contracttype]
pub enum TransferState {
    Initiated,    // Transfer created
    Processing,   // Agent started processing
    Completed,    // Successfully settled
    Refunded,     // Cancelled/failed with refund
}
```

### 2. State Transition Validation

**Valid Transitions:**
```
Initiated → Processing → Completed
Initiated → Refunded
Processing → Refunded
```

**Terminal States:**
- `Completed` - Cannot transition further
- `Refunded` - Cannot transition further

**Validation Logic:**
- Enforced via `can_transition_to()` method
- Returns `InvalidStateTransition` error for invalid transitions
- Idempotent: Same state transitions allowed

### 3. Storage (`src/storage.rs`)

**DataKey:**
- `TransferState(u64)` - Persistent storage indexed by transfer ID

**Functions:**
```rust
get_transfer_state(env, transfer_id) -> Option<TransferState>
set_transfer_state(env, transfer_id, new_state) -> Result<(), ContractError>
```

**Storage Efficiency:**
- Skips write if setting same state (idempotent optimization)
- Only writes on actual state changes

### 4. Integration (`src/lib.rs`)

**State Transitions:**
- `create_remittance()` → Sets `Initiated`
- `confirm_payout()` → Sets `Processing`, then `Completed`
- `cancel_remittance()` → Sets `Refunded`

**Read-Only Getter:**
```rust
pub fn get_transfer_state(env, transfer_id) -> Option<TransferState>
```

## Acceptance Criteria

✅ **State transitions must be validated (no skipping)**
- `can_transition_to()` enforces valid paths
- Returns `InvalidStateTransition` error for invalid transitions
- Cannot skip from `Initiated` to `Completed` without `Processing`

✅ **Provide read-only getter for indexers**
- `get_transfer_state()` public function
- No authentication required (read-only)
- Returns `Option<TransferState>`

✅ **Must be storage-efficient (avoid redundant writes)**
- Checks current state before writing
- Skips write if state unchanged
- Only persists actual state changes

## Usage Example

```rust
// Create remittance (auto-sets Initiated)
let id = contract.create_remittance(sender, agent, 1000, None);

// Check state
let state = contract.get_transfer_state(id); // Some(Initiated)

// Confirm payout (transitions: Processing → Completed)
contract.confirm_payout(id);
let state = contract.get_transfer_state(id); // Some(Completed)

// Or cancel (transitions: Initiated → Refunded)
contract.cancel_remittance(id);
let state = contract.get_transfer_state(id); // Some(Refunded)
```

## State Transition Diagram

```
    ┌───────────┐
    │ Initiated │
    └─────┬─────┘
          │
    ┌─────┴──────┐
    │            │
    ▼            ▼
┌──────────┐  ┌──────────┐
│Processing│  │ Refunded │ (terminal)
└─────┬────┘  └──────────┘
      │
  ┌───┴────┐
  │        │
  ▼        ▼
┌─────────┐ ┌──────────┐
│Completed│ │ Refunded │ (terminal)
└─────────┘ └──────────┘
(terminal)
```

## Testing

Tests in `src/test_transfer_state.rs`:
- `test_transfer_state_transitions` - Valid transition paths
- `test_invalid_state_transitions` - Validation enforcement
- `test_terminal_states_cannot_transition` - Terminal state immutability
- `test_refund_path` - Refund scenarios
- `test_idempotent_same_state` - Same state handling
- `test_storage_efficiency` - Redundant write prevention

## Indexer Integration

External indexers can:
1. Call `get_transfer_state(transfer_id)` to query current state
2. Monitor state changes via events
3. Build audit trails of transfer lifecycle
4. No authentication required for reads
