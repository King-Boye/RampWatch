use crate::types::{Attestation, QuoteData, QuoteRequest, ServiceType};
use soroban_sdk::{Bytes, BytesN, Env, IntoVal, Val};

/// Deterministic serialization utilities for signature generation
/// Ensures identical inputs always produce identical serialized output

/// Serialize an attestation for signing (without the signature field)
/// Field order is strictly defined to prevent signature drift
pub fn serialize_attestation_for_signing(
    env: &Env,
    id: u64,
    issuer: &soroban_sdk::Address,
    subject: &soroban_sdk::Address,
    timestamp: u64,
    payload_hash: &BytesN<32>,
) -> Bytes {
    let mut bytes = Bytes::new(env);

    // Field order: id, issuer, subject, timestamp, payload_hash
    // This order MUST NOT change to prevent signature drift

    // 1. id (8 bytes, big-endian)
    bytes.append(&Bytes::from_array(env, &id.to_be_bytes()));

    // 2. issuer (as Val - deterministic representation)
    let issuer_val: Val = issuer.clone().into_val(env);
    let issuer_u64: u64 = issuer_val.get_payload();
    bytes.append(&Bytes::from_array(env, &issuer_u64.to_be_bytes()));

    // 3. subject (as Val - deterministic representation)
    let subject_val: Val = subject.clone().into_val(env);
    let subject_u64: u64 = subject_val.get_payload();
    bytes.append(&Bytes::from_array(env, &subject_u64.to_be_bytes()));

    // 4. timestamp (8 bytes, big-endian)
    bytes.append(&Bytes::from_array(env, &timestamp.to_be_bytes()));

    // 5. payload_hash (32 bytes - convert BytesN to Bytes)
    let hash_bytes: Bytes = payload_hash.clone().into();
    bytes.append(&hash_bytes);

    bytes
}

/// Serialize a quote request for signing
/// Field order is strictly defined to prevent signature drift
pub fn serialize_quote_request(
    env: &Env,
    base_asset: &soroban_sdk::String,
    quote_asset: &soroban_sdk::String,
    amount: u64,
    operation_type: ServiceType,
) -> Bytes {
    let mut bytes = Bytes::new(env);

    // Field order: base_asset, quote_asset, amount, operation_type
    // This order MUST NOT change to prevent signature drift

    // 1. base_asset (as Val - deterministic representation)
    let base_val: Val = base_asset.clone().into_val(env);
    let base_u64: u64 = base_val.get_payload();
    bytes.append(&Bytes::from_array(env, &base_u64.to_be_bytes()));

    // 2. quote_asset (as Val - deterministic representation)
    let quote_val: Val = quote_asset.clone().into_val(env);
    let quote_u64: u64 = quote_val.get_payload();
    bytes.append(&Bytes::from_array(env, &quote_u64.to_be_bytes()));

    // 3. amount (8 bytes, big-endian)
    bytes.append(&Bytes::from_array(env, &amount.to_be_bytes()));

    // 4. operation_type (4 bytes, big-endian)
    let op_type_u32 = operation_type as u32;
    bytes.append(&Bytes::from_array(env, &op_type_u32.to_be_bytes()));

    bytes
}

/// Serialize quote data for signing
/// Field order is strictly defined to prevent signature drift
pub fn serialize_quote_data(
    env: &Env,
    anchor: &soroban_sdk::Address,
    base_asset: &soroban_sdk::String,
    quote_asset: &soroban_sdk::String,
    rate: u64,
    fee_percentage: u32,
    minimum_amount: u64,
    maximum_amount: u64,
    valid_until: u64,
    quote_id: u64,
) -> Bytes {
    let mut bytes = Bytes::new(env);

    // Field order: anchor, base_asset, quote_asset, rate, fee_percentage,
    //              minimum_amount, maximum_amount, valid_until, quote_id
    // This order MUST NOT change to prevent signature drift

    // 1. anchor (as Val - deterministic representation)
    let anchor_val: Val = anchor.clone().into_val(env);
    let anchor_u64: u64 = anchor_val.get_payload();
    bytes.append(&Bytes::from_array(env, &anchor_u64.to_be_bytes()));

    // 2. base_asset (as Val - deterministic representation)
    let base_val: Val = base_asset.clone().into_val(env);
    let base_u64: u64 = base_val.get_payload();
    bytes.append(&Bytes::from_array(env, &base_u64.to_be_bytes()));

    // 3. quote_asset (as Val - deterministic representation)
    let quote_val: Val = quote_asset.clone().into_val(env);
    let quote_u64: u64 = quote_val.get_payload();
    bytes.append(&Bytes::from_array(env, &quote_u64.to_be_bytes()));

    // 4. rate (8 bytes, big-endian)
    bytes.append(&Bytes::from_array(env, &rate.to_be_bytes()));

    // 5. fee_percentage (4 bytes, big-endian)
    bytes.append(&Bytes::from_array(env, &fee_percentage.to_be_bytes()));

    // 6. minimum_amount (8 bytes, big-endian)
    bytes.append(&Bytes::from_array(env, &minimum_amount.to_be_bytes()));

    // 7. maximum_amount (8 bytes, big-endian)
    bytes.append(&Bytes::from_array(env, &maximum_amount.to_be_bytes()));

    // 8. valid_until (8 bytes, big-endian)
    bytes.append(&Bytes::from_array(env, &valid_until.to_be_bytes()));

    // 9. quote_id (8 bytes, big-endian)
    bytes.append(&Bytes::from_array(env, &quote_id.to_be_bytes()));

    bytes
}

