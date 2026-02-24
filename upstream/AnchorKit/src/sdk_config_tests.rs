#[cfg(test)]
mod sdk_config_tests {
    use crate::types::*;
    use soroban_sdk::{Env, String, Vec};

    #[test]
    fn test_sdk_config_validation_valid() {
        let env = Env::default();
        
        let config = SdkConfig {
            network: NetworkType::Testnet,
            anchor_domain: String::from_str(&env, "example.com"),
            timeout_seconds: 30,
            custom_headers: Vec::new(&env),
        };
        
        assert!(config.validate(), "Valid config should pass validation");
    }

    #[test]
    fn test_sdk_config_validation_domain_too_short() {
        let env = Env::default();
        
        let config = SdkConfig {
            network: NetworkType::Mainnet,
            anchor_domain: String::from_str(&env, "ab"),
            timeout_seconds: 30,
            custom_headers: Vec::new(&env),
        };
        
        assert!(!config.validate(), "Domain too short should fail validation");
    }

    #[test]
    fn test_sdk_config_validation_timeout_too_low() {
        let env = Env::default();
        
        let config = SdkConfig {
            network: NetworkType::Testnet,
            anchor_domain: String::from_str(&env, "example.com"),
            timeout_seconds: 0,
            custom_headers: Vec::new(&env),
        };
        
        assert!(!config.validate(), "Timeout of 0 should fail validation");
    }

    #[test]
    fn test_sdk_config_validation_timeout_too_high() {
        let env = Env::default();
        
        let config = SdkConfig {
            network: NetworkType::Testnet,
            anchor_domain: String::from_str(&env, "example.com"),
            timeout_seconds: 301,
            custom_headers: Vec::new(&env),
        };
        
        assert!(!config.validate(), "Timeout > 300 should fail validation");
    }

    #[test]
    fn test_sdk_config_with_custom_headers() {
        let env = Env::default();
        
        let mut headers = Vec::new(&env);
        headers.push_back(HttpHeader {
            key: String::from_str(&env, "Authorization"),
            value: String::from_str(&env, "Bearer token123"),
        });
        headers.push_back(HttpHeader {
            key: String::from_str(&env, "X-Custom-Header"),
            value: String::from_str(&env, "custom-value"),
        });
        
        let config = SdkConfig {
            network: NetworkType::Mainnet,
            anchor_domain: String::from_str(&env, "anchor.stellar.org"),
            timeout_seconds: 60,
            custom_headers: headers,
        };
        
        assert!(config.validate(), "Config with valid headers should pass");
    }

    #[test]
    fn test_sdk_config_too_many_headers() {
        let env = Env::default();
        
        let mut headers = Vec::new(&env);
        for i in 0..21 {
            headers.push_back(HttpHeader {
                key: String::from_str(&env, &format!("Header-{}", i)),
                value: String::from_str(&env, "value"),
            });
        }
        
        let config = SdkConfig {
            network: NetworkType::Testnet,
            anchor_domain: String::from_str(&env, "example.com"),
            timeout_seconds: 30,
            custom_headers: headers,
        };
        
        assert!(!config.validate(), "More than 20 headers should fail validation");
    }

    #[test]
    fn test_sdk_config_header_key_too_long() {
        let env = Env::default();
        
        let mut headers = Vec::new(&env);
        let long_key = "a".repeat(65);
        headers.push_back(HttpHeader {
            key: String::from_str(&env, &long_key),
            value: String::from_str(&env, "value"),
        });
        
        let config = SdkConfig {
            network: NetworkType::Testnet,
            anchor_domain: String::from_str(&env, "example.com"),
            timeout_seconds: 30,
            custom_headers: headers,
        };
        
        assert!(!config.validate(), "Header key > 64 chars should fail validation");
    }

    #[test]
    fn test_network_type_enum() {
        assert_eq!(NetworkType::Testnet as u32, 1);
        assert_eq!(NetworkType::Mainnet as u32, 2);
    }
}
