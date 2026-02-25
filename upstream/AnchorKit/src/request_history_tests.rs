#![cfg(test)]

use crate::{
    request_history::ApiCallStatus,
    AnchorKitContract, AnchorKitContractClient,
};
use soroban_sdk::{testutils::Address as _, Address, Bytes, BytesN, Env, String};

#[test]
#[ignore]
fn test_record_api_call() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AnchorKitContract);
    let client = AnchorKitContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let attestor = Address::generate(&env);

    env.mock_all_auths();

    // Initialize contract
    client.initialize(&admin);

    // Register attestor with tracking
    client.register_attestor_tracked(&attestor);

    // Get request history
    let history = client.get_request_history(&10);
    assert_eq!(history.recent_calls.len(), 1);
    assert_eq!(history.total_calls, 1);
    assert_eq!(history.success_count, 1);
    assert_eq!(history.failed_count, 0);

    let call = history.recent_calls.get(0).unwrap();
    assert_eq!(call.status, ApiCallStatus::Success);
    assert_eq!(call.operation, String::from_str(&env, "register_attestor"));
}

#[test]
#[ignore]
fn test_submit_attestation_tracked() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, AnchorKitContract);
    let client = AnchorKitContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let attestor = Address::generate(&env);
    let subject = Address::generate(&env);

    // Initialize and register attestor
    client.initialize(&admin);
    client.register_attestor(&attestor);

    // Submit attestation with tracking
    let timestamp = env.ledger().timestamp();
    let payload_hash = BytesN::from_array(&env, &[1u8; 32]);
    let signature = Bytes::new(&env);

    let result = client.submit_attestation_tracked(
        &attestor,
        &subject,
        &timestamp,
        &payload_hash,
        &signature,
    );

    // Check request history
    let history = client.get_request_history(&10);
    assert_eq!(history.recent_calls.len(), 1);
    assert_eq!(history.success_count, 1);

    let call = history.recent_calls.get(0).unwrap();
    assert_eq!(call.status, ApiCallStatus::Success);
    assert_eq!(call.operation, String::from_str(&env, "submit_attestation"));
    assert_eq!(call.caller, attestor);
}

#[test]
#[ignore]
fn test_failed_api_call_tracking() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, AnchorKitContract);
    let client = AnchorKitContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let unregistered_attestor = Address::generate(&env);
    let subject = Address::generate(&env);

    // Initialize contract
    client.initialize(&admin);

    // Try to submit attestation with unregistered attestor
    let timestamp = env.ledger().timestamp();
    let payload_hash = BytesN::from_array(&env, &[1u8; 32]);
    let signature = Bytes::new(&env);

    let result = client.submit_attestation_tracked(
        &unregistered_attestor,
        &subject,
        &timestamp,
        &payload_hash,
        &signature,
    );

    // Check request history shows failure
    let history = client.get_request_history(&10);
    assert_eq!(history.recent_calls.len(), 1);
    assert_eq!(history.failed_count, 1);

    let call = history.recent_calls.get(0).unwrap();
    assert_eq!(call.status, ApiCallStatus::Failed);
    assert!(call.error_code.is_some());
    assert_eq!(call.error_code.unwrap(), 102); // UnauthorizedAttestor
}

#[test]
#[ignore]
fn test_get_api_call_details() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, AnchorKitContract);
    let client = AnchorKitContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let attestor = Address::generate(&env);
    let subject = Address::generate(&env);

    // Initialize and register attestor
    client.initialize(&admin);
    client.register_attestor(&attestor);

    // Submit attestation with tracking
    let timestamp = env.ledger().timestamp();
    let payload_hash = BytesN::from_array(&env, &[1u8; 32]);
    let signature = Bytes::new(&env);

    client
        .submit_attestation_tracked(&attestor, &subject, &timestamp, &payload_hash, &signature);

    // Get request history
    let history = client.get_request_history(&10);
    let call = history.recent_calls.get(0).unwrap();

    // Get detailed information
    let details = client.get_api_call_details(&call.call_id);
    assert!(details.is_some());

    let details = details.unwrap();
    assert_eq!(details.record.call_id, call.call_id);
    assert!(details.target_address.is_some());
    assert_eq!(details.target_address.unwrap(), subject);
}

#[test]
#[ignore]
fn test_multiple_api_calls() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, AnchorKitContract);
    let client = AnchorKitContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let attestor1 = Address::generate(&env);
    let attestor2 = Address::generate(&env);

    // Initialize contract
    client.initialize(&admin);

    // Register multiple attestors
    client.register_attestor_tracked(&attestor1);
    client.register_attestor_tracked(&attestor2);

    // Check request history
    let history = client.get_request_history(&10);
    assert_eq!(history.recent_calls.len(), 2);
    assert_eq!(history.total_calls, 2);
    assert_eq!(history.success_count, 2);

    // Most recent call should be first
    let first_call = history.recent_calls.get(0).unwrap();
    assert_eq!(first_call.operation, String::from_str(&env, "register_attestor"));
}

#[test]
#[ignore]
fn test_request_history_limit() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, AnchorKitContract);
    let client = AnchorKitContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);

    // Initialize contract
    client.initialize(&admin);

    // Register multiple attestors
    for _ in 0..15 {
        let attestor = Address::generate(&env);
        client.register_attestor_tracked(&attestor);
    }

    // Request only 5 most recent
    let history = client.get_request_history(&5);
    assert_eq!(history.recent_calls.len(), 5);
    assert_eq!(history.total_calls, 15);

    // Request all
    let history_all = client.get_request_history(&100);
    assert_eq!(history_all.recent_calls.len(), 15);
}

#[test]
#[ignore]
fn test_submit_quote_tracked() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, AnchorKitContract);
    let client = AnchorKitContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let anchor = Address::generate(&env);

    // Initialize and register anchor
    client.initialize(&admin);
    client.register_attestor(&anchor);

    // Configure services
    let mut services = soroban_sdk::Vec::new(&env);
    services.push_back(crate::types::ServiceType::Quotes);
    client.configure_services(&anchor, &services);

    // Submit quote with tracking
    let base_asset = String::from_str(&env, "USD");
    let quote_asset = String::from_str(&env, "USDC");
    let rate = 10000u64;
    let fee_percentage = 100u32;
    let minimum_amount = 100u64;
    let maximum_amount = 10000u64;
    let valid_until = env.ledger().timestamp() + 3600;

    client.submit_quote_tracked(
        &anchor,
        &base_asset,
        &quote_asset,
        &rate,
        &fee_percentage,
        &minimum_amount,
        &maximum_amount,
        &valid_until,
    );

    // Check request history
    let history = client.get_request_history(&10);
    assert!(history.recent_calls.len() > 0);

    // Find the submit_quote call
    let mut found = false;
    for i in 0..history.recent_calls.len() {
        let call = history.recent_calls.get(i).unwrap();
        if call.operation == String::from_str(&env, "submit_quote") {
            assert_eq!(call.status, ApiCallStatus::Success);
            assert_eq!(call.caller, anchor);
            found = true;
            break;
        }
    }
    assert!(found);
}
