# Deployment Guide with Secure Credentials

## Overview

This guide demonstrates how to deploy AnchorKit with secure credential injection instead of storing secrets in configuration files.

## Prerequisites

- Soroban CLI installed
- Access to secret management system (AWS Secrets Manager, HashiCorp Vault, etc.)
- Environment-specific credentials ready
- Admin account with sufficient XLM for deployment

## Deployment Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Secret Management                         │
│  (AWS Secrets Manager / HashiCorp Vault / K8s Secrets)      │
└────────────────────┬────────────────────────────────────────┘
                     │
                     │ Fetch at runtime
                     ▼
┌─────────────────────────────────────────────────────────────┐
│              Deployment Environment                          │
│  • Environment Variables                                     │
│  • Encrypted at rest                                         │
│  • Never logged or persisted                                 │
└────────────────────┬────────────────────────────────────────┘
                     │
                     │ Inject during initialization
                     ▼
┌─────────────────────────────────────────────────────────────┐
│              AnchorKit Contract                              │
│  • Runtime credential injection                              │
│  • Encrypted storage (if needed)                             │
│  • Automatic rotation                                        │
└─────────────────────────────────────────────────────────────┘
```

## Step-by-Step Deployment

### 1. Prepare Credentials

#### Option A: AWS Secrets Manager

```bash
# Store credentials in AWS Secrets Manager
aws secretsmanager create-secret \
    --name anchorkit/kyc-provider/api-key \
    --secret-string "sk_live_secure_key_abc123..."

aws secretsmanager create-secret \
    --name anchorkit/bank-integration/token \
    --secret-string "Bearer eyJhbGciOiJIUzI1NiIs..."

aws secretsmanager create-secret \
    --name anchorkit/compliance/auth \
    --secret-string "dXNlcm5hbWU6cGFzc3dvcmQ="
```

#### Option B: HashiCorp Vault

```bash
# Store credentials in Vault
vault kv put secret/anchorkit/kyc-provider \
    api_key="sk_live_secure_key_abc123..."

vault kv put secret/anchorkit/bank-integration \
    token="Bearer eyJhbGciOiJIUzI1NiIs..."

vault kv put secret/anchorkit/compliance \
    auth="dXNlcm5hbWU6cGFzc3dvcmQ="
```

#### Option C: Kubernetes Secrets

```yaml
# anchorkit-secrets.yaml
apiVersion: v1
kind: Secret
metadata:
  name: anchorkit-credentials
type: Opaque
stringData:
  kyc-api-key: "sk_live_secure_key_abc123..."
  bank-token: "Bearer eyJhbGciOiJIUzI1NiIs..."
  compliance-auth: "dXNlcm5hbWU6cGFzc3dvcmQ="
```

```bash
kubectl apply -f anchorkit-secrets.yaml
```

### 2. Build Contract

```bash
# Build optimized contract
cargo build --target wasm32-unknown-unknown --release

# Optimize WASM
soroban contract optimize \
    --wasm target/wasm32-unknown-unknown/release/anchorkit.wasm
```

### 3. Deploy Contract

```bash
# Deploy to Stellar network
CONTRACT_ID=$(soroban contract deploy \
    --wasm target/wasm32-unknown-unknown/release/anchorkit.wasm \
    --source admin-account \
    --network testnet)

echo "Contract deployed: $CONTRACT_ID"
```

### 4. Initialize Contract

```bash
# Initialize with admin address
soroban contract invoke \
    --id $CONTRACT_ID \
    --source admin-account \
    --network testnet \
    -- initialize \
    --admin GBAA5XKQC3KVDPD5OS3CHJJ24SB3BX7GI7XBXKNNCKQVPQVX6S3VT5O
```

### 5. Register Attestors

```bash
# Register KYC provider
soroban contract invoke \
    --id $CONTRACT_ID \
    --source admin-account \
    --network testnet \
    -- register_attestor \
    --attestor GBBD6A7KNZF5WNWQEPZP5DYJD2AYUTLXRB6VXJ4RCX4RTNPPQVNF3GQ

# Register bank integration
soroban contract invoke \
    --id $CONTRACT_ID \
    --source admin-account \
    --network testnet \
    -- register_attestor \
    --attestor GB7ZTQBJ7XXJQ6JDLHYQXQX3JQXJ3JQXJ3JQXJ3JQXJ3JQXJ3JQX

