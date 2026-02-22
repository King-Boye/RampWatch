# Validation Quick Reference Guide

## Quick Start

### Using Validation in Contract Functions

```rust
use crate::validation::*;

pub fn your_function(env: Env, param: Type) -> Result<(), ContractError> {
    // Step 1: Validate request
    validate_your_request(&env, &param)?;
    
    // Step 2: Execute business logic
    // ... your code here
    
    Ok(())
}
```

## Common Validation Patterns

### Pattern 1: Validate Single Field

```rust
// Validate amount
validate_amount(amount)?;

// Validate fee
validate_fee_bps(fee_bps)?;

// Validate address
validate_address(&address)?;
```

### Pattern 2: Validate Multiple Fields

```rust
validate_address(&sender)?;
validate_address(&agent)?;
validate_amount(amount)?;
validate_agent_registered(&env, &agent)?;
```

### Pattern 3: Use Comprehensive Validator

```rust
// Instead of multiple validations, use comprehensive validator
validate_create_remittance_request(&env, &sender, &agent, amount)?;
```

## Validation Functions Cheat Sheet

| Function | Validates | Error Returned |
|----------|-----------|----------------|
| `validate_address()` | Address format | InvalidAddress |
| `validate_fee_bps()` | Fee 0-10000 | InvalidFeeBps |
| `validate_amount()` | Amount > 0 | InvalidAmount |
| `validate_agent_registered()` | Agent exists | AgentNotRegistered |
| `validate_not_paused()` | Not paused | ContractPaused |
| `validate_remittance_exists()` | Remittance exists | RemittanceNotFound |
| `validate_remittance_pending()` | Status pending | InvalidStatus |
| `validate_settlement_not_expired()` | Not expired | SettlementExpired |
| `validate_no_duplicate_settlement()` | Not duplicate | DuplicateSettlement |
| `validate_fees_available()` | Fees > 0 | NoFeesToWithdraw |

## Comprehensive Validators

### Initialize Request
```rust
validate_initialize_request(&env, &admin, &token, fee_bps)?;
// Validates: admin, token, fee_bps, not initialized, token whitelisted
```

### Create Remittance Request
```rust
validate_create_remittance_request(&env, &sender, &agent, amount)?;
// Validates: sender, agent, amount, agent registered
```

### Confirm Payout Request
```rust
let remittance = validate_confirm_payout_request(&env, remittance_id)?;
// Validates: not paused, exists, pending, no duplicate, not expired, agent
// Returns: validated remittance
```

### Cancel Remittance Request
```rust
let remittance = validate_cancel_remittance_request(&env, remittance_id)?;
// Validates: exists, pending, sender
// Returns: validated remittance
```

### Withdraw Fees Request
```rust
let fees = validate_withdraw_fees_request(&env, &to)?;
// Validates: address, fees available
// Returns: fee amount
```

### Update Fee Request
```rust
validate_update_fee_request(fee_bps)?;
// Validates: fee_bps in range
```

### Admin Operation
```rust
validate_admin_operation(&env, &caller, &target)?;
// Validates: caller, target, caller is admin
```

## Error Handling

### Basic Error Handling
```rust
match validate_amount(amount) {
    Ok(_) => {
        // Continue processing
    }
    Err(e) => {
        // Handle error
        return Err(e);
    }
}
```

### Using ? Operator (Recommended)
```rust
// Automatically propagates error
validate_amount(amount)?;
validate_address(&sender)?;
```

## Common Error Codes

| Code | Error | When It Occurs |
|------|-------|----------------|
| 3 | InvalidAmount | Amount ≤ 0 |
| 4 | InvalidFeeBps | Fee > 10000 |
| 5 | AgentNotRegistered | Agent not in system |
| 6 | RemittanceNotFound | Invalid remittance ID |
| 7 | InvalidStatus | Wrong status for operation |
| 9 | NoFeesToWithdraw | Fees = 0 |
| 10 | InvalidAddress | Bad address format |
| 11 | SettlementExpired | Past expiry time |
| 12 | DuplicateSettlement | Already settled |
| 13 | ContractPaused | Contract paused |

## Testing Validation

### Test Invalid Input
```rust
#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn test_invalid_amount() {
    contract.create_remittance(&sender, &agent, &0, &None);
}
```

### Test Valid Input
```rust
#[test]
fn test_valid_amount() {
    let result = contract.create_remittance(&sender, &agent, &1000, &None);
    assert!(result.is_ok());
}
```

## Best Practices

### ✅ DO

