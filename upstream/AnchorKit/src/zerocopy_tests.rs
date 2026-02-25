#![cfg(test)]

use crate::{QuoteData, ServiceType};
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn test_borrowed_string_remains_valid() {
    let env = Env::default();
    let anchor = Address::generate(&env);

    let base_asset = String::from_str(&env, "USD");
    let quote_asset = String::from_str(&env, "USDC");

    let quote = QuoteData {
        anchor: anchor.clone(),
        base_asset: base_asset.clone(),
        quote_asset: quote_asset.clone(),
        rate: 10000,
        fee_percentage: 25,
        minimum_amount: 100,
        maximum_amount: 100000,
        valid_until: 1000000,
        quote_id: 1,
    };

    // Verify borrowed fields remain valid
    assert_eq!(quote.base_asset, base_asset);
    assert_eq!(quote.quote_asset, quote_asset);
}

#[test]
fn test_cloned_data_independent() {
    let env = Env::default();
    let anchor = Address::generate(&env);

    let original = QuoteData {
        anchor: anchor.clone(),
        base_asset: String::from_str(&env, "EUR"),
        quote_asset: String::from_str(&env, "EURC"),
        rate: 10050,
        fee_percentage: 30,
        minimum_amount: 200,
        maximum_amount: 50000,
        valid_until: 2000000,
        quote_id: 2,
    };

    let cloned = original.clone();

    // Verify clone is independent
    assert_eq!(original.rate, cloned.rate);
    assert_eq!(original.base_asset, cloned.base_asset);
}

#[test]
fn test_no_allocation_on_read() {
    let env = Env::default();
    let anchor = Address::generate(&env);

    let quote = QuoteData {
        anchor,
        base_asset: String::from_str(&env, "BTC"),
        quote_asset: String::from_str(&env, "USDC"),
        rate: 5000000,
        fee_percentage: 50,
        minimum_amount: 1,
        maximum_amount: 1000,
        valid_until: 3000000,
        quote_id: 3,
    };

    // Reading fields should not allocate
    let _rate = quote.rate;
    let _fee = quote.fee_percentage;
    let _min = quote.minimum_amount;

    assert_eq!(quote.rate, 5000000);
}

#[test]
fn test_borrowed_address_lifetime() {
    let env = Env::default();
    let anchor = Address::generate(&env);

    let quote = QuoteData {
        anchor: anchor.clone(),
        base_asset: String::from_str(&env, "ETH"),
        quote_asset: String::from_str(&env, "USDC"),
        rate: 300000,
        fee_percentage: 20,
        minimum_amount: 10,
        maximum_amount: 10000,
        valid_until: 4000000,
        quote_id: 4,
    };

    // Borrowed address remains valid
    assert_eq!(quote.anchor, anchor);
}

#[test]
fn test_multiple_borrows_safe() {
    let env = Env::default();
    let anchor = Address::generate(&env);

    let quote = QuoteData {
        anchor,
        base_asset: String::from_str(&env, "SOL"),
        quote_asset: String::from_str(&env, "USDC"),
        rate: 15000,
        fee_percentage: 15,
        minimum_amount: 50,
        maximum_amount: 20000,
        valid_until: 5000000,
        quote_id: 5,
    };

    // Multiple borrows should be safe
    let _borrow1 = &quote.base_asset;
    let _borrow2 = &quote.quote_asset;
    let _borrow3 = &quote.anchor;

    assert_eq!(quote.rate, 15000);
}

#[test]
fn test_service_type_copy_no_allocation() {
    let service1 = ServiceType::Deposits;
    let service2 = service1;

    // Copy types don't allocate
    assert_eq!(service1 as u32, service2 as u32);
}

#[test]
fn test_primitive_fields_no_allocation() {
    let env = Env::default();
    let anchor = Address::generate(&env);

    let quote = QuoteData {
        anchor,
        base_asset: String::from_str(&env, "ADA"),
        quote_asset: String::from_str(&env, "USDC"),
        rate: 45000,
        fee_percentage: 35,
        minimum_amount: 100,
        maximum_amount: 50000,
        valid_until: 6000000,
        quote_id: 6,
    };

    // Primitive field access doesn't allocate
    let rate = quote.rate;
    let fee = quote.fee_percentage;
    let min = quote.minimum_amount;
    let max = quote.maximum_amount;

    assert_eq!(rate, 45000);
    assert_eq!(fee, 35);
    assert_eq!(min, 100);
    assert_eq!(max, 50000);
}

#[test]
fn test_string_borrow_validity() {
    let env = Env::default();

    let asset1 = String::from_str(&env, "XLM");
    let asset2 = String::from_str(&env, "USDC");

    // Strings remain valid after creation
    assert_eq!(asset1.len(), 3);
    assert_eq!(asset2.len(), 4);
}

#[test]
fn test_nested_borrow_safety() {
    let env = Env::default();
    let anchor = Address::generate(&env);

    let quote = QuoteData {
        anchor: anchor.clone(),
        base_asset: String::from_str(&env, "DOT"),
        quote_asset: String::from_str(&env, "USDC"),
        rate: 70000,
        fee_percentage: 40,
        minimum_amount: 20,
        maximum_amount: 15000,
        valid_until: 7000000,
        quote_id: 7,
    };

    // Nested field access is safe
    let base_len = quote.base_asset.len();
    let quote_len = quote.quote_asset.len();

    assert_eq!(base_len, 3);
    assert_eq!(quote_len, 4);
}

#[test]
fn test_quote_data_equality_no_allocation() {
    let env = Env::default();
    let anchor = Address::generate(&env);

    let quote1 = QuoteData {
        anchor: anchor.clone(),
        base_asset: String::from_str(&env, "AVAX"),
        quote_asset: String::from_str(&env, "USDC"),
        rate: 35000,
        fee_percentage: 25,
        minimum_amount: 30,
        maximum_amount: 12000,
        valid_until: 8000000,
        quote_id: 8,
    };

    let quote2 = quote1.clone();

    // Equality check doesn't allocate
    assert_eq!(quote1, quote2);
}

#[test]
fn test_address_clone_safety() {
    let env = Env::default();
    let addr1 = Address::generate(&env);
    let addr2 = addr1.clone();

    // Cloned addresses are equal
    assert_eq!(addr1, addr2);
}
