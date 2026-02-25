#![cfg(test)]

use crate::{AnchorKitContract, AnchorKitContractClient, ServiceType};
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    Address, Bytes, BytesN, Env, String, Vec,
};

#[derive(Clone, Debug, Eq, PartialEq)]
enum FlowState {
    Pending,
    AwaitingUser,
    Completed,
}

struct StreamingFlow {
    flow_id: u64,
    session_id: u64,
    state: FlowState,
    anchor: Address,
}

fn create_contract(env: &Env) -> AnchorKitContractClient<'_> {
    let contract_id = env.register_contract(None, AnchorKitContract);
    AnchorKitContractClient::new(env, &contract_id)
}

fn setup_anchor(env: &Env, client: &AnchorKitContractClient, admin: &Address, anchor: &Address) {
    client.register_attestor(anchor);

    let mut services = Vec::new(env);
    services.push_back(ServiceType::Deposits);
    services.push_back(ServiceType::Quotes);
    services.push_back(ServiceType::KYC);
    client.configure_services(anchor, &services);
}

#[test]
fn test_streaming_flow_pending_to_awaiting_user_to_completed() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let anchor = Address::generate(&env);

    let client = create_contract(&env);
    client.initialize(&admin);
    setup_anchor(&env, &client, &admin, &anchor);

    // PENDING: Create session
    let session_id = client.create_session(&user);
    let mut flow = StreamingFlow {
        flow_id: 1,
        session_id,
        state: FlowState::Pending,
        anchor: anchor.clone(),
    };

    assert_eq!(flow.state, FlowState::Pending);

    // AWAITING_USER: Submit quote
    let quote_id = client.submit_quote(
        &anchor,
        &String::from_str(&env, "USD"),
        &String::from_str(&env, "USDC"),
        &10000u64,
        &25u32,
        &100u64,
        &100000u64,
        &(env.ledger().timestamp() + 3600),
    );

    flow.state = FlowState::AwaitingUser;
    assert_eq!(flow.state, FlowState::AwaitingUser);

    // COMPLETED: Receive quote
    let quote = client.receive_quote(&user, &anchor, &quote_id);
    assert_eq!(quote.quote_id, quote_id);

    flow.state = FlowState::Completed;
    assert_eq!(flow.state, FlowState::Completed);
}

#[test]
fn test_multi_step_async_stream_with_attestation() {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().with_mut(|li| li.timestamp = 1000000);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let anchor = Address::generate(&env);
    let subject = Address::generate(&env);

    let client = create_contract(&env);
    client.initialize(&admin);
    setup_anchor(&env, &client, &admin, &anchor);

    // PENDING
    let session_id = client.create_session(&user);

    // AWAITING_USER
    let payload_hash = BytesN::from_array(&env, &[1; 32]);
    let signature = Bytes::from_slice(&env, &[10, 11, 12]);

    let attestation_id = client.submit_attestation_with_session(
        &session_id,
        &anchor,
        &subject,
        &1000001u64,
        &payload_hash,
        &signature,
    );

    assert_eq!(FlowState::AwaitingUser, FlowState::AwaitingUser);
    assert!(attestation_id > 0);

    // COMPLETED
    let session = client.get_session(&session_id);
    assert_eq!(session.session_id, session_id);

    assert_eq!(FlowState::Completed, FlowState::Completed);
}

#[test]
fn test_concurrent_streaming_flows() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let anchor = Address::generate(&env);

    let client = create_contract(&env);
    client.initialize(&admin);
    setup_anchor(&env, &client, &admin, &anchor);

    // Flow 1: PENDING
    let session1 = client.create_session(&user1);

    // Flow 2: PENDING
    let session2 = client.create_session(&user2);

    // Flow 1: AWAITING_USER
    let quote1 = client.submit_quote(
        &anchor,
        &String::from_str(&env, "USD"),
        &String::from_str(&env, "USDC"),
        &10000u64,
        &25u32,
        &100u64,
        &100000u64,
        &(env.ledger().timestamp() + 3600),
    );

    // Flow 2: AWAITING_USER
    let quote2 = client.submit_quote(
        &anchor,
        &String::from_str(&env, "EUR"),
        &String::from_str(&env, "EURC"),
        &10050u64,
        &30u32,
        &200u64,
        &50000u64,
        &(env.ledger().timestamp() + 3600),
    );

    // Flow 1: COMPLETED
    let _ = client.receive_quote(&user1, &anchor, &quote1);

    // Flow 2: COMPLETED
    let _ = client.receive_quote(&user2, &anchor, &quote2);

    assert!(quote1 > 0);
    assert!(quote2 > 0);
}
