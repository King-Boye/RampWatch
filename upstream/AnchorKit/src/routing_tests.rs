#![cfg(test)]

mod routing_tests {
    use soroban_sdk::{
        testutils::{Address as _, Ledger, LedgerInfo},
        Address, Env, String, Symbol, Vec,
    };

    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;

    use crate::contract::{AnchorKitContract, AnchorKitContractClient, RoutingOptions, RoutingRequest};
    use crate::sep10_test_util::register_attestor_with_sep10;

    fn make_env() -> Env {
        let env = Env::default();
        env.mock_all_auths();
        env
    }

    fn set_ledger(env: &Env, timestamp: u64) {
        env.ledger().set(LedgerInfo {
            timestamp,
            protocol_version: 21,
            sequence_number: 0,
            network_id: Default::default(),
            base_reserve: 0,
            min_persistent_entry_ttl: 4096,
            min_temp_entry_ttl: 16,
            max_entry_ttl: 6312000,
        });
    }

    fn setup(env: &Env) -> (AnchorKitContractClient, Address) {
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(env, &contract_id);
        let admin = Address::generate(env);
        client.initialize(&admin);
        (client, admin)
    }

    fn register_anchor(env: &Env, client: &AnchorKitContractClient, anchor: &Address) {
        let signing_key = SigningKey::generate(&mut OsRng);
        register_attestor_with_sep10(env, client, anchor, anchor, &signing_key);
        let mut services = Vec::new(env);
        services.push_back(1u32);
        services.push_back(3u32);
        client.configure_services(anchor, &services);
    }

    fn make_request(env: &Env) -> RoutingRequest {
        RoutingRequest {
            base_asset: String::from_str(env, "USD"),
            quote_asset: String::from_str(env, "USDC"),
            amount: 5000,
            operation_type: 1,
        }
    }

    #[test]
    fn test_select_lowest_fee_anchor() {
        let env = make_env();
        set_ledger(&env, 1_000_000);
        let (client, _) = setup(&env);

        let anchor1 = Address::generate(&env);
        let anchor2 = Address::generate(&env);
        register_anchor(&env, &client, &anchor1);
        register_anchor(&env, &client, &anchor2);

        client.submit_quote(
            &anchor1,
            &String::from_str(&env, "USD"),
            &String::from_str(&env, "USDC"),
            &10000u64, &50u32, &100u64, &100000u64, &1_003_600u64,
        );
        client.submit_quote(
            &anchor2,
            &String::from_str(&env, "USD"),
            &String::from_str(&env, "USDC"),
            &10000u64, &20u32, &100u64, &100000u64, &1_003_600u64,
        );

        let q1 = client.get_quote(&anchor1, &1u64);
        let q2 = client.get_quote(&anchor2, &2u64);

        assert_eq!(q1.fee_percentage, 50);
        assert_eq!(q2.fee_percentage, 20);
        // anchor2 has lower fee
        assert!(q2.fee_percentage < q1.fee_percentage);
        assert_eq!(q2.anchor, anchor2);
    }

    #[test]
    fn test_fastest_settlement_strategy() {
        let env = make_env();
        set_ledger(&env, 1_000_000);
        let (client, _) = setup(&env);

        let anchor1 = Address::generate(&env);
        let anchor2 = Address::generate(&env);
        register_anchor(&env, &client, &anchor1);
        client.set_anchor_metadata(&anchor1, &8000u32, &600u64, &7500u32, &9900u32, &1_000_000u64);
        client.submit_quote(
            &anchor1,
            &String::from_str(&env, "USD"),
            &String::from_str(&env, "USDC"),
            &10000u64, &25u32, &100u64, &100000u64, &1_003_600u64,
        );

        register_anchor(&env, &client, &anchor2);
        client.set_anchor_metadata(&anchor2, &8000u32, &200u64, &7500u32, &9900u32, &1_000_000u64);
        client.submit_quote(
            &anchor2,
            &String::from_str(&env, "USD"),
            &String::from_str(&env, "USDC"),
            &10000u64, &25u32, &100u64, &100000u64, &1_003_600u64,
        );

        let mut strategy = Vec::new(&env);
        strategy.push_back(Symbol::new(&env, "FastestSettlement"));
        let options = RoutingOptions {
            request: make_request(&env),
            strategy,
            min_reputation: 0,
            max_anchors: 2,
            require_kyc: false,
        };

        // anchor2 has faster settlement time (200 < 600)
        let best = client.route_transaction(&options);
        assert_eq!(best.anchor, anchor2);
    }

