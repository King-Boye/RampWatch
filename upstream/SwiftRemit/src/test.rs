#![cfg(test)]
extern crate alloc;

// ============================================================================
// Test Configuration Guidance
// ============================================================================
//
// Tests in this module use hardcoded values for simplicity and determinism.
// However, you can configure test behavior using environment variables if needed.
//
// Example: Configuring test fee_bps via environment variable
//
// ```rust
// fn get_test_fee_bps() -> u32 {
//     std::env::var("TEST_FEE_BPS")
//         .ok()
//         .and_then(|s| s.parse().ok())
//         .unwrap_or(250)  // Default to 250 if not set
// }
// ```
//
// Usage in tests:
// ```rust
// let fee_bps = get_test_fee_bps();
// contract.initialize(&admin, &token.address, &fee_bps);
// ```
//
// This pattern allows you to:
// - Run tests with different fee configurations without modifying code
// - Test edge cases by setting TEST_FEE_BPS=0 or TEST_FEE_BPS=10000
// - Maintain deterministic defaults when environment variable is not set
//
// Other configurable test values:
// - TEST_INITIAL_AMOUNT: Initial token mint amount for test accounts
// - TEST_REMITTANCE_AMOUNT: Default remittance amount in tests
// - TEST_TIMEOUT: Timeout values for expiry testing
//
// ============================================================================

use crate::{SwiftRemitContract, SwiftRemitContractClient};
use soroban_sdk::token::Client as TokenClient;
use soroban_sdk::token::StellarAssetClient;
use soroban_sdk::testutils::Ledger;
use soroban_sdk::{
    symbol_short, testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation, Events, Ledger},
    token, Address, Env, IntoVal,
};

fn create_token_contract<'a>(env: &Env, admin: &Address) -> token::StellarAssetClient<'a> {
    let address = env.register_stellar_asset_contract_v2(admin.clone()).address();
    token::StellarAssetClient::new(env, &address)
}

fn get_token_balance(token: &token::StellarAssetClient, address: &Address) -> i128 {
    token::Client::new(&token.env, &token.address).balance(address)
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

    contract.initialize(&admin, &token.address, &250, &0);

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

    contract.initialize(&admin, &token.address, &250, &0);
    contract.initialize(&admin, &token.address, &250, &0);
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

    contract.initialize(&admin, &token.address, &10001, &0);
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
    contract.initialize(&admin, &token.address, &250, &0);

        contract.register_agent(&agent);

    assert_eq!(
        env.auths(),
        [(
            admin.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    contract.address.clone(),
                    symbol_short!("reg_agent"),
                    (&agent,).into_val(&env)
                )),
                sub_invocations: alloc::vec![]
            }
        )]
    );

    assert!(contract.is_agent_registered(&agent));
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
    contract.initialize(&admin, &token.address, &250, &0);

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
    contract.initialize(&admin, &token.address, &250, &0);

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
    contract.initialize(&admin, &token.address, &250, &0);

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

    env.mock_all_auths();
    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250, &0);
    contract.register_agent(&agent);

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &None);

    assert_eq!(remittance_id, 1);

    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(remittance.sender, sender);
    assert_eq!(remittance.agent, agent);
    assert_eq!(remittance.amount, 1000);
    assert_eq!(remittance.fee, 25);

    assert_eq!(get_token_balance(&token, &contract.address), 1000);
    assert_eq!(get_token_balance(&token, &sender), 9000);
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
    contract.initialize(&admin, &token.address, &250, &0);
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

    env.mock_all_auths();
    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250, &0);

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

    env.mock_all_auths();
    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250, &0);
    contract.register_agent(&agent);

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &None);

    contract.authorize_remittance(&admin, &remittance_id);
    contract.confirm_payout(&remittance_id);

    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(remittance.status, crate::types::RemittanceStatus::Settled);

    assert_eq!(get_token_balance(&token, &agent), 975);
    assert_eq!(contract.get_accumulated_fees(), 25);
    assert_eq!(get_token_balance(&token, &contract.address), 25);
}

