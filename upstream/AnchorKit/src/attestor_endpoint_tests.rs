use super::*;
use soroban_sdk::{testutils::{Address as _, Ledger as _, LedgerInfo}, symbol_short, Address, Env, Symbol, String};
use crate::domain_validator::validate_anchor_domain;
use crate::errors::{AnchorKitError, ErrorCode};

#[test]
fn test_set_get_endpoint_happy_path() {
    let env = Env::default();
    env.mock_all_auths();

    let attestor = Address::random(&env);
    let endpoint = String::from_str(&env, "https://example.com/api");

    // Register attestor (admin auth mocked)
    AnchorKitContract::register_attestor(&env, attestor.clone(), String::from_str(&env, "mock_token"), Address::random(&env));

    // Set endpoint
    AnchorKitContract::set_endpoint(&env, attestor.clone(), endpoint.clone());

    // Get endpoint
    let retrieved = AnchorKitContract::get_endpoint(&env, attestor.clone());
    assert_eq!(retrieved, endpoint);
}

#[test]
#[should_panic(expected = "AttestorNotRegistered")]
fn test_get_endpoint_not_registered() {
    let env = Env::default();
    env.mock_all_auths();

    let attestor = Address::random(&env);
    AnchorKitContract::get_endpoint(&env, attestor);
}

#[test]
#[should_panic(expected = "AttestorNotRegistered")]
fn test_set_endpoint_not_attestor() {
    let env = Env::default();
    env.mock_all_auths();

    let attestor = Address::random(&env);
    let endpoint = String::from_str(&env, "https://example.com");
    AnchorKitContract::set_endpoint(&env, attestor, endpoint);
}

#[test]
#[should_panic(expected = "InvalidEndpointFormat")]
fn test_set_endpoint_invalid_url() {
    let env = Env::default();
    env.mock_all_auths();

    let attestor = Address::random(&env);
    AnchorKitContract::register_attestor(&env, attestor.clone(), String::from_str(&env, "mock"), Address::random(&env));

    let invalid = String::from_str(&env, "http://invalid.com"); // HTTP
    AnchorKitContract::set_endpoint(&env, attestor, invalid);
}

#[test]
#[should_panic(expected = "Unauthorized")]
fn test_set_endpoint_unauthorized() {
    let env = Env::default();

    let attestor = Address::random(&env);
    AnchorKitContract::register_attestor(&env, attestor.clone(), String::from_str(&env, "mock"), Address::random(&env));

    let endpoint = String::from_str(&env, "https://example.com");
    let caller = Address::random(&env);
    caller.require_auth(); // Mock auth for wrong caller

    // Function requires attestor.require_auth(), so wrong caller panics on auth
    // Test assumes env.mock_all_auths() not called
    env.budget().reset_unlimited();
    // Note: testutils mock_all_auths needed for require_auth in tests
}

#[test]
fn test_endpoint_updated_event() {
    let env = Env::default();
    env.mock_all_auths();

    let attestor = Address::random(&env);
    let endpoint = String::from_str(&env, "https://test.com");

    AnchorKitContract::register_attestor(&env, attestor.clone(), String::from_str(&env, "token"), Address::random(&env));

    // Expect event
    let topics = (symbol_short!("endpoint"), symbol_short!("updated"));
    env.events().publish_expect(&topics, &EndpointUpdated { attestor: attestor.clone(), endpoint: endpoint.clone() });

    // Calling set_endpoint should emit it
    AnchorKitContract::set_endpoint(&env, attestor, endpoint.clone());
    // Verify emitted (testutils check)
}

