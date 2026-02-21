# Secure Credential Injection

## Overview

AnchorKit implements secure credential management to handle API secrets and authentication tokens at runtime instead of storing them in configuration files. This approach follows security best practices by:

- **Never storing plaintext credentials** in config files or contract storage
- **Runtime injection** from secure environment variables
- **Automatic credential rotation** based on configurable policies
- **Encrypted storage** when credentials must be persisted
- **Audit trail** for all credential operations

## Architecture

### Components

1. **CredentialManager**: Core module for credential validation and injection
2. **SecureCredential**: Encrypted credential storage structure
3. **RuntimeCredential**: Temporary credential holder (never persisted)
4. **CredentialPolicy**: Rotation and security policy configuration

### Credential Types

AnchorKit supports multiple authentication methods:

- **ApiKey**: API key-based authentication
- **BearerToken**: OAuth 2.0 bearer tokens
- **BasicAuth**: HTTP Basic Authentication
- **OAuth2**: Full OAuth 2.0 flow tokens
- **MutualTLS**: Certificate-based mutual TLS

## Usage Guide

### 1. Environment Variable Setup

Set credentials as environment variables before contract deployment:

```bash
# API Key example
export ANCHOR_KYC_PROVIDER_API_KEY="sk_live_abc123..."

# Bearer Token example
export ANCHOR_BANK_INTEGRATION_TOKEN="Bearer eyJhbGc..."

# Basic Auth example (base64 encoded username:password)
export ANCHOR_COMPLIANCE_AUTH="dXNlcm5hbWU6cGFzc3dvcmQ="

# OAuth2 Token
export ANCHOR_PAYMENT_OAUTH_TOKEN="ya29.a0AfH6SM..."
```

### 2. Runtime Injection

Inject credentials at contract initialization:

```rust
use anchorkit::{CredentialManager, CredentialType};

// During contract initialization
let kyc_credential = CredentialManager::inject_runtime_credential(
    &env,
    kyc_provider_address,
    CredentialType::ApiKey,
    String::from_str(&env, "https://kyc-provider.example.com/verify")
);

// Use the credential for API calls
// Note: RuntimeCredential is never persisted to storage
```

### 3. Credential Policy Configuration

Set rotation policies for each attestor:

```rust
// Set policy requiring rotation every 30 days
contract.set_credential_policy(
    attestor_address,
    86400 * 30,  // 30 days in seconds
    true         // require encryption
)?;
```

### 4. Encrypted Storage (Optional)

For credentials that must be stored, use encrypted storage:

```rust
// Encrypt credential before storage (use your encryption method)
let encrypted_value = encrypt_credential(&api_key);

// Store encrypted credential
contract.store_encrypted_credential(
    attestor_address,
    CredentialType::ApiKey,
    encrypted_value,
    expiry_timestamp
)?;
```

### 5. Credential Rotation

Check and rotate credentials automatically:

```rust
// Check if rotation is needed
let needs_rotation = contract.check_credential_rotation(attestor_address)?;

if needs_rotation {
    // Rotate credential
    let new_encrypted_value = encrypt_credential(&new_api_key);
    contract.rotate_credential(
        attestor_address,
        CredentialType::ApiKey,
        new_encrypted_value,
        new_expiry_timestamp
    )?;
}
```

### 6. Credential Revocation

Revoke credentials immediately when needed:

```rust
// Revoke credential (admin only)
contract.revoke_credential(attestor_address)?;
```

## Security Best Practices

### DO ✅

1. **Use environment variables** for credential injection
2. **Encrypt credentials** before storing in contract storage
3. **Rotate credentials regularly** based on policy
4. **Use short expiry times** for sensitive operations
5. **Audit credential access** through session logging
6. **Revoke immediately** when compromise is suspected
7. **Use separate credentials** for each environment (dev/staging/prod)

### DON'T ❌

1. **Never commit credentials** to version control
2. **Never store plaintext credentials** in config files
3. **Never log credential values** in application logs
4. **Never share credentials** between attestors
5. **Never use the same credential** across environments
6. **Never skip credential validation**
7. **Never disable encryption** for production credentials

## Configuration Examples

### Development Environment

```bash
# .env.development (never commit this file)
ANCHOR_KYC_API_KEY="sk_test_dev123..."
ANCHOR_BANK_TOKEN="Bearer test_token..."
CREDENTIAL_ROTATION_DAYS=7
REQUIRE_ENCRYPTION=false
```

### Production Environment

```bash
# Set via secure secret management (AWS Secrets Manager, HashiCorp Vault, etc.)
ANCHOR_KYC_API_KEY="sk_live_prod_secure_key..."
ANCHOR_BANK_TOKEN="Bearer prod_token..."
CREDENTIAL_ROTATION_DAYS=30
REQUIRE_ENCRYPTION=true
```

## Integration with External Services

### Example: KYC Provider Integration

```rust
// 1. Inject credential at runtime
let kyc_credential = CredentialManager::inject_runtime_credential(
    &env,
    kyc_provider_address,
    CredentialType::ApiKey,
    String::from_str(&env, "https://kyc-api.example.com")
);

// 2. Use credential for API authentication
// (Implementation depends on your HTTP client)
let response = http_client
    .post(&kyc_credential.endpoint)
    .header("X-API-Key", get_decrypted_credential(&kyc_credential))
    .body(kyc_request)
    .send()?;
```

### Example: Bank Integration with OAuth2

