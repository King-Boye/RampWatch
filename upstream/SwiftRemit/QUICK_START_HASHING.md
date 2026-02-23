# Quick Start: Deterministic Hashing

## For External Systems

### JavaScript/Node.js

```javascript
import { computeSettlementId, usdcToStroops } from './examples/settlement-id-generator.js';

const settlementId = computeSettlementId(
    1,                                    // remittance_id
    'GABC...XYZ',                        // sender address
    'GDEF...UVW',                        // agent address
    usdcToStroops(100),                  // 100 USDC
    usdcToStroops(2.5),                  // 2.5 USDC fee
    null                                  // no expiry
);

console.log('Settlement ID:', settlementId.toString('hex'));
```

### Python

```python
import hashlib
from stellar_sdk import Address

def compute_settlement_id(remittance_id, sender, agent, amount, fee, expiry):
    buffer = bytearray()
    buffer.extend(remittance_id.to_bytes(8, 'big', signed=False))
    buffer.extend(Address(sender).to_xdr_sc_address().to_xdr_bytes())
    buffer.extend(Address(agent).to_xdr_sc_address().to_xdr_bytes())
    buffer.extend(amount.to_bytes(16, 'big', signed=True))
    buffer.extend(fee.to_bytes(16, 'big', signed=True))
    buffer.extend((expiry or 0).to_bytes(8, 'big', signed=False))
    return hashlib.sha256(buffer).digest()
```

### Rust (Soroban)

```rust
use swiftremit::compute_settlement_id;

let settlement_id = compute_settlement_id(
    &env,
    1,              // remittance_id
    &sender,        // sender address
    &agent,         // agent address
    10000000,       // 1 USDC (7 decimals)
    250000,         // 0.025 USDC fee
    None,           // no expiry
);
```

## Verification

```javascript
// Compute locally
const localHash = computeSettlementId(...);

// Get from blockchain
const onChainHash = await contract.compute_settlement_hash(remittanceId);

// Verify match
if (localHash.equals(onChainHash)) {
    console.log('✅ Settlement ID verified!');
} else {
    console.error('❌ Hash mismatch - data corruption or version mismatch');
}
```

## Key Points

1. **Field Order Matters** - Always use: remittance_id, sender, agent, amount, fee, expiry
2. **Big-Endian Encoding** - All integers must be big-endian
3. **XDR for Addresses** - Use Stellar XDR encoding for addresses
4. **None = Zero** - expiry None and Some(0) produce identical hashes
5. **SHA-256** - Always use SHA-256 for hashing

## Documentation

- Full spec: `DETERMINISTIC_HASHING_SPEC.md`
- JavaScript examples: `examples/settlement-id-generator.js`
- Rust implementation: `src/hashing.rs`
