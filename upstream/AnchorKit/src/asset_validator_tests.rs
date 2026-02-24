#[cfg(test)]
mod asset_validator_tests {
    use crate::{AnchorKitContract, AnchorKitContractClient, Error, ServiceType};
    use soroban_sdk::{testutils::Address as _, vec, Address, Env, String};

    #[test]
    fn test_set_supported_assets() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let anchor = Address::generate(&env);

        client.initialize(&admin);
        client.register_attestor(&anchor);

        let assets = vec![
            &env,
            String::from_str(&env, "USDC"),
            String::from_str(&env, "BTC"),
            String::from_str(&env, "ETH"),
        ];

        client.set_supported_assets(&anchor, &assets);

        let supported = client.get_supported_assets(&anchor);
        assert!(supported.is_some());
        assert_eq!(supported.unwrap().len(), 3);
    }

    #[test]
    fn test_is_asset_supported() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let anchor = Address::generate(&env);

        client.initialize(&admin);
        client.register_attestor(&anchor);

        let assets = vec![
            &env,
            String::from_str(&env, "USDC"),
            String::from_str(&env, "BTC"),
        ];

        client.set_supported_assets(&anchor, &assets);

        assert!(client.is_asset_supported(&anchor, &String::from_str(&env, "USDC")));
        assert!(client.is_asset_supported(&anchor, &String::from_str(&env, "BTC")));
        assert!(!client.is_asset_supported(&anchor, &String::from_str(&env, "ETH")));
    }

    #[test]
    fn test_validate_asset_pair_success() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let anchor = Address::generate(&env);

        client.initialize(&admin);
        client.register_attestor(&anchor);

        let assets = vec![
            &env,
            String::from_str(&env, "USD"),
            String::from_str(&env, "USDC"),
        ];

        client.set_supported_assets(&anchor, &assets);

        let result = client.validate_asset_pair(
            &anchor,
            &String::from_str(&env, "USD"),
            &String::from_str(&env, "USDC"),
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_asset_pair_unsupported_base() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let anchor = Address::generate(&env);

        client.initialize(&admin);
        client.register_attestor(&anchor);

        let assets = vec![&env, String::from_str(&env, "USDC")];

        client.set_supported_assets(&anchor, &assets);

        let result = client.try_validate_asset_pair(
            &anchor,
            &String::from_str(&env, "BTC"),
            &String::from_str(&env, "USDC"),
        );

        assert_eq!(result, Err(Ok(Error::InvalidServiceType)));
    }

    #[test]
    fn test_validate_asset_pair_unsupported_quote() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let anchor = Address::generate(&env);

        client.initialize(&admin);
        client.register_attestor(&anchor);

        let assets = vec![&env, String::from_str(&env, "USD")];

        client.set_supported_assets(&anchor, &assets);

        let result = client.try_validate_asset_pair(
            &anchor,
            &String::from_str(&env, "USD"),
            &String::from_str(&env, "USDC"),
        );

        assert_eq!(result, Err(Ok(Error::InvalidServiceType)));
    }

    #[test]
    fn test_validate_asset_pair_not_configured() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let anchor = Address::generate(&env);

        client.initialize(&admin);
        client.register_attestor(&anchor);

        // Don't configure assets

        let result = client.try_validate_asset_pair(
            &anchor,
            &String::from_str(&env, "USD"),
            &String::from_str(&env, "USDC"),
        );

        assert_eq!(result, Err(Ok(Error::ServicesNotConfigured)));
    }

    #[test]
    fn test_submit_quote_validated_success() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let anchor = Address::generate(&env);

        client.initialize(&admin);
        client.register_attestor(&anchor);

        let services = vec![&env, ServiceType::Quotes];
        client.configure_services(&anchor, &services);

        let assets = vec![
            &env,
            String::from_str(&env, "USD"),
            String::from_str(&env, "USDC"),
        ];
        client.set_supported_assets(&anchor, &assets);

        let quote_id = client.submit_quote_validated(
            &anchor,
            &String::from_str(&env, "USD"),
            &String::from_str(&env, "USDC"),
            &10000,
            &100,
            &100,
            &10000,
            &(env.ledger().timestamp() + 3600),
        );

        assert!(quote_id > 0);
    }

    #[test]
    fn test_submit_quote_validated_rejects_unsupported() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let anchor = Address::generate(&env);

        client.initialize(&admin);
        client.register_attestor(&anchor);

        let services = vec![&env, ServiceType::Quotes];
        client.configure_services(&anchor, &services);

        let assets = vec![&env, String::from_str(&env, "USDC")];
        client.set_supported_assets(&anchor, &assets);

        let result = client.try_submit_quote_validated(
            &anchor,
            &String::from_str(&env, "BTC"),
            &String::from_str(&env, "USDC"),
            &10000,
            &100,
            &100,
            &10000,
            &(env.ledger().timestamp() + 3600),
        );

        assert_eq!(result, Err(Ok(Error::InvalidServiceType)));
    }
}
