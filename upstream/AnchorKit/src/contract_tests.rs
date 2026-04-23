#![cfg(test)]

use soroban_sdk::testutils::{Accounts, AuthorizedFunction, AuthorizedInvocation, Ledger};
use soroban_sdk::{Env, Symbol};
use crate::contract::AnchorKitContractClient;
use crate::errors::ErrorCode;
use crate::errors::AnchorKitError;

#[test]
fn test_initialize_first_call_succeeds() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = e.accounts().generate_and_create();
    let client = AnchorKitContractClient::new(&e, &e.register_contract(None, AnchorKitContract));

    client.initialize(&admin);

    // Verify admin is stored
    let stored_admin = client.get_admin();
    assert_eq!(stored_admin, admin);
}

#[test]
fn test_initialize_second_call_returns_already_initialized() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = e.accounts().generate_and_create();
    let client = AnchorKitContractClient::new(&e, &e.register_contract(None, AnchorKitContract));

    // First call succeeds
    client.initialize(&admin);

    // Second call should error with AlreadyInitialized
    let err = client.initialize(&admin);
    assert_eq!(err.err().unwrap().current_errors()[0].code, ErrorCode::AlreadyInitialized as u32);

    // Admin unchanged
    let stored_admin = client.get_admin();
    assert_eq!(stored_admin, admin);
}

#[test]
fn test_initialize_different_admin_fails() {
    let e = Env::default();
    e.mock_all_auths();

    let admin1 = e.accounts().generate_and_create();
    let admin2 = e.accounts().generate_and_create();
    let client = AnchorKitContractClient::new(&e, &e.register_contract(None, AnchorKitContract));

    // Initialize with admin1
    client.initialize(&admin1);

    // Try with admin2 - should fail AlreadyInitialized
    let err = client.with_authorization(&admin2, || client.initialize(&admin2));
    assert_eq!(err.err().unwrap().current_errors()[0].code, ErrorCode::AlreadyInitialized as u32);
}

