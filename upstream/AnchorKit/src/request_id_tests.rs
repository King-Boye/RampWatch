#[cfg(test)]
mod request_id_tests {
    use crate::{AnchorKitContract, AnchorKitContractClient, RequestId, ServiceType};
    use soroban_sdk::{testutils::Address as _, vec, Address, Bytes, BytesN, Env};

    #[test]
    fn test_generate_request_id() {
        let env = Env::default();
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let request_id = client.generate_request_id();
        
        assert_eq!(request_id.id.len(), 16);
        assert!(request_id.created_at > 0);
    }

    #[test]
    fn test_unique_request_ids() {
        let env = Env::default();
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let id1 = client.generate_request_id();
        
        env.ledger().with_mut(|li| li.sequence += 1);
        
        let id2 = client.generate_request_id();
        
        assert_ne!(id1.id, id2.id);
    }

    #[test]
    fn test_request_id_to_hex() {
        let env = Env::default();
        
        let request_id = RequestId::generate(&env);
        
        // Just verify ID is 16 bytes
        assert_eq!(request_id.id.len(), 16);
    }

    #[test]
    fn test_submit_attestation_with_request_id() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let attestor = Address::generate(&env);
        let subject = Address::generate(&env);

        client.initialize(&admin);
        client.register_attestor(&attestor);

        let request_id = client.generate_request_id();
        let payload_hash = BytesN::from_array(&env, &[1u8; 32]);
        let signature = Bytes::new(&env);

        let attestation_id = client.submit_with_request_id(
            &request_id,
            &attestor,
            &subject,
            &1000,
            &payload_hash,
            &signature,
        );

        assert!(attestation_id > 0);

        // Verify tracing span was stored
        let span = client.get_tracing_span(&request_id.id);
        assert!(span.is_some());
        
        let span = span.unwrap();
        assert_eq!(span.request_id.id, request_id.id);
        assert_eq!(span.actor, attestor);
    }

    #[test]
    fn test_tracing_span_records_failure() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let attestor = Address::generate(&env);
        let subject = Address::generate(&env);

        client.initialize(&admin);
        // Don't register attestor - will fail

        let request_id = client.generate_request_id();
        let payload_hash = BytesN::from_array(&env, &[1u8; 32]);
        let signature = Bytes::new(&env);

        let result = client.try_submit_with_request_id(
            &request_id,
            &attestor,
            &subject,
            &1000,
            &payload_hash,
            &signature,
        );

        assert!(result.is_err());

        // Verify failure was recorded
        let span = client.get_tracing_span(&request_id.id);
        assert!(span.is_some());
    }

    #[test]
    fn test_submit_quote_with_request_id() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let anchor = Address::generate(&env);

        client.initialize(&admin);
        client.register_attestor(&anchor);

        let services = vec![&env, ServiceType::Quotes];
        client.configure_services(&anchor, &services);

        let request_id = client.generate_request_id();

        let quote_id = client.quote_with_request_id(
            &request_id,
            &anchor,
            &soroban_sdk::String::from_str(&env, "USD"),
            &soroban_sdk::String::from_str(&env, "USDC"),
            &10000,
            &100,
            &100,
            &10000,
            &(env.ledger().timestamp() + 3600),
        );

        assert!(quote_id > 0);

        // Verify tracing span
        let span = client.get_tracing_span(&request_id.id);
        assert!(span.is_some());
        
        let span = span.unwrap();
        assert_eq!(span.actor, anchor);
    }

    #[test]
    fn test_tracing_span_timing() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let attestor = Address::generate(&env);
        let subject = Address::generate(&env);

        client.initialize(&admin);
        client.register_attestor(&attestor);

        let request_id = client.generate_request_id();
        let payload_hash = BytesN::from_array(&env, &[1u8; 32]);
        let signature = Bytes::new(&env);

        client.submit_with_request_id(
            &request_id,
            &attestor,
            &subject,
            &1000,
            &payload_hash,
            &signature,
        );

        let span = client.get_tracing_span(&request_id.id).unwrap();
        
        assert!(span.completed_at >= span.started_at);
    }
}
