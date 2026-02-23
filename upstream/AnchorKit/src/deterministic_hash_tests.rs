#[cfg(test)]
mod deterministic_hash_tests {
    use crate::types::*;
    use soroban_sdk::{testutils::Address as _, xdr::ToXdr, Address, BytesN, Env, String};

    fn create_test_env() -> Env {
        Env::default()
    }

    fn create_quote_request(env: &Env, base: &str, quote: &str, amount: u64) -> QuoteRequest {
        QuoteRequest {
            base_asset: String::from_str(env, base),
            quote_asset: String::from_str(env, quote),
            amount,
            operation_type: ServiceType::Deposits,
        }
    }

    fn hash_struct<T: ToXdr + Clone>(env: &Env, value: &T) -> BytesN<32> {
        let xdr = value.clone().to_xdr(env);
        env.crypto().sha256(&xdr).into()
    }

    #[test]
    fn test_quote_request_same_struct_same_hash() {
        let env = create_test_env();

        let req1 = create_quote_request(&env, "USD", "USDC", 1000);
        let req2 = create_quote_request(&env, "USD", "USDC", 1000);

        let hash1 = hash_struct(&env, &req1);
        let hash2 = hash_struct(&env, &req2);

        assert_eq!(
            hash1, hash2,
            "Identical QuoteRequest structs must produce identical hashes"
        );
    }

    #[test]
    fn test_quote_request_field_order_independent_hash() {
        let env = create_test_env();

        let req1 = QuoteRequest {
            base_asset: String::from_str(&env, "USD"),
            quote_asset: String::from_str(&env, "USDC"),
            amount: 1000,
            operation_type: ServiceType::Deposits,
        };

        let req2 = QuoteRequest {
            operation_type: ServiceType::Deposits,
            amount: 1000,
            quote_asset: String::from_str(&env, "USDC"),
            base_asset: String::from_str(&env, "USD"),
        };

        let hash1 = hash_struct(&env, &req1);
        let hash2 = hash_struct(&env, &req2);

        assert_eq!(
            hash1, hash2,
            "Field initialization order must not affect QuoteRequest hash"
        );
    }

    #[test]
    fn test_quote_request_different_values_different_hash() {
        let env = create_test_env();

        let req1 = create_quote_request(&env, "USD", "USDC", 1000);
        let req2 = create_quote_request(&env, "USD", "USDC", 2000);

        let hash1 = hash_struct(&env, &req1);
        let hash2 = hash_struct(&env, &req2);

        assert_ne!(
            hash1, hash2,
            "Different amounts must produce different hashes"
        );
    }

    #[test]
    fn test_routing_request_same_struct_same_hash() {
        let env = create_test_env();

        let quote_req = create_quote_request(&env, "USD", "USDC", 1000);

        let routing1 = RoutingRequest {
            request: quote_req.clone(),
            strategy: RoutingStrategy::BestRate,
            max_anchors: 5,
            require_kyc: true,
            min_reputation: 8000,
        };

        let routing2 = RoutingRequest {
            request: quote_req,
            strategy: RoutingStrategy::BestRate,
            max_anchors: 5,
            require_kyc: true,
            min_reputation: 8000,
        };

        let hash1 = hash_struct(&env, &routing1);
        let hash2 = hash_struct(&env, &routing2);

        assert_eq!(
            hash1, hash2,
            "Identical RoutingRequest structs must produce identical hashes"
        );
    }

    #[test]
    fn test_routing_request_field_order_independent_hash() {
        let env = create_test_env();
        let quote_req = create_quote_request(&env, "USD", "USDC", 1000);

        let routing1 = RoutingRequest {
            request: quote_req.clone(),
            strategy: RoutingStrategy::BestRate,
            max_anchors: 5,
            require_kyc: true,
            min_reputation: 8000,
        };

        let routing2 = RoutingRequest {
            min_reputation: 8000,
            require_kyc: true,
            max_anchors: 5,
            strategy: RoutingStrategy::BestRate,
            request: quote_req,
        };

        let hash1 = hash_struct(&env, &routing1);
        let hash2 = hash_struct(&env, &routing2);

        assert_eq!(
            hash1, hash2,
            "Field initialization order must not affect RoutingRequest hash"
        );
    }

    #[test]
    fn test_routing_request_different_strategy_different_hash() {
        let env = create_test_env();

        let quote_req = create_quote_request(&env, "USD", "USDC", 1000);

        let routing1 = RoutingRequest {
            request: quote_req.clone(),
            strategy: RoutingStrategy::BestRate,
            max_anchors: 5,
            require_kyc: true,
            min_reputation: 8000,
        };

        let routing2 = RoutingRequest {
            request: quote_req,
            strategy: RoutingStrategy::LowestFee,
            max_anchors: 5,
            require_kyc: true,
            min_reputation: 8000,
        };

        let hash1 = hash_struct(&env, &routing1);
        let hash2 = hash_struct(&env, &routing2);

        assert_ne!(
            hash1, hash2,
            "Different strategies must produce different hashes"
        );
    }

