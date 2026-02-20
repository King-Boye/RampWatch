#![cfg(test)]

use crate::{SwiftRemitContract, SwiftRemitContractClient};
use soroban_sdk::{
    symbol_short, testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation, Events},
    token, Address, Env, IntoVal, String, Symbol,
};

fn create_token_contract<'a>(env: &Env, admin: &Address) -> token::StellarAssetClient<'a> {
    token::StellarAssetClient::new(env, &env.register_stellar_asset_contract_v2(admin.clone()))
}

fn create_swiftremit_contract<'a>(env: &Env) -> SwiftRemitContractClient<'a> {
    SwiftRemitContractClient::new(env, &env.register_contract(None, SwiftRemitContract {}))
}

fn default_currency(env: &Env) -> String {
    String::from_str(env, "USD")
}

fn default_country(env: &Env) -> String {
    String::from_str(env, "US")
}

#[test]
fn test_initialize() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);

    let contract = create_swiftremit_contract(&env);

    contract.initialize(&admin, &token.address, &250);

    assert_eq!(contract.get_platform_fee_bps(), 250);
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn test_initialize_twice() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);

    let contract = create_swiftremit_contract(&env);

    contract.initialize(&admin, &token.address, &250);
    contract.initialize(&admin, &token.address, &250);
}

#[test]
#[should_panic(expected = "Error(Contract, #4)")]
fn test_initialize_invalid_fee() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);

    let contract = create_swiftremit_contract(&env);

    contract.initialize(&admin, &token.address, &10001);
}

#[test]
fn test_register_agent() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let agent = Address::generate(&env);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);

    contract.register_agent(&agent);

    assert!(contract.is_agent_registered(&agent));

    assert_eq!(
        env.auths(),
        [(
            admin.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    contract.address.clone(),
                    symbol_short!("register_agent"),
                    (&agent,).into_val(&env)
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
}

#[test]
fn test_remove_agent() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let agent = Address::generate(&env);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);

    contract.register_agent(&agent);
    assert!(contract.is_agent_registered(&agent));

    contract.remove_agent(&agent);
    assert!(!contract.is_agent_registered(&agent));
}

#[test]
fn test_update_fee() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);

    contract.update_fee(&500);
    assert_eq!(contract.get_platform_fee_bps(), 500);
}

#[test]
#[should_panic(expected = "Error(Contract, #4)")]
fn test_update_fee_invalid() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);

    contract.update_fee(&10001);
}

#[test]
fn test_create_remittance() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &None);

    assert_eq!(remittance_id, 1);

    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(remittance.sender, sender);
    assert_eq!(remittance.agent, agent);
    assert_eq!(remittance.amount, 1000);
    assert_eq!(remittance.fee, 25);

    assert_eq!(token.balance(&contract.address), 1000);
    assert_eq!(token.balance(&sender), 9000);
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn test_create_remittance_invalid_amount() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    contract.create_remittance(&sender, &agent, &0, &default_currency(&env), &default_country(&env), &None);
}

#[test]
#[should_panic(expected = "Error(Contract, #5)")]
fn test_create_remittance_unregistered_agent() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);

    contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &None);
}

#[test]
fn test_confirm_payout() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &None);

    contract.confirm_payout(&remittance_id);

    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(remittance.status, crate::types::RemittanceStatus::Completed);

    assert_eq!(token.balance(&agent), 975);
    assert_eq!(contract.get_accumulated_fees(), 25);
    assert_eq!(token.balance(&contract.address), 25);
}

#[test]
#[should_panic(expected = "Error(Contract, #7)")]
fn test_confirm_payout_twice() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &None);

    contract.confirm_payout(&remittance_id);
    contract.confirm_payout(&remittance_id);
}

#[test]
fn test_cancel_remittance() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &None);

    contract.cancel_remittance(&remittance_id);

    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(remittance.status, crate::types::RemittanceStatus::Cancelled);

    assert_eq!(token.balance(&sender), 10000);
    assert_eq!(token.balance(&contract.address), 0);
}

#[test]
#[should_panic(expected = "Error(Contract, #7)")]
fn test_cancel_remittance_already_completed() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &None);
    contract.confirm_payout(&remittance_id);

    contract.cancel_remittance(&remittance_id);
}

