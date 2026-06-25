# Response Normalization - Quick Reference

## Core Structure

```rust
pub struct NormalizedResponse {
    pub status: String,
    pub amount: u64,
    pub asset: String,
    pub fee: u64,
    pub id: String,
}
```

## Contract Methods

```rust
// Normalize deposit
normalize_deposit_response(
    env: Env,
    response: DepositResponse,
    amount: u64,
    asset: String,
    fee: u64,
) -> Result<NormalizedResponse, Error>

// Normalize withdrawal
normalize_withdraw_response(
    env: Env,
    response: WithdrawResponse,
    amount: u64,
    asset: String,
    fee: u64,
) -> Result<NormalizedResponse, Error>

// Normalize quote
normalize_quote_response(
    env: Env,
    anchor: Address,
    quote_id: u64,
    amount: u64,
    id_prefix: String,
) -> Result<NormalizedResponse, Error>
```

## Example

```rust
// Different anchors return different structures:
// Anchor A: { tx_id, state, ... }
// Anchor B: { transaction_id, status, ... }
// Anchor C: { id, processing_status, ... }

// All normalize to:
{
    status: "pending",
    amount: 100_0000000,
    asset: "USDC",
    fee: 1_0000000,
    id: "tx_123"
}
```

## Error Handling

- `ProtocolInvalidPayload` - Empty status or id
- `UnsupportedAsset` - Empty asset
- `InvalidQuote` - Quote not found

## Testing

Run tests:
```bash
cargo test --lib response_normalizer
```
