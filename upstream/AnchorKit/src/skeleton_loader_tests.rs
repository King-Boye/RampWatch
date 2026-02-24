#![cfg(test)]

use crate::{
    AnchorInfoSkeleton, AnchorKitContract, AnchorKitContractClient, AuthValidationSkeleton,
    TransactionStatusSkeleton,
};
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn test_anchor_info_skeleton_loading() {
    let env = Env::default();
    let anchor = Address::generate(&env);
    
    let skeleton = AnchorInfoSkeleton::loading(anchor.clone());
    
    assert_eq!(skeleton.anchor, anchor);
    assert_eq!(skeleton.is_loading, true);
    assert_eq!(skeleton.has_error, false);
    assert_eq!(skeleton.error_message, None);
}

#[test]
fn test_anchor_info_skeleton_loaded() {
    let env = Env::default();
    let anchor = Address::generate(&env);
    
    let skeleton = AnchorInfoSkeleton::loaded(anchor.clone());
    
    assert_eq!(skeleton.anchor, anchor);
    assert_eq!(skeleton.is_loading, false);
    assert_eq!(skeleton.has_error, false);
}

#[test]
fn test_anchor_info_skeleton_error() {
    let env = Env::default();
    let anchor = Address::generate(&env);
    let error_msg = String::from_str(&env, "Test error");
    
    let skeleton = AnchorInfoSkeleton::error(anchor.clone(), error_msg.clone());
    
    assert_eq!(skeleton.anchor, anchor);
    assert_eq!(skeleton.is_loading, false);
    assert_eq!(skeleton.has_error, true);
    assert_eq!(skeleton.error_message, Some(error_msg));
}

#[test]
fn test_transaction_status_skeleton_loading() {
    let env = Env::default();
    let tx_id = 123u64;
    
    let skeleton = TransactionStatusSkeleton::loading(tx_id);
    
    assert_eq!(skeleton.transaction_id, tx_id);
    assert_eq!(skeleton.is_loading, true);
    assert_eq!(skeleton.has_error, false);
    assert_eq!(skeleton.progress_percentage, 0);
}

#[test]
fn test_transaction_status_skeleton_with_progress() {
    let env = Env::default();
    let tx_id = 456u64;
    let progress = 5000u32; // 50%
    
    let skeleton = TransactionStatusSkeleton::loading_with_progress(tx_id, progress);
    
    assert_eq!(skeleton.transaction_id, tx_id);
    assert_eq!(skeleton.is_loading, true);
    assert_eq!(skeleton.progress_percentage, progress);
}

#[test]
fn test_transaction_status_skeleton_loaded() {
    let env = Env::default();
    let tx_id = 789u64;
    
    let skeleton = TransactionStatusSkeleton::loaded(tx_id);
    
    assert_eq!(skeleton.transaction_id, tx_id);
    assert_eq!(skeleton.is_loading, false);
    assert_eq!(skeleton.has_error, false);
    assert_eq!(skeleton.progress_percentage, 10000); // 100%
}

#[test]
fn test_auth_validation_skeleton_validating() {
    let env = Env::default();
    let attestor = Address::generate(&env);
    
    let skeleton = AuthValidationSkeleton::validating(&env, attestor.clone());
    
    assert_eq!(skeleton.attestor, attestor);
    assert_eq!(skeleton.is_validating, true);
    assert_eq!(skeleton.is_valid, false);
    assert_eq!(skeleton.has_error, false);
}

#[test]
fn test_auth_validation_skeleton_validated() {
    let env = Env::default();
    let attestor = Address::generate(&env);
    
    let skeleton = AuthValidationSkeleton::validated(&env, attestor.clone());
    
    assert_eq!(skeleton.attestor, attestor);
    assert_eq!(skeleton.is_validating, false);
    assert_eq!(skeleton.is_valid, true);
    assert_eq!(skeleton.has_error, false);
}

#[test]
fn test_get_anchor_info_skeleton_not_found() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register_contract(None, AnchorKitContract);
    let client = AnchorKitContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    client.initialize(&admin);
    
    let anchor = Address::generate(&env);
    let result = client.get_anchor_info_skeleton(&anchor);
    
    assert!(result.has_error);
}

#[test]
fn test_get_transaction_status_skeleton_not_found() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register_contract(None, AnchorKitContract);
    let client = AnchorKitContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    client.initialize(&admin);
    
    let result = client.get_transaction_status_skeleton(&999);
    
    assert!(result.has_error);
}

#[test]
fn test_get_transaction_status_skeleton_with_session() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register_contract(None, AnchorKitContract);
    let client = AnchorKitContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    client.initialize(&admin);
    
    let initiator = Address::generate(&env);
    let session_id = client.create_session(&initiator);
    
    let result = client.get_transaction_status_skeleton(&session_id);
    
    assert!(result.is_loading);
    assert_eq!(result.transaction_id, session_id);
}

#[test]
fn test_get_auth_validation_skeleton_not_registered() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register_contract(None, AnchorKitContract);
    let client = AnchorKitContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    client.initialize(&admin);
    
    let attestor = Address::generate(&env);
    let result = client.get_auth_validation_skeleton(&attestor);
    
    assert!(result.has_error);
}
