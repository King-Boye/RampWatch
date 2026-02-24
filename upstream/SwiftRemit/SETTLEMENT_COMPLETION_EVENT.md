# Settlement Completion Event

## Overview

The Settlement Completion Event is a structured event emitted exactly once when a settlement is finalized. This feature provides reliable notification of completed settlements with strong guarantees against duplicate emission, enabling off-chain systems to track settlement status with confidence.

## Key Features

- **Exactly-Once Emission**: Event is emitted once and only once per completed settlement
- **Post-Finalization**: Only emitted after all state transitions are successfully committed
- **Unique Identification**: Includes remittance_id for unambiguous reference
- **Deterministic**: Same settlement always produces same event
- **Re-entry Protection**: Protected against duplicate emission on retries or repeated calls

## Event Structure

### Topic
```rust
("settle", "complete")
```

### Data Fields
```rust
(
    schema_version: u32,      // Event schema version (currently 1)
    ledger_sequence: u32,     // Ledger sequence number when emitted
    timestamp: u64,           // Unix timestamp when emitted
    remittance_id: u64,       // Unique settlement identifier
    sender: Address,          // Original sender address
    receiver: Address,        // Receiver (agent) address
    asset: Address,           // Token contract address (e.g., USDC)
    amount: i128             // Settlement amount transferred
)
```

## Implementation Details

### Storage Tracking

The implementation uses persistent storage to track event emission status:

```rust
DataKey::SettlementEventEmitted(u64)  // Indexed by remittance_id
```

Two storage functions provide the tracking mechanism:

1. **has_settlement_event_emitted(env, remittance_id)** - Checks if event was emitted
2. **set_settlement_event_emitted(env, remittance_id)** - Marks event as emitted

### Emission Logic

The event is emitted in two places:

#### 1. Single Settlement (confirm_payout)

```rust
pub fn confirm_payout(env: Env, remittance_id: u64) -> Result<(), ContractError> {
    // ... validation and state transitions ...
    
    // Mark settlement as executed
    set_settlement_hash(&env, remittance_id);
    
    // Emit completion event exactly once
    if !has_settlement_event_emitted(&env, remittance_id) {
        emit_settlement_completed(
            &env,
            remittance_id,
            remittance.sender.clone(),
            remittance.agent.clone(),
            usdc_token.clone(),
            payout_amount
        );
        set_settlement_event_emitted(&env, remittance_id);
    }
    
    Ok(())
}
```

#### 2. Batch Settlement (batch_settle_with_netting)

```rust
pub fn batch_settle_with_netting(
    env: Env,
    entries: Vec<BatchSettlementEntry>,
) -> Result<BatchSettlementResult, ContractError> {
    // ... net settlement logic ...
    
    // Mark all remittances as completed
    for i in 0..remittances.len() {
        let mut remittance = remittances.get_unchecked(i);
        remittance.status = RemittanceStatus::Settled;
        set_remittance(&env, remittance.id, &remittance);
        set_settlement_hash(&env, remittance.id);
        
        // Emit completion event exactly once per remittance
        if !has_settlement_event_emitted(&env, remittance.id) {
            emit_settlement_completed(
                &env,
                remittance.id,
                remittance.sender.clone(),
                remittance.agent.clone(),
                usdc_token.clone(),
                payout_amount,
            );
            set_settlement_event_emitted(&env, remittance.id);
        }
    }
    
    Ok(BatchSettlementResult { settled_ids })
}
```

## Guarantees

### 1. Exactly-Once Emission

The event is emitted exactly once per settlement through:
- Pre-emission check: `has_settlement_event_emitted()`
- Post-emission flag: `set_settlement_event_emitted()`
- Persistent storage ensures flag survives contract restarts

### 2. Post-Finalization Only

The event is only emitted after:
1. All validations pass
2. Token transfer completes
3. Fee accumulation succeeds
4. Status updated to Settled
5. Settlement hash set

### 3. No Duplicate Emission

Protected against duplicates from:
- Re-entry attacks
- Retry logic
- Repeated function calls
- Contract upgrades/restarts

### 4. Deterministic Behavior

- Same settlement always produces same event
- Event data is derived from committed state
- No randomness or external dependencies

## Use Cases

### Off-Chain Settlement Tracking

```javascript
// Listen for settlement completion events
contract.on('settle.complete', (event) => {
    const {
        remittance_id,
        sender,
        receiver,
        asset,
        amount,
        timestamp
    } = event.data;
    
    // Update database
    db.settlements.update({
        id: remittance_id,
        status: 'completed',
        completed_at: timestamp,
        final_amount: amount
    });
    
    // Notify parties
    notifyUser(sender, `Settlement ${remittance_id} completed`);
    notifyAgent(receiver, `Received ${amount} for settlement ${remittance_id}`);
});
```

### Audit Trail

```javascript
// Query all completion events for audit
const completionEvents = await contract.queryEvents({
    topic: ['settle', 'complete'],
    from_ledger: start_ledger,
    to_ledger: end_ledger
});

// Generate audit report
const report = completionEvents.map(event => ({
    remittance_id: event.data.remittance_id,
    sender: event.data.sender,
    receiver: event.data.receiver,
    amount: event.data.amount,
    timestamp: event.data.timestamp,
    ledger: event.ledger_sequence
}));
```

### Reconciliation

