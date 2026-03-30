#![cfg(test)]

mod streaming_flow_tests {
    use soroban_sdk::{
        testutils::{Address as _, Ledger, LedgerInfo},
        Address, Bytes, Env, String,
    };

    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;

    use crate::contract::{AnchorKitContract, AnchorKitContractClient};
    use crate::sep10_test_util::register_attestor_with_sep10;

    fn make_env() -> Env {
        let env = Env::default();
        env.mock_all_auths();
        env
    }

    fn set_ts(env: &Env, ts: u64) {
        env.ledger().set(LedgerInfo {
            timestamp: ts,
            protocol_version: 21,
            sequence_number: 0,
            network_id: Default::default(),
            base_reserve: 0,
            min_persistent_entry_ttl: 4096,
            min_temp_entry_ttl: 16,
            max_entry_ttl: 6312000,
        });
    }

    #[test]
    fn test_streaming_flow_pending_to_awaiting_user_to_completed() {
        let env = make_env();
        set_ts(&env, 0);
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let anchor = Address::generate(&env);
        let user = Address::generate(&env);

        client.initialize(&admin);
        let sk = SigningKey::generate(&mut OsRng);
        register_attestor_with_sep10(&env, &client, &anchor, &anchor, &sk);

        let mut services = soroban_sdk::Vec::new(&env);
        services.push_back(1u32);
        services.push_back(3u32);
        services.push_back(4u32);
        client.configure_services(&anchor, &services);

        let session_id = client.create_session(&user);
        assert_eq!(session_id, 0);

        let quote_id = client.submit_quote(
            &anchor,
            &String::from_str(&env, "USD"),
            &String::from_str(&env, "USDC"),
            &10000u64,
            &25u32,
            &100u64,
            &100000u64,
            &3600u64,
        );
        assert_eq!(quote_id, 1);

        let quote = client.receive_quote(&user, &anchor, &quote_id);
        assert_eq!(quote.quote_id, 1);
        assert_eq!(quote.base_asset, String::from_str(&env, "USD"));
        assert_eq!(quote.fee_percentage, 25);
    }

    #[test]
    fn test_multi_step_async_stream_with_attestation() {
        let env = make_env();
        set_ts(&env, 1_000_000);
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let attestor = Address::generate(&env);
        let subject = Address::generate(&env);
        let user = Address::generate(&env);

        client.initialize(&admin);
        let sk = SigningKey::generate(&mut OsRng);
        register_attestor_with_sep10(&env, &client, &attestor, &attestor, &sk);

        let mut services = soroban_sdk::Vec::new(&env);
        services.push_back(1u32);
        services.push_back(3u32);
        services.push_back(4u32);
        client.configure_services(&attestor, &services);

        let session_id = client.create_session(&user);
        assert_eq!(session_id, 0);

        let mut payload = Bytes::new(&env);
        for _ in 0..32 { payload.push_back(0x01); }
        let mut sig = Bytes::new(&env);
        sig.push_back(0x0a); sig.push_back(0x0b); sig.push_back(0x0c);

        let attest_id = client.submit_attestation_with_session(
            &session_id,
            &attestor,
            &subject,
            &1_000_001u64,
            &payload,
            &sig,
        );
        assert_eq!(attest_id, 0);

        let op_count = client.get_session_operation_count(&session_id);
        assert_eq!(op_count, 1);

        let log = client.get_audit_log(&0u64);
        assert_eq!(log.session_id, 0);
        assert_eq!(log.operation.operation_type, String::from_str(&env, "attest"));
        assert_eq!(log.operation.status, String::from_str(&env, "success"));
    }

    #[test]
    fn test_concurrent_streaming_flows() {
        let env = make_env();
        set_ts(&env, 0);
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let anchor = Address::generate(&env);
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);

        client.initialize(&admin);
        let sk = SigningKey::generate(&mut OsRng);
        register_attestor_with_sep10(&env, &client, &anchor, &anchor, &sk);

        let mut services = soroban_sdk::Vec::new(&env);
        services.push_back(1u32);
        services.push_back(3u32);
        services.push_back(4u32);
        client.configure_services(&anchor, &services);

        // Two concurrent sessions
        let s1 = client.create_session(&user1);
        let s2 = client.create_session(&user2);
        assert_eq!(s1, 0);
        assert_eq!(s2, 1);

        // Two concurrent quotes
        let q1 = client.submit_quote(
            &anchor,
            &String::from_str(&env, "USD"),
            &String::from_str(&env, "USDC"),
            &10000u64, &25u32, &100u64, &100000u64, &3600u64,
        );
        let q2 = client.submit_quote(
            &anchor,
            &String::from_str(&env, "EUR"),
            &String::from_str(&env, "EURC"),
            &10050u64, &30u32, &200u64, &50000u64, &3600u64,
        );
        assert_eq!(q1, 1);
        assert_eq!(q2, 2);

        // Each user receives their own quote independently
        let r1 = client.receive_quote(&user1, &anchor, &q1);
        let r2 = client.receive_quote(&user2, &anchor, &q2);

        assert_eq!(r1.base_asset, String::from_str(&env, "USD"));
        assert_eq!(r2.base_asset, String::from_str(&env, "EUR"));

        // Sessions are isolated
        let sess1 = client.get_session(&s1);
        let sess2 = client.get_session(&s2);
        assert_eq!(sess1.initiator, user1);
        assert_eq!(sess2.initiator, user2);
    }
}
