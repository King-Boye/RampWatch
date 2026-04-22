//! Minimal SEP-10 JWT verification (JWS compact, Ed25519 / `EdDSA`) for Soroban.
//!
//! Verifies the anchor-signed token using a 32-byte Ed25519 public key stored on-chain.
//! Payload must include integer `exp` (Unix seconds) and string `sub` (Stellar strkey of the client).

#![cfg_attr(not(test), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use soroban_sdk::{Bytes, BytesN, Env, String};

/// Maximum JWT character length accepted by the contract (defensive bound).
pub const MAX_JWT_LEN: u32 = 2048;

fn decode_base64url_char(c: u8) -> Option<u8> {
    match c {
        b'A'..=b'Z' => Some(c - b'A'),
        b'a'..=b'z' => Some(c - b'a' + 26),
        b'0'..=b'9' => Some(c - b'0' + 52),
        b'-' => Some(62),
        b'_' => Some(63),
        _ => None,
    }
}

/// Base64url decode (no padding required).
pub fn base64url_decode(input: &[u8]) -> Result<Vec<u8>, ()> {
    let mut out: Vec<u8> = Vec::new();
    let mut buffer: u32 = 0;
    let mut bits: u32 = 0;
    for &ch in input {
        if ch == b'=' {
            break;
        }
        let val = decode_base64url_char(ch).ok_or(())?;
        buffer = (buffer << 6) | (val as u32);
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push(((buffer >> bits) & 0xFF) as u8);
        }
    }
    Ok(out)
}

fn contains_subslice(haystack: &[u8], needle: &[u8]) -> bool {
    haystack.windows(needle.len()).any(|w| w == needle)
}

fn find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() {
        return Some(0);
    }
    haystack
        .windows(needle.len())
        .position(|w| w == needle)
}

/// Parse `"exp": <digits>` (first occurrence).
fn parse_json_exp(payload: &[u8]) -> Result<u64, ()> {
    let key = b"\"exp\":";
    let pos = find_bytes(payload, key).ok_or(())?;
    let mut i = pos + key.len();
    while i < payload.len() && payload[i].is_ascii_whitespace() {
        i += 1;
    }
    let mut n: u64 = 0;
    let mut any = false;
    while i < payload.len() && payload[i].is_ascii_digit() {
        any = true;
        let d = (payload[i] - b'0') as u64;
        n = n
            .checked_mul(10)
            .and_then(|x| x.checked_add(d))
            .ok_or(())?;
        i += 1;
    }
    if !any {
        return Err(());
    }
    Ok(n)
}

/// Parse first `"sub":"..."` string value (no escape sequences inside value).
fn parse_json_sub(env: &Env, payload: &[u8]) -> Result<String, ()> {
    let key = b"\"sub\":";
    let pos = find_bytes(payload, key).ok_or(())?;
    let mut i = pos + key.len();
    while i < payload.len() && payload[i].is_ascii_whitespace() {
        i += 1;
    }
    if i >= payload.len() || payload[i] != b'"' {
        return Err(());
    }
    i += 1;
    let start = i;
    while i < payload.len() {
        if payload[i] == b'"' {
            let sub = &payload[start..i];
            return Ok(String::from_bytes(env, sub));
        }
        i += 1;
    }
    Err(())
}