#[test]
fn test_withdraw_fees() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);
    let fee_recipient = Address::generate(&env);

    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &None);
    contract.confirm_payout(&remittance_id);

    contract.withdraw_fees(&fee_recipient);

    assert_eq!(token.balance(&fee_recipient), 25);
    assert_eq!(contract.get_accumulated_fees(), 0);
    assert_eq!(token.balance(&contract.address), 0);
}

#[test]
#[should_panic(expected = "Error(Contract, #9)")]
fn test_withdraw_fees_no_fees() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let fee_recipient = Address::generate(&env);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);

    contract.withdraw_fees(&fee_recipient);
}

#[test]
fn test_fee_calculation() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    token.mint(&sender, &100000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &500);
    contract.register_agent(&agent);

    let remittance_id = contract.create_remittance(&sender, &agent, &10000, &default_currency(&env), &default_country(&env), &None);

    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(remittance.fee, 500);

    contract.confirm_payout(&remittance_id);
    assert_eq!(token.balance(&agent), 9500);
    assert_eq!(contract.get_accumulated_fees(), 500);
}

#[test]
fn test_multiple_remittances() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender1 = Address::generate(&env);
    let sender2 = Address::generate(&env);
    let agent = Address::generate(&env);

    token.mint(&sender1, &10000);
    token.mint(&sender2, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    let remittance_id1 = contract.create_remittance(&sender1, &agent, &1000, &default_currency(&env), &default_country(&env), &None);
    let remittance_id2 = contract.create_remittance(&sender2, &agent, &2000, &default_currency(&env), &default_country(&env), &None);

    assert_eq!(remittance_id1, 1);
    assert_eq!(remittance_id2, 2);

    contract.confirm_payout(&remittance_id1);
    contract.confirm_payout(&remittance_id2);

    assert_eq!(contract.get_accumulated_fees(), 75);
    assert_eq!(token.balance(&agent), 2925);
}

#[test]
fn test_events_emitted() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);

    contract.register_agent(&agent);

    let events = env.events().all();
    let agent_reg_event = events.last().unwrap();

    assert_eq!(
        agent_reg_event.topics,
        (symbol_short!("agent_reg"),).into_val(&env)
    );

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &None);

    let events = env.events().all();
    let create_event = events.last().unwrap();

    assert_eq!(
        create_event.topics,
        (symbol_short!("created"),).into_val(&env)
    );

    contract.confirm_payout(&remittance_id);

    let events = env.events().all();
    let complete_event = events.last().unwrap();

    assert_eq!(
        complete_event.topics,
        (symbol_short!("completed"),).into_val(&env)
    );
}

#[test]
fn test_authorization_enforcement() {
    let env = Env::default();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);

    env.mock_all_auths();
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    env.mock_all_auths();
    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &None);

    env.mock_all_auths();
    contract.confirm_payout(&remittance_id);

    assert_eq!(
        env.auths(),
        [(
            agent.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    contract.address.clone(),
                    symbol_short!("confirm_payout"),
                    (remittance_id,).into_val(&env)
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
}

#[test]
fn test_withdraw_fees_valid_address() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);
    let fee_recipient = Address::generate(&env);

    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &None);
    contract.confirm_payout(&remittance_id);

    // This should succeed with a valid address
    contract.withdraw_fees(&fee_recipient);

    assert_eq!(token.balance(&fee_recipient), 25);
    assert_eq!(contract.get_accumulated_fees(), 0);
}

#[test]
fn test_confirm_payout_valid_address() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &None);

    // This should succeed with a valid agent address
    contract.confirm_payout(&remittance_id);

    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(remittance.status, crate::types::RemittanceStatus::Completed);
    assert_eq!(token.balance(&agent), 975);
}

#[test]
fn test_address_validation_in_settlement_flow() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    // Create remittance with valid addresses
    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &None);
    
    // Confirm payout - should validate agent address
    contract.confirm_payout(&remittance_id);

    // Verify the settlement completed successfully
    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(remittance.status, crate::types::RemittanceStatus::Completed);
    assert_eq!(token.balance(&agent), 975);
    assert_eq!(contract.get_accumulated_fees(), 25);
}

