# Mock Anchor Usage Guide

The mock anchor allows developers to simulate attestation signing without external APIs.

## Quick Start

```rust
use anchorkit::mock_anchor::MockAnchor;
use soroban_sdk::{Bytes, Env};

#[test]
fn test_with_mock_anchor() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AnchorKitContract);
    let client = AnchorKitContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let issuer = Address::generate(&env);
    let subject = Address::generate(&env);
    
    // Initialize contract
    client.initialize(&admin);
    
    // Register attestor
    client.register_attestor(&issuer);
    
    // Create mock attestation
    let timestamp = env.ledger().timestamp();
    let payload = Bytes::from_slice(&env, b"KYC approved");
    let payload_hash = MockAnchor::hash_payload(&env, &payload);
    let signature = MockAnchor::sign(&env, &issuer, &subject, timestamp, &payload_hash);
    
    // Submit attestation
    let attestation_id = client.submit_attestation(
        &issuer,
        &subject,
        &timestamp,
        &payload_hash,
        &signature
    );
    
    assert!(attestation_id > 0);
}
```

## API

### `MockAnchor::sign()`
Generate a mock signature for testing.

**Parameters:**
- `env: &Env` - Soroban environment
- `issuer: &Address` - Attestor address
- `subject: &Address` - Subject address
- `timestamp: u64` - Timestamp
- `payload_hash: &BytesN<32>` - Payload hash

**Returns:** `Bytes` - Mock signature

### `MockAnchor::hash_payload()`
Create a payload hash from data.

**Parameters:**
- `env: &Env` - Soroban environment
- `data: &Bytes` - Payload data

**Returns:** `BytesN<32>` - SHA-256 hash

## Enabling in Your Project

Add the `testutils` feature to your `Cargo.toml`:

```toml
[dependencies]
anchorkit = { version = "0.1.0", features = ["testutils"] }
```

Or use it in tests (automatically available):

```toml
[dev-dependencies]
anchorkit = "0.1.0"
```

## Notes

- Mock signatures are deterministic based on inputs
- Not cryptographically secure - for testing only
- Automatically available in test builds