/// Verify a SEP-10-style JWT: JWS compact, EdDSA signature, `exp`, and optional `sub` match.
///
/// When `expected_sub` is [`None`], the token must still contain a parseable `sub` claim, but it
/// is not compared to a caller-supplied address (see contract `verify_sep10_token`).
pub fn verify_sep10_jwt(
    env: &Env,
    token: &String,
    anchor_public_key: &Bytes,
    expected_sub: Option<&String>,
) -> Result<(), ()> {
    if anchor_public_key.len() != 32 {
        return Err(());
    }

    let n = token.len();
    if n == 0 || n > MAX_JWT_LEN {
        return Err(());
    }
    let n_usize = n as usize;
    let mut buf = [0u8; MAX_JWT_LEN as usize];
    token.copy_into_slice(&mut buf[..n_usize]);

    let mut dots: [usize; 2] = [0; 2];
    let mut dot_count = 0usize;
    for i in 0..n_usize {
        if buf[i] == b'.' {
            if dot_count < 2 {
                dots[dot_count] = i;
                dot_count += 1;
            } else {
                return Err(());
            }
        }
    }
    if dot_count != 2 {
        return Err(());
    }

    let d0 = dots[0];
    let d1 = dots[1];
    if d0 == 0 || d1 <= d0 + 1 || d1 + 1 >= n_usize {
        return Err(());
    }

    let header_b64 = &buf[..d0];
    let payload_b64 = &buf[d0 + 1..d1];
    let sig_b64 = &buf[d1 + 1..n_usize];

    let header_dec = base64url_decode(header_b64).map_err(|_| ())?;
    if !contains_subslice(&header_dec, b"EdDSA") {
        return Err(());
    }

    let sig_dec = base64url_decode(sig_b64).map_err(|_| ())?;
    if sig_dec.len() != 64 {
        return Err(());
    }

    let signing_input = Bytes::from_slice(env, &buf[..d1]);
    let sig_bytes = Bytes::from_slice(env, sig_dec.as_slice());

    let pk_bytesn: BytesN<32> = anchor_public_key.clone().try_into().map_err(|_| ())?;
    let sig_bytesn: BytesN<64> = sig_bytes.try_into().map_err(|_| ())?;

    env.crypto().ed25519_verify(&pk_bytesn, &signing_input, &sig_bytesn);

    let payload_dec = base64url_decode(payload_b64).map_err(|_| ())?;
    let exp = parse_json_exp(&payload_dec)?;
    let now = env.ledger().timestamp();
    if exp <= now {
        return Err(());
    }

    let sub = parse_json_sub(env, &payload_dec)?;
    if let Some(expected) = expected_sub {
        if sub != *expected {
            return Err(());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;
    use alloc::format;
    use alloc::string::ToString;
    use ed25519_dalek::{Signer, SigningKey};
    use rand::rngs::OsRng;
    use soroban_sdk::testutils::{Address as _, Ledger, LedgerInfo};
    use soroban_sdk::{Address, Env};

    fn ledger(env: &Env, ts: u64) {
        env.ledger().set(LedgerInfo {
            timestamp: ts,
            protocol_version: 21,
            sequence_number: 0,
            network_id: Default::default(),
            base_reserve: 0,
            min_persistent_entry_ttl: 4096,
            min_temp_entry_ttl: 16,
            max_entry_ttl: 6312000,
        });
    }

    fn build_jwt(signing_key: &SigningKey, sub: &str, exp: u64) -> std::string::String {
        use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
        let header = r#"{"alg":"EdDSA","typ":"JWT"}"#;
        let payload = format!(r#"{{"sub":"{}","exp":{}}}"#, sub, exp);
        let header_b64 = URL_SAFE_NO_PAD.encode(header);
        let payload_b64 = URL_SAFE_NO_PAD.encode(payload);
        let signing_input = format!("{}.{}", header_b64, payload_b64);
        let sig = signing_key.sign(signing_input.as_bytes());
        let sig_b64 = URL_SAFE_NO_PAD.encode(sig.to_bytes());
        format!("{}.{}", signing_input, sig_b64)
    }

    #[test]
    fn base64url_roundtrip_simple() {
        let dec = base64url_decode(b"SGVsbG8").unwrap();
        assert_eq!(dec, b"Hello");
    }

    #[test]
    fn verify_accepts_valid_token() {
        let env = Env::default();
        ledger(&env, 1_000);
        let signing_key = SigningKey::generate(&mut OsRng);
        let pk = Bytes::from_slice(&env, signing_key.verifying_key().as_bytes());

        let attestor = Address::generate(&env);
        let sub = attestor.to_string();
        let sub_str: std::string::String = sub.to_string();
        let jwt = build_jwt(&signing_key, sub_str.as_str(), 2_000);
        let token = String::from_str(&env, jwt.as_str());

        assert!(verify_sep10_jwt(&env, &token, &pk, Some(&sub)).is_ok());
        assert!(verify_sep10_jwt(&env, &token, &pk, None).is_ok());
    }

    #[test]
    fn verify_rejects_expired_token() {
        let env = Env::default();
        ledger(&env, 5_000);
        let signing_key = SigningKey::generate(&mut OsRng);
        let pk = Bytes::from_slice(&env, signing_key.verifying_key().as_bytes());

        let attestor = Address::generate(&env);
        let sub = attestor.to_string();
        let sub_str: std::string::String = sub.to_string();
        let jwt = build_jwt(&signing_key, sub_str.as_str(), 1_000);
        let token = String::from_str(&env, jwt.as_str());

        assert!(verify_sep10_jwt(&env, &token, &pk, Some(&sub)).is_err());
    }

    #[test]
    fn verify_rejects_invalid_signature() {
        let env = Env::default();
        ledger(&env, 1_000);
        let signing_key = SigningKey::generate(&mut OsRng);
        let other_key = SigningKey::generate(&mut OsRng);
        let pk = Bytes::from_slice(&env, other_key.verifying_key().as_bytes());

        let attestor = Address::generate(&env);
        let sub = attestor.to_string();
        let sub_str: std::string::String = sub.to_string();
        let jwt = build_jwt(&signing_key, sub_str.as_str(), 2_000);
        let token = String::from_str(&env, jwt.as_str());

        assert!(verify_sep10_jwt(&env, &token, &pk, Some(&sub)).is_err());
    }
}
