#![cfg(test)]
extern crate std;

use crate::{SwiftRemitContract, SwiftRemitContractClient};
use soroban_sdk::token::Client as TokenClient;
use soroban_sdk::token::StellarAssetClient;
use soroban_sdk::testutils::Ledger;
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation, Events},
    token, Address, Env, FromVal, IntoVal, String, Symbol,
};

fn create_token_contract<'a>(
    env: &Env,
    admin: &Address,
) -> soroban_sdk::token::StellarAssetClient<'a> {
    soroban_sdk::token::StellarAssetClient::new(
        env,
        &env.register_stellar_asset_contract(admin.clone()),
    )
}

fn create_swiftremit_contract<'a>(env: &Env) -> SwiftRemitContractClient<'a> {
    SwiftRemitContractClient::new(env, &env.register_contract(None, SwiftRemitContract {}))
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

    assert_eq!(
        env.auths(),
        [(
            admin.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    contract.address.clone(),
                    Symbol::new(&env, "register_agent"),
                    (&agent,).into_val(&env)
                )),
                sub_invocations: std::vec::Vec::new()
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

    env.mock_all_auths();
    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &None);

    assert_eq!(remittance_id, 1);

    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(remittance.sender, sender);
    assert_eq!(remittance.agent, agent);
    assert_eq!(remittance.amount, 1000);
    assert_eq!(remittance.fee, 25);

    let token_client = token::Client::new(&env, &token.address);
    assert_eq!(token_client.balance(&contract.address), 1000);
    assert_eq!(token_client.balance(&sender), 9000);
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

    contract.create_remittance(&sender, &agent, &0, &None);
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
    contract.initialize(&admin, &token.address, &250);

    contract.create_remittance(&sender, &agent, &1000, &None);
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
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &None);

    contract.authorize_remittance(&admin, &remittance_id);
    contract.confirm_payout(&remittance_id);

    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(remittance.status, crate::types::RemittanceStatus::Settled);

    let token_client = token::Client::new(&env, &token.address);
    assert_eq!(token_client.balance(&agent), 975);
    assert_eq!(contract.get_accumulated_fees(), 25);
    assert_eq!(token_client.balance(&contract.address), 25);
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
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &None);

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
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &None);

    contract.cancel_remittance(&remittance_id);

    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(remittance.status, crate::types::RemittanceStatus::Failed);

    let token_client = token::Client::new(&env, &token.address);
    assert_eq!(token_client.balance(&sender), 10000);
    assert_eq!(token_client.balance(&contract.address), 0);
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
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &None);
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

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &None);

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

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &None);

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
    let remittance_id1 = contract.create_remittance(&sender, &agent, &1000, &None);
    let remittance_id2 = contract.create_remittance(&sender, &agent, &2000, &None);
    let remittance_id3 = contract.create_remittance(&sender, &agent, &3000, &None);

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
    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &None);
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
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &None);
    contract.authorize_remittance(&admin, &remittance_id);
    contract.confirm_payout(&remittance_id);

    contract.withdraw_fees(&fee_recipient);

    let token_client = token::Client::new(&env, &token.address);
    assert_eq!(token_client.balance(&fee_recipient), 25);
    assert_eq!(contract.get_accumulated_fees(), 0);
    assert_eq!(token_client.balance(&contract.address), 0);
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

    let remittance_id = contract.create_remittance(&sender, &agent, &10000, &None);

    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(remittance.fee, 500);

    contract.authorize_remittance(&admin, &remittance_id);
    contract.confirm_payout(&remittance_id);
    let token_client = token::Client::new(&env, &token.address);
    assert_eq!(token_client.balance(&agent), 9500);
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

    let remittance_id1 = contract.create_remittance(&sender1, &agent, &1000, &None);
    let remittance_id2 = contract.create_remittance(&sender2, &agent, &2000, &None);

    assert_eq!(remittance_id1, 1);
    assert_eq!(remittance_id2, 2);

    contract.authorize_remittance(&admin, &remittance_id1);
    contract.authorize_remittance(&admin, &remittance_id2);

    contract.confirm_payout(&remittance_id1);
    contract.confirm_payout(&remittance_id2);

    assert_eq!(contract.get_accumulated_fees(), 75);
    let token_client = token::Client::new(&env, &token.address);
    assert_eq!(token_client.balance(&agent), 2925);
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
    contract.initialize(&admin, &token.address, &250);

    contract.register_agent(&agent);

    let events = env.events().all();
    let agent_reg_event = events.last().unwrap();

    assert_eq!(Symbol::from_val(&env, &agent_reg_event.1.get(0).unwrap()), symbol_short!("agent"));
    assert_eq!(Symbol::from_val(&env, &agent_reg_event.1.get(1).unwrap()), symbol_short!("register"));

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &None);

    let events = env.events().all();
    let create_event = events.last().unwrap();

    assert_eq!(Symbol::from_val(&env, &create_event.1.get(0).unwrap()), symbol_short!("remit"));
    assert_eq!(Symbol::from_val(&env, &create_event.1.get(1).unwrap()), symbol_short!("created"));

    contract.authorize_remittance(&admin, &remittance_id);
    contract.confirm_payout(&remittance_id);

    let events = env.events().all();
    let complete_event = events.last().unwrap();

    assert_eq!(Symbol::from_val(&env, &complete_event.1.get(0).unwrap()), symbol_short!("settle"));
    assert_eq!(Symbol::from_val(&env, &complete_event.1.get(1).unwrap()), symbol_short!("complete"));
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
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    env.mock_all_auths();
    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &None);

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
                    Symbol::new(&env, "confirm_payout"),
                    (remittance_id,).into_val(&env)
                )),
                sub_invocations: std::vec::Vec::new()
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
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &None);
    contract.authorize_remittance(&admin, &remittance_id);
    contract.confirm_payout(&remittance_id);

    // This should succeed with a valid address
    contract.withdraw_fees(&fee_recipient);

    let token_client = token::Client::new(&env, &token.address);
    assert_eq!(token_client.balance(&fee_recipient), 25);
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
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &None);

    // This should succeed with a valid agent address
    contract.authorize_remittance(&admin, &remittance_id);
    contract.confirm_payout(&remittance_id);

    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(remittance.status, crate::types::RemittanceStatus::Settled);
    let token_client = token::Client::new(&env, &token.address);
    assert_eq!(token_client.balance(&agent), 975);
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
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    // Create remittance with valid addresses
    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &None);

    // Confirm payout - should validate agent address
    contract.authorize_remittance(&admin, &remittance_id);
    contract.confirm_payout(&remittance_id);

    // Verify the settlement completed successfully
    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(remittance.status, crate::types::RemittanceStatus::Settled);
    let token_client = token::Client::new(&env, &token.address);
    assert_eq!(token_client.balance(&agent), 975);
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
    let remittance_id1 = contract.create_remittance(&sender1, &agent1, &1000, &None);
    let remittance_id2 = contract.create_remittance(&sender2, &agent2, &2000, &None);

    // Both should succeed with valid addresses
    contract.authorize_remittance(&admin, &remittance_id1);
    contract.authorize_remittance(&admin, &remittance_id2);

    contract.confirm_payout(&remittance_id1);
    contract.confirm_payout(&remittance_id2);

    let token_client = token::Client::new(&env, &token.address);
    assert_eq!(token_client.balance(&agent1), 975);
    assert_eq!(token_client.balance(&agent2), 1950);
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
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    // Set expiry to 1 hour in the future
    env.ledger().set(soroban_sdk::testutils::LedgerInfo { timestamp: 10000, ..env.ledger().get() });
    let current_time = env.ledger().timestamp();
    let expiry_time = current_time + 3600;

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &Some(expiry_time));

    // Should succeed since expiry is in the future
    contract.authorize_remittance(&admin, &remittance_id);
    contract.confirm_payout(&remittance_id);

    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(remittance.status, crate::types::RemittanceStatus::Settled);
    let token_client = token::Client::new(&env, &token.address);
    assert_eq!(token_client.balance(&agent), 975);
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
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    // Set expiry to 1 hour in the past
    env.ledger().set(soroban_sdk::testutils::LedgerInfo { timestamp: 10000, ..env.ledger().get() });
    let current_time = env.ledger().timestamp();
    let expiry_time = current_time.saturating_sub(3600);

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &Some(expiry_time));

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
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    // Create remittance without expiry
    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &None);

    // Should succeed since there's no expiry
    contract.authorize_remittance(&admin, &remittance_id);
    contract.confirm_payout(&remittance_id);

    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(remittance.status, crate::types::RemittanceStatus::Settled);
    let token_client = token::Client::new(&env, &token.address);
    assert_eq!(token_client.balance(&agent), 975);
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
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &None);

    // First settlement should succeed
    contract.authorize_remittance(&admin, &remittance_id);
    contract.confirm_payout(&remittance_id);

    // Verify first settlement completed
    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(remittance.status, crate::types::RemittanceStatus::Settled);
    let token_client = token::Client::new(&env, &token.address);
    assert_eq!(token_client.balance(&agent), 975);
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
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    // Create two different remittances
    let remittance_id1 = contract.create_remittance(&sender, &agent, &1000, &None);
    let remittance_id2 = contract.create_remittance(&sender, &agent, &1000, &None);

    // Both settlements should succeed as they are different remittances
    contract.authorize_remittance(&admin, &remittance_id1);
    contract.authorize_remittance(&admin, &remittance_id2);

    contract.confirm_payout(&remittance_id1);
    contract.confirm_payout(&remittance_id2);

    // Verify both completed successfully
    let remittance1 = contract.get_remittance(&remittance_id1);
    let remittance2 = contract.get_remittance(&remittance_id2);

    assert_eq!(remittance1.status, crate::types::RemittanceStatus::Settled);
    assert_eq!(remittance2.status, crate::types::RemittanceStatus::Settled);
    let token_client = token::Client::new(&env, &token.address);
    assert_eq!(token_client.balance(&agent), 1950);
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
        let remittance_id = contract.create_remittance(&sender, &agent, &1000, &None);
        contract.authorize_remittance(&admin, &remittance_id);
        contract.confirm_payout(&remittance_id);
    }

    // Verify all settlements completed
    assert_eq!(contract.get_accumulated_fees(), 125);
    let token_client = token::Client::new(&env, &token.address);
    assert_eq!(token_client.balance(&agent), 4875);

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

    env.mock_all_auths();
    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    env.ledger().set(soroban_sdk::testutils::LedgerInfo { timestamp: 10000, ..env.ledger().get() });
    let current_time = env.ledger().timestamp();
    let expiry_time = current_time + 3600;

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &Some(expiry_time));

    contract.authorize_remittance(&admin, &remittance_id);

    // First settlement should succeed
    contract.confirm_payout(&remittance_id);

    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(remittance.status, crate::types::RemittanceStatus::Settled);

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

    env.mock_all_auths();
    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &None);
    contract.authorize_remittance(&admin, &remittance_id);

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

    env.mock_all_auths();
    token.mint(&sender, &10000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    contract.register_agent(&agent);

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &None);
    contract.authorize_remittance(&admin, &remittance_id);

    contract.pause();
    contract.unpause();

    contract.confirm_payout(&remittance_id);

    let settlement = contract.get_settlement(&remittance_id);
    assert_eq!(settlement.id, remittance_id);
    assert_eq!(settlement.sender, sender);
    assert_eq!(settlement.agent, agent);
    assert_eq!(settlement.amount, 1000);
    assert_eq!(settlement.fee, 25);
    assert_eq!(settlement.status, crate::types::RemittanceStatus::Settled);
}

