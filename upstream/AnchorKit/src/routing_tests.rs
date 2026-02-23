#![cfg(test)]

use crate::{AnchorKitContract, AnchorKitContractClient, ServiceType};
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    Address, Env, String, Vec,
};

fn create_contract(env: &Env) -> AnchorKitContractClient<'_> {
    let contract_id = env.register_contract(None, AnchorKitContract);
    AnchorKitContractClient::new(env, &contract_id)
}

#[test]
fn test_select_best_quote_from_multiple_anchors() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().with_mut(|li| li.timestamp = 1000000);

    let admin = Address::generate(&env);
    let anchor1 = Address::generate(&env);
    let anchor2 = Address::generate(&env);
    let anchor3 = Address::generate(&env);

    let client = create_contract(&env);
    client.initialize(&admin);

    // Setup anchors
    for anchor in [&anchor1, &anchor2, &anchor3] {
        client.register_attestor(anchor);
        let mut services = Vec::new(&env);
        services.push_back(ServiceType::Deposits);
        services.push_back(ServiceType::Quotes);
        client.configure_services(anchor, &services);
    }

    // Submit quotes with different rates
    let q1 = client.submit_quote(
        &anchor1,
        &String::from_str(&env, "USD"),
        &String::from_str(&env, "USDC"),
        &10100u64,
        &50u32,
        &100u64,
        &100000u64,
        &1003600u64,
    );

    let q2 = client.submit_quote(
        &anchor2,
        &String::from_str(&env, "USD"),
        &String::from_str(&env, "USDC"),
        &10000u64, // Best rate
        &25u32,
        &100u64,
        &100000u64,
        &1003600u64,
    );

    let q3 = client.submit_quote(
        &anchor3,
        &String::from_str(&env, "USD"),
        &String::from_str(&env, "USDC"),
        &10050u64,
        &30u32,
        &100u64,
        &100000u64,
        &1003600u64,
    );

    // Retrieve and compare quotes
    let quote1 = client.get_quote(&anchor1, &q1);
    let quote2 = client.get_quote(&anchor2, &q2);
    let quote3 = client.get_quote(&anchor3, &q3);

    // Verify anchor2 has best rate
    assert!(quote2.rate < quote1.rate);
    assert!(quote2.rate < quote3.rate);
    assert_eq!(quote2.rate, 10000);
}

#[test]
fn test_select_lowest_fee_anchor() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().with_mut(|li| li.timestamp = 1000000);

    let admin = Address::generate(&env);
    let anchor1 = Address::generate(&env);
    let anchor2 = Address::generate(&env);

    let client = create_contract(&env);
    client.initialize(&admin);

    for anchor in [&anchor1, &anchor2] {
        client.register_attestor(anchor);
        let mut services = Vec::new(&env);
        services.push_back(ServiceType::Deposits);
        services.push_back(ServiceType::Quotes);
        client.configure_services(anchor, &services);
    }

    let q1 = client.submit_quote(
        &anchor1,
        &String::from_str(&env, "USD"),
        &String::from_str(&env, "USDC"),
        &10000u64,
        &50u32, // Higher fee
        &100u64,
        &100000u64,
        &1003600u64,
    );

    let q2 = client.submit_quote(
        &anchor2,
        &String::from_str(&env, "USD"),
        &String::from_str(&env, "USDC"),
        &10000u64,
        &20u32, // Lower fee
        &100u64,
        &100000u64,
        &1003600u64,
    );

    let quote1 = client.get_quote(&anchor1, &q1);
    let quote2 = client.get_quote(&anchor2, &q2);

    assert!(quote2.fee_percentage < quote1.fee_percentage);
    assert_eq!(quote2.fee_percentage, 20);
}

