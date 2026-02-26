#![cfg(test)]
use crate::{SwiftRemitContract, SwiftRemitContractClient, Escrow, EscrowStatus};
use soroban_sdk::{
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    token, Address, Env, IntoVal, Symbol,
};

fn create_token_contract<'a>(env: &Env, admin: &Address) -> token::StellarAssetClient<'a> {
    token::StellarAssetClient::new(env, &env.register_stellar_asset_contract_v2(admin.clone()).address())
}

fn create_swiftremit_contract<'a>(env: &Env) -> SwiftRemitContractClient<'a> {
    SwiftRemitContractClient::new(env, &env.register(SwiftRemitContract, ()))
}

#[test]
fn test_create_escrow() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);

    let token = create_token_contract(&env, &admin);
    token.mint(&sender, &1000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250, &3600);

    let transfer_id = contract.create_escrow(&sender, &recipient, &500);

    assert_eq!(transfer_id, 1);
    assert_eq!(token.balance(&sender), 500);
    assert_eq!(token.balance(&contract.address), 500);

    let escrow = contract.get_escrow(&transfer_id);
    assert_eq!(escrow.sender, sender);
    assert_eq!(escrow.recipient, recipient);
    assert_eq!(escrow.amount, 500);
    assert_eq!(escrow.status, EscrowStatus::Pending);
}

#[test]
fn test_release_escrow() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);

    let token = create_token_contract(&env, &admin);
    token.mint(&sender, &1000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250, &3600);

    let transfer_id = contract.create_escrow(&sender, &recipient, &500);
    contract.release_escrow(&transfer_id);

    let escrow = contract.get_escrow(&transfer_id);
    assert_eq!(escrow.status, EscrowStatus::Released);
    assert_eq!(token.balance(&recipient), 500);
    assert_eq!(token.balance(&contract.address), 0);
}

#[test]
fn test_refund_escrow() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);

    let token = create_token_contract(&env, &admin);
    token.mint(&sender, &1000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250, &3600);

    let transfer_id = contract.create_escrow(&sender, &recipient, &500);
    contract.refund_escrow(&transfer_id);

    let escrow = contract.get_escrow(&transfer_id);
    assert_eq!(escrow.status, EscrowStatus::Refunded);
    assert_eq!(token.balance(&sender), 1000);
    assert_eq!(token.balance(&contract.address), 0);
}

#[test]
#[should_panic(expected = "InvalidEscrowStatus")]
fn test_double_release_prevented() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);

    let token = create_token_contract(&env, &admin);
    token.mint(&sender, &1000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250, &3600);

    let transfer_id = contract.create_escrow(&sender, &recipient, &500);
    contract.release_escrow(&transfer_id);
    contract.release_escrow(&transfer_id); // Should panic
}

#[test]
#[should_panic(expected = "InvalidEscrowStatus")]
fn test_double_refund_prevented() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);

    let token = create_token_contract(&env, &admin);
    token.mint(&sender, &1000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250, &3600);

    let transfer_id = contract.create_escrow(&sender, &recipient, &500);
    contract.refund_escrow(&transfer_id);
    contract.refund_escrow(&transfer_id); // Should panic
}

#[test]
#[should_panic(expected = "InvalidAmount")]
fn test_create_escrow_zero_amount() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);

    let token = create_token_contract(&env, &admin);
    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250, &3600);

    contract.create_escrow(&sender, &recipient, &0);
}

#[test]
fn test_escrow_events_emitted() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);

    let token = create_token_contract(&env, &admin);
    token.mint(&sender, &1000);

    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250, &3600);

    let transfer_id = contract.create_escrow(&sender, &recipient, &500);
    
    let events = env.events().all();
    let create_event = events.iter().find(|e| {
        e.topics.get(0).unwrap() == &Symbol::new(&env, "escrow").into_val(&env)
            && e.topics.get(1).unwrap() == &Symbol::new(&env, "created").into_val(&env)
    });
    assert!(create_event.is_some());

    contract.release_escrow(&transfer_id);
    
    let events = env.events().all();
    let release_event = events.iter().find(|e| {
        e.topics.get(0).unwrap() == &Symbol::new(&env, "escrow").into_val(&env)
            && e.topics.get(1).unwrap() == &Symbol::new(&env, "released").into_val(&env)
    });
    assert!(release_event.is_some());
}