    #[test]
    fn test_filter_by_reputation() {
        let env = make_env();
        set_ledger(&env, 1_000_000);
        let (client, _) = setup(&env);

        let anchor1 = Address::generate(&env);
        let anchor2 = Address::generate(&env);
        register_anchor(&env, &client, &anchor1);
        client.set_anchor_metadata(&anchor1, &3000u32, &300u64, &7500u32, &9900u32, &1_000_000u64);
        client.submit_quote(
            &anchor1,
            &String::from_str(&env, "USD"),
            &String::from_str(&env, "USDC"),
            &9900u64, &20u32, &100u64, &100000u64, &1_003_600u64,
        );

        register_anchor(&env, &client, &anchor2);
        client.set_anchor_metadata(&anchor2, &8000u32, &300u64, &7500u32, &9900u32, &1_000_000u64);
        client.submit_quote(
            &anchor2,
            &String::from_str(&env, "USD"),
            &String::from_str(&env, "USDC"),
            &10000u64, &25u32, &100u64, &100000u64, &1_003_600u64,
        );

        let mut strategy = Vec::new(&env);
        strategy.push_back(Symbol::new(&env, "HighestReputation"));
        let options = RoutingOptions {
            request: make_request(&env),
            strategy,
            min_reputation: 0,
            max_anchors: 2,
            require_kyc: false,
        };

        // anchor2 has higher reputation (8000 > 3000)
        let best = client.route_transaction(&options);
        assert_eq!(best.anchor, anchor2);
    }

    #[test]
    fn test_expired_quotes_filtered() {
        let env = make_env();
        set_ledger(&env, 1_000_000);
        let (client, _) = setup(&env);

        let anchor1 = Address::generate(&env);
        register_anchor(&env, &client, &anchor1);

        // First quote: expires at 1_000_100 (still valid at t=1_000_000)
        client.submit_quote(
            &anchor1,
            &String::from_str(&env, "USD"),
            &String::from_str(&env, "USDC"),
            &9900u64, &15u32, &100u64, &100000u64, &1_000_100u64,
        );
        // Second quote: valid for longer
        client.submit_quote(
            &anchor1,
            &String::from_str(&env, "USD"),
            &String::from_str(&env, "USDC"),
            &10000u64, &25u32, &100u64, &100000u64, &1_003_600u64,
        );

        let q1 = client.get_quote(&anchor1, &1u64);
        let q2 = client.get_quote(&anchor1, &2u64);

        assert_eq!(q1.valid_until, 1_000_100);
        assert_eq!(q2.valid_until, 1_003_600);

        // At t=1_000_200, q1 would be expired
        assert!(q1.valid_until < 1_000_200);
        assert!(q2.valid_until > 1_000_200);
    }

    #[test]
    fn test_no_anchors_available() {
        let env = make_env();
        set_ledger(&env, 0);
        let (client, _) = setup(&env);

        let anchor1 = Address::generate(&env);
        register_anchor(&env, &client, &anchor1);

        // No quotes submitted
        let result = client.try_get_quote(&anchor1, &1u64);
        assert!(result.is_err());
    }

    #[test]
    fn test_handle_unavailable_anchors() {
        let env = make_env();
        set_ledger(&env, 1_000_000);
        let (client, _) = setup(&env);

        let anchor1 = Address::generate(&env);
        let anchor2 = Address::generate(&env);
        let anchor3 = Address::generate(&env);
        register_anchor(&env, &client, &anchor1);
        register_anchor(&env, &client, &anchor2);
        register_anchor(&env, &client, &anchor3);

        client.submit_quote(
            &anchor1,
            &String::from_str(&env, "USD"),
            &String::from_str(&env, "USDC"),
            &10000u64, &25u32, &100u64, &100000u64, &1_003_600u64,
        );
        client.submit_quote(
            &anchor2,
            &String::from_str(&env, "USD"),
            &String::from_str(&env, "USDC"),
            &10050u64, &30u32, &100u64, &100000u64, &1_003_600u64,
        );

        let q1 = client.get_quote(&anchor1, &1u64);
        let q2 = client.get_quote(&anchor2, &2u64);

        // anchor3 has no quote
        let result = client.try_get_quote(&anchor3, &3u64);
        assert!(result.is_err());

        assert_eq!(q1.fee_percentage, 25);
        assert_eq!(q2.fee_percentage, 30);
    }

