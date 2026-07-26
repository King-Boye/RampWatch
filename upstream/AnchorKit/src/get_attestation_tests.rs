#![cfg(test)]

mod get_attestation_tests {
    use soroban_sdk::{
        testutils::{Address as _, Ledger, LedgerInfo},
        Address, Bytes, Env,
    };
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;

    use crate::contract::{AnchorKitContract, AnchorKitContractClient};
    use crate::sep10_test_util::{register_attestor_with_sep10, sign_payload};

    fn make_env() -> Env {
        let env = Env::default();
        env.mock_all_auths();
        env
    }

    fn setup_ledger(env: &Env) {
        env.ledger().set(LedgerInfo {
            timestamp: 1700000000,
            protocol_version: 21,
            sequence_number: 100,
            network_id: Default::default(),
            base_reserve: 100,
            min_persistent_entry_ttl: 4096,
            min_temp_entry_ttl: 16,
            max_entry_ttl: 6312000,
        });
    }

    fn payload(env: &Env, byte: u8) -> Bytes {
        let mut b = Bytes::new(env);
        for _ in 0..32 {
            b.push_back(byte);
        }
        b
    }

    #[test]
    fn returns_none_for_missing_id() {
        let env = make_env();
        setup_ledger(&env);
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        client.initialize(&admin, &100_u64, &None, &None);

        assert!(client.get_attestation(&999).is_none());
    }

    #[test]
    fn returns_some_for_existing_id() {
        let env = make_env();
        setup_ledger(&env);
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let attestor = Address::generate(&env);
        let subject = Address::generate(&env);
        client.initialize(&admin, &100_u64, &None, &None);

        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        register_attestor_with_sep10(&env, &client, &attestor, &attestor, &signing_key);

        let p = payload(&env, 0xAB);
        let id = client.submit_attestation(&attestor, &subject, &1700000000u64, &p, &sign_payload(&env, &signing_key, &p), &None::<soroban_sdk::Map<soroban_sdk::String, soroban_sdk::String>>);

        let result = client.get_attestation(&id);
        assert!(result.is_some());
        assert_eq!(result.unwrap().id, id);
    }

    /// Regression test: revoke an attestor, re-register them, submit a new
    /// attestation, and confirm `issuer_revoked` is false on the new record.
    /// Before the fix, the persistent `AttestorRevoked` marker was never
    /// cleared on re-registration, so every subsequent attestation would
    /// surface with `issuer_revoked = true`.
    #[test]
    fn reregistered_attestor_new_attestation_not_flagged_revoked() {
        let env = make_env();
        setup_ledger(&env);
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let attestor = Address::generate(&env);
        let subject = Address::generate(&env);
        client.initialize(&admin, &100_u64, &None, &None);

        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);

        // 1. First registration
        register_attestor_with_sep10(&env, &client, &attestor, &attestor, &signing_key);

        // 2. Revoke the attestor (sets AttestorRevoked marker)
        client.revoke_attestor(&attestor);
        assert!(!client.is_attestor(&attestor));

        // 3. Re-register the same attestor (must clear the revocation marker)
        register_attestor_with_sep10(&env, &client, &attestor, &attestor, &signing_key);
        assert!(client.is_attestor(&attestor));

        // 4. Submit a brand-new attestation after re-registration
        let p = payload(&env, 0xCD);
        let id = client.submit_attestation(
            &attestor,
            &subject,
            &1700000000u64,
            &p,
            &sign_payload(&env, &signing_key, &p),
            &None::<soroban_sdk::Map<soroban_sdk::String, soroban_sdk::String>>,
        );

        // 5. The new attestation must NOT be flagged as issuer_revoked
        let attestation = client.get_attestation(&id).expect("attestation should exist");
        assert!(
            !attestation.issuer_revoked,
            "re-registered attestor's new attestation should not be flagged issuer_revoked"
        );

        // 6. is_attestation_valid must also return true
        assert!(
            client.is_attestation_valid(&id),
            "is_attestation_valid should be true for a re-registered attestor's new attestation"
        );
    }
}
