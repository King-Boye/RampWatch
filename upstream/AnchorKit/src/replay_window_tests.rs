#![cfg(test)]

mod replay_window_tests {
    use soroban_sdk::{
        testutils::{Address as _, Ledger, LedgerInfo},
        Address, Bytes, Env,
    };

    use crate::contract::{AnchorKitContract, AnchorKitContractClient};
    use crate::errors::ErrorCode;

    fn make_env() -> Env {
        let env = Env::default();
        env.mock_all_auths();
        env
    }

    fn set_ts(env: &Env, ts: u64) {
        env.ledger().set(LedgerInfo {
            timestamp: ts,
            protocol_version: 20,
            sequence_number: 1,
            network_id: Default::default(),
            base_reserve: 10,
            min_temp_entry_ttl: 16,
            min_persistent_entry_ttl: 4096,
            max_entry_ttl: 6_312_000,
        });
    }

    fn setup(env: &Env) -> (AnchorKitContractClient, Address, Address) {
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(env, &contract_id);
        let admin = Address::generate(env);
        let attestor = Address::generate(env);
        (client, admin, attestor)
    }

    fn dummy_hash(env: &Env, seed: u8) -> Bytes {
        Bytes::from_slice(env, &[seed; 32])
    }

    // -----------------------------------------------------------------------
    // Default window (300 s)
    // -----------------------------------------------------------------------

    #[test]
    fn default_window_accepts_timestamp_within_300s() {
        let env = make_env();
        set_ts(&env, 1_000_000);
        let (client, admin, attestor) = setup(&env);

        // Initialize with no custom window → defaults to 300 s
        client.initialize(&admin, &None);
        client.register_attestor(
            &attestor,
            &soroban_sdk::String::from_str(&env, "mock"),
            &Address::generate(&env),
        );

        // timestamp = now - 299 s → inside window
        let ts: u64 = 1_000_000 - 299;
        let id = client.submit_attestation(
            &attestor,
            &Address::generate(&env),
            &ts,
            &dummy_hash(&env, 1),
            &Bytes::from_slice(&env, &[0u8; 64]),
        );
        assert_eq!(id, 0);
    }

    #[test]
    #[should_panic]
    fn default_window_rejects_timestamp_outside_300s() {
        let env = make_env();
        set_ts(&env, 1_000_000);
        let (client, admin, attestor) = setup(&env);

        client.initialize(&admin, &None);
        client.register_attestor(
            &attestor,
            &soroban_sdk::String::from_str(&env, "mock"),
            &Address::generate(&env),
        );

        // timestamp = now - 301 s → outside default 300 s window
        let ts: u64 = 1_000_000 - 301;
        client.submit_attestation(
            &attestor,
            &Address::generate(&env),
            &ts,
            &dummy_hash(&env, 2),
            &Bytes::from_slice(&env, &[0u8; 64]),
        );
    }

    // -----------------------------------------------------------------------
    // Custom window
    // -----------------------------------------------------------------------

    #[test]
    fn custom_window_accepts_timestamp_within_custom_range() {
        let env = make_env();
        set_ts(&env, 1_000_000);
        let (client, admin, attestor) = setup(&env);

        // Custom window: 3600 s (1 hour)
        client.initialize(&admin, &Some(3600u64));
        client.register_attestor(
            &attestor,
            &soroban_sdk::String::from_str(&env, "mock"),
            &Address::generate(&env),
        );

        // timestamp = now - 3599 s → inside custom window
        let ts: u64 = 1_000_000 - 3599;
        let id = client.submit_attestation(
            &attestor,
            &Address::generate(&env),
            &ts,
            &dummy_hash(&env, 3),
            &Bytes::from_slice(&env, &[0u8; 64]),
        );
        assert_eq!(id, 0);
    }

    #[test]
    #[should_panic]
    fn custom_window_rejects_timestamp_outside_custom_range() {
        let env = make_env();
        set_ts(&env, 1_000_000);
        let (client, admin, attestor) = setup(&env);

        // Custom window: 60 s
        client.initialize(&admin, &Some(60u64));
        client.register_attestor(
            &attestor,
            &soroban_sdk::String::from_str(&env, "mock"),
            &Address::generate(&env),
        );

        // timestamp = now - 61 s → outside 60 s window
        let ts: u64 = 1_000_000 - 61;
        client.submit_attestation(
            &attestor,
            &Address::generate(&env),
            &ts,
            &dummy_hash(&env, 4),
            &Bytes::from_slice(&env, &[0u8; 64]),
        );
    }

    #[test]
    fn custom_window_zero_only_accepts_exact_timestamp() {
        let env = make_env();
        set_ts(&env, 1_000_000);
        let (client, admin, attestor) = setup(&env);

        // Window = 0 → only exact ledger timestamp is valid
        client.initialize(&admin, &Some(0u64));
        client.register_attestor(
            &attestor,
            &soroban_sdk::String::from_str(&env, "mock"),
            &Address::generate(&env),
        );

        let ts: u64 = 1_000_000; // exact match
        let id = client.submit_attestation(
            &attestor,
            &Address::generate(&env),
            &ts,
            &dummy_hash(&env, 5),
            &Bytes::from_slice(&env, &[0u8; 64]),
        );
        assert_eq!(id, 0);
    }
}
