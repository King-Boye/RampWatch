#[cfg(test)]
mod metadata_cache_tests {
    use crate::{AnchorKitContract, AnchorKitContractClient, AnchorMetadata, Error};
    use soroban_sdk::{
        testutils::{Address as _, Ledger},
        Address, Env, String,
    };

    #[test]
    fn test_cache_and_retrieve_metadata() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let anchor = Address::generate(&env);

        client.initialize(&admin);

        let metadata = AnchorMetadata {
            anchor: anchor.clone(),
            reputation_score: 9000,
            average_settlement_time: 300,
            liquidity_score: 8500,
            uptime_percentage: 9900,
            total_volume: 1000000,
            is_active: true,
        };

        // Cache metadata with 3600 second TTL
        client.cache_metadata(&anchor, &metadata, &3600);

        // Retrieve cached metadata
        let cached = client.get_cached_metadata(&anchor);
        assert_eq!(cached.reputation_score, 9000);
        assert_eq!(cached.liquidity_score, 8500);
    }

    #[test]
    fn test_cache_expiration() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let anchor = Address::generate(&env);

        client.initialize(&admin);

        let metadata = AnchorMetadata {
            anchor: anchor.clone(),
            reputation_score: 9000,
            average_settlement_time: 300,
            liquidity_score: 8500,
            uptime_percentage: 9900,
            total_volume: 1000000,
            is_active: true,
        };

        // Cache with 10 second TTL
        client.cache_metadata(&anchor, &metadata, &10);

        // Should work immediately
        let cached = client.get_cached_metadata(&anchor);
        assert_eq!(cached.reputation_score, 9000);

        // Advance time by 11 seconds
        env.ledger().with_mut(|li| li.timestamp += 11);

        // Should be expired
        let result = client.try_get_cached_metadata(&anchor);
        assert_eq!(result, Err(Ok(Error::CacheExpired)));
    }

    #[test]
    fn test_manual_refresh() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let anchor = Address::generate(&env);

        client.initialize(&admin);

        let metadata = AnchorMetadata {
            anchor: anchor.clone(),
            reputation_score: 9000,
            average_settlement_time: 300,
            liquidity_score: 8500,
            uptime_percentage: 9900,
            total_volume: 1000000,
            is_active: true,
        };

        client.cache_metadata(&anchor, &metadata, &3600);

        // Verify cached
        let cached = client.get_cached_metadata(&anchor);
        assert_eq!(cached.reputation_score, 9000);

        // Manual refresh (invalidate)
        client.refresh_metadata_cache(&anchor);

        // Should not be found
        let result = client.try_get_cached_metadata(&anchor);
        assert_eq!(result, Err(Ok(Error::CacheNotFound)));
    }

    #[test]
    fn test_cache_capabilities() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let anchor = Address::generate(&env);

        client.initialize(&admin);

        let toml_url = String::from_str(&env, "https://anchor.example/.well-known/stellar.toml");
        let capabilities = String::from_str(&env, r#"{"deposits":true,"withdrawals":true}"#);

        client.cache_capabilities(&anchor, &toml_url, &capabilities, &3600);

        let cached = client.get_cached_capabilities(&anchor);
        assert_eq!(cached.toml_url, toml_url);
        assert_eq!(cached.capabilities, capabilities);
    }

    #[test]
    fn test_capabilities_expiration() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let anchor = Address::generate(&env);

        client.initialize(&admin);

        let toml_url = String::from_str(&env, "https://anchor.example/.well-known/stellar.toml");
        let capabilities = String::from_str(&env, r#"{"deposits":true}"#);

        client.cache_capabilities(&anchor, &toml_url, &capabilities, &5);

        // Advance time
        env.ledger().with_mut(|li| li.timestamp += 6);

        let result = client.try_get_cached_capabilities(&anchor);
        assert_eq!(result, Err(Ok(Error::CacheExpired)));
    }

    #[test]
    fn test_refresh_capabilities() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let anchor = Address::generate(&env);

        client.initialize(&admin);

        let toml_url = String::from_str(&env, "https://anchor.example/.well-known/stellar.toml");
        let capabilities = String::from_str(&env, r#"{"deposits":true}"#);

        client.cache_capabilities(&anchor, &toml_url, &capabilities, &3600);

        // Refresh
        client.refresh_capabilities_cache(&anchor);

        let result = client.try_get_cached_capabilities(&anchor);
        assert_eq!(result, Err(Ok(Error::CacheNotFound)));
    }

    #[test]
    fn test_cache_not_found() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let anchor = Address::generate(&env);

        client.initialize(&admin);

        let result = client.try_get_cached_metadata(&anchor);
        assert_eq!(result, Err(Ok(Error::CacheNotFound)));
    }
}
