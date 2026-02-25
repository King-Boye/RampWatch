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
}
