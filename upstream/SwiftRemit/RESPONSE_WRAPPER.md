# Standardized Response Wrapper

## Overview

This module provides a standardized response structure for smart contract query operations, making it easier for off-chain integrations to handle responses consistently.

## Response Structure

```rust
pub struct Response<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<u32>,
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
  "error": 7  // ContractError code
}
```

## Usage

### In Contract Functions

```rust
use crate::response::Response;

pub fn query_remittance(env: Env, remittance_id: u64) -> Response<Remittance> {
    match get_remittance(&env, remittance_id) {
        Ok(data) => Response::ok(data),
        Err(e) => Response::err(e as u32),
    }
}
```

### Creating Responses

```rust
// Success response
let response = Response::ok(remittance_data);

// Error response
let response = Response::err(ContractError::RemittanceNotFound as u32);
```

## Benefits

1. **Consistency**: All query functions return the same structure
2. **Type Safety**: Generic type parameter ensures correct data types
3. **Error Handling**: Standardized error codes from ContractError enum
4. **Off-Chain Integration**: Easy to parse and handle in client applications

## Integration with Existing Functions

The response wrapper can be used alongside existing functions:

- **Existing**: `get_remittance()` returns `Result<Remittance, ContractError>`
- **New**: `query_remittance()` returns `Response<Remittance>`

This maintains backward compatibility while providing a new standardized interface.

## Error Codes

Error codes correspond to the `ContractError` enum:

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

## Example: Off-Chain Client

```javascript
// JavaScript/TypeScript client example
const response = await contract.query_remittance({ remittance_id: 42 });

if (response.success) {
  console.log("Remittance data:", response.data);
} else {
  console.error("Error code:", response.error);
  handleError(response.error);
}
```

## Future Enhancements

Potential additions:
- Add error message strings
- Include timestamp in responses
- Add pagination metadata for list queries
- Support for batch queries

## Note

This module provides the response wrapper type. To use it in contract functions, you'll need to:

1. Import the module in `lib.rs`
2. Create wrapper functions that use `Response<T>`
3. Export the wrapper functions in the contract interface

The module is designed to be minimal and focused, following the principle of doing one thing well.
