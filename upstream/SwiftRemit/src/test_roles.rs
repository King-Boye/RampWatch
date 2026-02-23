#![cfg(test)]

use crate::{SwiftRemitContract, SwiftRemitContractClient, Role};
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_role_assignment_by_admin() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, SwiftRemitContract);
    let client = SwiftRemitContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let settler = Address::generate(&env);
    let usdc_token = Address::generate(&env);

    // Initialize contract
    client.initialize(&admin, &usdc_token, &250, &3600);

    // Admin should have Admin role
    assert!(client.has_role(&admin, &Role::Admin));

    // Assign Settler role
    client.assign_role(&admin, &settler, &Role::Settler);
    assert!(client.has_role(&settler, &Role::Settler));

    // Remove Settler role
    client.remove_role(&admin, &settler, &Role::Settler);
    assert!(!client.has_role(&settler, &Role::Settler));
}

#[test]
#[should_panic(expected = "Unauthorized")]
fn test_non_admin_cannot_assign_roles() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, SwiftRemitContract);
    let client = SwiftRemitContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let non_admin = Address::generate(&env);
    let settler = Address::generate(&env);
    let usdc_token = Address::generate(&env);

    // Initialize contract
    client.initialize(&admin, &usdc_token, &250, &3600);

    // Non-admin tries to assign role - should panic
    client.assign_role(&non_admin, &settler, &Role::Settler);
}

#[test]
#[should_panic(expected = "Unauthorized")]
fn test_confirm_payout_requires_settler_role() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, SwiftRemitContract);
    let client = SwiftRemitContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let agent = Address::generate(&env);
    let sender = Address::generate(&env);
    let usdc_token = Address::generate(&env);

    // Initialize contract
    client.initialize(&admin, &usdc_token, &250, &3600);

    // Register agent but don't assign Settler role
    client.register_agent(&agent);

    // Create remittance
    let remittance_id = client.create_remittance(&sender, &agent, &1000, &None);

    // Agent tries to confirm payout without Settler role - should panic
    client.confirm_payout(&remittance_id);
}

#[test]
fn test_settler_can_finalize_transfers() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, SwiftRemitContract);
    let client = SwiftRemitContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let agent = Address::generate(&env);
    let sender = Address::generate(&env);
    let usdc_token = Address::generate(&env);

    // Initialize contract
    client.initialize(&admin, &usdc_token, &250, &3600);

    // Register agent and assign Settler role
    client.register_agent(&agent);
    client.assign_role(&admin, &agent, &Role::Settler);

    // Verify agent has Settler role
    assert!(client.has_role(&agent, &Role::Settler));

    // Create remittance
    let remittance_id = client.create_remittance(&sender, &agent, &1000, &None);

    // Agent with Settler role can confirm payout
    client.confirm_payout(&remittance_id);
}

#[test]
fn test_role_persistence() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, SwiftRemitContract);
    let client = SwiftRemitContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let settler = Address::generate(&env);
    let usdc_token = Address::generate(&env);

    // Initialize contract
    client.initialize(&admin, &usdc_token, &250, &3600);

    // Assign Settler role
    client.assign_role(&admin, &settler, &Role::Settler);

    // Check role persists across multiple calls
    assert!(client.has_role(&settler, &Role::Settler));
    assert!(client.has_role(&settler, &Role::Settler));
    assert!(client.has_role(&settler, &Role::Settler));
}