```rust
// DO: Validate before business logic
pub fn operation(env: Env, param: Type) -> Result<(), ContractError> {
    validate_request(&env, &param)?;
    // business logic
}

// DO: Use comprehensive validators
validate_create_remittance_request(&env, &sender, &agent, amount)?;

// DO: Propagate errors with ?
validate_amount(amount)?;

// DO: Return early on validation failure
if amount <= 0 {
    return Err(ContractError::InvalidAmount);
}
```

### ❌ DON'T

```rust
// DON'T: Skip validation
pub fn operation(env: Env, param: Type) -> Result<(), ContractError> {
    // Missing validation!
    // business logic
}

// DON'T: Validate after state changes
set_state(&env, value);
validate_value(value)?; // Too late!

// DON'T: Ignore validation errors
let _ = validate_amount(amount); // Error ignored!

// DON'T: Duplicate validation logic
if amount <= 0 { return Err(...); }
if amount <= 0 { return Err(...); } // Use validate_amount() instead
```

## Validation Order

Always validate in this order:

1. **Contract State** (initialized, not paused)
2. **Addresses** (valid format)
3. **Amounts** (positive, non-zero)
4. **References** (agent registered, remittance exists)
5. **Status** (pending, not expired, not duplicate)
6. **Authorization** (require_auth)

Example:
```rust
pub fn confirm_payout(env: Env, remittance_id: u64) -> Result<(), ContractError> {
    // 1. Contract state
    validate_not_paused(&env)?;
    
    // 2-5. Comprehensive validation
    let remittance = validate_confirm_payout_request(&env, remittance_id)?;
    
    // 6. Authorization
    remittance.agent.require_auth();
    
    // Business logic
}
```

## Adding New Validation

### Step 1: Add Basic Validator
```rust
// In src/validation.rs
pub fn validate_new_field(field: &Type) -> Result<(), ContractError> {
    if !is_valid(field) {
        return Err(ContractError::NewError);
    }
    Ok(())
}
```

### Step 2: Add to Comprehensive Validator
```rust
pub fn validate_operation_request(
    env: &Env,
    field: &Type,
) -> Result<(), ContractError> {
    validate_new_field(field)?;
    // other validations
    Ok(())
}
```

### Step 3: Use in Contract
```rust
pub fn operation(env: Env, field: Type) -> Result<(), ContractError> {
    validate_operation_request(&env, &field)?;
    // business logic
}
```

### Step 4: Add Tests
```rust
#[test]
fn test_validate_new_field() {
    assert!(validate_new_field(&valid_value).is_ok());
    assert_eq!(validate_new_field(&invalid_value), Err(ContractError::NewError));
}
```

## Debugging Validation Errors

### Check Error Code
```rust
match result {
    Err(ContractError::InvalidAmount) => {
        // Amount validation failed
    }
    Err(ContractError::AgentNotRegistered) => {
        // Agent validation failed
    }
    _ => {}
}
```

### Common Issues

| Issue | Cause | Solution |
|-------|-------|----------|
| InvalidAmount | amount ≤ 0 | Use positive amount |
| InvalidFeeBps | fee > 10000 | Use 0-10000 range |
| AgentNotRegistered | Agent not added | Call register_agent first |
| RemittanceNotFound | Wrong ID | Check remittance_id |
| InvalidStatus | Already processed | Check status first |
| ContractPaused | Contract paused | Wait for unpause |

## Performance Tips

1. **Validate early**: Fail fast on invalid input
2. **Use comprehensive validators**: Reduce function calls
3. **Cache validation results**: Don't validate same field twice
4. **Order validations**: Cheap checks first (amount) before expensive (storage lookups)

## Examples

### Example 1: Simple Validation
```rust
pub fn set_fee(env: Env, fee_bps: u32) -> Result<(), ContractError> {
    validate_fee_bps(fee_bps)?;
    set_platform_fee_bps(&env, fee_bps);
    Ok(())
}
```

### Example 2: Multiple Validations
```rust
pub fn transfer(env: Env, from: Address, to: Address, amount: i128) -> Result<(), ContractError> {
    validate_address(&from)?;
    validate_address(&to)?;
    validate_amount(amount)?;
    // transfer logic
    Ok(())
}
```

### Example 3: Comprehensive Validation
```rust
pub fn create_remittance(
    env: Env,
    sender: Address,
    agent: Address,
    amount: i128,
) -> Result<u64, ContractError> {
    validate_create_remittance_request(&env, &sender, &agent, amount)?;
    sender.require_auth();
    // creation logic
    Ok(remittance_id)
}
```

## Summary

- **Always validate first** before business logic
- **Use comprehensive validators** for complete validation
- **Propagate errors** with `?` operator
- **Test both valid and invalid** inputs
- **Follow validation order** for consistency
- **Document validation rules** in code comments

For complete documentation, see [VALIDATION.md](VALIDATION.md)