```rust
// 1. Store OAuth2 token (encrypted)
let encrypted_token = encrypt_oauth_token(&oauth_token);
contract.store_encrypted_credential(
    bank_address,
    CredentialType::OAuth2,
    encrypted_token,
    token_expiry
)?;

// 2. Check expiry before use
let needs_rotation = contract.check_credential_rotation(bank_address)?;
if needs_rotation {
    // Refresh OAuth2 token
    let new_token = refresh_oauth_token(&refresh_token)?;
    let encrypted_new_token = encrypt_oauth_token(&new_token);
    contract.rotate_credential(
        bank_address,
        CredentialType::OAuth2,
        encrypted_new_token,
        new_token_expiry
    )?;
}
```

## Credential Rotation Automation

### Automated Rotation Script

```bash
#!/bin/bash
# rotate_credentials.sh

# Check all attestors for rotation needs
for attestor in $(get_attestor_list); do
    needs_rotation=$(check_rotation_status $attestor)
    
    if [ "$needs_rotation" = "true" ]; then
        echo "Rotating credential for $attestor"
        
        # Generate new credential
        new_credential=$(generate_new_credential $attestor)
        
        # Encrypt and rotate
        encrypted=$(encrypt_credential $new_credential)
        rotate_credential $attestor $encrypted
        
        # Update external service
        update_external_service $attestor $new_credential
        
        echo "Rotation complete for $attestor"
    fi
done
```

### Cron Job Setup

```bash
# Run credential rotation check daily at 2 AM
0 2 * * * /path/to/rotate_credentials.sh >> /var/log/credential_rotation.log 2>&1
```

## Monitoring and Alerts

### Key Metrics to Monitor

1. **Credential age**: Time since last rotation
2. **Rotation failures**: Failed rotation attempts
3. **Expiry warnings**: Credentials approaching expiry
4. **Access patterns**: Unusual credential usage
5. **Revocation events**: Credential revocations

### Alert Configuration

```toml
[[monitoring.alerts]]
condition = "credential_rotation_failed"
severity = "critical"
recipients = ["security@example.com"]

[[monitoring.alerts]]
condition = "credential_expiring_soon"
severity = "warning"
recipients = ["ops@example.com"]
threshold_days = 7

[[monitoring.alerts]]
condition = "credential_expired"
severity = "critical"
recipients = ["security@example.com", "ops@example.com"]
```

## API Reference

### Contract Methods

#### `set_credential_policy`
```rust
pub fn set_credential_policy(
    env: Env,
    attestor: Address,
    rotation_interval_seconds: u64,
    require_encryption: bool,
) -> Result<(), Error>
```
Set credential rotation policy for an attestor (admin only).

#### `get_credential_policy`
```rust
pub fn get_credential_policy(
    env: Env,
    attestor: Address,
) -> Result<CredentialPolicy, Error>
```
Get credential policy for an attestor.

#### `store_encrypted_credential`
```rust
pub fn store_encrypted_credential(
    env: Env,
    attestor: Address,
    credential_type: CredentialType,
    encrypted_value: Bytes,
    expires_at: u64,
) -> Result<(), Error>
```
Store encrypted credential (admin only).

#### `rotate_credential`
```rust
pub fn rotate_credential(
    env: Env,
    attestor: Address,
    credential_type: CredentialType,
    new_encrypted_value: Bytes,
    expires_at: u64,
) -> Result<(), Error>
```
Rotate credential with new encrypted value (admin only).

#### `check_credential_rotation`
```rust
pub fn check_credential_rotation(
    env: Env,
    attestor: Address,
) -> Result<bool, Error>
```
Check if credential needs rotation based on policy.

#### `revoke_credential`
```rust
pub fn revoke_credential(
    env: Env,
    attestor: Address,
) -> Result<(), Error>
```
Revoke credential immediately (admin only).

## Error Codes

| Code | Error | Description |
|------|-------|-------------|
| 25 | InvalidCredentialFormat | Credential format validation failed |
| 26 | CredentialExpired | Credential has expired |
| 27 | CredentialRotationRequired | Credential must be rotated |
| 28 | CredentialNotFound | Credential not found for attestor |
| 29 | InsecureCredentialStorage | Attempted insecure credential storage |

## Migration Guide

### Migrating from Config-Based Credentials

1. **Identify all credentials** in config files
2. **Move to environment variables** or secret management
3. **Update deployment scripts** to inject credentials
4. **Set rotation policies** for each attestor
5. **Remove credentials** from config files
6. **Test credential injection** in staging
7. **Deploy to production** with monitoring

### Example Migration

Before (insecure):
```toml
[[attestors.registry]]
name = "kyc-provider"
api_key = "sk_live_abc123..."  # ❌ Plaintext in config
```

After (secure):
```bash
# Environment variable
export ANCHOR_KYC_API_KEY="sk_live_abc123..."

# Runtime injection in deployment script
inject_credential "kyc-provider" "$ANCHOR_KYC_API_KEY"
```

## Compliance Considerations

### Regulatory Requirements

- **PCI DSS**: Credential encryption and rotation
- **SOC 2**: Access controls and audit logging
- **GDPR**: Secure credential handling for personal data access
- **ISO 27001**: Credential lifecycle management

### Audit Trail

All credential operations are logged:
- Policy changes
- Credential storage
- Rotation events
- Revocation events
- Access attempts

## Troubleshooting

### Common Issues

**Issue**: Credential validation fails
```
Error: InvalidCredentialFormat (25)
```
**Solution**: Check credential length and format requirements for the credential type.

**Issue**: Credential expired
```
Error: CredentialExpired (26)
```
**Solution**: Rotate credential immediately using `rotate_credential`.

**Issue**: Credential not found
```
Error: CredentialNotFound (28)
```
**Solution**: Ensure credential is injected or stored before use.

## Support

For questions or issues with credential management:
1. Review this documentation
2. Check error codes in `src/errors.rs`
3. Examine credential tests in `src/credentials.rs`
4. Contact security team for credential-related incidents

## License

See main project LICENSE file.
