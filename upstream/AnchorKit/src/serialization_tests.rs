#![cfg(test)]

extern crate alloc;
use alloc::vec;

use crate::{
    serialization::{
        compute_hash, serialize_attestation_for_signing, serialize_quote_data,
        serialize_quote_request, serialize_session_operation,
    },
    types::ServiceType,
};
use soroban_sdk::{testutils::Address as _, Address, BytesN, Env, String};

/// Test Goal 1: Prevent signature drift
/// Identical inputs must always produce identical serialized output

#[test]
fn test_attestation_no_signature_drift() {
    let env = Env::default();

    let id = 100u64;
    let issuer = Address::generate(&env);
    let subject = Address::generate(&env);
    let timestamp = 1700000000u64;
    let payload_hash = BytesN::from_array(&env, &[42u8; 32]);

    // Serialize the same attestation 10 times
    let mut hashes = vec![];
    for _ in 0..10 {
        let bytes = serialize_attestation_for_signing(
            &env,
            id,
            &issuer,
            &subject,
            timestamp,
            &payload_hash,
        );
        let hash = compute_hash(&env, &bytes);
        hashes.push(hash);
    }

    // All hashes must be identical (no drift)
    for i in 1..hashes.len() {
        assert_eq!(
            hashes[0], hashes[i],
            "Signature drift detected: hash {} differs from hash 0",
            i
        );
    }
}

#[test]
fn test_quote_request_no_signature_drift() {
    let env = Env::default();

    let base_asset = String::from_str(&env, "USD");
    let quote_asset = String::from_str(&env, "USDC");
    let amount = 5000u64;
    let operation_type = ServiceType::Withdrawals;

    // Serialize the same quote request 10 times
    let mut hashes = vec![];
    for _ in 0..10 {
        let bytes =
            serialize_quote_request(&env, &base_asset, &quote_asset, amount, operation_type);
        let hash = compute_hash(&env, &bytes);
        hashes.push(hash);
    }

    // All hashes must be identical (no drift)
    for i in 1..hashes.len() {
        assert_eq!(
            hashes[0], hashes[i],
            "Signature drift detected: hash {} differs from hash 0",
            i
        );
    }
}

#[test]
fn test_quote_data_no_signature_drift() {
    let env = Env::default();

    let anchor = Address::generate(&env);
    let base_asset = String::from_str(&env, "GBP");
    let quote_asset = String::from_str(&env, "GBPC");
    let rate = 10100u64;
    let fee_percentage = 30u32;
    let minimum_amount = 500u64;
    let maximum_amount = 50000u64;
    let valid_until = 1700000000u64;
    let quote_id = 777u64;

    // Serialize the same quote data 10 times
    let mut hashes = vec![];
    for _ in 0..10 {
        let bytes = serialize_quote_data(
            &env,
            &anchor,
            &base_asset,
            &quote_asset,
            rate,
            fee_percentage,
            minimum_amount,
            maximum_amount,
            valid_until,
            quote_id,
        );
        let hash = compute_hash(&env, &bytes);
        hashes.push(hash);
    }

    // All hashes must be identical (no drift)
    for i in 1..hashes.len() {
        assert_eq!(
            hashes[0], hashes[i],
            "Signature drift detected: hash {} differs from hash 0",
            i
        );
    }
}

#[test]
fn test_session_operation_no_signature_drift() {
    let env = Env::default();

    let session_id = 456u64;
    let operation_index = 12u64;
    let operation_type = String::from_str(&env, "register");
    let timestamp = 1700000000u64;

    // Serialize the same session operation 10 times
    let mut hashes = vec![];
    for _ in 0..10 {
        let bytes = serialize_session_operation(
            &env,
            session_id,
            operation_index,
            &operation_type,
            timestamp,
        );
        let hash = compute_hash(&env, &bytes);
        hashes.push(hash);
    }

    // All hashes must be identical (no drift)
    for i in 1..hashes.len() {
        assert_eq!(
            hashes[0], hashes[i],
            "Signature drift detected: hash {} differs from hash 0",
            i
        );
    }
}

/// Test Goal 2: Catch accidental field reordering
/// Different field values must produce different hashes

