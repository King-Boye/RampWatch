# Credential Security Guidelines

## ⚠️ IMPORTANT: Never Store Credentials in Config Files

The configuration files in this directory (`.toml` and `.json`) are for **structural configuration only**. They should NEVER contain:

- API keys
- Authentication tokens
- Passwords
- Private keys
- OAuth secrets
- Any other sensitive credentials

## What Belongs in Config Files ✅

Config files should only contain:

- Attestor addresses (public Stellar addresses)
- Endpoint URLs (without credentials)
- Service configurations
- Timeout settings
- Rate limits
- Feature flags
- Network settings

## What Does NOT Belong in Config Files ❌

- `api_key = "sk_live_..."`  ❌
- `password = "secret123"`  ❌
- `token = "Bearer eyJ..."`  ❌
- `private_key = "SABC..."`  ❌
- `client_secret = "abc123"`  ❌

## Correct Approach: Runtime Injection

### Step 1: Store Credentials Securely

Use a secret management system:

```bash
# AWS Secrets Manager
aws secretsmanager create-secret \
    --name anchorkit/kyc-provider/api-key \
    --secret-string "sk_live_secure_key..."

# HashiCorp Vault
vault kv put secret/anchorkit/kyc-provider \
    api_key="sk_live_secure_key..."

# Kubernetes Secrets
kubectl create secret generic anchorkit-credentials \
    --from-literal=kyc-api-key="sk_live_secure_key..."
```

### Step 2: Inject at Runtime

```bash
# Fetch from secret manager
export KYC_API_KEY=$(aws secretsmanager get-secret-value \
    --secret-id anchorkit/kyc-provider/api-key \
    --query SecretString --output text)

# Use in deployment
./deploy_with_credentials.sh
```

### Step 3: Use in Application

```rust
// Credentials are injected at runtime, never stored in config
let credential = CredentialManager::inject_runtime_credential(
    &env,
    attestor_address,
    CredentialType::ApiKey,
    endpoint_url
);
```

## Example: Secure Configuration

### ✅ CORRECT: Config File (configs/fiat-on-off-ramp.toml)

```toml
[[attestors.registry]]
name = "kyc-provider"
address = "GBBD6A7KNZF5WNWQEPZP5DYJD2AYUTLXRB6VXJ4RCX4RTNPPQVNF3GQ"
endpoint = "https://kyc-provider.example.com/verify"
role = "kyc-issuer"
enabled = true
# NOTE: API key is NOT stored here - injected at runtime
```

### ❌ INCORRECT: Config File with Credentials

```toml
[[attestors.registry]]
name = "kyc-provider"
address = "GBBD6A7KNZF5WNWQEPZP5DYJD2AYUTLXRB6VXJ4RCX4RTNPPQVNF3GQ"
endpoint = "https://kyc-provider.example.com/verify"
api_key = "sk_live_abc123..."  # ❌ NEVER DO THIS!
role = "kyc-issuer"
enabled = true
```

## Migration from Insecure Configs

If you have credentials in config files:

1. **Immediately remove them** from version control
2. **Rotate all exposed credentials** (assume they're compromised)
3. **Move to secret manager** (AWS Secrets Manager, Vault, etc.)
4. **Update deployment scripts** to inject at runtime
5. **Add to .gitignore** any files with credentials
6. **Audit git history** and remove credential commits

```bash
# Remove credentials from git history
git filter-branch --force --index-filter \
  "git rm --cached --ignore-unmatch configs/secrets.toml" \
  --prune-empty --tag-name-filter cat -- --all

# Force push (coordinate with team first!)
git push origin --force --all
```

## Environment-Specific Credentials

### Development

```bash
# .env.development (add to .gitignore)
ANCHOR_KYC_API_KEY="sk_test_dev123..."
ANCHOR_BANK_TOKEN="Bearer test_token..."
```

### Staging

```bash
# Stored in CI/CD secrets
ANCHOR_KYC_API_KEY="sk_test_staging456..."
ANCHOR_BANK_TOKEN="Bearer staging_token..."
```

### Production

```bash
# Stored in AWS Secrets Manager / Vault
# Fetched at deployment time
# Never logged or persisted
```

## Credential Rotation

Set up automatic rotation:

```bash
# Check rotation status
soroban contract invoke \
    --id $CONTRACT_ID \
    -- check_credential_rotation \
    --attestor $ATTESTOR_ADDRESS

# Rotate if needed
./rotate_credentials.sh $CONTRACT_ID $ATTESTOR_ADDRESS
```

## Security Checklist

Before deploying:

- [ ] No credentials in config files
- [ ] All credentials in secret manager
- [ ] Deployment script fetches credentials at runtime
- [ ] Credentials encrypted before storage
- [ ] Rotation policies configured
- [ ] Monitoring and alerting enabled
- [ ] .gitignore includes credential files
- [ ] Git history cleaned of credentials
- [ ] Team trained on secure practices
- [ ] Incident response plan documented

## Additional Resources

- [SECURE_CREDENTIALS.md](../SECURE_CREDENTIALS.md) - Complete credential management guide
- [DEPLOYMENT_WITH_CREDENTIALS.md](../DEPLOYMENT_WITH_CREDENTIALS.md) - Deployment guide
- [AWS Secrets Manager](https://aws.amazon.com/secrets-manager/)
- [HashiCorp Vault](https://www.vaultproject.io/)
- [Kubernetes Secrets](https://kubernetes.io/docs/concepts/configuration/secret/)

## Questions?

If you're unsure whether something should be in a config file:

1. Ask: "Would it be a security issue if this was public?"
2. If yes → Use secret manager and runtime injection
3. If no → Config file is OK

When in doubt, use runtime injection. It's always safer.