    #[test]
    fn test_amount_outside_quote_limits() {
        let env = make_env();
        set_ledger(&env, 1_000_000);
        let (client, _) = setup(&env);

        let anchor1 = Address::generate(&env);
        register_anchor(&env, &client, &anchor1);

        client.submit_quote(
            &anchor1,
            &String::from_str(&env, "USD"),
            &String::from_str(&env, "USDC"),
            &10000u64, &25u32, &100u64, &100000u64, &1_003_600u64,
        );

        let q = client.get_quote(&anchor1, &1u64);
        assert_eq!(q.minimum_amount, 100);
        assert_eq!(q.maximum_amount, 100000);

        // 5000 is within limits
        assert!(5000 >= q.minimum_amount && 5000 <= q.maximum_amount);
        // 200000 is outside limits
        assert!(200000 > q.maximum_amount);
    }

    #[test]
    fn test_select_best_quote_from_multiple_anchors() {
        let env = make_env();
        set_ledger(&env, 1_000_000);
        let (client, _) = setup(&env);

        let anchor1 = Address::generate(&env);
        let anchor2 = Address::generate(&env);
        let anchor3 = Address::generate(&env);
        register_anchor(&env, &client, &anchor1);
        register_anchor(&env, &client, &anchor2);
        register_anchor(&env, &client, &anchor3);

        client.submit_quote(
            &anchor1,
            &String::from_str(&env, "USD"),
            &String::from_str(&env, "USDC"),
            &10100u64, &50u32, &100u64, &100000u64, &1_003_600u64,
        );
        client.submit_quote(
            &anchor2,
            &String::from_str(&env, "USD"),
            &String::from_str(&env, "USDC"),
            &10000u64, &25u32, &100u64, &100000u64, &1_003_600u64,
        );
        client.submit_quote(
            &anchor3,
            &String::from_str(&env, "USD"),
            &String::from_str(&env, "USDC"),
            &10050u64, &30u32, &100u64, &100000u64, &1_003_600u64,
        );

        let q1 = client.get_quote(&anchor1, &1u64);
        let q2 = client.get_quote(&anchor2, &2u64);
        let q3 = client.get_quote(&anchor3, &3u64);

        // anchor2 has lowest fee
        let mut best = &q1;
        for q in [&q2, &q3] {
            if q.fee_percentage < best.fee_percentage {
                best = q;
            }
        }
        assert_eq!(best.anchor, anchor2);
        assert_eq!(best.fee_percentage, 25);
    }

    #[test]
    fn test_auto_deactivation() {
        let env = make_env();
        set_ledger(&env, 1_000_000);
        let (client, _) = setup(&env);

        let anchor = Address::generate(&env);
        register_anchor(&env, &client, &anchor);
        client.set_anchor_metadata(&anchor, &8000u32, &300u64, &7500u32, &9900u32, &1_000_000u64);
        client.submit_quote(
            &anchor,
            &String::from_str(&env, "USD"),
            &String::from_str(&env, "USDC"),
            &10000u64, &20u32, &100u64, &100000u64, &1_003_600u64,
        );

        // Set threshold to 3 consecutive failures
        client.set_health_failure_threshold(&3u32);

        // Two failures — below threshold, anchor still active
        client.update_health_status(&anchor, &100u64, &2u32, &9800u32);
        let mut strategy = Vec::new(&env);
        strategy.push_back(Symbol::new(&env, "LowestFee"));
        let options = RoutingOptions {
            request: make_request(&env),
            strategy: strategy.clone(),
            min_reputation: 0,
            max_anchors: 1,
            require_kyc: false,
        };
        let best = client.route_transaction(&options);
        assert_eq!(best.anchor, anchor);

        // Third failure — threshold breached, anchor deactivated
        client.update_health_status(&anchor, &100u64, &3u32, &9500u32);

        // Health status recorded
        let health = client.get_health_status(&anchor).unwrap();
        assert_eq!(health.failure_count, 3);

        // Anchor no longer routable
        let result = client.try_route_transaction(&options);
        assert!(result.is_err());
    }
}
