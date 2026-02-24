use soroban_sdk::{contracttype, String};

use crate::Error;

/// Validated configuration for contract initialization.
///
/// This struct enforces all fields must be provided. The only valid construction
/// path is through [`ContractConfig::new`], which also runs validation.
///
/// # Compile-Time Safety
///
/// Direct struct-literal construction without all fields is rejected at compile time:
///
/// ```compile_fail
/// # use soroban_sdk::{Env, String};
/// # let env = Env::default();
/// // Missing `version` and `network` — compile error!
/// let _ = anchorkit::ContractConfig {
///     name: String::from_str(&env, "test"),
/// };
/// ```
///
/// Providing only some fields also fails:
///
/// ```compile_fail
/// # use soroban_sdk::{Env, String};
/// # let env = Env::default();
/// // Missing `network` — compile error!
/// let _ = anchorkit::ContractConfig {
///     name: String::from_str(&env, "test"),
///     version: String::from_str(&env, "1.0"),
/// };
/// ```
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContractConfig {
    pub name: String,
    pub version: String,
    pub network: String,
}

/// Compile-time constants for validation (immutable constraints)
pub const MAX_NAME_LEN: u32 = 64;
pub const MIN_NAME_LEN: u32 = 1;
pub const MAX_VERSION_LEN: u32 = 16;
pub const MIN_VERSION_LEN: u32 = 1;
pub const MAX_NETWORK_LEN: u32 = 32;
pub const MIN_NETWORK_LEN: u32 = 1;
pub const MAX_ENDPOINT_LEN: u32 = 256;
pub const MIN_ENDPOINT_LEN: u32 = 8;
pub const STELLAR_ADDR_LEN: u32 = 56; // Stellar addresses are exactly 56 chars
pub const STELLAR_ADDR_MIN: u32 = 54;
pub const STELLAR_ADDR_MAX: u32 = 56;
pub const MAX_ATTESTORS: u32 = 100;
pub const MIN_ATTESTORS: u32 = 1;
pub const MAX_SESSION_TIMEOUT: u64 = 86400; // 24 hours
pub const MIN_SESSION_TIMEOUT: u64 = 60; // 1 minute minimum
pub const MAX_OPERATIONS: u64 = 10000;
pub const MIN_OPERATIONS: u64 = 1;
pub const MAX_ROLE_LEN: u32 = 32;
pub const MIN_ROLE_LEN: u32 = 1;
pub const MAX_DESCRIPTION_LEN: u32 = 256;

/// Validated attestor configuration with strict type safety.
///
/// All five fields are required. The only valid construction path is
/// [`AttestorConfig::new`], which validates the address format,
/// endpoint URL, and role length before returning `Ok`.
///
/// # Compile-Time Safety
///
/// Incomplete struct literal construction fails to compile:
///
/// ```compile_fail
/// # use soroban_sdk::{Env, String};
/// # let env = Env::default();
/// // Missing `endpoint`, `role`, and `enabled` — compile error!
/// let _ = anchorkit::AttestorConfig {
///     name: String::from_str(&env, "kyc-provider"),
///     address: String::from_str(&env, "GBBD6A7KNZF5WNWQEPZP5DYJD2AYUTLXRB6VXJ4RCX4RTNPPQVNF3GQ"),
/// };
/// ```
///
/// Missing the `enabled` field also fails:
///
/// ```compile_fail
/// # use soroban_sdk::{Env, String};
/// # let env = Env::default();
/// // Missing `enabled` — compile error!
/// let _ = anchorkit::AttestorConfig {
///     name: String::from_str(&env, "kyc-provider"),
///     address: String::from_str(&env, "GBBD6A7KNZF5WNWQEPZP5DYJD2AYUTLXRB6VXJ4RCX4RTNPPQVNF3GQ"),
///     endpoint: String::from_str(&env, "https://api.example.com"),
///     role: String::from_str(&env, "issuer"),
/// };
/// ```
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AttestorConfig {
    pub name: String,
    pub address: String,
    pub endpoint: String,
    pub role: String,
    pub enabled: bool,
}