/// Serialize a session operation for signing
pub fn serialize_session_operation(
    env: &Env,
    session_id: u64,
    operation_index: u64,
    operation_type: &soroban_sdk::String,
    timestamp: u64,
) -> Bytes {
    let mut bytes = Bytes::new(env);

    // Field order: session_id, operation_index, operation_type, timestamp
    // This order MUST NOT change to prevent signature drift

    // 1. session_id (8 bytes, big-endian)
    bytes.append(&Bytes::from_array(env, &session_id.to_be_bytes()));

    // 2. operation_index (8 bytes, big-endian)
    bytes.append(&Bytes::from_array(env, &operation_index.to_be_bytes()));

    // 3. operation_type (as Val - deterministic representation)
    let op_val: Val = operation_type.clone().into_val(env);
    let op_u64: u64 = op_val.get_payload();
    bytes.append(&Bytes::from_array(env, &op_u64.to_be_bytes()));

    // 4. timestamp (8 bytes, big-endian)
    bytes.append(&Bytes::from_array(env, &timestamp.to_be_bytes()));

    bytes
}

/// Compute a deterministic hash of serialized data
pub fn compute_hash(env: &Env, data: &Bytes) -> BytesN<32> {
    env.crypto().sha256(data).into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env, String};

    #[test]
    fn test_attestation_serialization_deterministic() {
        let env = Env::default();

        let id = 42u64;
        let issuer = Address::generate(&env);
        let subject = Address::generate(&env);
        let timestamp = 1234567890u64;
        let payload_hash = BytesN::from_array(&env, &[1u8; 32]);

        // Serialize twice with same inputs
        let bytes1 = serialize_attestation_for_signing(
            &env,
            id,
            &issuer,
            &subject,
            timestamp,
            &payload_hash,
        );

        let bytes2 = serialize_attestation_for_signing(
            &env,
            id,
            &issuer,
            &subject,
            timestamp,
            &payload_hash,
        );

        // Must be identical
        assert_eq!(bytes1, bytes2);

        // Compute hashes - must be identical
        let hash1 = compute_hash(&env, &bytes1);
        let hash2 = compute_hash(&env, &bytes2);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_quote_request_serialization_deterministic() {
        let env = Env::default();

        let base_asset = String::from_str(&env, "USD");
        let quote_asset = String::from_str(&env, "USDC");
        let amount = 1000u64;
        let operation_type = ServiceType::Deposits;

        // Serialize twice with same inputs
        let bytes1 =
            serialize_quote_request(&env, &base_asset, &quote_asset, amount, operation_type);

        let bytes2 =
            serialize_quote_request(&env, &base_asset, &quote_asset, amount, operation_type);

        // Must be identical
        assert_eq!(bytes1, bytes2);

        // Compute hashes - must be identical
        let hash1 = compute_hash(&env, &bytes1);
        let hash2 = compute_hash(&env, &bytes2);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_quote_data_serialization_deterministic() {
        let env = Env::default();

        let anchor = Address::generate(&env);
        let base_asset = String::from_str(&env, "EUR");
        let quote_asset = String::from_str(&env, "EURC");
        let rate = 10050u64;
        let fee_percentage = 25u32;
        let minimum_amount = 100u64;
        let maximum_amount = 100000u64;
        let valid_until = 1234567890u64;
        let quote_id = 999u64;

        // Serialize twice with same inputs
        let bytes1 = serialize_quote_data(
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

        let bytes2 = serialize_quote_data(
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

        // Must be identical
        assert_eq!(bytes1, bytes2);

        // Compute hashes - must be identical
        let hash1 = compute_hash(&env, &bytes1);
        let hash2 = compute_hash(&env, &bytes2);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_session_operation_serialization_deterministic() {
        let env = Env::default();

        let session_id = 123u64;
        let operation_index = 5u64;
        let operation_type = String::from_str(&env, "attest");
        let timestamp = 1234567890u64;

        // Serialize twice with same inputs
        let bytes1 = serialize_session_operation(
            &env,
            session_id,
            operation_index,
            &operation_type,
            timestamp,
        );

        let bytes2 = serialize_session_operation(
            &env,
            session_id,
            operation_index,
            &operation_type,
            timestamp,
        );

        // Must be identical
        assert_eq!(bytes1, bytes2);

        // Compute hashes - must be identical
        let hash1 = compute_hash(&env, &bytes1);
        let hash2 = compute_hash(&env, &bytes2);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_different_inputs_produce_different_output() {
        let env = Env::default();

        let base_asset = String::from_str(&env, "USD");
        let quote_asset = String::from_str(&env, "USDC");

        // Serialize with amount 1000
        let bytes1 = serialize_quote_request(
            &env,
            &base_asset,
            &quote_asset,
            1000u64,
            ServiceType::Deposits,
        );

        // Serialize with amount 2000
        let bytes2 = serialize_quote_request(
            &env,
            &base_asset,
            &quote_asset,
            2000u64,
            ServiceType::Deposits,
        );

        // Must be different
        assert_ne!(bytes1, bytes2);

        // Hashes must be different
        let hash1 = compute_hash(&env, &bytes1);
        let hash2 = compute_hash(&env, &bytes2);
        assert_ne!(hash1, hash2);
    }
}
