# Deterministic Hashing Standard for Cross-System Compatibility

## Overview

This document specifies the canonical method for generating settlement IDs in the SwiftRemit system. External systems (banks, anchors, payment APIs) can use this specification to independently generate identical settlement IDs from the same remittance data, enabling cross-system verification and reconciliation.

## Schema Version

**Current Version:** `1`

External systems should record this version alongside stored settlement IDs to handle future schema changes.

## Hash Algorithm

**Algorithm:** SHA-256  
**Output:** 32 bytes (256 bits)  
**Encoding:** Raw bytes (not hex-encoded)

## Canonical Input Ordering

Settlement IDs are computed by hashing a deterministic byte sequence constructed from remittance fields in this **exact order**:

| Field | Type | Size | Encoding | Notes |
|-------|------|------|----------|-------|
| 1. `remittance_id` | u64 | 8 bytes | Big-endian | Unique remittance counter |
| 2. `sender` | Address | Variable | XDR | Stellar address (sender) |
| 3. `agent` | Address | Variable | XDR | Stellar address (agent) |
| 4. `amount` | i128 | 16 bytes | Big-endian | Payment amount in stroops |
| 5. `fee` | i128 | 16 bytes | Big-endian | Fee amount in stroops |
| 6. `expiry` | u64 | 8 bytes | Big-endian | Unix timestamp or 0 if None |

### Important Notes

- **Status is excluded**: The `status` field (Pending/Completed/Cancelled) is intentionally not included in the hash, as it changes during the remittance lifecycle
- **No separators**: Fields are concatenated directly with no delimiters
- **Fixed-width integers**: All integers use fixed-width encoding (big-endian)
- **Optional fields**: `expiry` uses `0x0000000000000000` when None

## Serialization Rules

### 1. Integer Encoding

All integers use **big-endian (network byte order)** encoding:

```
u64:  8 bytes, most significant byte first
i128: 16 bytes, most significant byte first
```

**Example:**
```
remittance_id = 1
Encoded: 0x0000000000000001

amount = 1000000 (1 USDC with 7 decimals)
Encoded: 0x00000000000000000000000000000000000F4240
```

### 2. Address Encoding

Stellar addresses are encoded using **XDR (External Data Representation)** format as defined in the Stellar protocol.

**Address Types:**
- Account addresses: `G...` (56 characters, base32-encoded Ed25519 public key)
- Contract addresses: `C...` (56 characters, base32-encoded contract ID)

**XDR Encoding Process:**
1. Decode the base32 Stellar address to get the raw public key or contract ID
2. Encode using Stellar XDR format for `ScAddress` type
3. The XDR encoding includes type discriminator and the raw bytes

**Reference Implementation:**
- Stellar SDK: `Address.toXDR()`
- Rust: `soroban_sdk::Address::to_xdr()`
- JavaScript: `stellar-sdk` Address XDR encoding

### 3. Optional Field Encoding

The `expiry` field is optional:
- **When present:** Encode the u64 timestamp value
- **When absent (None):** Use 8 zero bytes: `0x0000000000000000`

**Critical:** `None` and `Some(0)` produce identical hashes.

## Step-by-Step Algorithm

### Pseudocode

```
function compute_settlement_id(remittance_id, sender, agent, amount, fee, expiry):
    buffer = empty_byte_array()
    
    // Step 1: Append remittance_id (u64, big-endian)
    buffer.append(to_big_endian_u64(remittance_id))
    
    // Step 2: Append sender address (XDR-encoded)
    buffer.append(address_to_xdr(sender))
    
    // Step 3: Append agent address (XDR-encoded)
    buffer.append(address_to_xdr(agent))
    
    // Step 4: Append amount (i128, big-endian)
    buffer.append(to_big_endian_i128(amount))
    
    // Step 5: Append fee (i128, big-endian)
    buffer.append(to_big_endian_i128(fee))
    
    // Step 6: Append expiry (u64, big-endian, 0 if None)
    expiry_value = expiry if expiry is not None else 0
    buffer.append(to_big_endian_u64(expiry_value))
    
    // Step 7: Compute SHA-256 hash
    settlement_id = sha256(buffer)
    
    return settlement_id  // 32 bytes
```

## Implementation Examples

### Rust (Soroban)

