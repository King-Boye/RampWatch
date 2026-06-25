#[cfg(test)]
mod tests {
    use crate::sdk_config::{Network, SdkConfig};
    use soroban_sdk::{Env, String};

    #[test]
    fn test_valid_testnet_config() {
        let env = Env::default();
        let config = SdkConfig::new(
            Network::Testnet,
            30,
            3,
            String::from_str(&env, "testanchor.stellar.org"),
        );
        assert!(config.is_ok());
    }

    #[test]
    fn test_valid_mainnet_config() {
        let env = Env::default();
        let config = SdkConfig::new(
            Network::Mainnet,
            60,
            5,
            String::from_str(&env, "anchor.stellar.org"),
        );
        assert!(config.is_ok());
    }

    #[test]
    fn test_timeout_too_low() {
        let env = Env::default();
        let config = SdkConfig::new(
            Network::Testnet,
            2,
            3,
            String::from_str(&env, "anchor.stellar.org"),
        );
        assert!(config.is_err());
    }

    #[test]
    fn test_timeout_too_high() {
        let env = Env::default();
        let config = SdkConfig::new(
            Network::Testnet,
            400,
            3,
            String::from_str(&env, "anchor.stellar.org"),
        );
        assert!(config.is_err());
    }

    #[test]
    fn test_retry_too_high() {
        let env = Env::default();
        let config = SdkConfig::new(
            Network::Testnet,
            30,
            15,
            String::from_str(&env, "anchor.stellar.org"),
        );
        assert!(config.is_err());
    }

    #[test]
    fn test_anchor_too_short() {
        let env = Env::default();
        let config = SdkConfig::new(Network::Testnet, 30, 3, String::from_str(&env, "ab"));
        assert!(config.is_err());
    }

    #[test]
    fn test_min_retry_zero() {
        let env = Env::default();
        let config = SdkConfig::new(
            Network::Testnet,
            30,
            0,
            String::from_str(&env, "anchor.stellar.org"),
        );
        assert!(config.is_ok());
    }

    #[test]
    fn test_max_retry_ten() {
        let env = Env::default();
        let config = SdkConfig::new(
            Network::Mainnet,
            30,
            10,
            String::from_str(&env, "anchor.stellar.org"),
        );
        assert!(config.is_ok());
    }

    #[test]
    fn test_min_timeout_five() {
        let env = Env::default();
        let config = SdkConfig::new(
            Network::Testnet,
            5,
            3,
            String::from_str(&env, "anchor.stellar.org"),
        );
        assert!(config.is_ok());
    }

    #[test]
    fn test_max_timeout_300() {
        let env = Env::default();
        let config = SdkConfig::new(
            Network::Mainnet,
            300,
            3,
            String::from_str(&env, "anchor.stellar.org"),
        );
        assert!(config.is_ok());
    }

    #[test]
    fn test_default_timeout_is_10_seconds() {
        use crate::sdk_config::DEFAULT_TIMEOUT_SECONDS;
        assert_eq!(DEFAULT_TIMEOUT_SECONDS, 10);
    }

    #[test]
    fn test_with_defaults_uses_default_timeout() {
        let env = Env::default();
        let config = SdkConfig::with_defaults(
            Network::Testnet,
            String::from_str(&env, "anchor.stellar.org"),
        );
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.timeout_seconds, 10);
        assert_eq!(config.retry_attempts, 3);
    }

    #[test]
    fn test_custom_timeout_overrides_default() {
        let env = Env::default();
        let config = SdkConfig::new(
            Network::Testnet,
            25,
            3,
            String::from_str(&env, "anchor.stellar.org"),
        );
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.timeout_seconds, 25);
    }
}