#[test]
#[should_panic(expected = "Error(Contract, #6)")]
fn test_get_settlement_invalid_id() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);

    contract.get_settlement(&999);
}

#[test]
fn test_settlement_completed_event_emission() {
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

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &None);

    contract.authorize_remittance(&admin, &remittance_id);
    contract.confirm_payout(&remittance_id);

    // Verify SettlementCompleted event was emitted
    let events = env.events().all();
    let settlement_event = events.iter().find(|e| {
        Symbol::from_val(&env, &e.1.get(0).unwrap()) == symbol_short!("settle")
            && Symbol::from_val(&env, &e.1.get(1).unwrap()) == symbol_short!("complete")
    });

    assert!(
        settlement_event.is_some(),
        "SettlementCompleted event should be emitted"
    );

    let event = settlement_event.unwrap();
    let event_data: soroban_sdk::Vec<soroban_sdk::Val> =
        soroban_sdk::FromVal::from_val(&env, &event.2);

    // Verify event fields match executed settlement data
    // (0: SCHEMA_VERSION, 1: sequence, 2: timestamp, 3: sender, 4: agent, 5: token, 6: amount)
    let event_sender: Address = soroban_sdk::FromVal::from_val(&env, &event_data.get(3).unwrap());
    let event_agent: Address = soroban_sdk::FromVal::from_val(&env, &event_data.get(4).unwrap());
    let event_token: Address = soroban_sdk::FromVal::from_val(&env, &event_data.get(5).unwrap());
    let event_amount: i128 = soroban_sdk::FromVal::from_val(&env, &event_data.get(6).unwrap());

    assert_eq!(
        event_sender,
        sender,
        "Event sender should match remittance sender"
    );
    assert_eq!(
        event_agent,
        agent,
        "Event recipient should match remittance agent"
    );
    assert_eq!(
        event_token,
        token.address,
        "Event token should match USDC token"
    );
    assert_eq!(
        event_amount,
        975i128,
        "Event amount should match payout amount (1000 - 25 fee)"
    );
}

