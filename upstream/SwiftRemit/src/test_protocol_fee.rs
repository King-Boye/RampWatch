#![cfg(test)]

use crate::ContractError;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_protocol_fee_storage() {
    let env = Env::default();
    
    // Set protocol fee
    crate::storage::set_protocol_fee_bps(&env, 100).unwrap(); // 1%
    assert_eq!(crate::storage::get_protocol_fee_bps(&env), 100);
    
    // Update protocol fee
    crate::storage::set_protocol_fee_bps(&env, 150).unwrap(); // 1.5%
    assert_eq!(crate::storage::get_protocol_fee_bps(&env), 150);
}

#[test]
fn test_protocol_fee_cap() {
    let env = Env::default();
    
    // Max allowed: 200 bps
    assert!(crate::storage::set_protocol_fee_bps(&env, 200).is_ok());
    
    // Over max: should fail
    let result = crate::storage::set_protocol_fee_bps(&env, 201);
    assert_eq!(result, Err(ContractError::InvalidFeeBps));
    
    // Way over max: should fail
    let result = crate::storage::set_protocol_fee_bps(&env, 1000);
    assert_eq!(result, Err(ContractError::InvalidFeeBps));
}

#[test]
fn test_treasury_storage() {
    let env = Env::default();
    
    let treasury = Address::generate(&env);
    
    // Set treasury
    crate::storage::set_treasury(&env, &treasury);
    assert_eq!(crate::storage::get_treasury(&env).unwrap(), treasury);
    
    // Update treasury
    let new_treasury = Address::generate(&env);
    crate::storage::set_treasury(&env, &new_treasury);
    assert_eq!(crate::storage::get_treasury(&env).unwrap(), new_treasury);
}

#[test]
fn test_protocol_fee_calculation() {
    let env = Env::default();
    
    // Set 1% protocol fee (100 bps)
    crate::storage::set_protocol_fee_bps(&env, 100).unwrap();
    
    let amount = 10000i128;
    let fee_bps = crate::storage::get_protocol_fee_bps(&env);
    
    // Calculate fee: 10000 * 100 / 10000 = 100
    let protocol_fee = amount * (fee_bps as i128) / 10000;
    assert_eq!(protocol_fee, 100);
    
    // Set 2% protocol fee (200 bps - max)
    crate::storage::set_protocol_fee_bps(&env, 200).unwrap();
    let fee_bps = crate::storage::get_protocol_fee_bps(&env);
    
    // Calculate fee: 10000 * 200 / 10000 = 200
    let protocol_fee = amount * (fee_bps as i128) / 10000;
    assert_eq!(protocol_fee, 200);
}

#[test]
fn test_zero_protocol_fee() {
    let env = Env::default();
    
    // Zero fee should be allowed
    assert!(crate::storage::set_protocol_fee_bps(&env, 0).is_ok());
    assert_eq!(crate::storage::get_protocol_fee_bps(&env), 0);
    
    let amount = 10000i128;
    let protocol_fee = amount * 0 / 10000;
    assert_eq!(protocol_fee, 0);
}

#[test]
fn test_default_protocol_fee() {
    let env = Env::default();
    
    // Default should be 0 if not set
    assert_eq!(crate::storage::get_protocol_fee_bps(&env), 0);
}
