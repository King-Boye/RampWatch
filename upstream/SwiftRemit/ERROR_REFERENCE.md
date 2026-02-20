# Error Reference Guide

## Overview

SwiftRemit uses descriptive error codes to help developers quickly identify and resolve integration issues. Each error provides specific context about what went wrong.

## Error Codes

### Initialization Errors

#### `AlreadyInitialized` (Code: 1)
**Meaning**: Contract has already been initialized and cannot be initialized again.

**Common Causes**:
- Calling `initialize()` multiple times
- Attempting to reinitialize after deployment

**Solution**: Check if contract is already initialized before calling `initialize()`.

---

#### `NotInitialized` (Code: 2)
**Meaning**: Contract has not been initialized yet.

**Common Causes**:
- Calling contract functions before `initialize()`
- Using wrong contract address

**Solution**: Call `initialize()` with admin, USDC token address, and fee_bps first.

---

### Validation Errors

#### `InvalidAmount` (Code: 3)
**Meaning**: Amount must be greater than zero.

**Common Causes**:
- Passing 0 to `create_remittance()`
- Passing negative amount

**Solution**: Ensure amount > 0 when creating remittances.

**Example**:
```rust
// ❌ Wrong
contract.create_remittance(&sender, &agent, &0, &None);

// ✅ Correct
contract.create_remittance(&sender, &agent, &1000, &None);
```

---

#### `InvalidFeeBps` (Code: 4)
**Meaning**: Fee basis points must be between 0-10000 (representing 0%-100%).

**Common Causes**:
- Setting fee_bps > 10000 in `initialize()`
- Setting fee_bps > 10000 in `update_fee()`

**Solution**: Use valid basis points (e.g., 250 = 2.5%, 500 = 5%).

**Example**:
```rust
// ❌ Wrong
contract.initialize(&admin, &token, &10001);

// ✅ Correct
contract.initialize(&admin, &token, &250); // 2.5%
```

---

#### `InvalidAddress` (Code: 10)
**Meaning**: Address validation failed.

**Common Causes**:
- Malformed address
- Invalid address format

**Solution**: Ensure addresses are properly formatted Stellar addresses.

---

### Agent Errors

#### `AgentNotRegistered` (Code: 5)
**Meaning**: The specified agent is not registered in the system.

**Common Causes**:
- Creating remittance with unregistered agent
- Agent was removed after remittance creation

**Solution**: Register agent using `register_agent()` before creating remittances.

**Example**:
```rust
// Register agent first
contract.register_agent(&agent);

// Then create remittance
contract.create_remittance(&sender, &agent, &1000, &None);
```

---

### Remittance Errors

#### `RemittanceNotFound` (Code: 6)
**Meaning**: The remittance ID does not exist in storage.

**Common Causes**:
- Using wrong remittance_id
- Remittance was never created
- Typo in remittance_id

**Solution**: Verify remittance_id from `create_remittance()` return value.

---

#### `InvalidStatus` (Code: 7)
**Meaning**: Operation not allowed in current remittance status.

**Common Causes**:
- Confirming already completed remittance
- Cancelling already completed remittance
- Operating on cancelled remittance

**Solution**: Check remittance status before operations. Only `Pending` remittances can be confirmed or cancelled.

**Status Flow**:
```
Pending → Completed (via confirm_payout)
Pending → Cancelled (via cancel_remittance)
```

---

#### `SettlementExpired` (Code: 11)
**Meaning**: Settlement window has expired.

**Common Causes**:
- Calling `confirm_payout()` after expiry timestamp
- Agent took too long to confirm

**Solution**: Ensure settlements happen before expiry time, or sender can cancel.

---

#### `DuplicateSettlement` (Code: 12)
**Meaning**: Settlement already executed for this remittance.

**Common Causes**:
- Attempting to settle the same remittance twice
- Race condition in settlement calls

**Solution**: Check remittance status before confirming. This is a safety check to prevent double-spending.

---

### Fee Errors

#### `NoFeesToWithdraw` (Code: 9)
**Meaning**: No accumulated fees available to withdraw.

**Common Causes**:
- Calling `withdraw_fees()` when fees are zero
- All fees already withdrawn

**Solution**: Check `get_accumulated_fees()` before withdrawing.

**Example**:
```rust
let fees = contract.get_accumulated_fees();
if fees > 0 {
    contract.withdraw_fees(&recipient);
}
```

---

### System Errors

#### `Overflow` (Code: 8)
**Meaning**: Arithmetic operation resulted in overflow.

**Common Causes**:
- Amount too large for i128
- Fee calculation overflow
- Accumulated fees overflow

**Solution**: Use reasonable amounts within i128 limits.

---

#### `ContractPaused` (Code: 13)
**Meaning**: Contract is paused. Settlements are temporarily disabled.

**Common Causes**:
- Admin paused contract for emergency
- Attempting `confirm_payout()` during pause

**Solution**: Wait for admin to unpause contract, or contact admin. Users can still cancel remittances.

---

## Error Handling Best Practices

### 1. Check Before Operations
```rust
// Check agent registration
if !contract.is_agent_registered(&agent) {
    // Register agent first
    contract.register_agent(&agent);
}

// Check pause status
if !contract.is_paused() {
    contract.confirm_payout(&remittance_id);
}
```

### 2. Handle Errors Gracefully
```rust
match contract.create_remittance(&sender, &agent, &amount, &None) {
    Ok(id) => println!("Remittance created: {}", id),
    Err(ContractError::AgentNotRegistered) => {
        println!("Agent not registered. Please register first.");
    },
    Err(ContractError::InvalidAmount) => {
        println!("Amount must be greater than zero.");
    },
    Err(e) => println!("Error: {:?}", e),
}
```

### 3. Validate Inputs
```rust
// Validate amount
if amount <= 0 {
    return Err("Amount must be positive");
}

// Validate fee
if fee_bps > 10000 {
    return Err("Fee must be 0-10000 bps");
}
```

## Quick Reference Table

| Code | Error | When It Happens | Quick Fix |
|------|-------|----------------|-----------|
| 1 | AlreadyInitialized | Calling initialize() twice | Don't reinitialize |
| 2 | NotInitialized | Using uninitialized contract | Call initialize() first |
| 3 | InvalidAmount | Amount ≤ 0 | Use positive amount |
| 4 | InvalidFeeBps | Fee > 10000 bps | Use 0-10000 range |
| 5 | AgentNotRegistered | Agent not in system | Register agent first |
| 6 | RemittanceNotFound | Invalid remittance_id | Check remittance_id |
| 7 | InvalidStatus | Wrong status for operation | Check status first |
| 8 | Overflow | Amount too large | Use smaller amounts |
| 9 | NoFeesToWithdraw | Fees are zero | Check fees before withdraw |
| 10 | InvalidAddress | Bad address format | Use valid Stellar address |
| 11 | SettlementExpired | Past expiry time | Settle before expiry |
| 12 | DuplicateSettlement | Already settled | Check if settled |
| 13 | ContractPaused | Contract paused | Wait for unpause |

## Testing Errors

All error conditions are covered in the test suite. See `src/test.rs` for examples of triggering and handling each error.
