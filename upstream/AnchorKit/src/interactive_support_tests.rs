#![cfg(test)]
use crate::interactive_support::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn test_generate_url() {
    let env = Env::default();
    let anchor = Address::generate(&env);
    let token = String::from_str(&env, "test_token");
    let tx_id = String::from_str(&env, "tx_123");

    let result = InteractiveSupport::generate_url(&env, &anchor, &token, &tx_id);

    assert_eq!(result.transaction_id, tx_id);
    assert!(result.expires_at > env.ledger().timestamp());
}

#[test]
fn test_handle_callback() {
    let env = Env::default();
    let tx_id = String::from_str(&env, "tx_456");
    let status = String::from_str(&env, "completed");

    let result = InteractiveSupport::handle_callback(&env, &tx_id, &status);

    assert_eq!(result.transaction_id, tx_id);
    assert_eq!(result.status, status);
    assert_eq!(result.timestamp, env.ledger().timestamp());
}

#[test]
fn test_poll_status() {
    let env = Env::default();
    let tx_id = String::from_str(&env, "tx_789");

    let result = InteractiveSupport::poll_status(&env, &tx_id);

    assert_eq!(result.id, tx_id);
    assert_eq!(result.updated_at, env.ledger().timestamp());
}

#[test]
fn test_inject_token() {
    let env = Env::default();
    let token = String::from_str(&env, "bearer_token");

    let headers = InteractiveSupport::inject_token(&env, &token);

    assert!(headers.contains_key(String::from_str(&env, "Authorization")));
}
