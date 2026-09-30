use crate::errors::ContractError;
use crate::verification::{
    require_valid_proof, validate_proof, verify_proof_commitment, Condition, Proof,
    VerificationResult,
};
use soroban_sdk::{Bytes, BytesN, Env, String};

fn condition(env: &Env) -> Condition {
    Condition {
        id: String::from_str(env, "payout-confirmed"),
        authorized_signer: String::from_str(env, "trusted-oracle"),
    }
}

fn valid_proof(env: &Env) -> Proof {
    Proof {
        condition_id: String::from_str(env, "payout-confirmed"),
        signer: String::from_str(env, "trusted-oracle"),
        payload: Bytes::from_slice(env, &[1, 2, 3, 4]),
    }
}

#[test]
fn verification_accepts_matching_commitment() {
    let env = Env::default();
    let expected = BytesN::from_array(&env, &[42u8; 32]);
    let submitted = BytesN::from_array(&env, &[42u8; 32]);

    assert!(verify_proof_commitment(&submitted, &expected));
}

#[test]
fn verification_rejects_mismatched_commitment() {
    let env = Env::default();
    let expected = BytesN::from_array(&env, &[42u8; 32]);
    let submitted = BytesN::from_array(&env, &[24u8; 32]);

    assert!(!verify_proof_commitment(&submitted, &expected));
}

#[test]
fn verification_accepts_valid_structural_proof() {
    let env = Env::default();
    let proof = valid_proof(&env);
    let condition = condition(&env);

    assert_eq!(
        validate_proof(&proof, &condition),
        VerificationResult::Valid
    );
    assert_eq!(require_valid_proof(&proof, &condition), Ok(()));
}

#[test]
fn verification_rejects_condition_mismatch() {
    let env = Env::default();
    let mut proof = valid_proof(&env);
    let condition = condition(&env);

    proof.condition_id = String::from_str(&env, "different-condition");

    assert_eq!(
        validate_proof(&proof, &condition),
        VerificationResult::ConditionMismatch
    );
    assert_eq!(
        require_valid_proof(&proof, &condition),
        Err(ContractError::InvalidProof)
    );
}

#[test]
fn verification_rejects_unauthorized_signer() {
    let env = Env::default();
    let mut proof = valid_proof(&env);
    let condition = condition(&env);

    proof.signer = String::from_str(&env, "untrusted-oracle");

    assert_eq!(
        validate_proof(&proof, &condition),
        VerificationResult::UnauthorizedSigner
    );
    assert_eq!(
        require_valid_proof(&proof, &condition),
        Err(ContractError::InvalidProof)
    );
}

#[test]
fn verification_rejects_empty_payload() {
    let env = Env::default();
    let mut proof = valid_proof(&env);
    let condition = condition(&env);

    proof.payload = Bytes::new(&env);

    assert_eq!(
        validate_proof(&proof, &condition),
        VerificationResult::MalformedProof
    );
    assert_eq!(
        require_valid_proof(&proof, &condition),
        Err(ContractError::InvalidProof)
    );
}
