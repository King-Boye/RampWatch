#![cfg(test)]

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use ed25519_dalek::{Signer, SigningKey};
use soroban_sdk::{Address, Bytes, Env, String};

use crate::contract::AnchorKitContractClient;

pub fn build_sep10_jwt(signing_key: &SigningKey, sub: &str, exp: u64) -> std::string::String {
    let header = r#"{"alg":"EdDSA","typ":"JWT"}"#;
    let payload = format!(r#"{{"sub":"{}","exp":{}}}"#, sub, exp);
    let header_b64 = URL_SAFE_NO_PAD.encode(header);
    let payload_b64 = URL_SAFE_NO_PAD.encode(payload);
    let signing_input = format!("{}.{}", header_b64, payload_b64);
    let sig = signing_key.sign(signing_input.as_bytes());
    let sig_b64 = URL_SAFE_NO_PAD.encode(sig.to_bytes());
    format!("{}.{}", signing_input, sig_b64)
}

/// Registers an [`SigningKey`] as the SEP-10 JWT verifier for `sep10_issuer` and registers `attestor`
/// using a JWT whose `sub` matches `attestor`'s strkey.
pub fn register_attestor_with_sep10(
    env: &Env,
    client: &AnchorKitContractClient,
    attestor: &Address,
    sep10_issuer: &Address,
    signing_key: &SigningKey,
) {
    let pk = Bytes::from_slice(env, signing_key.verifying_key().as_bytes());
    client.set_sep10_jwt_verifying_key(sep10_issuer, &pk);

    let sub = attestor.to_string();
    let sub_str: std::string::String = sub.to_string();
    let exp = env.ledger().timestamp().saturating_add(86_400);
    let jwt = build_sep10_jwt(signing_key, sub_str.as_str(), exp);
    let token = String::from_str(env, jwt.as_str());
    client.register_attestor(attestor, &token, sep10_issuer);
}
