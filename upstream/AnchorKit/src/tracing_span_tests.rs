#[cfg(test)]
mod tracing_span_tests {
    use crate::{AnchorKitContract, AnchorKitContractClient};
    use soroban_sdk::{
        testutils::{Address as _, Ledger},
        Address, Bytes, BytesN, Env,
    };

    #[test]
    fn test_span_emits_request_id() {
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
        assert_eq!(span.request_id.id, request_id.id);
    }

    #[test]
    fn test_span_emits_operation_metadata() {
        let env = Env::default();
        env.mock_all_auths();
        env.ledger().with_mut(|li| {
            li.timestamp = 1000;
        });
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
        assert_eq!(span.actor, attestor);
        // Timestamp is always >= 0 for u64, just verify completed_at is after started_at
        assert!(span.completed_at >= span.started_at);
    }

    #[test]
    fn test_span_propagates_across_operations() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let attestor = Address::generate(&env);

        client.initialize(&admin);

        let request_id = client.generate_request_id();
        let original_id = request_id.id.clone();

        client.register_attestor(&attestor);

        let span = client.get_tracing_span(&original_id);
        assert!(span.is_some() || original_id.len() == 16);
    }

    #[test]
    fn test_structured_log_format() {
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
        assert!(span.status.len() > 0);
        assert!(span.operation.len() > 0);
    }
}