# Register compliance officer
soroban contract invoke \
    --id $CONTRACT_ID \
    --source admin-account \
    --network testnet \
    -- register_attestor \
    --attestor GBCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZAB
```

### 6. Set Credential Policies

```bash
# Set policy for KYC provider (30-day rotation)
soroban contract invoke \
    --id $CONTRACT_ID \
    --source admin-account \
    --network testnet \
    -- set_credential_policy \
    --attestor GBBD6A7KNZF5WNWQEPZP5DYJD2AYUTLXRB6VXJ4RCX4RTNPPQVNF3GQ \
    --rotation_interval_seconds 2592000 \
    --require_encryption true

# Set policy for bank integration (7-day rotation)
soroban contract invoke \
    --id $CONTRACT_ID \
    --source admin-account \
    --network testnet \
    -- set_credential_policy \
    --attestor GB7ZTQBJ7XXJQ6JDLHYQXQX3JQXJ3JQXJ3JQXJ3JQXJ3JQXJ3JQX \
    --rotation_interval_seconds 604800 \
    --require_encryption true
```

### 7. Inject Credentials

Create a deployment script that fetches and injects credentials:

```bash
#!/bin/bash
# deploy_with_credentials.sh

set -e

CONTRACT_ID="$1"
NETWORK="${2:-testnet}"

echo "Fetching credentials from secret manager..."

# Fetch from AWS Secrets Manager
KYC_API_KEY=$(aws secretsmanager get-secret-value \
    --secret-id anchorkit/kyc-provider/api-key \
    --query SecretString \
    --output text)

BANK_TOKEN=$(aws secretsmanager get-secret-value \
    --secret-id anchorkit/bank-integration/token \
    --query SecretString \
    --output text)

COMPLIANCE_AUTH=$(aws secretsmanager get-secret-value \
    --secret-id anchorkit/compliance/auth \
    --query SecretString \
    --output text)

echo "Encrypting credentials..."

# Encrypt credentials (use your encryption method)
ENCRYPTED_KYC=$(encrypt_credential "$KYC_API_KEY")
ENCRYPTED_BANK=$(encrypt_credential "$BANK_TOKEN")
ENCRYPTED_COMPLIANCE=$(encrypt_credential "$COMPLIANCE_AUTH")

echo "Storing encrypted credentials..."

# Store KYC provider credential
soroban contract invoke \
    --id $CONTRACT_ID \
    --source admin-account \
    --network $NETWORK \
    -- store_encrypted_credential \
    --attestor GBBD6A7KNZF5WNWQEPZP5DYJD2AYUTLXRB6VXJ4RCX4RTNPPQVNF3GQ \
    --credential_type ApiKey \
    --encrypted_value "$ENCRYPTED_KYC" \
    --expires_at $(($(date +%s) + 2592000))

# Store bank integration credential
soroban contract invoke \
    --id $CONTRACT_ID \
    --source admin-account \
    --network $NETWORK \
    -- store_encrypted_credential \
    --attestor GB7ZTQBJ7XXJQ6JDLHYQXQX3JQXJ3JQXJ3JQXJ3JQXJ3JQXJ3JQX \
    --credential_type BearerToken \
    --encrypted_value "$ENCRYPTED_BANK" \
    --expires_at $(($(date +%s) + 604800))

# Store compliance credential
soroban contract invoke \
    --id $CONTRACT_ID \
    --source admin-account \
    --network $NETWORK \
    -- store_encrypted_credential \
    --attestor GBCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZAB \
    --credential_type BasicAuth \
    --encrypted_value "$ENCRYPTED_COMPLIANCE" \
    --expires_at $(($(date +%s) + 2592000))

echo "Credentials injected successfully!"
```

### 8. Configure Services

```bash
# Configure KYC provider services
soroban contract invoke \
    --id $CONTRACT_ID \
    --source GBBD6A7KNZF5WNWQEPZP5DYJD2AYUTLXRB6VXJ4RCX4RTNPPQVNF3GQ \
    --network testnet \
    -- configure_services \
    --anchor GBBD6A7KNZF5WNWQEPZP5DYJD2AYUTLXRB6VXJ4RCX4RTNPPQVNF3GQ \
    --services '["KYC"]'