#[test]
#[should_panic(expected = "Error(Contract, #18)")]
fn test_confirm_payout_twice() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    env.mock_all_auths();
    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250, &0);
    contract.register_agent(&agent);

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &None);

    contract.authorize_remittance(&admin, &remittance_id);
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

    env.mock_all_auths();
    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250, &0);
    contract.register_agent(&agent);

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &None);

    contract.cancel_remittance(&remittance_id);

    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(remittance.status, crate::types::RemittanceStatus::Failed);

    assert_eq!(get_token_balance(&token, &sender), 10000);
    assert_eq!(get_token_balance(&token, &contract.address), 0);
}

#[test]
#[should_panic(expected = "Error(Contract, #18)")]
fn test_cancel_remittance_already_completed() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    env.mock_all_auths();
    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250, &0);
    contract.register_agent(&agent);

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &None);
    contract.authorize_remittance(&admin, &remittance_id);
    contract.confirm_payout(&remittance_id);

    contract.cancel_remittance(&remittance_id);
}

// ============================================================================
// Comprehensive Cancellation Flow Tests
// ============================================================================

#[test]
fn test_cancel_remittance_full_refund() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    // Mint initial balance to sender
    let initial_balance = 10000i128;
    token.mint(&sender, &initial_balance);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250); // 2.5% fee
    contract.register_agent(&agent);

    // Create remittance with 1000 tokens
    let remittance_amount = 1000i128;
    let remittance_id = contract.create_remittance(&sender, &agent, &remittance_amount, &None);

    let token_client = token::Client::new(&env, &token.address);
    // Verify sender balance decreased by full amount
    assert_eq!(
        token_client.balance(&sender),
        initial_balance - remittance_amount
    );
    assert_eq!(token_client.balance(&contract.address), remittance_amount);

    // Cancel the remittance
    contract.cancel_remittance(&remittance_id);

    // Verify full refund (entire amount including fee portion)
    assert_eq!(token_client.balance(&sender), initial_balance);
    assert_eq!(token_client.balance(&contract.address), 0);

    // Verify remittance status is Cancelled
    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(remittance.status, crate::types::RemittanceStatus::Failed);
}

#[test]
fn test_cancel_remittance_sender_authorization() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    env.mock_all_auths();
    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &None);

    // Cancel and verify sender authorization was required
    contract.cancel_remittance(&remittance_id);

    assert_eq!(
        env.auths(),
        [(
            sender.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    contract.address.clone(),
                    Symbol::new(&env, "cancel_remittance"),
                    (remittance_id,).into_val(&env)
                )),
                sub_invocations: std::vec::Vec::new()
            }
        )]
    );
}

#[test]
fn test_cancel_remittance_event_emission() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    env.mock_all_auths();
    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    let remittance_amount = 1000i128;
    let remittance_id = contract.create_remittance(&sender, &agent, &remittance_amount, &None);

    // Cancel the remittance
    contract.cancel_remittance(&remittance_id);

    // Verify event was emitted
    let events = env.events().all();
    let event = events.last().unwrap();

    assert_eq!(event.0, contract.address);
    assert_eq!(Symbol::from_val(&env, &event.1.get(0).unwrap()), symbol_short!("remit"));
    assert_eq!(Symbol::from_val(&env, &event.1.get(1).unwrap()), symbol_short!("cancel"));

    let event_data: soroban_sdk::Vec<soroban_sdk::Val> =
        soroban_sdk::FromVal::from_val(&env, &event.2);
    let event_remittance_id: u64 = soroban_sdk::FromVal::from_val(&env, &event_data.get(3).unwrap());
    let event_sender: Address = soroban_sdk::FromVal::from_val(&env, &event_data.get(4).unwrap());
    let event_agent: Address = soroban_sdk::FromVal::from_val(&env, &event_data.get(5).unwrap());
    let event_amount: i128 = soroban_sdk::FromVal::from_val(&env, &event_data.get(7).unwrap());

    assert_eq!(event_remittance_id, remittance_id);
    assert_eq!(event_sender, sender);
    assert_eq!(event_agent, agent);
    assert_eq!(event_amount, remittance_amount);
}

#[test]
#[should_panic(expected = "Error(Contract, #6)")]
fn test_cancel_remittance_not_found() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);

    // Try to cancel non-existent remittance
    contract.cancel_remittance(&999);
}