```rust
use soroban_sdk::{Address, Bytes, BytesN, Env};
use soroban_sdk::xdr::ToXdr;

pub fn compute_settlement_id(
    env: &Env,
    remittance_id: u64,
    sender: &Address,
    agent: &Address,
    amount: i128,
    fee: i128,
    expiry: Option<u64>,
) -> BytesN<32> {
    let mut buf = Bytes::new(env);

    // Field 1: remittance_id
    buf.extend_from_array(&remittance_id.to_be_bytes());

    // Field 2: sender address (XDR)
    buf.append(&sender.to_xdr(env));

    // Field 3: agent address (XDR)
    buf.append(&agent.to_xdr(env));

    // Field 4: amount
    buf.extend_from_array(&amount.to_be_bytes());

    // Field 5: fee
    buf.extend_from_array(&fee.to_be_bytes());

    // Field 6: expiry (0 if None)
    let expiry_val: u64 = expiry.unwrap_or(0);
    buf.extend_from_array(&expiry_val.to_be_bytes());

    // SHA-256 hash
    env.crypto().sha256(&buf).into()
}
```

### JavaScript/TypeScript

```javascript
import { Address, xdr } from '@stellar/stellar-sdk';
import crypto from 'crypto';

function computeSettlementId(
    remittanceId,    // number
    senderAddress,   // string (Stellar address)
    agentAddress,    // string (Stellar address)
    amount,          // bigint
    fee,             // bigint
    expiry           // number | null
) {
    const buffers = [];

    // Field 1: remittance_id (u64, big-endian)
    const remittanceIdBuf = Buffer.alloc(8);
    remittanceIdBuf.writeBigUInt64BE(BigInt(remittanceId));
    buffers.push(remittanceIdBuf);

    // Field 2: sender address (XDR)
    const sender = Address.fromString(senderAddress);
    buffers.push(sender.toXDRObject().toXDR());

    // Field 3: agent address (XDR)
    const agent = Address.fromString(agentAddress);
    buffers.push(agent.toXDRObject().toXDR());

    // Field 4: amount (i128, big-endian)
    buffers.push(i128ToBigEndian(amount));

    // Field 5: fee (i128, big-endian)
    buffers.push(i128ToBigEndian(fee));

    // Field 6: expiry (u64, big-endian, 0 if null)
    const expiryValue = expiry ?? 0;
    const expiryBuf = Buffer.alloc(8);
    expiryBuf.writeBigUInt64BE(BigInt(expiryValue));
    buffers.push(expiryBuf);

    // Concatenate all buffers
    const input = Buffer.concat(buffers);

    // Compute SHA-256
    const hash = crypto.createHash('sha256').update(input).digest();

    return hash; // 32 bytes
}

function i128ToBigEndian(value) {
    const buf = Buffer.alloc(16);
    const isNegative = value < 0n;
    const absValue = isNegative ? -value : value;
    
    // Write as big-endian
    buf.writeBigUInt64BE(absValue >> 64n, 0);
    buf.writeBigUInt64BE(absValue & 0xFFFFFFFFFFFFFFFFn, 8);
    
    // Two's complement for negative
    if (isNegative) {
        for (let i = 0; i < 16; i++) {
            buf[i] = ~buf[i];
        }
        let carry = 1;
        for (let i = 15; i >= 0; i--) {
            const sum = buf[i] + carry;
            buf[i] = sum & 0xFF;
            carry = sum >> 8;
        }
    }
    
    return buf;
}
```

### Python

```python
import hashlib
from stellar_sdk import Address
from stellar_sdk.xdr import SCAddress

def compute_settlement_id(
    remittance_id: int,
    sender_address: str,
    agent_address: str,
    amount: int,
    fee: int,
    expiry: int | None
) -> bytes:
    buffer = bytearray()
    
    # Field 1: remittance_id (u64, big-endian)
    buffer.extend(remittance_id.to_bytes(8, 'big', signed=False))
    
    # Field 2: sender address (XDR)
    sender = Address(sender_address)
    buffer.extend(sender.to_xdr_sc_address().to_xdr_bytes())
    
    # Field 3: agent address (XDR)
    agent = Address(agent_address)
    buffer.extend(agent.to_xdr_sc_address().to_xdr_bytes())
    
    # Field 4: amount (i128, big-endian)
    buffer.extend(amount.to_bytes(16, 'big', signed=True))
    
    # Field 5: fee (i128, big-endian)
    buffer.extend(fee.to_bytes(16, 'big', signed=True))
    
    # Field 6: expiry (u64, big-endian, 0 if None)
    expiry_value = expiry if expiry is not None else 0
    buffer.extend(expiry_value.to_bytes(8, 'big', signed=False))
    
    # Compute SHA-256
    settlement_id = hashlib.sha256(buffer).digest()
    
    return settlement_id  # 32 bytes
```