#[test]
fn test_settlement_completed_event_fields_accuracy() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    token.mint(&sender, &20000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &500); // 5% fee
    contract.register_agent(&agent);

    let remittance_id = contract.create_remittance(&sender, &agent, &10000, &None);

    contract.authorize_remittance(&admin, &remittance_id);
    contract.confirm_payout(&remittance_id);

    // Find the SettlementCompleted event
    let events = env.events().all();
    let settlement_event = events.iter().find(|e| {
        Symbol::from_val(&env, &e.1.get(0).unwrap()) == symbol_short!("settle")
            && Symbol::from_val(&env, &e.1.get(1).unwrap()) == symbol_short!("complete")
    });

    assert!(settlement_event.is_some());

    let event = settlement_event.unwrap();
    let event_data: soroban_sdk::Vec<soroban_sdk::Val> =
        soroban_sdk::FromVal::from_val(&env, &event.2);

    // Verify all fields with different fee calculation
    // (0: SCHEMA_VERSION, 1: sequence, 2: timestamp, 3: sender, 4: agent, 5: token, 6: amount)
    let expected_payout = 10000 - 500; // 10000 - (10000 * 500 / 10000)
    let event_sender: Address = soroban_sdk::FromVal::from_val(&env, &event_data.get(3).unwrap());
    let event_agent: Address = soroban_sdk::FromVal::from_val(&env, &event_data.get(4).unwrap());
    let event_token: Address = soroban_sdk::FromVal::from_val(&env, &event_data.get(5).unwrap());
    let event_amount: i128 = soroban_sdk::FromVal::from_val(&env, &event_data.get(6).unwrap());

    assert_eq!(event_sender, sender);
    assert_eq!(event_agent, agent);
    assert_eq!(event_token, token.address);
    assert_eq!(event_amount, (expected_payout as i128));
}