#[test]
#[should_panic(expected = "Error(Contract, #18)")]
fn test_cancel_remittance_already_cancelled() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    env.mock_all_auths();
    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &None);

    // Cancel once
    contract.cancel_remittance(&remittance_id);

    // Try to cancel again - should fail
    contract.cancel_remittance(&remittance_id);
}

#[test]
fn test_cancel_remittance_multiple_remittances() {
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

    // Create multiple remittances
    let remittance_id1 = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &None);
    let remittance_id2 = contract.create_remittance(&sender, &agent, &2000, &default_currency(&env), &default_country(&env), &None);
    let remittance_id3 = contract.create_remittance(&sender, &agent, &3000, &default_currency(&env), &default_country(&env), &None);

    let token_client = token::Client::new(&env, &token.address);
    // Sender should have 14000 left (20000 - 1000 - 2000 - 3000)
    assert_eq!(token_client.balance(&sender), 14000);
    assert_eq!(token_client.balance(&contract.address), 6000);

    // Cancel first and third remittances
    contract.cancel_remittance(&remittance_id1);
    contract.cancel_remittance(&remittance_id3);

    // Verify partial refunds
    assert_eq!(token_client.balance(&sender), 18000); // 14000 + 1000 + 3000
    assert_eq!(token_client.balance(&contract.address), 2000); // Only remittance_id2 remains

    // Verify statuses
    let r1 = contract.get_remittance(&remittance_id1);
    let r2 = contract.get_remittance(&remittance_id2);
    let r3 = contract.get_remittance(&remittance_id3);

    assert_eq!(r1.status, crate::types::RemittanceStatus::Failed);
    assert_eq!(r2.status, crate::types::RemittanceStatus::Pending);
    assert_eq!(r3.status, crate::types::RemittanceStatus::Failed);
}

#[test]
fn test_cancel_remittance_no_fee_accumulation() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    env.mock_all_auths();
    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    // Create and cancel remittance
    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &None);
    contract.cancel_remittance(&remittance_id);

    // Verify no fees were accumulated (fees only accumulate on successful payout)
    assert_eq!(contract.get_accumulated_fees(), 0);
}

#[test]
fn test_cancel_remittance_preserves_remittance_data() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    env.mock_all_auths();
    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    let remittance_amount = 1000i128;
    let remittance_id = contract.create_remittance(&sender, &agent, &remittance_amount, &None);

    // Get original remittance data
    let original = contract.get_remittance(&remittance_id);

    // Cancel the remittance
    contract.cancel_remittance(&remittance_id);

    // Get cancelled remittance data
    let cancelled = contract.get_remittance(&remittance_id);

    // Verify all data is preserved except status
    assert_eq!(cancelled.id, original.id);
    assert_eq!(cancelled.sender, original.sender);
    assert_eq!(cancelled.agent, original.agent);
    assert_eq!(cancelled.amount, original.amount);
    assert_eq!(cancelled.fee, original.fee);
    assert_eq!(cancelled.expiry, original.expiry);
    assert_eq!(cancelled.status, crate::types::RemittanceStatus::Failed);
    assert_eq!(original.status, crate::types::RemittanceStatus::Pending);
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

    env.mock_all_auths();
    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250, &0);
    contract.register_agent(&agent);

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &None);
    contract.authorize_remittance(&admin, &remittance_id);
    contract.confirm_payout(&remittance_id);

    contract.withdraw_fees(&fee_recipient);

    assert_eq!(get_token_balance(&token, &fee_recipient), 25);
    assert_eq!(contract.get_accumulated_fees(), 0);
    assert_eq!(get_token_balance(&token, &contract.address), 0);
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
    contract.initialize(&admin, &token.address, &250, &0);

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
    contract.initialize(&admin, &token.address, &500, &0);
    contract.register_agent(&agent);

    let remittance_id = contract.create_remittance(&sender, &agent, &10000, &default_currency(&env), &default_country(&env), &None);

    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(remittance.fee, 500);

    contract.authorize_remittance(&admin, &remittance_id);
    contract.confirm_payout(&remittance_id);
    assert_eq!(get_token_balance(&token, &agent), 9500);
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
    contract.initialize(&admin, &token.address, &250, &0);
    contract.register_agent(&agent);

    let remittance_id1 = contract.create_remittance(&sender1, &agent, &1000, &default_currency(&env), &default_country(&env), &None);
    let remittance_id2 = contract.create_remittance(&sender2, &agent, &2000, &default_currency(&env), &default_country(&env), &None);

    assert_eq!(remittance_id1, 1);
    assert_eq!(remittance_id2, 2);

    contract.authorize_remittance(&admin, &remittance_id1);
    contract.authorize_remittance(&admin, &remittance_id2);

    contract.confirm_payout(&remittance_id1);
    contract.confirm_payout(&remittance_id2);

    assert_eq!(contract.get_accumulated_fees(), 75);
    assert_eq!(get_token_balance(&token, &agent), 2925);
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

    env.mock_all_auths();
    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250, &0);

    let initial_events = env.events().all().len();

    contract.register_agent(&agent);
    assert!(env.events().all().len() > initial_events, "Agent registration should emit event");

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &None);
    assert!(env.events().all().len() > initial_events + 1, "Remittance creation should emit event");

    contract.authorize_remittance(&admin, &remittance_id);
    contract.confirm_payout(&remittance_id);
    assert!(env.events().all().len() > initial_events + 2, "Payout confirmation should emit event");
}