#[test]
fn test_attestation_field_order_matters() {
    let env = Env::default();

    let issuer = Address::generate(&env);
    let subject = Address::generate(&env);
    let payload_hash = BytesN::from_array(&env, &[1u8; 32]);

    // Serialize with id=1, timestamp=1000
    let bytes1 =
        serialize_attestation_for_signing(&env, 1u64, &issuer, &subject, 1000u64, &payload_hash);

    // Serialize with id=1000, timestamp=1 (swapped values)
    let bytes2 =
        serialize_attestation_for_signing(&env, 1000u64, &issuer, &subject, 1u64, &payload_hash);

    // Must produce different output (field order is preserved)
    assert_ne!(bytes1, bytes2);

    let hash1 = compute_hash(&env, &bytes1);
    let hash2 = compute_hash(&env, &bytes2);
    assert_ne!(hash1, hash2);
}

#[test]
fn test_quote_request_field_order_matters() {
    let env = Env::default();

    let asset1 = String::from_str(&env, "USD");
    let asset2 = String::from_str(&env, "USDC");

    // Serialize with base=USD, quote=USDC
    let bytes1 = serialize_quote_request(&env, &asset1, &asset2, 1000u64, ServiceType::Deposits);

    // Serialize with base=USDC, quote=USD (swapped)
    let bytes2 = serialize_quote_request(&env, &asset2, &asset1, 1000u64, ServiceType::Deposits);

    // Must produce different output (field order is preserved)
    assert_ne!(bytes1, bytes2);

    let hash1 = compute_hash(&env, &bytes1);
    let hash2 = compute_hash(&env, &bytes2);
    assert_ne!(hash1, hash2);
}

#[test]
fn test_quote_data_amount_fields_distinguishable() {
    let env = Env::default();

    let anchor = Address::generate(&env);
    let base_asset = String::from_str(&env, "USD");
    let quote_asset = String::from_str(&env, "USDC");

    // Serialize with min=100, max=10000
    let bytes1 = serialize_quote_data(
        &env,
        &anchor,
        &base_asset,
        &quote_asset,
        10000u64,
        25u32,
        100u64,
        10000u64,
        1700000000u64,
        1u64,
    );

    // Serialize with min=10000, max=100 (swapped)
    let bytes2 = serialize_quote_data(
        &env,
        &anchor,
        &base_asset,
        &quote_asset,
        10000u64,
        25u32,
        10000u64,
        100u64,
        1700000000u64,
        1u64,
    );

    // Must produce different output
    assert_ne!(bytes1, bytes2);

    let hash1 = compute_hash(&env, &bytes1);
    let hash2 = compute_hash(&env, &bytes2);
    assert_ne!(hash1, hash2);
}

#[test]
fn test_session_operation_index_vs_id_distinguishable() {
    let env = Env::default();

    let operation_type = String::from_str(&env, "attest");

    // Serialize with session_id=5, operation_index=10
    let bytes1 = serialize_session_operation(&env, 5u64, 10u64, &operation_type, 1700000000u64);

    // Serialize with session_id=10, operation_index=5 (swapped)
    let bytes2 = serialize_session_operation(&env, 10u64, 5u64, &operation_type, 1700000000u64);

    // Must produce different output
    assert_ne!(bytes1, bytes2);

    let hash1 = compute_hash(&env, &bytes1);
    let hash2 = compute_hash(&env, &bytes2);
    assert_ne!(hash1, hash2);
}

/// Test: Single field changes produce different hashes