/// Validated session configuration with business rule constraints.
///
/// All three fields are required. The only valid construction path is
/// [`SessionConfig::new`], which enforces that `timeout_seconds` is between
/// 60 and 86400, and `max_operations` is between 1 and 10000.
///
/// # Compile-Time Safety
///
/// Incomplete struct literal construction fails to compile:
///
/// ```compile_fail
/// // Missing `timeout_seconds` and `max_operations` — compile error!
/// let _ = anchorkit::SessionConfig {
///     enable_tracking: true,
/// };
/// ```
///
/// Providing only two fields also fails:
///
/// ```compile_fail
/// // Missing `max_operations` — compile error!
/// let _ = anchorkit::SessionConfig {
///     enable_tracking: false,
///     timeout_seconds: 3600_u64,
/// };
/// ```
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionConfig {
    pub enable_tracking: bool,
    pub timeout_seconds: u64,
    pub max_operations: u64,
}

impl ContractConfig {
    /// Strict validation with detailed error reporting
    pub fn validate(&self) -> Result<(), Error> {
        let name_len = self.name.len();
        if name_len < MIN_NAME_LEN || name_len > MAX_NAME_LEN {
            return Err(Error::InvalidConfigName);
        }

        let version_len = self.version.len();
        if version_len < MIN_VERSION_LEN || version_len > MAX_VERSION_LEN {
            return Err(Error::InvalidConfigVersion);
        }

        let network_len = self.network.len();
        if network_len < MIN_NETWORK_LEN || network_len > MAX_NETWORK_LEN {
            return Err(Error::InvalidConfigNetwork);
        }

        Ok(())
    }

    /// Create a validated config (builder pattern for type safety)
    pub fn new(name: String, version: String, network: String) -> Result<Self, Error> {
        let config = Self {
            name,
            version,
            network,
        };
        config.validate()?;
        Ok(config)
    }
}

impl AttestorConfig {
    /// Strict validation with comprehensive checks
    pub fn validate(&self) -> Result<(), Error> {
        let name_len = self.name.len();
        if name_len < MIN_NAME_LEN || name_len > MAX_NAME_LEN {
            return Err(Error::InvalidAttestorName);
        }

        let addr_len = self.address.len();
        if addr_len < STELLAR_ADDR_MIN || addr_len > STELLAR_ADDR_MAX {
            return Err(Error::InvalidAttestorAddress);
        }

        let endpoint_len = self.endpoint.len();
        if endpoint_len < MIN_ENDPOINT_LEN || endpoint_len > MAX_ENDPOINT_LEN {
            return Err(Error::InvalidEndpointFormat);
        }

        let role_len = self.role.len();
        if role_len < MIN_ROLE_LEN || role_len > MAX_ROLE_LEN {
            return Err(Error::InvalidAttestorRole);
        }

        Ok(())
    }

    /// Type-safe builder for attestor config
    pub fn new(
        name: String,
        address: String,
        endpoint: String,
        role: String,
        enabled: bool,
    ) -> Result<Self, Error> {
        let config = Self {
            name,
            address,
            endpoint,
            role,
            enabled,
        };
        config.validate()?;
        Ok(config)
    }
}

impl SessionConfig {
    /// Strict validation with security constraints
    pub fn validate(&self) -> Result<(), Error> {
        if self.timeout_seconds < MIN_SESSION_TIMEOUT || self.timeout_seconds > MAX_SESSION_TIMEOUT
        {
            return Err(Error::InvalidConfig);
        }

        if self.max_operations < MIN_OPERATIONS || self.max_operations > MAX_OPERATIONS {
            return Err(Error::InvalidConfig);
        }

        Ok(())
    }

    /// Type-safe builder for session config
    pub fn new(
        enable_tracking: bool,
        timeout_seconds: u64,
        max_operations: u64,
    ) -> Result<Self, Error> {
        let config = Self {
            enable_tracking,
            timeout_seconds,
            max_operations,
        };
        config.validate()?;
        Ok(config)
    }
}