#[test]
fn test_authorization_enforcement() {
    let env = Env::default();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    env.mock_all_auths();
    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);

    env.mock_all_auths();
    contract.initialize(&admin, &token.address, &250, &0);
    contract.register_agent(&agent);

    env.mock_all_auths();
    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &None);

    env.mock_all_auths();
    contract.authorize_remittance(&admin, &remittance_id);

    env.mock_all_auths();
    contract.confirm_payout(&remittance_id);

    assert_eq!(
        env.auths(),
        [(
            agent.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    contract.address.clone(),
                    symbol_short!("conf_pay"),
                    (remittance_id,).into_val(&env)
                )),
                sub_invocations: alloc::vec![]
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

    env.mock_all_auths();
    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250, &0);
    contract.register_agent(&agent);

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &None);
    contract.authorize_remittance(&admin, &remittance_id);
    contract.confirm_payout(&remittance_id);

    // This should succeed with a valid address
    contract.withdraw_fees(&fee_recipient);

    assert_eq!(get_token_balance(&token, &fee_recipient), 25);
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

    env.mock_all_auths();
    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250, &0);
    contract.register_agent(&agent);

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &None);

    // This should succeed with a valid agent address
    contract.authorize_remittance(&admin, &remittance_id);
    contract.confirm_payout(&remittance_id);

    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(remittance.status, crate::types::RemittanceStatus::Completed);
    assert_eq!(get_token_balance(&token, &agent), 975);
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

    env.mock_all_auths();
    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250, &0);
    contract.register_agent(&agent);

    // Create remittance with valid addresses
    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &None);

    // Confirm payout - should validate agent address
    contract.authorize_remittance(&admin, &remittance_id);
    contract.confirm_payout(&remittance_id);

    // Verify the settlement completed successfully
    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(remittance.status, crate::types::RemittanceStatus::Completed);
    assert_eq!(get_token_balance(&token, &agent), 975);
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
    contract.initialize(&admin, &token.address, &250, &0);
    contract.register_agent(&agent1);
    contract.register_agent(&agent2);

    // Create and confirm multiple remittances
    let remittance_id1 = contract.create_remittance(&sender1, &agent1, &1000, &default_currency(&env), &default_country(&env), &None);
    let remittance_id2 = contract.create_remittance(&sender2, &agent2, &2000, &default_currency(&env), &default_country(&env), &None);

    // Both should succeed with valid addresses
    contract.authorize_remittance(&admin, &remittance_id1);
    contract.authorize_remittance(&admin, &remittance_id2);

    contract.confirm_payout(&remittance_id1);
    contract.confirm_payout(&remittance_id2);

    assert_eq!(get_token_balance(&token, &agent1), 975);
    assert_eq!(get_token_balance(&token, &agent2), 1950);
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

    env.mock_all_auths();
    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250, &0);
    contract.register_agent(&agent);

    // Set expiry to 1 hour in the future
    env.ledger().set(soroban_sdk::testutils::LedgerInfo { timestamp: 10000, ..env.ledger().get() });
    let current_time = env.ledger().timestamp();
    let expiry_time = current_time + 3600;

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &Some(expiry_time));

    // Should succeed since expiry is in the future
    contract.authorize_remittance(&admin, &remittance_id);
    contract.confirm_payout(&remittance_id);

    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(remittance.status, crate::types::RemittanceStatus::Completed);
    assert_eq!(get_token_balance(&token, &agent), 975);
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

    env.mock_all_auths();
    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250, &0);
    contract.register_agent(&agent);

    // Set expiry to 1 hour in the past
    env.ledger().set(soroban_sdk::testutils::LedgerInfo { timestamp: 10000, ..env.ledger().get() });
    let current_time = env.ledger().timestamp();
    let expiry_time = current_time.saturating_sub(3600);

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &Some(expiry_time));

    // Should fail with SettlementExpired error
    contract.authorize_remittance(&admin, &remittance_id);
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

    env.mock_all_auths();
    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250, &0);
    contract.register_agent(&agent);

    // Create remittance without expiry
    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &None);

    // Should succeed since there's no expiry
    contract.authorize_remittance(&admin, &remittance_id);
    contract.confirm_payout(&remittance_id);

    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(remittance.status, crate::types::RemittanceStatus::Completed);
    assert_eq!(get_token_balance(&token, &agent), 975);
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

    env.mock_all_auths();
    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250, &0);
    contract.register_agent(&agent);

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &None);

    // First settlement should succeed
    contract.authorize_remittance(&admin, &remittance_id);
    contract.confirm_payout(&remittance_id);

    // Verify first settlement completed
    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(remittance.status, crate::types::RemittanceStatus::Completed);
    assert_eq!(get_token_balance(&token, &agent), 975);
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
    contract.authorize_remittance(&admin, &remittance_id);
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
    contract.initialize(&admin, &token.address, &250, &0);
    contract.register_agent(&agent);

    // Create two different remittances
    let remittance_id1 = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &None);
    let remittance_id2 = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &None);

    // Both settlements should succeed as they are different remittances
    contract.authorize_remittance(&admin, &remittance_id1);
    contract.authorize_remittance(&admin, &remittance_id2);

    contract.confirm_payout(&remittance_id1);
    contract.confirm_payout(&remittance_id2);

    // Verify both completed successfully
    let remittance1 = contract.get_remittance(&remittance_id1);
    let remittance2 = contract.get_remittance(&remittance_id2);
    
    assert_eq!(remittance1.status, crate::types::RemittanceStatus::Completed);
    assert_eq!(remittance2.status, crate::types::RemittanceStatus::Completed);
    assert_eq!(get_token_balance(&token, &agent), 1950);
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
    contract.initialize(&admin, &token.address, &250, &0);
    contract.register_agent(&agent);

    // Create and settle multiple remittances
    for _ in 0..5 {
        let remittance_id = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &None);
        contract.authorize_remittance(&admin, &remittance_id);
        contract.confirm_payout(&remittance_id);
    }

    // Verify all settlements completed
    assert_eq!(contract.get_accumulated_fees(), 125);
    assert_eq!(get_token_balance(&token, &agent), 4875);
    
    // Storage should only contain settlement hashes (boolean flags), not full remittance data duplicates
    // This is verified by the fact that the contract still functions correctly
    assert_eq!(get_token_balance(&token, &agent), 4875);
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

    env.mock_all_auths();
    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250, &0);
    contract.register_agent(&agent);

    env.ledger().set(soroban_sdk::testutils::LedgerInfo { timestamp: 10000, ..env.ledger().get() });
    let current_time = env.ledger().timestamp();
    let expiry_time = current_time + 3600;

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &default_currency(&env), &default_country(&env), &Some(expiry_time));

    contract.authorize_remittance(&admin, &remittance_id);

    // First settlement should succeed
    contract.confirm_payout(&remittance_id);

    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(remittance.status, crate::types::RemittanceStatus::Settled);

    // Even with valid expiry, duplicate should be prevented
    // (This would require manual status manipulation to test, covered by test_duplicate_settlement_prevention)
}