```javascript
// Reconcile on-chain events with off-chain records
const onChainSettlements = await getCompletionEvents();
const offChainSettlements = await db.settlements.findCompleted();

const missing = offChainSettlements.filter(offline => 
    !onChainSettlements.some(online => 
        online.remittance_id === offline.id
    )
);

if (missing.length > 0) {
    console.error('Reconciliation mismatch:', missing);
}
```

## Testing

The implementation includes comprehensive tests covering all scenarios:

### Test Coverage

1. **test_settlement_completion_event_emitted_once** - Verifies exactly-once emission
2. **test_settlement_completion_event_not_emitted_before_finalization** - Ensures event only after finalization
3. **test_settlement_completion_event_includes_remittance_id** - Validates remittance_id in event
4. **test_settlement_completion_event_not_emitted_on_cancellation** - No event on cancellation
5. **test_settlement_completion_event_multiple_settlements** - Multiple settlements work correctly
6. **test_settlement_completion_event_batch_settlement** - Batch settlement emits per remittance
7. **test_settlement_completion_event_deterministic** - Same input produces same event
8. **test_settlement_completion_event_after_state_commit** - Event after state changes
9. **test_settlement_completion_event_unique_per_settlement** - Each settlement gets unique event
10. **test_settlement_completion_event_not_emitted_on_failed_settlement** - No event on failure

### Running Tests

```bash
# Run all completion event tests
cargo test settlement_completion_event

# Run specific test
cargo test test_settlement_completion_event_emitted_once

# Run with output
cargo test settlement_completion_event -- --nocapture
```

## Security Considerations

### 1. Storage Integrity

- Event emission flag stored in persistent storage
- Survives contract upgrades and restarts
- Cannot be manipulated by non-admin users

### 2. Re-entry Protection

- Flag checked before emission
- Flag set immediately after emission
- Atomic operation prevents race conditions

### 3. Authorization

- Event emission follows same authorization as settlement
- Only authorized agents can trigger settlement
- Event data derived from validated state

### 4. Data Integrity

- All event data comes from committed contract state
- No external inputs can corrupt event data
- Deterministic encoding ensures consistency

## Migration Considerations

The event emission tracking is included in the migration system:

```rust
// Export includes event emission flags
pub fn export_migration_state(env: Env, caller: Address) 
    -> Result<MigrationSnapshot, ContractError>

// Import restores event emission flags
pub fn import_migration_state(env: Env, caller: Address, snapshot: MigrationSnapshot) 
    -> Result<(), ContractError>
```

When migrating to a new contract:
1. Export state from old contract (includes emission flags)
2. Verify snapshot hash
3. Import state to new contract
4. Event emission history preserved

## Performance Impact

### Storage Cost

- One persistent storage entry per settlement
- Key: `SettlementEventEmitted(remittance_id)`
- Value: `bool` (minimal storage)

### Computation Cost

- One storage read per settlement (check flag)
- One storage write per settlement (set flag)
- Negligible impact on gas costs

### Event Cost

- One event emission per settlement
- Standard Soroban event cost
- No additional overhead

## Best Practices

### For Contract Integrators

1. **Always listen for completion events** - Don't rely solely on function return values
2. **Use remittance_id for deduplication** - Event may be received multiple times by listener
3. **Validate event data** - Cross-reference with on-chain state if critical
4. **Handle event ordering** - Events may arrive out of order in distributed systems

### For Contract Developers

1. **Never emit event before state commit** - Ensures consistency
2. **Always check emission flag** - Prevents duplicates
3. **Set flag immediately after emission** - Minimizes re-entry window
4. **Include sufficient identifiers** - Enable unambiguous reference

## Troubleshooting

### Event Not Received

**Possible causes:**
- Settlement not finalized (check status)
- Event listener not configured correctly
- Network issues with event subscription

**Solution:**
```javascript
// Query event directly from ledger
const events = await contract.queryEvents({
    topic: ['settle', 'complete'],
    filters: { remittance_id: target_id }
});
```

### Duplicate Events Received

**Possible causes:**
- Multiple event listeners configured
- Event replay in distributed system
- Client-side deduplication missing

**Solution:**
```javascript
// Deduplicate by remittance_id
const seen = new Set();
contract.on('settle.complete', (event) => {
    const id = event.data.remittance_id;
    if (seen.has(id)) return;
    seen.add(id);
    // Process event
});
```

### Event Data Mismatch

**Possible causes:**
- Reading stale on-chain state
- Event from different contract instance
- Schema version mismatch

**Solution:**
```javascript
// Verify event schema version
if (event.data.schema_version !== 1) {
    console.warn('Unexpected schema version:', event.data.schema_version);
}

// Cross-reference with on-chain state
const remittance = await contract.get_remittance(event.data.remittance_id);
assert(remittance.status === 'Settled');
```

## Future Enhancements

Potential improvements for future versions:

1. **Event Batching** - Emit single event for batch settlements with array of IDs
2. **Event Metadata** - Include additional context (gas used, execution time)
3. **Event Versioning** - Support multiple schema versions for backward compatibility
4. **Event Compression** - Reduce event size for large batches

## Related Documentation

- [Net Settlement](NET_SETTLEMENT.md) - Net settlement logic and batch processing
- [Migration System](MIGRATION.md) - State migration including event flags
- [Error Handling](ERROR_HANDLING.md) - Error codes and handling patterns
- [API Reference](API.md) - Complete API documentation
