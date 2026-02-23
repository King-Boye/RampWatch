#[cfg(test)]
mod connection_pool_tests {
    use crate::{AnchorKitContract, AnchorKitContractClient};
    use soroban_sdk::{testutils::Address as _, Address, Env, String};

    #[test]
    fn test_configure_pool() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        client.initialize(&admin);

        client.configure_connection_pool(&20, &600, &60, &true);

        let config = client.get_pool_config();
        assert_eq!(config.max_connections, 20);
        assert_eq!(config.idle_timeout_seconds, 600);
        assert_eq!(config.connection_timeout_seconds, 60);
        assert_eq!(config.reuse_connections, true);
    }

    #[test]
    fn test_connection_reuse() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        client.initialize(&admin);

        client.configure_connection_pool(&10, &300, &30, &true);

        let endpoint = String::from_str(&env, "https://anchor.example.com");

        // First connection - should be new
        client.get_pooled_connection(&endpoint);
        let stats = client.get_pool_stats();
        assert_eq!(stats.new_connections, 1);
        assert_eq!(stats.reused_connections, 0);

        // Second connection - should be reused
        client.get_pooled_connection(&endpoint);
        let stats = client.get_pool_stats();
        assert_eq!(stats.new_connections, 1);
        assert_eq!(stats.reused_connections, 1);

        // Third connection - should be reused
        client.get_pooled_connection(&endpoint);
        let stats = client.get_pool_stats();
        assert_eq!(stats.new_connections, 1);
        assert_eq!(stats.reused_connections, 2);
    }

    #[test]
    fn test_connection_expiry() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        client.initialize(&admin);

        // Short idle timeout
        client.configure_connection_pool(&10, &5, &30, &true);

        let endpoint = String::from_str(&env, "https://anchor.example.com");

        // First connection
        client.get_pooled_connection(&endpoint);
        let stats = client.get_pool_stats();
        assert_eq!(stats.new_connections, 1);

        // Advance time past idle timeout
        env.ledger().with_mut(|li| li.timestamp += 10);

        // Should create new connection (old one expired)
        client.get_pooled_connection(&endpoint);
        let stats = client.get_pool_stats();
        assert_eq!(stats.new_connections, 2);
    }

    #[test]
    fn test_multiple_endpoints() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        client.initialize(&admin);

        client.configure_connection_pool(&10, &300, &30, &true);

        let endpoint1 = String::from_str(&env, "https://anchor1.example.com");
        let endpoint2 = String::from_str(&env, "https://anchor2.example.com");

        // Connect to endpoint1
        client.get_pooled_connection(&endpoint1);
        client.get_pooled_connection(&endpoint1);

        // Connect to endpoint2
        client.get_pooled_connection(&endpoint2);
        client.get_pooled_connection(&endpoint2);

        let stats = client.get_pool_stats();
        assert_eq!(stats.new_connections, 2); // One per endpoint
        assert_eq!(stats.reused_connections, 2); // One reuse per endpoint
    }

    #[test]
    fn test_disable_reuse() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        client.initialize(&admin);

        // Disable connection reuse
        client.configure_connection_pool(&10, &300, &30, &false);

        let endpoint = String::from_str(&env, "https://anchor.example.com");

        // All connections should be new
        client.get_pooled_connection(&endpoint);
        client.get_pooled_connection(&endpoint);
        client.get_pooled_connection(&endpoint);

        let stats = client.get_pool_stats();
        assert_eq!(stats.new_connections, 3);
        assert_eq!(stats.reused_connections, 0);
    }

    #[test]
    fn test_reset_stats() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        client.initialize(&admin);

        client.configure_connection_pool(&10, &300, &30, &true);

        let endpoint = String::from_str(&env, "https://anchor.example.com");

        client.get_pooled_connection(&endpoint);
        client.get_pooled_connection(&endpoint);

        let stats = client.get_pool_stats();
        assert!(stats.total_requests > 0);

        client.reset_pool_stats();

        let stats = client.get_pool_stats();
        assert_eq!(stats.total_requests, 0);
        assert_eq!(stats.new_connections, 0);
        assert_eq!(stats.reused_connections, 0);
    }

    #[test]
    fn test_benchmark_improvement() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        client.initialize(&admin);

        let endpoint = String::from_str(&env, "https://anchor.example.com");

        // Benchmark without pooling
        client.configure_connection_pool(&10, &300, &30, &false);
        client.reset_pool_stats();

        for _ in 0..10 {
            client.get_pooled_connection(&endpoint);
        }

        let stats_no_pool = client.get_pool_stats();
        assert_eq!(stats_no_pool.new_connections, 10);

        // Benchmark with pooling
        client.configure_connection_pool(&10, &300, &30, &true);
        client.reset_pool_stats();

        for _ in 0..10 {
            client.get_pooled_connection(&endpoint);
        }

        let stats_with_pool = client.get_pool_stats();
        assert_eq!(stats_with_pool.new_connections, 1);
        assert_eq!(stats_with_pool.reused_connections, 9);

        // Improvement: 90% reduction in new connections
        let improvement = (stats_no_pool.new_connections - stats_with_pool.new_connections) as f64
            / stats_no_pool.new_connections as f64
            * 100.0;
        assert!(improvement >= 90.0);
    }
}