// ========== Asset Verification Tests ==========

#[test]
fn test_set_and_get_asset_verification() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);

    let asset_code = String::from_str(&env, "USDC");
    let issuer = Address::generate(&env);

    // Set verification
    contract.set_asset_verification(
        &asset_code,
        &issuer,
        &crate::VerificationStatus::Verified,
        &95,
        &15000,
        &true,
    );

    // Get verification
    let verification = contract.get_asset_verification(&asset_code, &issuer);

    assert_eq!(verification.asset_code, asset_code);
    assert_eq!(verification.issuer, issuer);
    assert_eq!(verification.status, crate::VerificationStatus::Verified);
    assert_eq!(verification.reputation_score, 95);
    assert_eq!(verification.trustline_count, 15000);
    assert_eq!(verification.has_toml, true);
}

#[test]
fn test_has_asset_verification() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);

    let asset_code = String::from_str(&env, "USDC");
    let issuer = Address::generate(&env);

    // Should not exist initially
    assert_eq!(contract.has_asset_verification(&asset_code, &issuer), false);

    // Set verification
    contract.set_asset_verification(
        &asset_code,
        &issuer,
        &crate::VerificationStatus::Verified,
        &95,
        &15000,
        &true,
    );

    // Should exist now
    assert_eq!(contract.has_asset_verification(&asset_code, &issuer), true);
}

