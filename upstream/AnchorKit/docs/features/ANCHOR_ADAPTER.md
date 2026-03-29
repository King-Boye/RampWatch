# Unified Anchor Adapter Interface

The `AnchorAdapter` trait provides a consistent API for integrating with multiple anchor services, regardless of their underlying protocol (SEP-24, SEP-31, custom implementations).

## Interface Design

```rust
pub trait AnchorAdapter {
    fn authenticate(&self, env: &Env, anchor: &Address, credentials: &Bytes) -> AuthResult;
    fn deposit(&self, env: &Env, auth: &AuthResult, request: &DepositRequest) -> DepositResponse;
    fn withdraw(&self, env: &Env, auth: &AuthResult, request: &WithdrawRequest) -> WithdrawResponse;
    fn get_info(&self, env: &Env, anchor: &Address) -> AnchorInfo;
    fn get_quote(&self, env: &Env, auth: &AuthResult, request: &QuoteRequest) -> Option<QuoteData>;
}
```

## Core Methods

### `authenticate()`
Establishes authentication with an anchor service.

**Parameters:**
- `env`: Soroban environment
- `anchor`: Anchor's address
- `credentials`: Authentication credentials (API key, JWT, etc.)

**Returns:** `AuthResult` containing:
- `token`: Authentication token
- `expires_at`: Token expiration timestamp
- `anchor`: Anchor address

### `deposit()`
Initiates a deposit transaction.

**Parameters:**
- `env`: Soroban environment
- `auth`: Authentication result from `authenticate()`
- `request`: Deposit parameters (asset, amount, destination, memo)

**Returns:** `DepositResponse` containing:
- `transaction_id`: Unique transaction identifier
- `status`: Current transaction status
- `deposit_address`: Address to send funds to
- `expires_at`: Deposit window expiration

### `withdraw()`
Initiates a withdrawal transaction.

**Parameters:**
- `env`: Soroban environment
- `auth`: Authentication result from `authenticate()`
- `request`: Withdrawal parameters (asset, amount, destination, memo)

**Returns:** `WithdrawResponse` containing:
- `transaction_id`: Unique transaction identifier
- `status`: Current transaction status
- `estimated_completion`: Expected completion timestamp

### `get_info()`
Retrieves anchor capabilities and configuration.

**Parameters:**
- `env`: Soroban environment
- `anchor`: Anchor's address

**Returns:** `AnchorInfo` containing:
- `name`: Anchor name
- `supported_services`: List of supported service types
- `supported_assets`: List of supported assets
- `min_deposit` / `max_deposit`: Deposit limits
- `min_withdrawal` / `max_withdrawal`: Withdrawal limits

### `get_quote()` (Optional)
Requests an exchange rate quote.

**Parameters:**
- `env`: Soroban environment
- `auth`: Authentication result
- `request`: Quote parameters (assets, amount, operation type)

**Returns:** `Option<QuoteData>` - `None` if quotes not supported

## Usage Example

```rust
use anchorkit::anchor_adapter::*;
use anchorkit::sep24_adapter::Sep24Adapter;

// Initialize adapter
let adapter = Sep24Adapter;

// Authenticate
let auth = adapter.authenticate(&env, &anchor_address, &credentials);

// Get anchor info
let info = adapter.get_info(&env, &anchor_address);

// Initiate deposit
let deposit_req = DepositRequest {
    asset: String::from_str(&env, "USDC"),
    amount: 100_0000000,
    destination: user_address.clone(),
    memo: None,
};
let deposit_resp = adapter.deposit(&env, &auth, &deposit_req);

// Initiate withdrawal
let withdraw_req = WithdrawRequest {
    asset: String::from_str(&env, "USDC"),
    amount: 50_0000000,
    destination: String::from_str(&env, "bank_account_123"),
    memo: None,
};
let withdraw_resp = adapter.withdraw(&env, &auth, &withdraw_req);
```

## Implementing Custom Adapters

To integrate a new anchor protocol:

1. **Create adapter struct:**
```rust
pub struct CustomAdapter;
```

2. **Implement AnchorAdapter trait:**
```rust
impl AnchorAdapter for CustomAdapter {
    fn authenticate(&self, env: &Env, anchor: &Address, credentials: &Bytes) -> AuthResult {
        // Custom authentication logic
    }
    
    fn deposit(&self, env: &Env, auth: &AuthResult, request: &DepositRequest) -> DepositResponse {
        // Custom deposit logic
    }
    
    // ... implement other methods
}
```

3. **Handle protocol-specific details internally:**
   - HTTP endpoints and request formats
   - Authentication mechanisms
   - Response parsing
   - Error handling

## Built-in Adapters

### SEP-24 Adapter
Reference implementation for SEP-24 compliant anchors (interactive deposits/withdrawals).

**File:** `src/sep24_adapter.rs`

**Features:**
- Standard SEP-24 authentication flow
- Interactive deposit/withdrawal support
- Info endpoint integration

## Benefits

- **Consistency**: Same API across all anchor integrations
- **Flexibility**: Easy to add new anchor protocols
- **Maintainability**: Protocol changes isolated to specific adapters
- **Testability**: Mock adapters for testing
- **Composability**: Adapters can be swapped at runtime

## Integration with AnchorKit

The adapter interface integrates with existing AnchorKit features:

- **Session Management**: Operations can be tracked in sessions
- **Health Monitoring**: Adapter calls can report health metrics
- **Request ID Propagation**: Request IDs flow through adapter calls
- **Retry Logic**: Failed adapter calls can use retry mechanisms
- **Rate Limiting**: Adapter calls respect rate limits

## Error Handling

Adapters should handle errors gracefully and return appropriate error codes. Consider using AnchorKit's error mapping for consistent error reporting.

## Security Considerations

- **Credentials**: Never log or expose credentials
- **Token Management**: Implement token refresh logic
- **Validation**: Validate all inputs before making external calls
- **Rate Limiting**: Respect anchor rate limits
- **Timeouts**: Implement appropriate timeouts for all operations

## Future Extensions

Potential additions to the interface:

- `get_transaction_status()` - Poll transaction status
- `cancel_transaction()` - Cancel pending transactions
- `get_transaction_history()` - Retrieve transaction history
- `update_kyc()` - Submit KYC information
- `get_fees()` - Query fee structure
