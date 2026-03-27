//! Off-chain proof verification for settlement validation.
//!
//! This module provides cryptographic proof validation using Ed25519 signatures.
//! Proofs are used to verify that off-chain conditions (e.g., fiat payment confirmation,
//! oracle attestation) have been met before executing on-chain settlements.

use soroban_sdk::{Address, Env};
use crate::{ContractError, ProofData};

/// Verify a cryptographic proof using Ed25519 signature validation.
///
/// Validates that the proof signature is valid and signed by the expected signer.
/// Uses Stellar-compatible Ed25519 signature verification.
///
/// # Arguments
/// * `env` - Soroban environment
/// * `proof` - ProofData containing signature, payload, and signer
/// * `expected_signer` - Expected signer address for validation
///
/// # Returns
/// * `Ok(true)` - Signature is valid and signer matches
/// * `Ok(false)` - Signature is invalid or signer doesn't match
/// * `Err(ContractError)` - Validation error
pub fn verify_proof(
    env: &Env,
    proof: &ProofData,
    expected_signer: &Address,
) -> Result<bool, ContractError> {
    // Verify that the signer matches the expected signer
    if proof.signer != *expected_signer {
        return Ok(false);
    }

    // Use Stellar SDK's Ed25519 verification
    // The signature is valid if verification succeeds
    let is_valid = env.crypto().ed25519_verify(
        &proof.signer,
        &proof.payload,
        &proof.signature,
    );

    Ok(is_valid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;

    #[test]
    fn test_verify_proof_wrong_signer() {
        let env = Env::default();
        let signer = Address::generate(&env);
        let wrong_signer = Address::generate(&env);

        let proof = ProofData {
            signature: soroban_sdk::BytesN::from_array(&env, &[0u8; 64]),
            payload: soroban_sdk::Bytes::new(&env),
            signer: signer.clone(),
        };

        let result = verify_proof(&env, &proof, &wrong_signer);
        assert!(result.is_ok());
        assert!(!result.unwrap(), "Wrong signer should fail verification");
    }

    #[test]
    fn test_verify_proof_same_signer() {
        let env = Env::default();
        let signer = Address::generate(&env);

        let proof = ProofData {
            signature: soroban_sdk::BytesN::from_array(&env, &[0u8; 64]),
            payload: soroban_sdk::Bytes::new(&env),
            signer: signer.clone(),
        };

        let result = verify_proof(&env, &proof, &signer);
        assert!(result.is_ok(), "Verification should not error");
    }
}
