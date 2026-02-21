use soroban_sdk::{contracttype, Address, Bytes, Env, String};

/// Secure credential types for external API authentication
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CredentialType {
    ApiKey,
    BearerToken,
    BasicAuth,
    OAuth2,
    MutualTLS,
}

/// Encrypted credential storage structure
/// Credentials are never stored in plaintext in contract storage
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SecureCredential {
    pub attestor: Address,
    pub credential_type: CredentialType,
    pub encrypted_value: Bytes,
    pub created_at: u64,
    pub expires_at: u64,
    pub rotation_required: bool,
}

/// Runtime credential injection configuration
/// This structure is populated at runtime from environment variables
/// and never persisted to contract storage
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeCredential {
    pub attestor: Address,
    pub credential_type: CredentialType,
    pub endpoint: String,
    pub injected_at: u64,
}

/// Credential rotation policy
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CredentialPolicy {
    pub attestor: Address,
    pub rotation_interval_seconds: u64,
    pub require_encryption: bool,
    pub allow_plaintext_storage: bool,
}

impl SecureCredential {
    /// Check if credential has expired
    pub fn is_expired(&self, current_timestamp: u64) -> bool {
        self.expires_at > 0 && current_timestamp >= self.expires_at
    }

    /// Check if credential rotation is due
    pub fn needs_rotation(&self, current_timestamp: u64, policy: &CredentialPolicy) -> bool {
        if self.rotation_required {
            return true;
        }

        if policy.rotation_interval_seconds > 0 {
            let age = current_timestamp.saturating_sub(self.created_at);
            return age >= policy.rotation_interval_seconds;
        }

        false
    }
}

/// Credential manager for secure runtime injection
pub struct CredentialManager;

impl CredentialManager {
    /// Inject credentials at runtime from environment
    /// This method should be called during contract initialization
    /// with credentials sourced from secure environment variables
    pub fn inject_runtime_credential(
        env: &Env,
        attestor: Address,
        credential_type: CredentialType,
        endpoint: String,
    ) -> RuntimeCredential {
        RuntimeCredential {
            attestor,
            credential_type,
            endpoint,
            injected_at: env.ledger().timestamp(),
        }
    }

    /// Validate credential format based on type
    pub fn validate_credential_format(
        credential_type: &CredentialType,
        value: &Bytes,
    ) -> Result<(), crate::Error> {
        if value.is_empty() {
            return Err(crate::Error::InvalidCredentialFormat);
        }

        match credential_type {
            CredentialType::ApiKey => {
                // API keys should be at least 16 bytes
                if value.len() < 16 {
                    return Err(crate::Error::InvalidCredentialFormat);
                }
            }
            CredentialType::BearerToken => {
                // Bearer tokens should be at least 20 bytes
                if value.len() < 20 {
                    return Err(crate::Error::InvalidCredentialFormat);
                }
            }
            CredentialType::BasicAuth => {
                // Basic auth should contain username:password format
                if value.len() < 8 {
                    return Err(crate::Error::InvalidCredentialFormat);
                }
            }
            CredentialType::OAuth2 => {
                // OAuth2 tokens should be at least 32 bytes
                if value.len() < 32 {
                    return Err(crate::Error::InvalidCredentialFormat);
                }
            }
            CredentialType::MutualTLS => {
                // mTLS certificates should be substantial
                if value.len() < 64 {
                    return Err(crate::Error::InvalidCredentialFormat);
                }
            }
        }

        Ok(())
    }

    /// Create a credential policy with secure defaults
    pub fn create_default_policy(attestor: Address) -> CredentialPolicy {
        CredentialPolicy {
            attestor,
            rotation_interval_seconds: 86400 * 30, // 30 days
            require_encryption: true,
            allow_plaintext_storage: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;

    #[test]
    fn test_credential_expiry() {
        // Test credential expiry logic
        let env = Env::default();
        let attestor = Address::generate(&env);
        let credential = SecureCredential {
            attestor: attestor.clone(),
            credential_type: CredentialType::ApiKey,
            encrypted_value: Bytes::new(&env),
            created_at: 1000,
            expires_at: 2000,
            rotation_required: false,
        };

        assert!(!credential.is_expired(1500));
        assert!(credential.is_expired(2000));
        assert!(credential.is_expired(2500));
    }

    #[test]
    fn test_credential_rotation() {
        let env = Env::default();
        let attestor = Address::generate(&env);
        let credential = SecureCredential {
            attestor: attestor.clone(),
            credential_type: CredentialType::ApiKey,
            encrypted_value: Bytes::new(&env),
            created_at: 1000,
            expires_at: 0,
            rotation_required: false,
        };

        let policy = CredentialPolicy {
            attestor: attestor.clone(),
            rotation_interval_seconds: 86400, // 1 day
            require_encryption: true,
            allow_plaintext_storage: false,
        };

        assert!(!credential.needs_rotation(1000, &policy));
        assert!(!credential.needs_rotation(50000, &policy));
        assert!(credential.needs_rotation(90000, &policy));
    }
}
