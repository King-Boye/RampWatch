# CLI Example - Deposit/Withdraw Workflow

## Overview

Demonstrates AnchorKit usage with a complete deposit/withdraw workflow using mock transport.

## Running the Example

### Bash Script (Quick Demo)

```bash
cd examples
./cli_example.sh
```

### Rust Example (Full Integration)

```bash
cargo run --example cli_example
```

## Workflow Steps

### 1. Initialize Contract
```rust
client.initialize(&admin);
```

### 2. Register Anchor
```rust
client.register_attestor(&anchor);
```

### 3. Configure Services
```rust
let services = vec![&env, ServiceType::Deposits, ServiceType::Withdrawals];
client.configure_services(&anchor, &services);
```

### 4. Configure Assets
```rust
let assets = vec![&env, String::from_str(&env, "USDC")];
client.set_supported_assets(&anchor, &assets);
```

### 5. Deposit Flow
```rust
// Validate asset
let is_supported = client.is_asset_supported(&anchor, &usdc);

// Generate request ID
let request_id = client.generate_request_id();

// Submit attestation
let attestation_id = client.submit_with_request_id(
    &request_id,
    &anchor,
    &user,
    &timestamp,
    &payload_hash,
    &signature,
);
```

### 6. Request Quote
```rust
let quote_id = client.submit_quote(
    &anchor,
    &base_asset,
    &quote_asset,
    &rate,
    &fee_percentage,
    &min_amount,
    &max_amount,
    &valid_until,
);
```

### 7. Withdraw Flow
```rust
// Same as deposit, with different request ID
let request_id = client.generate_request_id();
let attestation_id = client.submit_with_request_id(...);
```

### 8. Health Check
```rust
client.update_health_status(&anchor, &latency_ms, &failure_count, &availability);
let health = client.get_health_status(&anchor);
```

### 9. Audit Trail
```rust
let span = client.get_tracing_span(&request_id.id);
```

## Features Demonstrated

- ✅ Contract initialization
- ✅ Anchor registration
- ✅ Service configuration
- ✅ Asset validation
- ✅ Request ID tracking
- ✅ Attestation submission
- ✅ Quote management
- ✅ Health monitoring
- ✅ Audit trail

## Mock Transport

The example uses Soroban's built-in mock environment:
- No network connection required
- Instant execution
- Perfect for testing and development

## Output Example

```
🚀 AnchorKit CLI Example - Deposit/Withdraw Workflow
==================================================

📋 Configuration:
  Admin:  GADMIN...
  Anchor: GANCHOR...
  User:   GUSER...

1️⃣  Initializing contract...
   ✅ Contract initialized

2️⃣  Registering anchor...
   ✅ Anchor registered

3️⃣  Configuring anchor services...
   → Services: Deposits, Withdrawals
   ✅ Services configured

4️⃣  Configuring supported assets...
   → Assets: USDC, BTC, ETH
   ✅ Assets configured

5️⃣  Initiating deposit flow...
   → User: GUSER...
   → Asset: USDC
   → Amount: 1000
   ✅ Deposit attestation recorded (ID: 1)

6️⃣  Requesting quote...
   → Pair: USDC/USD
   → Rate: 1.0000
   → Fee: 1%
   ✅ Quote received (ID: 1)

7️⃣  Initiating withdraw flow...
   → User: GUSER...
   → Asset: USDC
   → Amount: 500
   ✅ Withdraw attestation recorded (ID: 2)

8️⃣  Checking anchor health...
   → Latency: 45ms
   → Availability: 99.9%
   ✅ Anchor healthy

9️⃣  Retrieving audit trail...
   → Total operations: 2
   ✅ Audit trail complete

✅ Workflow completed successfully!

📊 Summary:
  - Deposits: 1 (1000 USDC)
  - Withdrawals: 1 (500 USDC)
  - Net balance: 500 USDC
  - Total attestations: 2
```

## Adapting for Production

Replace mock transport with real Stellar network:

```rust
// Instead of mock environment
let env = Env::default();

// Use real network
let network = Network::Testnet; // or Network::Public
let contract_address = "CCONTRACT...";
```

## Next Steps

1. **Customize workflow** - Modify for your use case
2. **Add error handling** - Handle failures gracefully
3. **Connect to network** - Deploy and test on testnet
4. **Add UI** - Build frontend integration
5. **Monitor operations** - Use audit trail for debugging

## Files

- `cli_example.sh` - Bash demo script
- `cli_example.rs` - Full Rust implementation
- `CLI_EXAMPLE.md` - This documentation
