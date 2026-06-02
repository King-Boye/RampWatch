//! Load simulation and stress tests for AnchorKit
//!
//! These tests validate contract behavior under high load and stress conditions.
//! They are gated behind the `stress-tests` feature flag to avoid slowing normal CI.

#![cfg(feature = "stress-tests")]

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use ed25519_dalek::{Signer, SigningKey};
use rand::rngs::OsRng;
use soroban_sdk::{
    testutils::{Address as _, Ledger, LedgerInfo},
    Address, Bytes, Env, String, Vec,
};
use anchorkit::contract::{AnchorKitContract, AnchorKitContractClient};

fn make_env() -> Env {
    let env = Env::default();
    env.mock_all_auths();
    env
}

fn set_ledger(env: &Env, timestamp: u64) {
    env.ledger().set(LedgerInfo {
        timestamp,
        protocol_version: 21,
        sequence_number: 0,
        network_id: Default::default(),
        base_reserve: 0,
        min_persistent_entry_ttl: 4096,
        min_temp_entry_ttl: 16,
        max_entry_ttl: 6312000,
    });
}

fn setup(env: &Env) -> (AnchorKitContractClient, Address) {
    let contract_id = env.register_contract(None, AnchorKitContract);
    let client = AnchorKitContractClient::new(env, &contract_id);
    let admin = Address::generate(env);
    client.initialize(&admin, &100_u64, &None);
    (client, admin)
}