## Test Vectors

### Test Case 1: Basic Remittance

**Input:**
```
remittance_id: 1
sender: GABC...XYZ (example address)
agent: GDEF...UVW (example address)
amount: 10000000 (1 USDC, 7 decimals)
fee: 250000 (0.025 USDC, 2.5% fee)
expiry: None
```

**Expected Behavior:**
- Same inputs → identical hash across all implementations
- Hash is deterministic and reproducible

### Test Case 2: With Expiry

**Input:**
```
remittance_id: 2
sender: GABC...XYZ
agent: GDEF...UVW
amount: 50000000 (5 USDC)
fee: 1250000 (0.125 USDC)
expiry: 1735689600 (2025-01-01 00:00:00 UTC)
```

**Expected Behavior:**
- Hash differs from Test Case 1
- Reproducible across systems

### Test Case 3: Expiry None vs Zero

**Input A:**
```
expiry: None
```

**Input B:**
```
expiry: Some(0)
```

**Expected Behavior:**
- Both produce **identical** hashes (critical requirement)

## Validation Checklist

External systems implementing this specification should verify:

- [ ] Same inputs produce identical hashes across multiple runs
- [ ] Different remittance IDs produce different hashes
- [ ] Swapping sender and agent produces different hashes
- [ ] `expiry: None` and `expiry: Some(0)` produce identical hashes
- [ ] Hash output is exactly 32 bytes
- [ ] All integers use big-endian encoding
- [ ] Addresses use Stellar XDR encoding
- [ ] No separators between fields

## Integration Guidelines

### For Banks and Payment Processors

1. **Store Schema Version**: Record `HASH_SCHEMA_VERSION = 1` with each settlement
2. **Pre-compute Hashes**: Generate settlement IDs before submitting to blockchain
3. **Verify On-Chain**: Compare your computed hash with on-chain settlement ID
4. **Reconciliation**: Use settlement IDs as primary keys for matching transactions

### For Anchors and APIs

1. **Idempotency**: Use settlement IDs to prevent duplicate processing
2. **Audit Trail**: Log input parameters and computed hashes
3. **Cross-System Verification**: Share settlement IDs with partners for reconciliation
4. **Error Detection**: Mismatched hashes indicate data corruption or version mismatch

### For Monitoring Systems

1. **Real-time Verification**: Compute expected settlement ID from event data
2. **Anomaly Detection**: Flag transactions where computed hash doesn't match
3. **Compliance**: Use deterministic IDs for regulatory reporting

## Security Considerations

### Hash Collision Resistance

SHA-256 provides:
- **Collision resistance**: Computationally infeasible to find two inputs with same hash
- **Pre-image resistance**: Cannot reverse hash to recover inputs
- **Determinism**: Same inputs always produce same output

### Data Integrity

The deterministic hash ensures:
- **Tamper detection**: Any change to input parameters changes the hash
- **Cross-system verification**: Independent systems can verify settlement authenticity
- **Audit trail**: Immutable settlement IDs for compliance

### Privacy Considerations

Settlement IDs reveal:
- ✅ Unique identifier for the transaction
- ❌ No sensitive data (addresses, amounts) can be extracted from hash alone

## Troubleshooting

### Common Issues

**Issue:** Hashes don't match between systems

**Possible Causes:**
1. **Byte order**: Ensure big-endian encoding for all integers
2. **Address encoding**: Must use Stellar XDR format, not raw base32
3. **Field order**: Verify exact order specified in this document
4. **Expiry handling**: Ensure `None` is encoded as 8 zero bytes
5. **Integer size**: i128 must be 16 bytes, u64 must be 8 bytes

**Debugging Steps:**
1. Log the raw byte buffer before hashing
2. Compare byte-by-byte with reference implementation
3. Verify XDR encoding of addresses matches Stellar SDK
4. Test with known test vectors

### Reference Implementation

The canonical implementation is in the SwiftRemit contract:
- **File:** `src/hashing.rs`
- **Function:** `compute_settlement_id()`
- **Repository:** https://github.com/Haroldwonder/SwiftRemit

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 1 | 2026-02-21 | Initial specification |

## Support

For questions or issues with this specification:
1. Review the reference implementation in `src/hashing.rs`
2. Check test cases in `src/hashing.rs` (tests module)
3. Open an issue on the SwiftRemit repository

---

**Status:** ✅ PRODUCTION READY  
**Schema Version:** 1  
**Last Updated:** 2026-02-21