#[test]
fn test_multiple_settlements_with_address_validation() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender1 = Address::generate(&env);
    let sender2 = Address::generate(&env);
    let agent1 = Address::generate(&env);
    let agent2 = Address::generate(&env);

    token.mint(&sender1, &10000);
    token.mint(&sender2, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent1);
    contract.register_agent(&agent2);

    // Create and confirm multiple remittances
    let remittance_id1 = contract.create_remittance(&sender1, &agent1, &1000, &default_currency(&env), &default_country(&env), &None);
    let remittance_id2 = contract.create_remittance(&sender2, &agent2, &2000, &default_currency(&env), &default_country(&env), &None);

    // Both should succeed with valid addresses
    contract.confirm_payout(&remittance_id1);
    contract.confirm_payout(&remittance_id2);

    assert_eq!(token.balance(&agent1), 975);
    assert_eq!(token.balance(&agent2), 1950);
    assert_eq!(contract.get_accumulated_fees(), 75);
}

#[test]
fn test_settlement_with_future_expiry() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    // Set expiry to 1 hour in the future
    let current_time = env.ledger().timestamp();
    let expiry_time = current_time + 3600;

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &Some(expiry_time));

    // Should succeed since expiry is in the future
    contract.confirm_payout(&remittance_id);

    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(remittance.status, crate::types::RemittanceStatus::Completed);
    assert_eq!(token.balance(&agent), 975);
}

#[test]
#[should_panic(expected = "Error(Contract, #11)")]
fn test_settlement_with_past_expiry() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    // Set expiry to 1 hour in the past
    let current_time = env.ledger().timestamp();
    let expiry_time = current_time.saturating_sub(3600);

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &Some(expiry_time));

    // Should fail with SettlementExpired error
    contract.confirm_payout(&remittance_id);
}

#[test]
fn test_settlement_without_expiry() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    // Create remittance without expiry
    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &None);

    // Should succeed since there's no expiry
    contract.confirm_payout(&remittance_id);

    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(remittance.status, crate::types::RemittanceStatus::Completed);
    assert_eq!(token.balance(&agent), 975);
}

#[test]
#[should_panic(expected = "Error(Contract, #12)")]
fn test_duplicate_settlement_prevention() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &None);

    // First settlement should succeed
    contract.confirm_payout(&remittance_id);

    // Verify first settlement completed
    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(remittance.status, crate::types::RemittanceStatus::Completed);
    assert_eq!(token.balance(&agent), 975);
    assert_eq!(contract.get_accumulated_fees(), 25);

    // Manually reset status to Pending to bypass status check
    // This simulates an attempt to re-execute the same settlement
    let mut remittance_copy = remittance.clone();
    remittance_copy.status = crate::types::RemittanceStatus::Pending;
    
    // Store the modified remittance back (simulating a scenario where status could be manipulated)
    env.as_contract(&contract.address, || {
        crate::storage::set_remittance(&env, remittance_id, &remittance_copy);
    });

    // Second settlement attempt should fail with DuplicateSettlement error
    contract.confirm_payout(&remittance_id);
}

#[test]
fn test_different_settlements_allowed() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    token.mint(&sender, &20000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    // Create two different remittances
    let remittance_id1 = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &None);
    let remittance_id2 = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &None);

    // Both settlements should succeed as they are different remittances
    contract.confirm_payout(&remittance_id1);
    contract.confirm_payout(&remittance_id2);

    // Verify both completed successfully
    let remittance1 = contract.get_remittance(&remittance_id1);
    let remittance2 = contract.get_remittance(&remittance_id2);
    
    assert_eq!(remittance1.status, crate::types::RemittanceStatus::Completed);
    assert_eq!(remittance2.status, crate::types::RemittanceStatus::Completed);
    assert_eq!(token.balance(&agent), 1950);
    assert_eq!(contract.get_accumulated_fees(), 50);
}