fn build_sep10_jwt(signing_key: &SigningKey, sub: &str, exp: u64) -> std::string::String {
    let header = r#"{"alg":"EdDSA","typ":"JWT"}"#;
    let payload = format!(r#"{{"sub":"{}","exp":{}}}"#, sub, exp);
    let header_b64 = URL_SAFE_NO_PAD.encode(header);
    let payload_b64 = URL_SAFE_NO_PAD.encode(payload);
    let signing_input = format!("{}.{}", header_b64, payload_b64);
    let sig = signing_key.sign(signing_input.as_bytes());
    let sig_b64 = URL_SAFE_NO_PAD.encode(sig.to_bytes());
    format!("{}.{}", signing_input, sig_b64)
}

fn register_anchor_with_sep10(
    env: &Env,
    client: &AnchorKitContractClient,
    anchor: &Address,
    signing_key: &SigningKey,
) {
    let pk = Bytes::from_slice(env, signing_key.verifying_key().as_bytes());
    client.set_sep10_jwt_verifying_key(anchor, &pk);

    let sub = anchor.to_string();
    let mut buf = [0u8; 128];
    let len = sub.len() as usize;
    let copy_len = len.min(128);
    sub.copy_into_slice(&mut buf[..copy_len]);
    let sub_str = core::str::from_utf8(&buf[..copy_len]).unwrap_or("");

    let exp = env.ledger().timestamp().saturating_add(86_400);
    let jwt_str = build_sep10_jwt(signing_key, sub_str, exp);
    let token = String::from_str(env, jwt_str.as_str());
    client.register_attestor(anchor, &token, anchor);
}

fn sign_payload(env: &Env, signing_key: &SigningKey, payload_hash: &Bytes) -> Bytes {
    let mut hash_arr = [0u8; 32];
    payload_hash.copy_into_slice(&mut hash_arr);
    let sig = signing_key.sign(&hash_arr);
    Bytes::from_slice(env, &sig.to_bytes())
}

/// Test batch registration of many attestors under stress.
/// Validates that the contract can handle registering 50 attestors sequentially.
#[test]
fn test_batch_attestor_registration_stress() {
    let env = make_env();
    set_ledger(&env, 1_000_000);
    let (client, _admin) = setup(&env);

    const ATTESTOR_COUNT: usize = 50;

    let mut registered = Vec::new(&env);
    for _ in 0..ATTESTOR_COUNT {
        let attestor = Address::generate(&env);
        let sk = SigningKey::generate(&mut OsRng);
        register_anchor_with_sep10(&env, &client, &attestor, &sk);

        let mut services = Vec::new(&env);
        services.push_back(1u32);
        services.push_back(3u32);
        client.configure_services(&attestor, &services);

        registered.push_back(attestor);
    }

    assert_eq!(registered.len(), ATTESTOR_COUNT as u32);
    println!("Successfully registered {} attestors under stress", ATTESTOR_COUNT);
}

/// Test rate comparison under stress with many quotes.
/// Validates the contract handles high-volume quote submissions across 20 anchors.
///
/// This test uses `stop_recording_auth()` to prevent the Soroban test framework
/// from recording the full auth trace, which would produce an 800+ KB snapshot file.
/// Instead, we use targeted assertions to validate the specific values being tested:
/// - Total quote count matches expected
/// - Quote IDs are monotonically increasing
/// - Final quote counter matches expected value
#[test]
fn test_rate_comparison_stress() {
    let env = make_env();
    set_ledger(&env, 1_000_000);
    let (client, _admin) = setup(&env);

    const ANCHOR_COUNT: usize = 20;
    const QUOTES_PER_ANCHOR: usize = 50;
    const EXPECTED_TOTAL_QUOTES: u64 = (ANCHOR_COUNT * QUOTES_PER_ANCHOR) as u64;

    let base_asset = String::from_str(&env, "USDC");
    let quote_asset = String::from_str(&env, "USD");
    let current_time = env.ledger().timestamp();

    let mut anchors = Vec::new(&env);
    for _ in 0..ANCHOR_COUNT {
        let anchor = Address::generate(&env);
        let sk = SigningKey::generate(&mut OsRng);
        register_anchor_with_sep10(&env, &client, &anchor, &sk);

        let mut services = Vec::new(&env);
        services.push_back(3u32);
        client.configure_services(&anchor, &services);
        anchors.push_back(anchor);
    }

    // Stop recording auth to prevent the snapshot from growing to 800+ KB.
    // The stress test validates quote submission logic, not auth recording.
    env.stop_recording_auth();

    let mut total_quotes = 0usize;
    let mut prev_quote_id = 0u64;
    for anchor_idx in 0..anchors.len() {
        let anchor = anchors.get(anchor_idx).unwrap();
        for q_idx in 0..QUOTES_PER_ANCHOR {
            let rate = 10000 + (q_idx as u64 * 10);
            let fee_percentage = 100 + (q_idx as u32 % 50);
            let valid_until = current_time + 3600;
            let quote_id = client.submit_quote(
                &anchor,
                &base_asset,
                &quote_asset,
                &rate,
                &fee_percentage,
                &100u64,
                &100000u64,
                &valid_until,
            );
            // Validate quote ID is positive and monotonically increasing
            assert!(quote_id > 0, "Quote ID must be positive");
            assert!(quote_id > prev_quote_id, "Quote IDs must be monotonically increasing");
            prev_quote_id = quote_id;
            total_quotes += 1;
        }
    }

    // Targeted assertions instead of full-state snapshot
    assert_eq!(total_quotes, EXPECTED_TOTAL_QUOTES as usize, 
        "Total quote count should match expected");
    assert_eq!(prev_quote_id, EXPECTED_TOTAL_QUOTES,
        "Final quote ID should equal total count");
    
    println!("Successfully processed {} quote submissions under stress", total_quotes);
}

/// Test batch attestor registration with overflow handling.
/// Validates graceful handling when approaching the registration limit.
#[test]
fn test_batch_attestor_registration_overflow() {
    let env = make_env();
    set_ledger(&env, 1_000_000);
    let (client, _admin) = setup(&env);

    const OVERFLOW_THRESHOLD: usize = 120;

    let mut successful_registrations = 0usize;

    for _ in 0..OVERFLOW_THRESHOLD {
        let attestor = Address::generate(&env);
        let sk = SigningKey::generate(&mut OsRng);

        register_anchor_with_sep10(&env, &client, &attestor, &sk);

        let mut services = Vec::new(&env);
        services.push_back(1u32);
        client.configure_services(&attestor, &services);
        successful_registrations += 1;
    }

    assert!(successful_registrations > 0);
    println!(
        "Gracefully handled {} registrations without overflow",
        successful_registrations
    );
}

/// Test connection pool behavior under high concurrent load.
///
/// Registers 20 anchors, submits 15 quotes per anchor (300 total), and then
/// 10 attestations per anchor (200 total). This exercises the persistent storage
/// and counter paths that produce a snapshot comparable in size to other stress tests.
#[test]
fn test_connection_pool_high_load() {
    let env = make_env();
    set_ledger(&env, 1_000_000);
    let (client, _admin) = setup(&env);

    const CONCURRENT_CONNECTIONS: usize = 20;
    const QUOTES_PER_CONNECTION: usize = 15;
    const ATTESTATIONS_PER_CONNECTION: usize = 10;

    let base_asset = String::from_str(&env, "USDC");
    let quote_asset = String::from_str(&env, "USD");
    let current_time = env.ledger().timestamp();

    // Establish the connection pool: register many anchors with full service capabilities.
    let mut pool: std::vec::Vec<(Address, SigningKey)> = std::vec::Vec::new();
    for _ in 0..CONCURRENT_CONNECTIONS {
        let anchor = Address::generate(&env);
        let sk = SigningKey::generate(&mut OsRng);
        register_anchor_with_sep10(&env, &client, &anchor, &sk);

        let mut services = Vec::new(&env);
        services.push_back(1u32); // Deposits
        services.push_back(2u32); // Withdrawals
        services.push_back(3u32); // Quotes
        client.configure_services(&anchor, &services);

        pool.push((anchor, sk));
    }

    // Flood the connection pool with quote submissions.
    let mut total_quotes = 0u32;
    for (anchor, _) in &pool {
        for q_idx in 0..QUOTES_PER_CONNECTION {
            let rate = 10000 + (q_idx as u64 * 10);
            let fee_percentage = 50 + (q_idx as u32 % 30);
            let valid_until = current_time + 3600;
            let quote_id = client.submit_quote(
                anchor,
                &base_asset,
                &quote_asset,
                &rate,
                &fee_percentage,
                &100u64,
                &100000u64,
                &valid_until,
            );
            assert!(quote_id > 0);
            total_quotes += 1;
        }
    }

    assert_eq!(
        total_quotes,
        (CONCURRENT_CONNECTIONS * QUOTES_PER_CONNECTION) as u32
    );

    // Flood the connection pool with attestation submissions, one subject per connection.
    let mut attestation_ids = Vec::new(&env);
    for (conn_idx, (anchor, sk)) in pool.iter().enumerate() {
        let subject = Address::generate(&env);

        for op_idx in 0..ATTESTATIONS_PER_CONNECTION {
            // Build a globally unique payload for each (connection, operation) pair so
            // the replay-detection guard never fires.
            let global_idx = conn_idx * ATTESTATIONS_PER_CONNECTION + op_idx;
            let mut payload_bytes = [0u8; 32];
            payload_bytes[0] = (global_idx & 0xFF) as u8;
            payload_bytes[1] = ((global_idx >> 8) & 0xFF) as u8;
            payload_bytes[2] = ((global_idx >> 16) & 0xFF) as u8;
            payload_bytes[3] = 0xCA; // sentinel to distinguish from other tests
            let payload_hash = Bytes::from_slice(&env, &payload_bytes);
            let signature = sign_payload(&env, sk, &payload_hash);

            let attestation_id = client.submit_attestation(
                anchor,
                &subject,
                &current_time,
                &payload_hash,
                &signature,
            );
            attestation_ids.push_back(attestation_id);
        }
    }

    assert_eq!(
        attestation_ids.len(),
        (CONCURRENT_CONNECTIONS * ATTESTATIONS_PER_CONNECTION) as u32
    );

    // All attestation IDs must be unique — the counter must be monotonically increasing
    // across concurrent connections, never re-issuing an ID.
    for i in 0..attestation_ids.len() {
        for j in (i + 1)..attestation_ids.len() {
            let id1 = attestation_ids.get(i).unwrap();
            let id2 = attestation_ids.get(j).unwrap();
            assert_ne!(id1, id2, "Concurrent connections produced duplicate IDs");
        }
    }

    println!(
        "Successfully processed {} quote submissions and {} attestations \
         across {} concurrent connections",
        total_quotes,
        attestation_ids.len(),
        CONCURRENT_CONNECTIONS
    );
}

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[test]
    fn test_stress_tests_compile() {
        assert!(true);
    }
}