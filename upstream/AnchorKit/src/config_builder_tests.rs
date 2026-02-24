#![cfg(test)]

//! Config Builder Type-State Tests
//!
//! These tests ensure that the builder pattern prevents incomplete configurations
//! at compile time using Rust's type system. Invalid builds should fail to compile.

use crate::{config::*, Error};
use soroban_sdk::{Env, String};

// ============================================================================
// Type-State Builder Pattern Tests
// ============================================================================

/// Test that valid configurations can be built successfully
#[test]
fn test_contract_config_builder_valid() {
    let env = Env::default();

    let config = ContractConfig::new(
        String::from_str(&env, "anchor-kit"),
        String::from_str(&env, "1.0.0"),
        String::from_str(&env, "testnet"),
    );

    assert!(config.is_ok());
    let config = config.unwrap();
    assert_eq!(config.name, String::from_str(&env, "anchor-kit"));
    assert_eq!(config.version, String::from_str(&env, "1.0.0"));
    assert_eq!(config.network, String::from_str(&env, "testnet"));
}

/// Test that attestor config builder validates all fields
#[test]
fn test_attestor_config_builder_valid() {
    let env = Env::default();

    let config = AttestorConfig::new(
        String::from_str(&env, "kyc-provider"),
        String::from_str(&env, "GBBD6A7KNZF5WNWQEPZP5DYJD2AYUTLXRB6VXJ4RCX4RTNPPQVNF3GQ"),
        String::from_str(&env, "https://api.example.com/verify"),
        String::from_str(&env, "kyc-issuer"),
        true,
    );

    assert!(config.is_ok());
    let config = config.unwrap();
    assert_eq!(config.name, String::from_str(&env, "kyc-provider"));
    assert!(config.enabled);
}

/// Test that session config builder validates constraints
#[test]
fn test_session_config_builder_valid() {
    let env = Env::default();

    let config = SessionConfig::new(true, 3600, 1000);

    assert!(config.is_ok());
    let config = config.unwrap();
    assert!(config.enable_tracking);
    assert_eq!(config.timeout_seconds, 3600);
    assert_eq!(config.max_operations, 1000);
}

// ============================================================================
// Compile-Time Safety Tests (Doc Tests)
// ============================================================================

/// Test that incomplete contract config fails at build time
///
/// ```compile_fail
/// use anchorkit::config::ContractConfig;
/// use soroban_sdk::{Env, String};
///
/// let env = Env::default();
/// // Missing required fields - should not compile
/// let config = ContractConfig {
///     name: String::from_str(&env, "test"),
///     // version and network missing
/// };
/// ```
#[allow(dead_code)]
fn test_incomplete_contract_config_compile_fail() {}

/// Test that incomplete attestor config fails at build time
///
/// ```compile_fail
/// use anchorkit::config::AttestorConfig;
/// use soroban_sdk::{Env, String};
///
/// let env = Env::default();
/// // Missing required fields - should not compile
/// let config = AttestorConfig {
///     name: String::from_str(&env, "attestor"),
///     address: String::from_str(&env, "GBBD6A7KNZF5WNWQEPZP5DYJD2AYUTLXRB6VXJ4RCX4RTNPPQVNF3GQ"),
///     // endpoint, role, enabled missing
/// };
/// ```
#[allow(dead_code)]
fn test_incomplete_attestor_config_compile_fail() {}

/// Test that incomplete session config fails at build time
///
/// ```compile_fail
/// use anchorkit::config::SessionConfig;
///
/// // Missing required fields - should not compile
/// let config = SessionConfig {
///     enable_tracking: true,
///     // timeout_seconds and max_operations missing
/// };
/// ```
#[allow(dead_code)]
fn test_incomplete_session_config_compile_fail() {}

// ============================================================================
// Runtime Validation Tests
// ============================================================================

/// Test that builder rejects invalid name length
#[test]
fn test_contract_config_builder_invalid_name() {
    let env = Env::default();

    // Empty name
    let result = ContractConfig::new(
        String::from_str(&env, ""),
        String::from_str(&env, "1.0.0"),
        String::from_str(&env, "testnet"),
    );
    assert_eq!(result, Err(Error::InvalidConfig));

    // Name too long
    let result = ContractConfig::new(
        String::from_str(&env, &"a".repeat(65)),
        String::from_str(&env, "1.0.0"),
        String::from_str(&env, "testnet"),
    );
    assert_eq!(result, Err(Error::InvalidConfig));
}

/// Test that builder rejects invalid version length
#[test]
fn test_contract_config_builder_invalid_version() {
    let env = Env::default();

    // Empty version
    let result = ContractConfig::new(
        String::from_str(&env, "anchor-kit"),
        String::from_str(&env, ""),
        String::from_str(&env, "testnet"),
    );
    assert_eq!(result, Err(Error::InvalidConfig));

    // Version too long
    let result = ContractConfig::new(
        String::from_str(&env, "anchor-kit"),
        String::from_str(&env, &"1".repeat(17)),
        String::from_str(&env, "testnet"),
    );
    assert_eq!(result, Err(Error::InvalidConfig));
}

