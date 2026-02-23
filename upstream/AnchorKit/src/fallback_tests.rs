#[cfg(test)]
mod fallback_tests {
    use crate::{AnchorKitContract, AnchorKitContractClient, ServiceType};
    use soroban_sdk::{testutils::Address as _, vec, Address, Env};

    #[test]
    fn test_configure_fallback() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let anchor1 = Address::generate(&env);
        let anchor2 = Address::generate(&env);
        let anchor3 = Address::generate(&env);

        client.initialize(&admin);

        let order = vec![&env, anchor1.clone(), anchor2.clone(), anchor3.clone()];
        client.configure_fallback(&order, &3, &2);

        let config = client.get_fallback_config();
        assert!(config.is_some());
        
        let config = config.unwrap();
        assert_eq!(config.anchor_order.len(), 3);
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.failure_threshold, 2);
    }

    #[test]
    fn test_record_failure() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let anchor = Address::generate(&env);

        client.initialize(&admin);

        let order = vec![&env, anchor.clone()];
        client.configure_fallback(&order, &3, &2);

        // Record first failure
        client.record_anchor_failure(&anchor);
        
        let state = client.get_anchor_failure_state(&anchor);
        assert!(state.is_some());
        
        let state = state.unwrap();
        assert_eq!(state.failure_count, 1);
        assert!(!state.is_down);

        // Record second failure - should mark as down
        client.record_anchor_failure(&anchor);
        
        let state = client.get_anchor_failure_state(&anchor).unwrap();
        assert_eq!(state.failure_count, 2);
        assert!(state.is_down);
    }

    #[test]
    fn test_record_success_clears_failure() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let anchor = Address::generate(&env);

        client.initialize(&admin);

        let order = vec![&env, anchor.clone()];
        client.configure_fallback(&order, &3, &2);

        client.record_anchor_failure(&anchor);
        assert!(client.get_anchor_failure_state(&anchor).is_some());

        client.record_anchor_success(&anchor);
        assert!(client.get_anchor_failure_state(&anchor).is_none());
    }

    #[test]
    fn test_select_fallback_anchor() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let anchor1 = Address::generate(&env);
        let anchor2 = Address::generate(&env);
        let anchor3 = Address::generate(&env);

        client.initialize(&admin);

        let order = vec![&env, anchor1.clone(), anchor2.clone(), anchor3.clone()];
        client.configure_fallback(&order, &3, &2);

        // First selection should return anchor1
        let selected = client.select_fallback_anchor(&None);
        assert_eq!(selected, anchor1);

        // Mark anchor1 as failed, should select anchor2
        client.record_anchor_failure(&anchor1);
        client.record_anchor_failure(&anchor1);
        
        let selected = client.select_fallback_anchor(&Some(anchor1.clone()));
        assert_eq!(selected, anchor2);
    }

    #[test]
    fn test_fallback_skips_down_anchors() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let anchor1 = Address::generate(&env);
        let anchor2 = Address::generate(&env);
        let anchor3 = Address::generate(&env);

        client.initialize(&admin);

        let order = vec![&env, anchor1.clone(), anchor2.clone(), anchor3.clone()];
        client.configure_fallback(&order, &3, &2);

        // Mark anchor1 and anchor2 as down
        client.record_anchor_failure(&anchor1);
        client.record_anchor_failure(&anchor1);
        client.record_anchor_failure(&anchor2);
        client.record_anchor_failure(&anchor2);

        // Should skip to anchor3
        let selected = client.select_fallback_anchor(&None);
        assert_eq!(selected, anchor3);
    }

    #[test]
    fn test_no_anchors_available() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let anchor1 = Address::generate(&env);

        client.initialize(&admin);

        let order = vec![&env, anchor1.clone()];
        client.configure_fallback(&order, &3, &2);

        // Mark all anchors as down
        client.record_anchor_failure(&anchor1);
        client.record_anchor_failure(&anchor1);

        let result = client.try_select_fallback_anchor(&None);
        assert!(result.is_err());
    }

    #[test]
    fn test_fallback_order_preserved() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let anchor1 = Address::generate(&env);
        let anchor2 = Address::generate(&env);
        let anchor3 = Address::generate(&env);

        client.initialize(&admin);

        let order = vec![&env, anchor1.clone(), anchor2.clone(), anchor3.clone()];
        client.configure_fallback(&order, &3, &2);

        let config = client.get_fallback_config().unwrap();
        
        assert_eq!(config.anchor_order.get(0).unwrap(), anchor1);
        assert_eq!(config.anchor_order.get(1).unwrap(), anchor2);
        assert_eq!(config.anchor_order.get(2).unwrap(), anchor3);
    }
}