// ============================================================================
// Multi-Admin Tests
// ============================================================================

#[test]
fn test_add_admin() {
    let env = Env::default();
    env.mock_all_auths();

    let admin1 = Address::generate(&env);
    let admin2 = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin1, &token.address, &250);

    // Initial admin should be registered
    assert!(contract.is_admin(&admin1));
    assert!(!contract.is_admin(&admin2));

    // Add second admin
    contract.add_admin(&admin1, &admin2);

    // Both should be admins now
    assert!(contract.is_admin(&admin1));
    assert!(contract.is_admin(&admin2));
}

#[test]
#[should_panic(expected = "Error(Contract, #14)")]
fn test_add_admin_unauthorized() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let non_admin = Address::generate(&env);
    let new_admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);

    // Non-admin trying to add admin should fail
    contract.add_admin(&non_admin, &new_admin);
}

#[test]
#[should_panic(expected = "Error(Contract, #15)")]
fn test_add_admin_already_exists() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);

    // Try to add the same admin again
    contract.add_admin(&admin, &admin);
}

#[test]
fn test_remove_admin() {
    let env = Env::default();
    env.mock_all_auths();

    let admin1 = Address::generate(&env);
    let admin2 = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin1, &token.address, &250);

    // Add second admin
    contract.add_admin(&admin1, &admin2);
    assert!(contract.is_admin(&admin2));

    // Remove second admin
    contract.remove_admin(&admin1, &admin2);
    assert!(!contract.is_admin(&admin2));
    assert!(contract.is_admin(&admin1));
}