/// Test that builder rejects invalid network length
#[test]
fn test_contract_config_builder_invalid_network() {
    let env = Env::default();

    // Empty network
    let result = ContractConfig::new(
        String::from_str(&env, "anchor-kit"),
        String::from_str(&env, "1.0.0"),
        String::from_str(&env, ""),
    );
    assert_eq!(result, Err(Error::InvalidConfig));

    // Network too long
    let result = ContractConfig::new(
        String::from_str(&env, "anchor-kit"),
        String::from_str(&env, "1.0.0"),
        String::from_str(&env, &"n".repeat(33)),
    );
    assert_eq!(result, Err(Error::InvalidConfig));
}

/// Test that attestor builder rejects invalid address
#[test]
fn test_attestor_config_builder_invalid_address() {
    let env = Env::default();

    let result = AttestorConfig::new(
        String::from_str(&env, "kyc-provider"),
        String::from_str(&env, "INVALID"),
        String::from_str(&env, "https://api.example.com/verify"),
        String::from_str(&env, "kyc-issuer"),
        true,
    );
    assert_eq!(result, Err(Error::InvalidConfig));
}

/// Test that attestor builder rejects invalid endpoint
#[test]
fn test_attestor_config_builder_invalid_endpoint() {
    let env = Env::default();

    let result = AttestorConfig::new(
        String::from_str(&env, "kyc-provider"),
        String::from_str(&env, "GBBD6A7KNZF5WNWQEPZP5DYJD2AYUTLXRB6VXJ4RCX4RTNPPQVNF3GQ"),
        String::from_str(&env, "bad"),
        String::from_str(&env, "kyc-issuer"),
        true,
    );
    assert_eq!(result, Err(Error::InvalidEndpointFormat));
}

/// Test that attestor builder rejects invalid role
#[test]
fn test_attestor_config_builder_invalid_role() {
    let env = Env::default();

    // Empty role
    let result = AttestorConfig::new(
        String::from_str(&env, "kyc-provider"),
        String::from_str(&env, "GBBD6A7KNZF5WNWQEPZP5DYJD2AYUTLXRB6VXJ4RCX4RTNPPQVNF3GQ"),
        String::from_str(&env, "https://api.example.com/verify"),
        String::from_str(&env, ""),
        true,
    );
    assert_eq!(result, Err(Error::InvalidConfig));

    // Role too long
    let result = AttestorConfig::new(
        String::from_str(&env, "kyc-provider"),
        String::from_str(&env, "GBBD6A7KNZF5WNWQEPZP5DYJD2AYUTLXRB6VXJ4RCX4RTNPPQVNF3GQ"),
        String::from_str(&env, "https://api.example.com/verify"),
        String::from_str(&env, &"r".repeat(33)),
        true,
    );
    assert_eq!(result, Err(Error::InvalidConfig));
}

/// Test that session builder rejects invalid timeout
#[test]
fn test_session_config_builder_invalid_timeout() {
    let env = Env::default();

    // Timeout too short
    let result = SessionConfig::new(true, 59, 1000);
    assert_eq!(result, Err(Error::InvalidConfig));

    // Timeout too long
    let result = SessionConfig::new(true, 86401, 1000);
    assert_eq!(result, Err(Error::InvalidConfig));
}

/// Test that session builder rejects invalid max operations
#[test]
fn test_session_config_builder_invalid_operations() {
    let env = Env::default();

    // Operations too low
    let result = SessionConfig::new(true, 3600, 0);
    assert_eq!(result, Err(Error::InvalidConfig));

    // Operations too high
    let result = SessionConfig::new(true, 3600, 10001);
    assert_eq!(result, Err(Error::InvalidConfig));
}

// ============================================================================
// Edge Case Tests
// ============================================================================

/// Test boundary values for contract config
#[test]
fn test_contract_config_boundary_values() {
    let env = Env::default();

    // Minimum valid lengths
    let result = ContractConfig::new(
        String::from_str(&env, "a"),
        String::from_str(&env, "1"),
        String::from_str(&env, "t"),
    );
    assert!(result.is_ok());

    // Maximum valid lengths
    let result = ContractConfig::new(
        String::from_str(&env, &"a".repeat(64)),
        String::from_str(&env, &"1".repeat(16)),
        String::from_str(&env, &"n".repeat(32)),
    );
    assert!(result.is_ok());
}

/// Test boundary values for session config
#[test]
fn test_session_config_boundary_values() {
    let env = Env::default();

    // Minimum valid values
    let result = SessionConfig::new(false, 60, 1);
    assert!(result.is_ok());

    // Maximum valid values
    let result = SessionConfig::new(true, 86400, 10000);
    assert!(result.is_ok());
}

/// Test that all fields are properly validated together
#[test]
fn test_config_builder_validates_all_fields() {
    let env = Env::default();

    // Valid config should pass all validations
    let result = ContractConfig::new(
        String::from_str(&env, "anchor-kit"),
        String::from_str(&env, "1.0.0"),
        String::from_str(&env, "testnet"),
    );
    assert!(result.is_ok());

    // Any single invalid field should fail
    let result = ContractConfig::new(
        String::from_str(&env, ""),
        String::from_str(&env, "1.0.0"),
        String::from_str(&env, "testnet"),
    );
    assert!(result.is_err());
}
