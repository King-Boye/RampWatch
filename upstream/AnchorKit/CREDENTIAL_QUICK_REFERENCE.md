# Secure Credential Management - Quick Reference

## 🚀 Quick Start

### 1. Store Credential in Secret Manager

```bash
# AWS Secrets Manager
aws secretsmanager create-secret \
    --name anchorkit/attestor/api-key \
    --secret-string "your-secret-key"

# HashiCorp Vault
vault kv put secret/anchorkit/attestor api_key="your-secret-key"
```

### 2. Set Rotation Policy

```bash
soroban contract invoke \
    --id $CONTRACT_ID \
    --source admin-account \
    -- set_credential_policy \
    --attestor $ATTESTOR_ADDRESS \
    --rotation_interval_seconds 2592000 \
    --require_encryption true
```

### 3. Store Encrypted Credential

```bash
# Fetch and encrypt
SECRET=$(aws secretsmanager get-secret-value --secret-id anchorkit/attestor/api-key --query SecretString --output text)
ENCRYPTED=$(echo -n "$SECRET" | openssl enc -aes-256-cbc -a -salt -pass pass:$ENCRYPTION_KEY)

# Store in contract
soroban contract invoke \
    --id $CONTRACT_ID \
    --source admin-account \
    -- store_encrypted_credential \
    --attestor $ATTESTOR_ADDRESS \
    --credential_type ApiKey \
    --encrypted_value "$ENCRYPTED" \
    --expires_at $(($(date +%s) + 2592000))
```

### 4. Check Rotation Status

```bash
soroban contract invoke \
    --id $CONTRACT_ID \
    -- check_credential_rotation \
    --attestor $ATTESTOR_ADDRESS
```

## 📋 Contract Methods

| Method | Description | Admin Only |
|--------|-------------|------------|
| `set_credential_policy` | Set rotation policy | ✅ |
| `get_credential_policy` | Get rotation policy | ❌ |
| `store_encrypted_credential` | Store encrypted credential | ✅ |
| `rotate_credential` | Rotate credential | ✅ |
| `check_credential_rotation` | Check if rotation needed | ❌ |
| `revoke_credential` | Revoke credential | ✅ |

## 🔐 Credential Types

| Type | Use Case | Min Length |
|------|----------|------------|
| `ApiKey` | API key authentication | 16 bytes |
| `BearerToken` | OAuth bearer tokens | 20 bytes |
| `BasicAuth` | HTTP Basic Auth | 8 bytes |
| `OAuth2` | OAuth 2.0 tokens | 32 bytes |
| `MutualTLS` | Certificate-based auth | 64 bytes |

## ⚠️ Error Codes

| Code | Error | Solution |
|------|-------|----------|
| 25 | InvalidCredentialFormat | Check credential length/format |
| 26 | CredentialExpired | Rotate credential immediately |
| 27 | CredentialRotationRequired | Rotate per policy |
| 28 | CredentialNotFound | Store credential first |
| 29 | InsecureCredentialStorage | Enable encryption |

## 🔄 Rotation Intervals

| Interval | Seconds | Use Case |
|----------|---------|----------|
| 1 day | 86400 | High-security |
| 7 days | 604800 | Standard |
| 30 days | 2592000 | Low-risk |
| 90 days | 7776000 | Minimal |

## ✅ Security Checklist

- [ ] Credentials in secret manager (not config files)
- [ ] Rotation policy configured
- [ ] Encryption enabled
- [ ] Monitoring set up
- [ ] Automated rotation scheduled
- [ ] Incident response plan documented

## 🛠️ Common Commands

### Fetch from AWS Secrets Manager
```bash
aws secretsmanager get-secret-value \
    --secret-id anchorkit/attestor/api-key \
    --query SecretString --output text
```

### Fetch from HashiCorp Vault
```bash
vault kv get -field=api_key secret/anchorkit/attestor
```

### Encrypt Credential
```bash
echo -n "$CREDENTIAL" | openssl enc -aes-256-cbc -a -salt -pass pass:$KEY
```

### Decrypt Credential
```bash
echo "$ENCRYPTED" | openssl enc -aes-256-cbc -d -a -pass pass:$KEY
```

## 📊 Monitoring

### Key Metrics
- Credential age
- Rotation failures
- Expiry warnings
- Access patterns

### Alert Thresholds
- Warning: 7 days before expiry
- Critical: 1 day before expiry
- Critical: Rotation failure

## 🔗 Resources

- **Full Guide**: [SECURE_CREDENTIALS.md](./SECURE_CREDENTIALS.md)
- **Deployment**: [DEPLOYMENT_WITH_CREDENTIALS.md](./DEPLOYMENT_WITH_CREDENTIALS.md)
- **Example**: [examples/credential_management.sh](./examples/credential_management.sh)
- **Config Guidelines**: [configs/CREDENTIAL_SECURITY.md](./configs/CREDENTIAL_SECURITY.md)

## 💡 Best Practices

### DO ✅
- Use secret managers
- Encrypt before storage
- Rotate regularly
- Monitor credential age
- Revoke on compromise

### DON'T ❌
- Store in config files
- Commit to git
- Log credential values
- Share between environments
- Skip validation

## 🚨 Emergency Procedures

### Credential Compromise
```bash
# 1. Revoke immediately
soroban contract invoke --id $CONTRACT_ID --source admin-account \
    -- revoke_credential --attestor $ATTESTOR_ADDRESS

# 2. Generate new credential
NEW_CRED=$(generate_new_credential)

# 3. Encrypt and store
ENCRYPTED=$(encrypt_credential "$NEW_CRED")
soroban contract invoke --id $CONTRACT_ID --source admin-account \
    -- store_encrypted_credential --attestor $ATTESTOR_ADDRESS \
    --credential_type ApiKey --encrypted_value "$ENCRYPTED" \
    --expires_at $(($(date +%s) + 2592000))

# 4. Update external service
update_external_service $ATTESTOR_ADDRESS "$NEW_CRED"
```

## 📞 Support

- Documentation: See links above
- Issues: Check error codes
- Security: Contact security team immediately
- Questions: Review examples and tests

---

**Quick Reference Version**: 1.0  
**Last Updated**: February 2026