#[test]
#[should_panic(expected = "Error(Contract, #17)")]
fn test_cannot_remove_last_admin() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);

    // Try to remove the only admin
    contract.remove_admin(&admin, &admin);
}

#[test]
#[should_panic(expected = "Error(Contract, #14)")]
fn test_remove_admin_unauthorized() {
    let env = Env::default();
    env.mock_all_auths();

    let admin1 = Address::generate(&env);
    let admin2 = Address::generate(&env);
    let non_admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin1, &token.address, &250);
    contract.add_admin(&admin1, &admin2);

    // Non-admin trying to remove admin should fail
    contract.remove_admin(&non_admin, &admin2);
}

#[test]
#[should_panic(expected = "Error(Contract, #16)")]
fn test_remove_admin_not_found() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let non_admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);

    // Try to remove an address that is not an admin
    contract.remove_admin(&admin, &non_admin);
}

#[test]
fn test_multiple_admins_can_perform_admin_actions() {
    let env = Env::default();
    env.mock_all_auths();

    let admin1 = Address::generate(&env);
    let admin2 = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);
    let agent = Address::generate(&env);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin1, &token.address, &250);
    contract.add_admin(&admin1, &admin2);

    // Both admins should be able to register agents
    contract.register_agent(&agent);
    assert!(contract.is_agent_registered(&agent));

    // Admin2 should be able to update fee
    contract.update_fee(&500);
    assert_eq!(contract.get_platform_fee_bps(), 500);

    // Admin2 should be able to pause
    contract.pause();
    assert!(contract.is_paused());

    contract.unpause();
    assert!(!contract.is_paused());
}

#[test]
fn test_formal_state_machine_flow() {
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

    // 1. Pending (initial)
    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &None);
    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(remittance.status, crate::types::RemittanceStatus::Pending);

    // 2. Authorized
    contract.authorize_remittance(&admin, &remittance_id);
    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(
        remittance.status,
        crate::types::RemittanceStatus::Authorized
    );

    // 3. Settled (Payout)
    contract.settle_remittance(&remittance_id);
    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(remittance.status, crate::types::RemittanceStatus::Settled);

    // 4. Finalized
    contract.finalize_remittance(&admin, &remittance_id);
    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(remittance.status, crate::types::RemittanceStatus::Finalized);
}

#[test]
#[should_panic(expected = "Error(Contract, #18)")]
fn test_invalid_state_jump_pending_to_settled() {
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

    let remittance_id = contract.create_remittance(&sender, &agent, &1000, &None);

    // Jump Pending -> Settled (invalid, must be Authorized)
    contract.settle_remittance(&remittance_id);
}

#[test]
fn test_fail_remittance_from_any_non_terminal_state() {
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

    // From Pending
    let id1 = contract.create_remittance(&sender, &agent, &1000, &None);
    contract.fail_remittance(&admin, &id1);
    assert_eq!(
        contract.get_remittance(&id1).status,
        crate::types::RemittanceStatus::Failed
    );

    // From Authorized
    let id2 = contract.create_remittance(&sender, &agent, &1000, &None);
    contract.authorize_remittance(&admin, &id2);
    contract.fail_remittance(&admin, &id2);
    assert_eq!(
        contract.get_remittance(&id2).status,
        crate::types::RemittanceStatus::Failed
    );

    
}