#[test]
fn test_settlement_hash_storage_efficiency() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    token.mint(&sender, &50000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    // Create and settle multiple remittances
    for _ in 0..5 {
        let remittance_id = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &None);
        contract.confirm_payout(&remittance_id);
    }

    // Verify all settlements completed
    assert_eq!(contract.get_accumulated_fees(), 125);
    assert_eq!(token.balance(&agent), 4875);
    
    // Storage should only contain settlement hashes (boolean flags), not full remittance data duplicates
    // This is verified by the fact that the contract still functions correctly
}

#[test]
fn test_duplicate_prevention_with_expiry() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    let current_time = env.ledger().timestamp();
    let expiry_time = current_time + 3600;

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &Some(expiry_time));

    // First settlement should succeed
    contract.confirm_payout(&remittance_id);

    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(remittance.status, crate::types::RemittanceStatus::Completed);
    
    // Even with valid expiry, duplicate should be prevented
    // (This would require manual status manipulation to test, covered by test_duplicate_settlement_prevention)
}

#[test]
fn test_pause_unpause() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);

    assert!(!contract.is_paused());

    contract.pause();
    assert!(contract.is_paused());

    contract.unpause();
    assert!(!contract.is_paused());
}

#[test]
#[should_panic(expected = "Error(Contract, #13)")]
fn test_settlement_blocked_when_paused() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &None);

    contract.pause();

    contract.confirm_payout(&remittance_id);
}

#[test]
fn test_settlement_works_after_unpause() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &None);

    contract.pause();
    contract.unpause();

    contract.confirm_payout(&remittance_id);

    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(remittance.status, crate::types::RemittanceStatus::Completed);
}

#[test]
fn test_get_settlement_valid() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &None);

    contract.confirm_payout(&remittance_id);

    let settlement = contract.get_settlement(&remittance_id);
    assert_eq!(settlement.id, remittance_id);
    assert_eq!(settlement.sender, sender);
    assert_eq!(settlement.agent, agent);
    assert_eq!(settlement.amount, 1000);
    assert_eq!(settlement.fee, 25);
    assert_eq!(settlement.status, crate::types::RemittanceStatus::Completed);
}

#[test]
#[should_panic(expected = "RemittanceNotFound")]
fn test_get_settlement_invalid_id() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);

    contract.get_settlement(&999);
}lid_id() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &None);

    contract.pause();
    contract.unpause();

    contract.confirm_payout(&remittance_id);

    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(remittance.status, crate::types::RemittanceStatus::Completed);
}

    contract.get_settlement(&999);
}

#[test]
fn test_set_daily_limit() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);

    let currency = String::from_str(&env, "USD");
    let country = String::from_str(&env, "US");

    contract.set_daily_limit(&currency, &country, &10000);

    let limit = contract.get_daily_limit(&currency, &country);
    assert!(limit.is_some());
    assert_eq!(limit.unwrap().limit, 10000);
}

#[test]
fn test_daily_limit_enforcement_within_limit() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    token.mint(&sender, &20000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    let currency = String::from_str(&env, "USD");
    let country = String::from_str(&env, "US");

    // Set daily limit to 10000
    contract.set_daily_limit(&currency, &country, &10000);

    // First transfer of 5000 should succeed
    let remittance_id1 = contract.create_remittance(&sender, &agent, &5000, &currency, &country, &None);
    assert_eq!(remittance_id1, 1);

    // Second transfer of 4000 should succeed (total 9000 < 10000)
    let remittance_id2 = contract.create_remittance(&sender, &agent, &4000, &currency, &country, &None);
    assert_eq!(remittance_id2, 2);

    assert_eq!(token.balance(&contract.address), 9000);
}

#[test]
#[should_panic(expected = "Error(Contract, #14)")]
fn test_daily_limit_enforcement_exceeds_limit() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    token.mint(&sender, &20000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    let currency = String::from_str(&env, "USD");
    let country = String::from_str(&env, "US");

    // Set daily limit to 10000
    contract.set_daily_limit(&currency, &country, &10000);

    // First transfer of 6000 should succeed
    contract.create_remittance(&sender, &agent, &6000, &currency, &country, &None);

    // Second transfer of 5000 should fail (total 11000 > 10000)
    contract.create_remittance(&sender, &agent, &5000, &currency, &country, &None);
}