#[test]
fn test_attestation_single_field_change_detected() {
    let env = Env::default();

    let issuer = Address::generate(&env);
    let subject = Address::generate(&env);
    let payload_hash = BytesN::from_array(&env, &[1u8; 32]);

    let base_bytes = serialize_attestation_for_signing(
        &env,
        100u64,
        &issuer,
        &subject,
        1700000000u64,
        &payload_hash,
    );
    let base_hash = compute_hash(&env, &base_bytes);

    // Change id
    let bytes_id = serialize_attestation_for_signing(
        &env,
        101u64, // Changed
        &issuer,
        &subject,
        1700000000u64,
        &payload_hash,
    );
    let hash_id = compute_hash(&env, &bytes_id);
    assert_ne!(base_hash, hash_id, "ID change not detected");

    // Change timestamp
    let bytes_ts = serialize_attestation_for_signing(
        &env,
        100u64,
        &issuer,
        &subject,
        1700000001u64, // Changed
        &payload_hash,
    );
    let hash_ts = compute_hash(&env, &bytes_ts);
    assert_ne!(base_hash, hash_ts, "Timestamp change not detected");

    // Change payload_hash
    let payload_hash2 = BytesN::from_array(&env, &[2u8; 32]);
    let bytes_hash = serialize_attestation_for_signing(
        &env,
        100u64,
        &issuer,
        &subject,
        1700000000u64,
        &payload_hash2, // Changed
    );
    let hash_hash = compute_hash(&env, &bytes_hash);
    assert_ne!(base_hash, hash_hash, "Payload hash change not detected");
}

#[test]
fn test_quote_request_single_field_change_detected() {
    let env = Env::default();

    let base_asset = String::from_str(&env, "USD");
    let quote_asset = String::from_str(&env, "USDC");

    let base_bytes = serialize_quote_request(
        &env,
        &base_asset,
        &quote_asset,
        1000u64,
        ServiceType::Deposits,
    );
    let base_hash = compute_hash(&env, &base_bytes);

    // Change amount
    let bytes_amount = serialize_quote_request(
        &env,
        &base_asset,
        &quote_asset,
        1001u64, // Changed
        ServiceType::Deposits,
    );
    let hash_amount = compute_hash(&env, &bytes_amount);
    assert_ne!(base_hash, hash_amount, "Amount change not detected");

    // Change operation_type
    let bytes_op = serialize_quote_request(
        &env,
        &base_asset,
        &quote_asset,
        1000u64,
        ServiceType::Withdrawals, // Changed
    );
    let hash_op = compute_hash(&env, &bytes_op);
    assert_ne!(base_hash, hash_op, "Operation type change not detected");

    // Change base_asset
    let base_asset2 = String::from_str(&env, "EUR");
    let bytes_base = serialize_quote_request(
        &env,
        &base_asset2, // Changed
        &quote_asset,
        1000u64,
        ServiceType::Deposits,
    );
    let hash_base = compute_hash(&env, &bytes_base);
    assert_ne!(base_hash, hash_base, "Base asset change not detected");
}

#[test]
fn test_quote_data_all_fields_affect_hash() {
    let env = Env::default();

    let anchor = Address::generate(&env);
    let base_asset = String::from_str(&env, "USD");
    let quote_asset = String::from_str(&env, "USDC");

    let base_bytes = serialize_quote_data(
        &env,
        &anchor,
        &base_asset,
        &quote_asset,
        10000u64,
        25u32,
        100u64,
        10000u64,
        1700000000u64,
        1u64,
    );
    let base_hash = compute_hash(&env, &base_bytes);

    // Test each field individually
    let test_cases = vec![
        (
            "rate",
            10001u64,
            25u32,
            100u64,
            10000u64,
            1700000000u64,
            1u64,
        ),
        (
            "fee",
            10000u64,
            26u32,
            100u64,
            10000u64,
            1700000000u64,
            1u64,
        ),
        (
            "min",
            10000u64,
            25u32,
            101u64,
            10000u64,
            1700000000u64,
            1u64,
        ),
        (
            "max",
            10000u64,
            25u32,
            100u64,
            10001u64,
            1700000000u64,
            1u64,
        ),
        (
            "valid",
            10000u64,
            25u32,
            100u64,
            10000u64,
            1700000001u64,
            1u64,
        ),
        ("id", 10000u64, 25u32, 100u64, 10000u64, 1700000000u64, 2u64),
    ];

    for (field_name, rate, fee, min, max, valid, id) in test_cases {
        let bytes = serialize_quote_data(
            &env,
            &anchor,
            &base_asset,
            &quote_asset,
            rate,
            fee,
            min,
            max,
            valid,
            id,
        );
        let hash = compute_hash(&env, &bytes);
        assert_ne!(
            base_hash, hash,
            "Change in {} field not detected",
            field_name
        );
    }
}

