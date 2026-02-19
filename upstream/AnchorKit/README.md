# AnchorKit
AnchorKit is a Soroban-native toolkit for anchoring off-chain attestations to Stellar. It enables smart contracts to verify real-world events such as KYC approvals, payment confirmations, and signed claims in a trust-minimized way.

## Features

- Attestation management with replay attack protection
- Attestor registration and revocation
- Endpoint configuration for attestors
- Service capability discovery (deposits, withdrawals, quotes, KYC)
- Event emission for all state changes
- Comprehensive error handling with stable error codes

## Supported Services

Anchors can configure which services they support:

- **Deposits**: Accept incoming deposits from users
- **Withdrawals**: Process withdrawal requests
- **Quotes**: Provide exchange rate quotes
- **KYC**: Perform Know Your Customer verification

## Usage Example

```rust
// Initialize the contract
contract.initialize(&admin);

// Register an attestor/anchor
contract.register_attestor(&anchor);

// Configure supported services for the anchor
let mut services = Vec::new(&env);
services.push_back(ServiceType::Deposits);
services.push_back(ServiceType::Withdrawals);
services.push_back(ServiceType::KYC);
contract.configure_services(&anchor, &services);

// Query supported services
let supported = contract.get_supported_services(&anchor);

// Check if a specific service is supported
if contract.supports_service(&anchor, &ServiceType::Deposits) {
    // Process deposit
}
```