    #[test]
    fn test_transaction_intent_builder_same_struct_same_hash() {
        let env = create_test_env();
        let anchor = Address::generate(&env);
        let quote_req = create_quote_request(&env, "USD", "USDC", 1000);

        let builder1 = TransactionIntentBuilder {
            anchor: anchor.clone(),
            request: quote_req.clone(),
            quote_id: 42,
            require_kyc: true,
            session_id: 100,
            ttl_seconds: 300,
        };

        let builder2 = TransactionIntentBuilder {
            anchor,
            request: quote_req,
            quote_id: 42,
            require_kyc: true,
            session_id: 100,
            ttl_seconds: 300,
        };

        let hash1 = hash_struct(&env, &builder1);
        let hash2 = hash_struct(&env, &builder2);

        assert_eq!(
            hash1, hash2,
            "Identical TransactionIntentBuilder structs must produce identical hashes"
        );
    }

    #[test]
    fn test_anchor_metadata_same_struct_same_hash() {
        let env = create_test_env();
        let anchor = Address::generate(&env);

        let metadata1 = AnchorMetadata {
            anchor: anchor.clone(),
            reputation_score: 9500,
            average_settlement_time: 120,
            liquidity_score: 8000,
            uptime_percentage: 9900,
            total_volume: 1_000_000,
            is_active: true,
        };

        let metadata2 = AnchorMetadata {
            anchor,
            reputation_score: 9500,
            average_settlement_time: 120,
            liquidity_score: 8000,
            uptime_percentage: 9900,
            total_volume: 1_000_000,
            is_active: true,
        };

        let hash1 = hash_struct(&env, &metadata1);
        let hash2 = hash_struct(&env, &metadata2);

        assert_eq!(
            hash1, hash2,
            "Identical AnchorMetadata structs must produce identical hashes"
        );
    }

    #[test]
    fn test_health_status_same_struct_same_hash() {
        let env = create_test_env();
        let anchor = Address::generate(&env);

        let health1 = HealthStatus {
            anchor: anchor.clone(),
            latency_ms: 50,
            failure_count: 2,
            availability_percent: 9950,
            last_check: 1234567890,
        };

        let health2 = HealthStatus {
            anchor,
            latency_ms: 50,
            failure_count: 2,
            availability_percent: 9950,
            last_check: 1234567890,
        };

        let hash1 = hash_struct(&env, &health1);
        let hash2 = hash_struct(&env, &health2);

        assert_eq!(
            hash1, hash2,
            "Identical HealthStatus structs must produce identical hashes"
        );
    }

    #[test]
    fn test_cross_environment_determinism() {
        // Create two separate environments to simulate different execution contexts
        let env1 = Env::default();
        let env2 = Env::default();

        let req1 = create_quote_request(&env1, "USD", "USDC", 1000);
        let req2 = create_quote_request(&env2, "USD", "USDC", 1000);

        let hash1 = hash_struct(&env1, &req1);
        let hash2 = hash_struct(&env2, &req2);

        assert_eq!(
            hash1.to_array(),
            hash2.to_array(),
            "Same struct across different environments must produce identical hashes"
        );
    }

    #[test]
    fn test_service_type_enum_deterministic() {
        let env = create_test_env();

        let service1 = ServiceType::Deposits;
        let service2 = ServiceType::Deposits;

        let hash1 = hash_struct(&env, &service1);
        let hash2 = hash_struct(&env, &service2);

        assert_eq!(
            hash1, hash2,
            "Same enum variant must produce identical hashes"
        );
    }

    #[test]
    fn test_routing_strategy_enum_deterministic() {
        let env = create_test_env();

        let strategy1 = RoutingStrategy::BestRate;
        let strategy2 = RoutingStrategy::BestRate;

        let hash1 = hash_struct(&env, &strategy1);
        let hash2 = hash_struct(&env, &strategy2);

        assert_eq!(
            hash1, hash2,
            "Same enum variant must produce identical hashes"
        );
    }

    #[test]
    fn test_multiple_runs_same_hash() {
        let env = create_test_env();
        let req = create_quote_request(&env, "USD", "USDC", 1000);

        // Hash the same struct multiple times
        let hash1 = hash_struct(&env, &req);
        let hash2 = hash_struct(&env, &req);
        let hash3 = hash_struct(&env, &req);

        assert_eq!(
            hash1, hash2,
            "Multiple hashes of same struct must be identical (run 1 vs 2)"
        );
        assert_eq!(
            hash2, hash3,
            "Multiple hashes of same struct must be identical (run 2 vs 3)"
        );
        assert_eq!(
            hash1, hash3,
            "Multiple hashes of same struct must be identical (run 1 vs 3)"
        );
    }
}