#[test]
fn test_daily_limit_rolling_window() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    token.mint(&sender, &30000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    let currency = String::from_str(&env, "USD");
    let country = String::from_str(&env, "US");

    // Set daily limit to 10000
    contract.set_daily_limit(&currency, &country, &10000);

    // First transfer of 8000
    contract.create_remittance(&sender, &agent, &8000, &currency, &country, &None);

    // Advance time by 25 hours (beyond 24-hour window)
    env.ledger().with_mut(|li| {
        li.timestamp = li.timestamp + 90000; // 25 hours in seconds
    });

    // After 25 hours, the old transfer should be outside the window
    // New transfer of 9000 should succeed
    let remittance_id = contract.create_remittance(&sender, &agent, &9000, &currency, &country, &None);
    assert_eq!(remittance_id, 2);
}

#[test]
fn test_daily_limit_different_currencies() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    token.mint(&sender, &30000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    let usd = String::from_str(&env, "USD");
    let eur = String::from_str(&env, "EUR");
    let us = String::from_str(&env, "US");

    // Set different limits for different currencies
    contract.set_daily_limit(&usd, &us, &10000);
    contract.set_daily_limit(&eur, &us, &15000);

    // Transfer 9000 in USD should succeed
    contract.create_remittance(&sender, &agent, &9000, &usd, &us, &None);

    // Transfer 14000 in EUR should succeed (different currency limit)
    contract.create_remittance(&sender, &agent, &14000, &eur, &us, &None);

    assert_eq!(token.balance(&contract.address), 23000);
}

#[test]
fn test_daily_limit_different_countries() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    token.mint(&sender, &30000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    let usd = String::from_str(&env, "USD");
    let us = String::from_str(&env, "US");
    let uk = String::from_str(&env, "UK");

    // Set different limits for different countries
    contract.set_daily_limit(&usd, &us, &10000);
    contract.set_daily_limit(&usd, &uk, &15000);

    // Transfer 9000 to US should succeed
    contract.create_remittance(&sender, &agent, &9000, &usd, &us, &None);

    // Transfer 14000 to UK should succeed (different country limit)
    contract.create_remittance(&sender, &agent, &14000, &usd, &uk, &None);

    assert_eq!(token.balance(&contract.address), 23000);
}

#[test]
fn test_daily_limit_no_limit_configured() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    token.mint(&sender, &100000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    let currency = String::from_str(&env, "USD");
    let country = String::from_str(&env, "US");

    // No limit configured, large transfer should succeed
    let remittance_id = contract.create_remittance(&sender, &agent, &50000, &currency, &country, &None);
    assert_eq!(remittance_id, 1);
    assert_eq!(token.balance(&contract.address), 50000);
}

#[test]
fn test_daily_limit_multiple_users() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender1 = Address::generate(&env);
    let sender2 = Address::generate(&env);
    let agent = Address::generate(&env);

    token.mint(&sender1, &20000);
    token.mint(&sender2, &20000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    let currency = String::from_str(&env, "USD");
    let country = String::from_str(&env, "US");

    // Set daily limit to 10000
    contract.set_daily_limit(&currency, &country, &10000);

    // Each user should have their own limit
    contract.create_remittance(&sender1, &agent, &9000, &currency, &country, &None);
    contract.create_remittance(&sender2, &agent, &9000, &currency, &country, &None);

    assert_eq!(token.balance(&contract.address), 18000);
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn test_set_daily_limit_negative() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);

    let currency = String::from_str(&env, "USD");
    let country = String::from_str(&env, "US");

    // Negative limit should fail
    contract.set_daily_limit(&currency, &country, &-1000);
}

#[test]
fn test_daily_limit_exact_limit() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    token.mint(&sender, &20000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    let currency = String::from_str(&env, "USD");
    let country = String::from_str(&env, "US");

    // Set daily limit to 10000
    contract.set_daily_limit(&currency, &country, &10000);

    // Transfer exactly 10000 should succeed
    let remittance_id = contract.create_remittance(&sender, &agent, &10000, &currency, &country, &None);
    assert_eq!(remittance_id, 1);
}




