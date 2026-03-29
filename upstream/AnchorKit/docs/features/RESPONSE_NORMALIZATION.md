# Response Normalization Layer

This module provides a normalization layer for different anchor response structures, ensuring a consistent output format across all anchor operations.

## Output Format

All normalized responses follow this structure:

```rust
{
    status: String,   // Operation status (e.g., "pending", "processing", "quoted")
    amount: u64,      // Transaction amount in stroops
    asset: String,    // Asset code (e.g., "USDC", "XLM")
    fee: u64,         // Fee amount in stroops
    id: String,       // Transaction or quote identifier
}
```

## Usage

### Normalize Deposit Response

```rust
let deposit_response = DepositResponse {
    transaction_id: String::from_str(&env, "dep_123"),
    status: String::from_str(&env, "pending"),
    deposit_address: String::from_str(&env, "GDEPOSIT..."),
    expires_at: 1000,
};

let normalized = contract.normalize_deposit_response(
    deposit_response,
    100_0000000,  // 100 USDC
    String::from_str(&env, "USDC"),
    1_0000000,    // 1 USDC fee
);
```

### Normalize Withdraw Response

```rust
let withdraw_response = WithdrawResponse {
    transaction_id: String::from_str(&env, "wd_456"),
    status: String::from_str(&env, "processing"),
    estimated_completion: 2000,
};

let normalized = contract.normalize_withdraw_response(
    withdraw_response,
    50_0000000,   // 50 USDC
    String::from_str(&env, "USDC"),
    500000,       // 0.5 USDC fee
);
```

### Normalize Quote Response

```rust
let normalized = contract.normalize_quote_response(
    anchor_address,
    quote_id,
    100_0000000,  // 100 USDC
    String::from_str(&env, "quote_789"),
);
```

## Benefits

- **Consistency**: All anchor responses follow the same structure
- **Simplicity**: Easy to work with normalized data
- **Validation**: Built-in validation ensures data integrity
- **Flexibility**: Supports different anchor implementations

## Fee Calculation

Fees are automatically calculated for quotes based on the fee percentage:

```rust
fee = (amount * fee_percentage) / 10000
```

Where `fee_percentage` is in basis points (e.g., 50 = 0.5%).