# Configure bank integration services
soroban contract invoke \
    --id $CONTRACT_ID \
    --source GB7ZTQBJ7XXJQ6JDLHYQXQX3JQXJ3JQXJ3JQXJ3JQXJ3JQXJ3JQX \
    --network testnet \
    -- configure_services \
    --anchor GB7ZTQBJ7XXJQ6JDLHYQXQX3JQXJ3JQXJ3JQXJ3JQXJ3JQXJ3JQX \
    --services '["Deposits", "Withdrawals"]'
```

### 9. Verify Deployment

```bash
# Check credential policies
soroban contract invoke \
    --id $CONTRACT_ID \
    --network testnet \
    -- get_credential_policy \
    --attestor GBBD6A7KNZF5WNWQEPZP5DYJD2AYUTLXRB6VXJ4RCX4RTNPPQVNF3GQ

# Check rotation status
soroban contract invoke \
    --id $CONTRACT_ID \
    --network testnet \
    -- check_credential_rotation \
    --attestor GBBD6A7KNZF5WNWQEPZP5DYJD2AYUTLXRB6VXJ4RCX4RTNPPQVNF3GQ
```

## Credential Rotation Setup

### Automated Rotation Script

```bash
#!/bin/bash
# rotate_credentials.sh

set -e

CONTRACT_ID="$1"
ATTESTOR="$2"
NETWORK="${3:-testnet}"

echo "Checking rotation status for $ATTESTOR..."

NEEDS_ROTATION=$(soroban contract invoke \
    --id $CONTRACT_ID \
    --network $NETWORK \
    -- check_credential_rotation \
    --attestor $ATTESTOR)

if [ "$NEEDS_ROTATION" = "true" ]; then
    echo "Rotation required. Generating new credential..."
    
    # Generate new credential (implementation depends on credential type)
    NEW_CREDENTIAL=$(generate_new_credential $ATTESTOR)
    
    # Encrypt new credential
    ENCRYPTED_NEW=$(encrypt_credential "$NEW_CREDENTIAL")
    
    # Rotate in contract
    soroban contract invoke \
        --id $CONTRACT_ID \
        --source admin-account \
        --network $NETWORK \
        -- rotate_credential \
        --attestor $ATTESTOR \
        --credential_type ApiKey \
        --new_encrypted_value "$ENCRYPTED_NEW" \
        --expires_at $(($(date +%s) + 2592000))
    
    # Update external service
    update_external_service $ATTESTOR "$NEW_CREDENTIAL"
    
    echo "Rotation complete for $ATTESTOR"
else
    echo "No rotation needed for $ATTESTOR"
fi
```

### Cron Job Configuration

```bash
# Add to crontab
crontab -e

# Check rotation daily at 2 AM
0 2 * * * /path/to/rotate_credentials.sh $CONTRACT_ID GBBD6A7KNZF5WNWQEPZP5DYJD2AYUTLXRB6VXJ4RCX4RTNPPQVNF3GQ testnet >> /var/log/credential_rotation.log 2>&1
```

## Docker Deployment

### Dockerfile

```dockerfile
FROM rust:1.75 as builder

WORKDIR /app
COPY . .

RUN cargo build --target wasm32-unknown-unknown --release

FROM stellar/quickstart:latest

COPY --from=builder /app/target/wasm32-unknown-unknown/release/anchorkit.wasm /contracts/
COPY deploy_with_credentials.sh /scripts/

# Install AWS CLI for secret fetching
RUN apt-get update && apt-get install -y awscli

ENTRYPOINT ["/scripts/deploy_with_credentials.sh"]
```

### Docker Compose

```yaml
version: '3.8'

services:
  anchorkit:
    build: .
    environment:
      - AWS_REGION=us-east-1
      - AWS_ACCESS_KEY_ID=${AWS_ACCESS_KEY_ID}
      - AWS_SECRET_ACCESS_KEY=${AWS_SECRET_ACCESS_KEY}
      - STELLAR_NETWORK=testnet
      - CONTRACT_ID=${CONTRACT_ID}
    volumes:
      - ./logs:/var/log
    secrets:
      - admin_key
    command: ["${CONTRACT_ID}", "testnet"]

secrets:
  admin_key:
    external: true
