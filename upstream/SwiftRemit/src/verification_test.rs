#![cfg(test)]

use crate::{
    types::ProofData,
    verification::{compute_proof_signature, verify_proof},
};
use soroban_sdk::{
    testutils::Address as _,
    Address, Bytes, BytesN, Env,
};

/// #1504 — Off-chain proof validation: Write test test_verify_proof_valid_signature()
/// Valid signature from correct signer should return Ok(true).
#[test]
fn test_verify_proof_valid_signature() {
    let env = Env::default();
    let signer = Address::generate(&env);
    let payload = Bytes::from_slice(&env, b"settlement-data-12345");
    let signature = compute_proof_signature(&env, &signer, &payload);

    let proof = ProofData {
        signature,
        payload,
        signer: signer.clone(),
    };

    let result = verify_proof(&env, &proof, &signer);
    assert_eq!(result, Ok(true));
}

/// #1505 — Off-chain proof validation: Write test test_verify_proof_invalid_signature()
/// Invalid signature should return Ok(false).
#[test]
fn test_verify_proof_invalid_signature() {
    let env = Env::default();
    let signer = Address::generate(&env);
    let payload = Bytes::from_slice(&env, b"settlement-data-12345");

    // Case 1: All-zero signature
    let invalid_signature = BytesN::from_array(&env, &[0u8; 64]);
    let proof = ProofData {
        signature: invalid_signature,
        payload: payload.clone(),
        signer: signer.clone(),
    };
    let result = verify_proof(&env, &proof, &signer);
    assert_eq!(result, Ok(false));

    // Case 2: Corrupted non-zero signature bytes
    let mut bad_bytes = [0x55u8; 64];
    bad_bytes[0] = 0xef;
    bad_bytes[63] = 0xbe;
    let corrupted_signature = BytesN::from_array(&env, &bad_bytes);
    let proof_corrupted = ProofData {
        signature: corrupted_signature,
        payload,
        signer: signer.clone(),
    };
    let result_corrupted = verify_proof(&env, &proof_corrupted, &signer);
    assert_eq!(result_corrupted, Ok(false));
}

/// #1506 — Off-chain proof validation: Write test test_verify_proof_wrong_signer()
/// Valid signature from wrong signer should return Ok(false).
#[test]
fn test_verify_proof_wrong_signer() {
    let env = Env::default();
    let signer = Address::generate(&env);
    let wrong_signer = Address::generate(&env);
    let payload = Bytes::from_slice(&env, b"settlement-data-12345");
    let signature = compute_proof_signature(&env, &signer, &payload);

    let proof = ProofData {
        signature,
        payload,
        signer: signer.clone(),
    };

    // Passed with wrong_signer as expected_signer
    let result = verify_proof(&env, &proof, &wrong_signer);
    assert_eq!(result, Ok(false));
}

/// #1507 — Off-chain proof validation: Write test test_verify_proof_empty_payload()
/// Edge case with empty payload should return Ok(false).
#[test]
fn test_verify_proof_empty_payload() {
    let env = Env::default();
    let signer = Address::generate(&env);
    let empty_payload = Bytes::new(&env);
    let signature = BytesN::from_array(&env, &[1u8; 64]);

    let proof = ProofData {
        signature,
        payload: empty_payload,
        signer: signer.clone(),
    };

    let result = verify_proof(&env, &proof, &signer);
    assert_eq!(result, Ok(false));
}
