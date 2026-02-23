#[cfg(test)]
mod rate_limiter_tests {
    use crate::{
        AnchorKitContract, AnchorKitContractClient, Error, RateLimitConfig, RateLimitStrategy,
        ServiceType,
    };
    use soroban_sdk::{testutils::Address as _, vec, Address, Env, String};

    #[test]
    fn test_fixed_window_rate_limit() {
        let env = Env::default();
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let anchor = Address::generate(&env);

        client.initialize(&admin);
        client.register_attestor(&anchor);

        let services = vec![&env, ServiceType::Quotes];
        client.configure_services(&anchor, &services);

        // Configure rate limit: 2 requests per 60 seconds
        let config = RateLimitConfig {
            strategy: RateLimitStrategy::FixedWindow,
            max_requests: 2,
            window_seconds: 60,
            refill_rate: 0,
        };
        client.configure_rate_limit(&anchor, &config);

        // First two requests should succeed
        let quote_id_1 = client.submit_quote(
            &anchor,
            &String::from_str(&env, "USD"),
            &String::from_str(&env, "USDC"),
            &10000,
            &100,
            &100,
            &10000,
            &(env.ledger().timestamp() + 3600),
        );
        assert!(quote_id_1 > 0);

        let quote_id_2 = client.submit_quote(
            &anchor,
            &String::from_str(&env, "USD"),
            &String::from_str(&env, "USDC"),
            &10000,
            &100,
            &100,
            &10000,
            &(env.ledger().timestamp() + 3600),
        );
        assert!(quote_id_2 > 0);

        // Third request should fail
        let result = client.try_submit_quote(
            &anchor,
            &String::from_str(&env, "USD"),
            &String::from_str(&env, "USDC"),
            &10000,
            &100,
            &100,
            &10000,
            &(env.ledger().timestamp() + 3600),
        );
        assert_eq!(result, Err(Ok(Error::RateLimitExceeded)));
    }

    #[test]
    fn test_token_bucket_rate_limit() {
        let env = Env::default();
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let anchor = Address::generate(&env);

        client.initialize(&admin);
        client.register_attestor(&anchor);

        let services = vec![&env, ServiceType::Quotes];
        client.configure_services(&anchor, &services);

        // Configure rate limit: 3 tokens, refill 1 per second
        let config = RateLimitConfig {
            strategy: RateLimitStrategy::TokenBucket,
            max_requests: 3,
            window_seconds: 0,
            refill_rate: 1,
        };
        client.configure_rate_limit(&anchor, &config);

        // Use all 3 tokens
        for _ in 0..3 {
            let result = client.submit_quote(
                &anchor,
                &String::from_str(&env, "USD"),
                &String::from_str(&env, "USDC"),
                &10000,
                &100,
                &100,
                &10000,
                &(env.ledger().timestamp() + 3600),
            );
            assert!(result > 0);
        }

        // Fourth request should fail
        let result = client.try_submit_quote(
            &anchor,
            &String::from_str(&env, "USD"),
            &String::from_str(&env, "USDC"),
            &10000,
            &100,
            &100,
            &10000,
            &(env.ledger().timestamp() + 3600),
        );
        assert_eq!(result, Err(Ok(Error::RateLimitExceeded)));
    }

    #[test]
    fn test_per_anchor_rate_limit() {
        let env = Env::default();
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let anchor1 = Address::generate(&env);
        let anchor2 = Address::generate(&env);

        client.initialize(&admin);
        client.register_attestor(&anchor1);
        client.register_attestor(&anchor2);

        let services = vec![&env, ServiceType::Quotes];
        client.configure_services(&anchor1, &services);
        client.configure_services(&anchor2, &services);

        // Configure different limits for each anchor
        let config1 = RateLimitConfig {
            strategy: RateLimitStrategy::FixedWindow,
            max_requests: 1,
            window_seconds: 60,
            refill_rate: 0,
        };
        client.configure_rate_limit(&anchor1, &config1);

        let config2 = RateLimitConfig {
            strategy: RateLimitStrategy::FixedWindow,
            max_requests: 5,
            window_seconds: 60,
            refill_rate: 0,
        };
        client.configure_rate_limit(&anchor2, &config2);

        // Anchor1: first request succeeds
        let result1 = client.submit_quote(
            &anchor1,
            &String::from_str(&env, "USD"),
            &String::from_str(&env, "USDC"),
            &10000,
            &100,
            &100,
            &10000,
            &(env.ledger().timestamp() + 3600),
        );
        assert!(result1 > 0);

        // Anchor1: second request fails
        let result2 = client.try_submit_quote(
            &anchor1,
            &String::from_str(&env, "USD"),
            &String::from_str(&env, "USDC"),
            &10000,
            &100,
            &100,
            &10000,
            &(env.ledger().timestamp() + 3600),
        );
        assert_eq!(result2, Err(Ok(Error::RateLimitExceeded)));

        // Anchor2: can still make requests
        for _ in 0..5 {
            let result = client.submit_quote(
                &anchor2,
                &String::from_str(&env, "USD"),
                &String::from_str(&env, "USDC"),
                &10000,
                &100,
                &100,
                &10000,
                &(env.ledger().timestamp() + 3600),
            );
            assert!(result > 0);
        }
    }

    #[test]
    fn test_no_rate_limit_configured() {
        let env = Env::default();
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let anchor = Address::generate(&env);

        client.initialize(&admin);
        client.register_attestor(&anchor);

        let services = vec![&env, ServiceType::Quotes];
        client.configure_services(&anchor, &services);

        // No rate limit configured - should allow unlimited requests
        for _ in 0..10 {
            let result = client.submit_quote(
                &anchor,
                &String::from_str(&env, "USD"),
                &String::from_str(&env, "USDC"),
                &10000,
                &100,
                &100,
                &10000,
                &(env.ledger().timestamp() + 3600),
            );
            assert!(result > 0);
        }
    }
}