```

## Kubernetes Deployment

### Deployment YAML

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: anchorkit-deployment
spec:
  replicas: 3
  selector:
    matchLabels:
      app: anchorkit
  template:
    metadata:
      labels:
        app: anchorkit
    spec:
      containers:
      - name: anchorkit
        image: anchorkit:latest
        env:
        - name: CONTRACT_ID
          valueFrom:
            configMapKeyRef:
              name: anchorkit-config
              key: contract-id
        - name: STELLAR_NETWORK
          value: "testnet"
        volumeMounts:
        - name: credentials
          mountPath: /secrets
          readOnly: true
      volumes:
      - name: credentials
        secret:
          secretName: anchorkit-credentials
```

### CronJob for Rotation

```yaml
apiVersion: batch/v1
kind: CronJob
metadata:
  name: credential-rotation
spec:
  schedule: "0 2 * * *"  # Daily at 2 AM
  jobTemplate:
    spec:
      template:
        spec:
          containers:
          - name: rotate
            image: anchorkit:latest
            command: ["/scripts/rotate_credentials.sh"]
            args: ["$(CONTRACT_ID)", "$(ATTESTOR)", "testnet"]
            env:
            - name: CONTRACT_ID
              valueFrom:
                configMapKeyRef:
                  name: anchorkit-config
                  key: contract-id
            volumeMounts:
            - name: credentials
              mountPath: /secrets
              readOnly: true
          restartPolicy: OnFailure
          volumes:
          - name: credentials
            secret:
              secretName: anchorkit-credentials
```

## Monitoring and Alerting

### CloudWatch Alarms (AWS)

```bash
# Create alarm for credential expiry
aws cloudwatch put-metric-alarm \
    --alarm-name anchorkit-credential-expiring \
    --alarm-description "Alert when credentials are expiring soon" \
    --metric-name CredentialAge \
    --namespace AnchorKit \
    --statistic Average \
    --period 86400 \
    --threshold 604800 \
    --comparison-operator GreaterThanThreshold \
    --evaluation-periods 1 \
    --alarm-actions arn:aws:sns:us-east-1:123456789012:anchorkit-alerts
```

### Prometheus Metrics

```yaml
# prometheus.yml
scrape_configs:
  - job_name: 'anchorkit'
    static_configs:
      - targets: ['anchorkit:9090']
    metrics_path: '/metrics'
```

### Grafana Dashboard

```json
{
  "dashboard": {
    "title": "AnchorKit Credentials",
    "panels": [
      {
        "title": "Credential Age",
        "targets": [
          {
            "expr": "anchorkit_credential_age_seconds"
          }
        ]
      },
      {
        "title": "Rotation Events",
        "targets": [
          {
            "expr": "rate(anchorkit_credential_rotations_total[5m])"
          }
        ]
      }
    ]
  }
}
```

## Troubleshooting

### Common Issues

**Issue**: Credential fetch fails from secret manager
```bash
# Check AWS credentials
aws sts get-caller-identity

# Verify secret exists
aws secretsmanager describe-secret --secret-id anchorkit/kyc-provider/api-key
```

**Issue**: Encryption fails
```bash
# Verify encryption key is available
echo $ENCRYPTION_KEY | wc -c  # Should be 32+ bytes

# Test encryption
echo "test" | openssl enc -aes-256-cbc -a -salt -pass pass:$ENCRYPTION_KEY
```

**Issue**: Contract invocation fails
```bash
# Check contract ID
soroban contract info --id $CONTRACT_ID --network testnet

# Verify admin account
soroban keys show admin-account
```

## Security Checklist

- [ ] Credentials stored in secure secret manager
- [ ] Environment variables never logged
- [ ] Encryption keys rotated regularly
- [ ] Admin keys secured with hardware wallet
- [ ] Rotation policies configured
- [ ] Monitoring and alerting enabled
- [ ] Audit logging configured
- [ ] Access controls implemented
- [ ] Backup and recovery tested
- [ ] Incident response plan documented

## Next Steps

1. Review [SECURE_CREDENTIALS.md](./SECURE_CREDENTIALS.md) for detailed credential management
2. Set up monitoring and alerting
3. Configure automated rotation
4. Test disaster recovery procedures
5. Document operational procedures

## Support

For deployment issues:
1. Check deployment logs
2. Verify secret manager access
3. Review contract events
4. Contact DevOps team
