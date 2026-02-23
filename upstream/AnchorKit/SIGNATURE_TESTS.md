# Signature Generation Consistency Tests

## Overview
Tests verifying request signing is reproducible across executions with consistent signature generation.

## Test Implementation

### File: `src/signature_tests.rs`

### Test Cases

#### 1. `test_same_input_produces_identical_signature`
**Purpose**: Verify same input always produces identical signature.

**Test**:
- Generate signature twice with same input
- Verify both signatures are identical

**Assertions**:
- ✅ sig1 == sig2

#### 2. `test_signature_reproducible_across_executions`
**Purpose**: Signatures are reproducible across different execution environments.

**Test**:
- Create two separate environments
- Generate signatures with identical input
- Compare signatures

**Assertions**:
- ✅ Signatures match across environments

#### 3. `test_different_input_produces_different_signature`
**Purpose**: Different inputs produce different signatures.

**Test**:
- Generate signatures with different inputs
- Verify signatures differ

**Assertions**:
- ✅ sig1 != sig2

#### 4. `test_signature_with_key_is_deterministic`
**Purpose**: Signatures with keys are deterministic.

**Test**:
- Generate signature twice with same data and key
- Verify consistency

**Assertions**:
- ✅ Deterministic output

#### 5. `test_different_keys_produce_different_signatures`
**Purpose**: Different keys produce different signatures.

**Test**:
- Same data with two different keys
- Verify signatures differ

**Assertions**:
- ✅ Key affects signature

#### 6. `test_empty_key_produces_signature`
**Purpose**: Empty keys produce consistent signatures.

**Test**:
- Generate signature with empty key twice
- Verify consistency

**Assertions**:
- ✅ Consistent with empty key

#### 7. `test_signature_consistency_multiple_runs`
**Purpose**: Multiple runs produce identical signatures.

**Test**:
- Generate signature 5 times
- Verify all are identical

**Assertions**:
- ✅ All 5 signatures match

#### 8. `test_signature_with_zero_data`
**Purpose**: Zero data produces consistent signature.

**Test**:
- Generate signature with all zeros
- Verify reproducibility

**Assertions**:
- ✅ Consistent with zero data

#### 9. `test_signature_with_max_data`
**Purpose**: Maximum value data produces consistent signature.

**Test**:
- Generate signature with all 255s
- Verify reproducibility

**Assertions**:
- ✅ Consistent with max data

#### 10. `test_key_order_matters`
**Purpose**: Key byte order affects signature.

**Test**:
- Same data with reversed key bytes
- Verify signatures differ

**Assertions**:
- ✅ Order matters

#### 11. `test_signature_length_consistency`
**Purpose**: All signatures have consistent length.

**Test**:
- Generate signatures with different inputs
- Verify all are 32 bytes

**Assertions**:
- ✅ All signatures are 32 bytes

## Key Features

✅ **Reproducibility**: Same input → identical signature  
✅ **Determinism**: Consistent across executions  
✅ **Key Sensitivity**: Different keys → different signatures  
✅ **Input Sensitivity**: Different data → different signatures  
✅ **Length Consistency**: All signatures are 32 bytes  
✅ **Edge Cases**: Zero data, max data, empty keys

## Signature Generation

### Basic Signature
```rust
fn generate_signature(env: &Env, data: &BytesN<32>) -> BytesN<32> {
    env.crypto().sha256(&Bytes::from_array(env, &data.to_array())).into()
}
```

### Signature with Key
```rust
fn generate_signature_with_key(env: &Env, data: &BytesN<32>, key: &Bytes) -> BytesN<32> {
    let mut combined = Bytes::new(env);
    combined.append(&Bytes::from_array(env, &data.to_array()));
    combined.append(key);
    env.crypto().sha256(&combined).into()
}
```

## Test Scenarios

### Scenario 1: Reproducibility
```
Input: [1; 32]
Run 1: sig_a
Run 2: sig_b
Result: sig_a == sig_b ✓
```

### Scenario 2: Key Sensitivity
```
Data: [5; 32]
Key1: [10, 20, 30]
Key2: [40, 50, 60]
Result: sig1 != sig2 ✓
```

### Scenario 3: Multiple Executions
```
Input: [100; 32]
Runs: 5 times
Result: All identical ✓
```

## Running Tests

```bash
cargo test signature_tests --lib
```

## Test Results
```
running 11 tests
test signature_tests::test_different_input_produces_different_signature ... ok
test signature_tests::test_different_keys_produce_different_signatures ... ok
test signature_tests::test_empty_key_produces_signature ... ok
test signature_tests::test_key_order_matters ... ok
test signature_tests::test_same_input_produces_identical_signature ... ok
test signature_tests::test_signature_consistency_multiple_runs ... ok
test signature_tests::test_signature_length_consistency ... ok
test signature_tests::test_signature_reproducible_across_executions ... ok
test signature_tests::test_signature_with_key_is_deterministic ... ok
test signature_tests::test_signature_with_max_data ... ok
test signature_tests::test_signature_with_zero_data ... ok

test result: ok. 11 passed; 0 failed
```

## Properties Verified

✅ **Determinism**: f(x) always produces same output  
✅ **Consistency**: Works across environments  
✅ **Uniqueness**: Different inputs → different outputs  
✅ **Key Binding**: Keys affect signature  
✅ **Length**: Always 32 bytes (SHA-256)