#[test]
fn test_handle_unavailable_anchors() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().with_mut(|li| li.timestamp = 1000000);

    let admin = Address::generate(&env);
    let anchor1 = Address::generate(&env);
    let anchor2 = Address::generate(&env);
    let unavailable = Address::generate(&env);

    let client = create_contract(&env);
    client.initialize(&admin);

    // Setup only anchor1 and anchor2
    for anchor in [&anchor1, &anchor2] {
        client.register_attestor(anchor);
        let mut services = Vec::new(&env);
        services.push_back(ServiceType::Deposits);
        services.push_back(ServiceType::Quotes);
        client.configure_services(anchor, &services);
    }

    client.submit_quote(
        &anchor1,
        &String::from_str(&env, "USD"),
        &String::from_str(&env, "USDC"),
        &10000u64,
        &25u32,
        &100u64,
        &100000u64,
        &1003600u64,
    );

    client.submit_quote(
        &anchor2,
        &String::from_str(&env, "USD"),
        &String::from_str(&env, "USDC"),
        &10050u64,
        &30u32,
        &100u64,
        &100000u64,
        &1003600u64,
    );

    // Verify unavailable anchor is not registered
    assert!(!client.supports_service(&unavailable, &ServiceType::Quotes));
}

#[test]
fn test_expired_quotes_filtered() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().with_mut(|li| li.timestamp = 1000000);

    let admin = Address::generate(&env);
    let anchor = Address::generate(&env);

    let client = create_contract(&env);
    client.initialize(&admin);

    client.register_attestor(&anchor);
    let mut services = Vec::new(&env);
    services.push_back(ServiceType::Deposits);
    services.push_back(ServiceType::Quotes);
    client.configure_services(&anchor, &services);

    // Submit quote that will expire soon
    let soon_expired_id = client.submit_quote(
        &anchor,
        &String::from_str(&env, "USD"),
        &String::from_str(&env, "USDC"),
        &9900u64,
        &15u32,
        &100u64,
        &100000u64,
        &1000100u64, // Expires in 100 seconds
    );

    // Submit valid quote
    let valid_id = client.submit_quote(
        &anchor,
        &String::from_str(&env, "USD"),
        &String::from_str(&env, "USDC"),
        &10000u64,
        &25u32,
        &100u64,
        &100000u64,
        &1003600u64, // Valid for 3600 seconds
    );

    let soon_expired = client.get_quote(&anchor, &soon_expired_id);
    let valid_quote = client.get_quote(&anchor, &valid_id);

    // Verify expiration times
    assert!(soon_expired.valid_until < valid_quote.valid_until);
    assert!(valid_quote.valid_until > env.ledger().timestamp());
}

#[test]
fn test_no_anchors_available() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let unavailable = Address::generate(&env);

    let client = create_contract(&env);
    client.initialize(&admin);

    // Verify anchor is not registered
    let result = client.try_get_quote(&unavailable, &1);
    assert!(result.is_err());
}

#[test]
fn test_amount_outside_quote_limits() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().with_mut(|li| li.timestamp = 1000000);

    let admin = Address::generate(&env);
    let anchor = Address::generate(&env);

    let client = create_contract(&env);
    client.initialize(&admin);

    client.register_attestor(&anchor);
    let mut services = Vec::new(&env);
    services.push_back(ServiceType::Deposits);
    services.push_back(ServiceType::Quotes);
    client.configure_services(&anchor, &services);

    let quote_id = client.submit_quote(
        &anchor,
        &String::from_str(&env, "USD"),
        &String::from_str(&env, "USDC"),
        &10000u64,
        &25u32,
        &100u64,    // Min
        &100000u64, // Max
        &1003600u64,
    );

    let quote = client.get_quote(&anchor, &quote_id);

    // Verify limits
    assert_eq!(quote.minimum_amount, 100);
    assert_eq!(quote.maximum_amount, 100000);

    // Amount 50 would be outside limits (< 100)
    assert!(50 < quote.minimum_amount);
    // Amount 200000 would be outside limits (> 100000)
    assert!(200000 > quote.maximum_amount);
}