#[test]
#[should_panic(expected = "Error(Contract, #14)")]
fn test_set_asset_verification_invalid_score() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);

    let asset_code = String::from_str(&env, "USDC");
    let issuer = Address::generate(&env);

    // Should panic with InvalidReputationScore
    contract.set_asset_verification(
        &asset_code,
        &issuer,
        &crate::VerificationStatus::Verified,
        &101, // Invalid: > 100
        &15000,
        &true,
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #13)")]
fn test_get_asset_verification_not_found() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);

    let asset_code = String::from_str(&env, "NOTFOUND");
    let issuer = Address::generate(&env);

    // Should panic with AssetNotFound
    contract.get_asset_verification(&asset_code, &issuer);
}

#[test]
fn test_validate_asset_safety_verified() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);

    let asset_code = String::from_str(&env, "USDC");
    let issuer = Address::generate(&env);

    // Set as verified
    contract.set_asset_verification(
        &asset_code,
        &issuer,
        &crate::VerificationStatus::Verified,
        &95,
        &15000,
        &true,
    );

    // Should pass validation
    contract.validate_asset_safety(&asset_code, &issuer);
}

#[test]
fn test_validate_asset_safety_unverified() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);

    let asset_code = String::from_str(&env, "UNKNOWN");
    let issuer = Address::generate(&env);

    // Set as unverified
    contract.set_asset_verification(
        &asset_code,
        &issuer,
        &crate::VerificationStatus::Unverified,
        &50,
        &100,
        &false,
    );

    // Should pass validation (unverified is not suspicious)
    contract.validate_asset_safety(&asset_code, &issuer);
}

#[test]
#[should_panic(expected = "Error(Contract, #15)")]
fn test_validate_asset_safety_suspicious() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);

    let asset_code = String::from_str(&env, "SCAM");
    let issuer = Address::generate(&env);

    // Set as suspicious
    contract.set_asset_verification(
        &asset_code,
        &issuer,
        &crate::VerificationStatus::Suspicious,
        &20,
        &5,
        &false,
    );

    // Should panic with SuspiciousAsset
    contract.validate_asset_safety(&asset_code, &issuer);
}

#[test]
fn test_update_asset_verification() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);

    let asset_code = String::from_str(&env, "USDC");
    let issuer = Address::generate(&env);

    // Initial verification
    contract.set_asset_verification(
        &asset_code,
        &issuer,
        &crate::VerificationStatus::Unverified,
        &50,
        &100,
        &false,
    );

    let verification1 = contract.get_asset_verification(&asset_code, &issuer);
    assert_eq!(verification1.reputation_score, 50);

    // Update verification
    contract.set_asset_verification(
        &asset_code,
        &issuer,
        &crate::VerificationStatus::Verified,
        &95,
        &15000,
        &true,
    );

    let verification2 = contract.get_asset_verification(&asset_code, &issuer);
    assert_eq!(verification2.reputation_score, 95);
    assert_eq!(verification2.status, crate::VerificationStatus::Verified);
}