/// Test: Different service types produce different hashes

#[test]
fn test_service_type_serialization_unique() {
    let env = Env::default();

    let base_asset = String::from_str(&env, "USD");
    let quote_asset = String::from_str(&env, "USDC");
    let amount = 1000u64;

    let service_types = vec![
        ServiceType::Deposits,
        ServiceType::Withdrawals,
        ServiceType::Quotes,
        ServiceType::KYC,
    ];

    let mut hashes = vec![];
    for service_type in service_types {
        let bytes = serialize_quote_request(&env, &base_asset, &quote_asset, amount, service_type);
        let hash = compute_hash(&env, &bytes);
        hashes.push(hash);
    }

    // All hashes must be unique
    for i in 0..hashes.len() {
        for j in (i + 1)..hashes.len() {
            assert_ne!(
                hashes[i], hashes[j],
                "Service types {} and {} produce same hash",
                i, j
            );
        }
    }
}

/// Test: Empty strings vs non-empty strings

#[test]
fn test_empty_string_vs_non_empty() {
    let env = Env::default();

    let empty = String::from_str(&env, "");
    let non_empty = String::from_str(&env, "A");
    let amount = 1000u64;

    let bytes1 = serialize_quote_request(&env, &empty, &non_empty, amount, ServiceType::Deposits);

    let bytes2 = serialize_quote_request(&env, &non_empty, &empty, amount, ServiceType::Deposits);

    assert_ne!(bytes1, bytes2);

    let hash1 = compute_hash(&env, &bytes1);
    let hash2 = compute_hash(&env, &bytes2);
    assert_ne!(hash1, hash2);
}

/// Test: Large numbers don't overflow or wrap

#[test]
fn test_large_numbers_serialization() {
    let env = Env::default();

    let issuer = Address::generate(&env);
    let subject = Address::generate(&env);
    let payload_hash = BytesN::from_array(&env, &[1u8; 32]);

    // Test with maximum u64 values
    let bytes_max = serialize_attestation_for_signing(
        &env,
        u64::MAX,
        &issuer,
        &subject,
        u64::MAX,
        &payload_hash,
    );

    // Test with zero values
    let bytes_zero =
        serialize_attestation_for_signing(&env, 0u64, &issuer, &subject, 0u64, &payload_hash);

    // Must be different
    assert_ne!(bytes_max, bytes_zero);

    let hash_max = compute_hash(&env, &bytes_max);
    let hash_zero = compute_hash(&env, &bytes_zero);
    assert_ne!(hash_max, hash_zero);
}

/// Test: Byte order consistency (big-endian)

#[test]
fn test_byte_order_consistency() {
    let env = Env::default();

    let base_asset = String::from_str(&env, "USD");
    let quote_asset = String::from_str(&env, "USDC");

    // Test with values that would differ in little-endian vs big-endian
    // 0x0100 in big-endian = 256, in little-endian = 1
    let bytes1 = serialize_quote_request(
        &env,
        &base_asset,
        &quote_asset,
        256u64,
        ServiceType::Deposits,
    );

    let bytes2 =
        serialize_quote_request(&env, &base_asset, &quote_asset, 1u64, ServiceType::Deposits);

    // Must be different (proves consistent byte ordering)
    assert_ne!(bytes1, bytes2);

    let hash1 = compute_hash(&env, &bytes1);
    let hash2 = compute_hash(&env, &bytes2);
    assert_ne!(hash1, hash2);
}

/// Test: Address serialization is deterministic

#[test]
fn test_address_serialization_deterministic() {
    let env = Env::default();

    let issuer = Address::generate(&env);
    let subject = Address::generate(&env);
    let payload_hash = BytesN::from_array(&env, &[1u8; 32]);

    // Serialize same addresses multiple times
    let mut hashes = vec![];
    for _ in 0..5 {
        let bytes = serialize_attestation_for_signing(
            &env,
            1u64,
            &issuer,
            &subject,
            1000u64,
            &payload_hash,
        );
        let hash = compute_hash(&env, &bytes);
        hashes.push(hash);
    }

    // All must be identical
    for i in 1..hashes.len() {
        assert_eq!(hashes[0], hashes[i]);
    }
}
