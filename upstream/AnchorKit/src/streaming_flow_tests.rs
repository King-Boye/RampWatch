/// Polling-based state update flow tests.
///
/// Soroban contracts are synchronous — there is no streaming API. Clients
/// observe state changes by polling contract storage after each transaction.
/// These tests verify that multi-step anchor flows produce the expected
/// on-chain state at each polling point.
#[cfg(test)]
mod streaming_flow_tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Bytes, BytesN, Env, String, Vec};

    fn setup(env: &Env) -> (AnchorKitContractClient<'_>, Address, Address) {
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(env, &contract_id);
        let admin = Address::generate(env);
        let anchor = Address::generate(env);
        client.initialize(&admin);
        client.register_attestor(&anchor);
        (client, admin, anchor)
    }

    /// Poll 1: session created → operation_count == 0
    /// Poll 2: after register_attestor_with_session → operation_count == 1
    /// Poll 3: after revoke_attestor_with_session → operation_count == 2
    #[test]
    fn test_session_operation_count_increments_on_each_step() {
        let env = Env::default();
        env.mock_all_auths();
        let (client, _admin, _anchor) = setup(&env);

        let initiator = Address::generate(&env);
        let session_id = client.create_session(&initiator);

        // Poll 1: no operations yet
        assert_eq!(client.get_session_operation_count(&session_id), 0);

        let new_attestor = Address::generate(&env);
        client.register_attestor_with_session(&session_id, &new_attestor);

        // Poll 2: one operation logged
        assert_eq!(client.get_session_operation_count(&session_id), 1);

        client.revoke_attestor_with_session(&session_id, &new_attestor);

        // Poll 3: two operations logged
        assert_eq!(client.get_session_operation_count(&session_id), 2);
    }

    /// Verifies that audit log entries reflect the correct actor and status
    /// for a successful attestation submitted within a session.
    #[test]
    fn test_audit_log_reflects_attestation_state() {
        let env = Env::default();
        env.mock_all_auths();
        let (client, _admin, anchor) = setup(&env);

        let mut services = Vec::new(&env);
        services.push_back(ServiceType::Deposits);
        client.configure_services(&anchor, &services);

        let session_id = client.create_session(&anchor);

        let subject = Address::generate(&env);
        let payload_hash = BytesN::from_array(&env, &[1u8; 32]);
        let signature = Bytes::from_array(&env, &[0u8; 64]);
        let timestamp = env.ledger().timestamp() + 1;

        client.submit_attestation_with_session(
            &session_id,
            &anchor,
            &subject,
            &timestamp,
            &payload_hash,
            &signature,
        );

        // Poll: audit log entry 1 should record a successful attestation
        let log = client.get_audit_log(&1);
        assert_eq!(log.session_id, session_id);
        assert_eq!(log.operation.operation_type, String::from_str(&env, "attest"));
        assert_eq!(log.operation.status, String::from_str(&env, "success"));
        assert_eq!(log.actor, anchor);
    }

    /// Verifies that a failed operation (replay attack) is recorded in the
    /// audit log with status "failed" before the error propagates.
    #[test]
    fn test_audit_log_records_failed_operation() {
        let env = Env::default();
        env.mock_all_auths();
        let (client, _admin, anchor) = setup(&env);

        let mut services = Vec::new(&env);
        services.push_back(ServiceType::Deposits);
        client.configure_services(&anchor, &services);

        let session_id = client.create_session(&anchor);

        let subject = Address::generate(&env);
        let payload_hash = BytesN::from_array(&env, &[2u8; 32]);
        let signature = Bytes::from_array(&env, &[0u8; 64]);
        let timestamp = env.ledger().timestamp() + 1;

        // First submission succeeds
        client.submit_attestation_with_session(
            &session_id,
            &anchor,
            &subject,
            &timestamp,
            &payload_hash,
            &signature,
        );

        // Second submission with same hash is a replay — should fail
        let result = client.try_submit_attestation_with_session(
            &session_id,
            &anchor,
            &subject,
            &timestamp,
            &payload_hash,
            &signature,
        );
        assert!(result.is_err());

        // Poll: operation count reflects both attempts (success + failure)
        assert_eq!(client.get_session_operation_count(&session_id), 2);

        // The second audit log entry should be "failed"
        let failed_log = client.get_audit_log(&2);
        assert_eq!(failed_log.operation.status, String::from_str(&env, "failed"));
    }

    /// Simulates a client polling session state across a full deposit flow:
    /// create session → submit quote → build intent → verify final state.
    #[test]
    fn test_full_deposit_flow_state_visible_via_polling() {
        let env = Env::default();
        env.mock_all_auths();
        let (client, _admin, anchor) = setup(&env);

        let mut services = Vec::new(&env);
        services.push_back(ServiceType::Deposits);
        services.push_back(ServiceType::Quotes);
        client.configure_services(&anchor, &services);

        let initiator = Address::generate(&env);
        let session_id = client.create_session(&initiator);

        // Poll: session exists with zero operations
        let session = client.get_session(&session_id);
        assert_eq!(session.session_id, session_id);
        assert_eq!(session.operation_count, 0);

        // Step 1: anchor submits a quote
        let base = String::from_str(&env, "USD");
        let quote_asset_str = String::from_str(&env, "USDC");
        let valid_until = env.ledger().timestamp() + 600;
        let quote_id = client.submit_quote(
            &anchor,
            &base,
            &quote_asset_str,
            &10000u64,
            &25u32,
            &100_000000u64,
            &10_000_000000u64,
            &valid_until,
        );

        // Poll: quote is readable on-chain
        let quote = client.get_quote(&anchor, &quote_id);
        assert_eq!(quote.rate, 10000u64);

        // Step 2: build a transaction intent tied to the session and quote
        let request = QuoteRequest {
            base_asset: base.clone(),
            quote_asset: quote_asset_str.clone(),
            amount: 500_000000u64,
            operation_type: ServiceType::Deposits,
        };
        let builder = TransactionIntentBuilder::new(&env, anchor.clone(), request)
            .with_quote_id(quote_id)
            .with_session(session_id)
            .with_ttl(300);

        let intent = client.build_transaction_intent(&builder);

        // Poll: intent reflects the quote and session
        assert_eq!(intent.session_id, session_id);
        assert_eq!(intent.quote_id, quote_id);
        assert!(intent.has_quote);
        assert_eq!(intent.rate, 10000u64);

        // Poll: session now has one logged operation (the intent)
        assert_eq!(client.get_session_operation_count(&session_id), 1);
    }
}
