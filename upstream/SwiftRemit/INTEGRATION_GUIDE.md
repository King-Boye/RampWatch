# Response Wrapper Integration Guide

## Current Status

✅ **Response wrapper module created** (`src/response.rs`)  
✅ **Documentation complete** (`RESPONSE_WRAPPER.md`)  
⏳ **Integration pending** (awaiting codebase compilation fixes)

## How to Integrate

Once the existing compilation errors are resolved, add these wrapper functions to `lib.rs`:

### 1. Import the Response Module

```rust
// In src/lib.rs
mod response;
pub use response::*;
```

### 2. Add Query Wrapper Functions

Add these functions to the `impl SwiftRemitContract` block:

```rust
// ═══════════════════════════════════════════════════════════════════════
// Standardized Query Wrappers
// ═══════════════════════════════════════════════════════════════════════

/// Query remittance with standardized response format.
pub fn query_remittance(env: Env, remittance_id: u64) -> Response<Remittance> {
    match get_remittance(&env, remittance_id) {
        Ok(data) => Response::ok(data),
        Err(e) => Response::err(e as u32),
    }
}

/// Query accumulated fees with standardized response format.
pub fn query_fees(env: Env) -> Response<i128> {
    match get_accumulated_fees(&env) {
        Ok(data) => Response::ok(data),
        Err(e) => Response::err(e as u32),
    }
}

/// Query platform fee with standardized response format.
pub fn query_platform_fee(env: Env) -> Response<u32> {
    match get_platform_fee_bps(&env) {
        Ok(data) => Response::ok(data),
        Err(e) => Response::err(e as u32),
    }
}

/// Query agent registration status with standardized response format.
pub fn query_agent_status(env: Env, agent: Address) -> Response<bool> {
    Response::ok(is_agent_registered(&env, &agent))
}

/// Query contract pause status with standardized response format.
pub fn query_pause_status(env: Env) -> Response<bool> {
    Response::ok(is_paused(&env))
}

/// Query contract version with standardized response format.
pub fn query_version(env: Env) -> Response<soroban_sdk::String> {
    Response::ok(soroban_sdk::String::from_str(&env, env!("CARGO_PKG_VERSION")))
}
```

### 3. Usage Examples

#### From Off-Chain Client (JavaScript/TypeScript)

```javascript
// Query remittance
const response = await contract.query_remittance({ remittance_id: 42 });

if (response.success) {
  console.log("Remittance:", response.data);
  console.log("Amount:", response.data.amount);
  console.log("Status:", response.data.status);
} else {
  console.error("Error code:", response.error);
  handleError(response.error);
}

// Query fees
const feesResponse = await contract.query_fees();
if (feesResponse.success) {
  console.log("Accumulated fees:", feesResponse.data);
}

// Query agent status
const agentResponse = await contract.query_agent_status({ 
  agent: "GXXX...XXX" 
});
if (agentResponse.success && agentResponse.data) {
  console.log("Agent is registered");
}
```

#### From Rust Tests

```rust
#[test]
fn test_query_remittance_success() {
    let env = Env::default();
    let contract = create_contract(&env);
    
    // Create a remittance
    let id = contract.create_remittance(&sender, &agent, &1000, &None);
    
    // Query with standardized response
    let response = contract.query_remittance(&id);
    
    assert!(response.success);
    assert!(response.data.is_some());
    assert!(response.error.is_none());
    
    let remittance = response.data.unwrap();
    assert_eq!(remittance.amount, 1000);
}

#[test]
fn test_query_remittance_not_found() {
    let env = Env::default();
    let contract = create_contract(&env);
    
    // Query non-existent remittance
    let response = contract.query_remittance(&999);
    
    assert!(!response.success);
    assert!(response.data.is_none());
    assert_eq!(response.error, Some(ContractError::RemittanceNotFound as u32));
}
```

## Response Structure

All query functions return:

```rust
Response<T> {
    success: bool,
    data: Option<T>,
    error: Option<u32>,
}
```

### Success Response
```json
{
  "success": true,
  "data": { /* actual data */ },
  "error": null
}
```

### Error Response
```json
{
  "success": false,
  "data": null,
  "error": 6  // ContractError::RemittanceNotFound
}
```

## Benefits

✅ **Consistent structure** across all query operations  
✅ **Type-safe** with generic implementation  
✅ **Backward compatible** - existing functions unchanged  
✅ **Easy error handling** with standardized codes  
✅ **Off-chain friendly** - simple to parse in any language  

## Error Code Reference

| Code | Error | Description |
|------|-------|-------------|
| 1 | AlreadyInitialized | Contract already initialized |
| 2 | NotInitialized | Contract not initialized |
| 3 | InvalidAmount | Amount must be greater than 0 |
| 4 | InvalidFeeBps | Fee must be between 0-10000 bps |
| 5 | AgentNotRegistered | Agent not in approved list |
| 6 | RemittanceNotFound | Remittance ID does not exist |
| 7 | InvalidStatus | Operation not allowed in current status |
| 8 | Overflow | Arithmetic overflow detected |
| 9 | NoFeesToWithdraw | No accumulated fees available |

## Next Steps

1. ✅ Response module created
2. ✅ Documentation written
3. ⏳ Fix existing compilation errors in codebase
4. ⏳ Add wrapper functions to lib.rs
5. ⏳ Add tests for wrapper functions
6. ⏳ Update API documentation

## Note

The response wrapper module is complete and ready to use. Integration is blocked by pre-existing compilation errors in the codebase that are unrelated to this feature. Once those are resolved, the wrapper functions can be added following this guide.
