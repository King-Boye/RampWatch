# Interactive Support

## Overview
Minimal implementation abstracting interactive anchor flows from frontend.

## Features
- ✅ Interactive URL generation
- ✅ Token injection
- ✅ Callback handling  
- ✅ Status polling

## Types

### `InteractiveUrl`
```rust
pub struct InteractiveUrl {
    pub url: String,
    pub transaction_id: String,
    pub expires_at: u64,
}
```

### `CallbackData`
```rust
pub struct CallbackData {
    pub transaction_id: String,
    pub status: String,
    pub timestamp: u64,
}
```

### `TransactionStatus`
```rust
pub struct TransactionStatus {
    pub id: String,
    pub status: String,
    pub updated_at: u64,
}
```

## Contract Methods

```rust
// Generate interactive URL
generate_interactive_url(env, anchor, token, tx_id) -> InteractiveUrl

// Handle callback
handle_anchor_callback(env, tx_id, status) -> CallbackData

// Poll status
poll_transaction_status(env, tx_id) -> TransactionStatus
```

## Usage

```rust
// Generate URL
let url = contract.generate_interactive_url(
    &env, anchor, token, tx_id
);

// Handle callback
let callback = contract.handle_anchor_callback(
    &env, tx_id, status
);

// Poll status
let status = contract.poll_transaction_status(
    &env, tx_id
);
```

## Testing

```bash
cargo test interactive_support --lib
```

## Integration

Fully integrated into AnchorKit contract with exported types.
