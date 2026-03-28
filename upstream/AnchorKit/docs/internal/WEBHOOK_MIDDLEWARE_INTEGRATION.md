# Webhook Middleware Integration Guide

## Quick Start

### 1. Import the Middleware

```rust
use anchorkit::{
    WebhookMiddleware, WebhookSecurityConfig, WebhookRequest,
    SignatureAlgorithm, WebhookDeliveryStatus,
};
```

### 2. Configure Security

```rust
fn create_webhook_config(env: &Env) -> WebhookSecurityConfig {
    WebhookSecurityConfig {
        algorithm: SignatureAlgorithm::Sha256,
        secret_key: Bytes::from_array(env, &SECRET_KEY_BYTES),
        timestamp_tolerance_seconds: 300,      // 5 minutes
        max_payload_size_bytes: 10000,          // 10 KB
        enable_replay_protection: true,
    }
}
```

### 3. Validate Incoming Webhooks

```rust
pub fn handle_webhook(
    env: &Env,
    payload: Bytes,
    signature: Bytes,
    timestamp: u64,
    webhook_id: u64,
) -> Result<(), Error> {
    let config = create_webhook_config(env);
    
    let request = WebhookRequest {
        payload: payload.clone(),
        signature,
        timestamp,
        webhook_id,
        source_address: None,
    };
    
    let result = WebhookMiddleware::validate_webhook(env, &request, &config)?;
    
    if result.is_valid {
        // Process webhook
        process_webhook_payload(env, &payload)?;
        Ok(())
    } else {
        // Log validation failure
        Err(Error::WebhookValidationFailed)
    }
}
```

## Real-World Examples

### Example 1: Deposit Webhook Handler

```rust
#[derive(Clone, Debug)]
pub struct DepositWebhook {
    pub user_address: Address,
    pub amount: u64,
    pub asset: String,
    pub transaction_id: String,
}

pub fn handle_deposit_webhook(
    env: &Env,
    webhook_data: DepositWebhook,
    signature: Bytes,
    timestamp: u64,
) -> Result<(), Error> {
    let config = create_webhook_config(env);
    
    // Serialize webhook data
    let payload = serialize_webhook(&webhook_data)?;
    
    let request = WebhookRequest {
        payload,
        signature,
        timestamp,
        webhook_id: hash_transaction_id(&webhook_data.transaction_id),
        source_address: Some(webhook_data.user_address.clone()),
    };
    
    // Validate webhook
    let result = WebhookMiddleware::validate_webhook(env, &request, &config)?;
    
    if !result.is_valid {
        WebhookMiddleware::record_delivery_attempt(
            env,
            request.webhook_id,
            WebhookDeliveryStatus::Rejected,
            0,
            Some(400),
        );
        return Err(Error::WebhookValidationFailed);
    }
    
    // Process deposit
    process_deposit(env, &webhook_data)?;
    
    // Record successful delivery
    WebhookMiddleware::record_delivery_attempt(
        env,
        request.webhook_id,
        WebhookDeliveryStatus::Delivered,
        10,
        None,
    );
    
    Ok(())
}
```

### Example 2: KYC Status Update Webhook

```rust
#[derive(Clone, Debug)]
pub struct KYCStatusWebhook {
    pub user_id: String,
    pub status: String,
    pub verification_level: u32,
    pub timestamp: u64,
}

pub fn handle_kyc_webhook(
    env: &Env,
    webhook_data: KYCStatusWebhook,
    signature: Bytes,
    webhook_timestamp: u64,
) -> Result<(), Error> {
    let config = create_webhook_config(env);
    
    let payload = serialize_webhook(&webhook_data)?;
    let webhook_id = hash_user_id(&webhook_data.user_id);
    
    let request = WebhookRequest {
        payload,
        signature,
        timestamp: webhook_timestamp,
        webhook_id,
        source_address: None,
    };
    
    // Validate
    let result = WebhookMiddleware::validate_webhook(env, &request, &config)?;
    
    if !result.is_valid {
        // Log suspicious activity if signature failed
        if let Some(err) = &result.error {
            if err.contains("Signature") {
                WebhookMiddleware::log_suspicious_activity(
                    env,
                    SuspiciousActivityType::InvalidSignature,
                    ActivitySeverity::Critical,
                    String::from_str(env, "KYC webhook signature verification failed"),
                    None,
                );
            }
        }
        return Err(Error::WebhookValidationFailed);
    }
    
    // Update KYC status
    update_kyc_status(env, &webhook_data)?;
    
    Ok(())
}
```

### Example 3: Quote Update Webhook with Retry Logic

```rust
pub fn handle_quote_webhook_with_retry(
    env: &Env,
    webhook_data: QuoteWebhook,
    signature: Bytes,
    timestamp: u64,
    max_retries: u32,
) -> Result<(), Error> {
    let config = create_webhook_config(env);
    let webhook_id = hash_quote_id(&webhook_data.quote_id);
    
    let mut attempt = 0;
    let mut last_error = None;
    
    loop {
        attempt += 1;
        
        let payload = serialize_webhook(&webhook_data)?;
        let request = WebhookRequest {
            payload,
            signature: signature.clone(),
            timestamp,
            webhook_id,
            source_address: None,
        };
        
        // Validate
        match WebhookMiddleware::validate_webhook(env, &request, &config) {
            Ok(result) if result.is_valid => {
                // Process quote
                process_quote(env, &webhook_data)?;
                
                // Record success
                WebhookMiddleware::record_delivery_attempt(
                    env,
                    webhook_id,
                    WebhookDeliveryStatus::Delivered,
                    50,
                    None,
                );
                
                return Ok(());
            }
            Ok(result) => {
                last_error = result.error;
                
                // Record failed attempt
                WebhookMiddleware::record_delivery_attempt(
                    env,
                    webhook_id,
                    WebhookDeliveryStatus::Failed,
                    100,
                    Some(400),
                );
            }
            Err(e) => {
                last_error = Some(String::from_str(env, "Validation error"));
                
                // Record error
                WebhookMiddleware::record_delivery_attempt(
                    env,
                    webhook_id,
                    WebhookDeliveryStatus::Failed,
                    100,
                    Some(500),
                );
            }
        }
        
        if attempt >= max_retries {
            break;
        }
        
        // Exponential backoff
        let delay = 100 * (2_u64.pow(attempt - 1));
        // In production, implement actual delay
    }
    
    Err(Error::WebhookValidationFailed)
}
```

### Example 4: Webhook Monitoring Dashboard

```rust
pub fn get_webhook_health_status(env: &Env) -> WebhookHealthStatus {
    let mut total_webhooks = 0;
    let mut successful = 0;
    let mut failed = 0;
    let mut suspicious_count = 0;
    
    // Iterate through recent delivery records
    for webhook_id in 1..=100 {
        for attempt in 1..=10 {
            if let Some(record) = WebhookMiddleware::get_delivery_record(env, webhook_id, attempt) {
                total_webhooks += 1;
                
                match record.status {
                    WebhookDeliveryStatus::Delivered => successful += 1,
                    WebhookDeliveryStatus::Failed => failed += 1,
                    WebhookDeliveryStatus::Suspicious => suspicious_count += 1,
                    _ => {}
                }
            }
        }
    }
    
    let success_rate = if total_webhooks > 0 {
        (successful as f64 / total_webhooks as f64) * 100.0
    } else {
        100.0
    };
    
    WebhookHealthStatus {
        total_webhooks,
        successful,
        failed,
        suspicious_count,
        success_rate,
    }
}

pub fn get_recent_suspicious_activities(
    env: &Env,
    limit: u32,
) -> Vec<SuspiciousActivityRecord> {
    let mut activities = Vec::new();
    
    for activity_id in 1..=limit {
        if let Some(activity) = WebhookMiddleware::get_suspicious_activity(env, activity_id as u64) {
            activities.push(activity);
        }
    }
    
    activities
}
```

## Environment Configuration

### Development Environment

```bash
# .env.development
WEBHOOK_SECRET_KEY=dev_secret_key_32_bytes_long_here
WEBHOOK_ALGORITHM=Sha256
WEBHOOK_TIMESTAMP_TOLERANCE=600  # 10 minutes for dev
WEBHOOK_MAX_PAYLOAD_SIZE=50000   # 50 KB for dev
WEBHOOK_REPLAY_PROTECTION=false  # Disabled for testing
```

### Production Environment

```bash
# .env.production
WEBHOOK_SECRET_KEY=$(openssl rand -hex 32)  # Generate random 32-byte key
WEBHOOK_ALGORITHM=Sha256
WEBHOOK_TIMESTAMP_TOLERANCE=300  # 5 minutes
WEBHOOK_MAX_PAYLOAD_SIZE=10000   # 10 KB
WEBHOOK_REPLAY_PROTECTION=true   # Always enabled
```

### Staging Environment

```bash
# .env.staging
WEBHOOK_SECRET_KEY=$(openssl rand -hex 32)
WEBHOOK_ALGORITHM=Sha256
WEBHOOK_TIMESTAMP_TOLERANCE=300
WEBHOOK_MAX_PAYLOAD_SIZE=10000
WEBHOOK_REPLAY_PROTECTION=true
```

## Signature Generation (Client Side)

### Node.js Example

```javascript
const crypto = require('crypto');

function generateWebhookSignature(payload, secret, timestamp) {
    const message = Buffer.concat([
        Buffer.from(timestamp.toString()),
        Buffer.from(payload)
    ]);
    
    const signature = crypto
        .createHmac('sha256', secret)
        .update(message)
        .digest('hex');
    
    return signature;
}

// Usage
const payload = JSON.stringify({ user_id: '123', amount: 100 });
const secret = Buffer.from(process.env.WEBHOOK_SECRET, 'hex');
const timestamp = Math.floor(Date.now() / 1000);
const signature = generateWebhookSignature(payload, secret, timestamp);

// Send webhook
fetch('https://api.example.com/webhooks/deposit', {
    method: 'POST',
    headers: {
        'Content-Type': 'application/json',
        'X-Webhook-Signature': signature,
        'X-Webhook-Timestamp': timestamp.toString(),
        'X-Webhook-ID': 'webhook_123',
    },
    body: payload,
});
```

### Python Example

```python
import hmac
import hashlib
import json
import time
from typing import Dict, Any

def generate_webhook_signature(
    payload: Dict[str, Any],
    secret: bytes,
    timestamp: int
) -> str:
    """Generate HMAC-SHA256 signature for webhook."""
    message = str(timestamp).encode() + json.dumps(payload).encode()
    signature = hmac.new(
        secret,
        message,
        hashlib.sha256
    ).hexdigest()
    return signature

# Usage
payload = {"user_id": "123", "amount": 100}
secret = bytes.fromhex(os.environ['WEBHOOK_SECRET'])
timestamp = int(time.time())
signature = generate_webhook_signature(payload, secret, timestamp)

# Send webhook
import requests
response = requests.post(
    'https://api.example.com/webhooks/deposit',
    json=payload,
    headers={
        'X-Webhook-Signature': signature,
        'X-Webhook-Timestamp': str(timestamp),
        'X-Webhook-ID': 'webhook_123',
    }
)
```

### Go Example

```go
package main

import (
    "crypto/hmac"
    "crypto/sha256"
    "encoding/hex"
    "encoding/json"
    "fmt"
    "time"
)

func generateWebhookSignature(
    payload interface{},
    secret []byte,
    timestamp int64,
) string {
    payloadJSON, _ := json.Marshal(payload)
    message := append(
        []byte(fmt.Sprintf("%d", timestamp)),
        payloadJSON...,
    )
    
    h := hmac.New(sha256.New, secret)
    h.Write(message)
    return hex.EncodeToString(h.Sum(nil))
}

// Usage
payload := map[string]interface{}{
    "user_id": "123",
    "amount":  100,
}
secret := []byte(os.Getenv("WEBHOOK_SECRET"))
timestamp := time.Now().Unix()
signature := generateWebhookSignature(payload, secret, timestamp)

// Send webhook
// ... HTTP request code ...
```

## Monitoring & Alerting

### Prometheus Metrics

```rust
pub fn export_webhook_metrics(env: &Env) -> WebhookMetrics {
    WebhookMetrics {
        total_webhooks: get_total_webhooks(env),
        successful_deliveries: get_successful_deliveries(env),
        failed_deliveries: get_failed_deliveries(env),
        signature_failures: get_signature_failures(env),
        replay_attacks: get_replay_attacks(env),
        timestamp_violations: get_timestamp_violations(env),
        avg_response_time_ms: get_avg_response_time(env),
    }
}
```

### Alert Rules

```yaml
# prometheus-rules.yml
groups:
  - name: webhook_alerts
    rules:
      - alert: HighSignatureFailureRate
        expr: rate(webhook_signature_failures[5m]) > 0.1
        for: 5m
        annotations:
          summary: "High webhook signature failure rate"
          
      - alert: ReplayAttackDetected
        expr: rate(webhook_replay_attacks[5m]) > 0
        for: 1m
        annotations:
          summary: "Replay attacks detected"
          
      - alert: LowWebhookSuccessRate
        expr: webhook_success_rate < 0.95
        for: 10m
        annotations:
          summary: "Webhook success rate below 95%"
```

## Testing Webhooks

### Manual Testing with curl

```bash
#!/bin/bash

# Generate signature
TIMESTAMP=$(date +%s)
SECRET="your_secret_key_here"
PAYLOAD='{"user_id":"123","amount":100}'

# Generate HMAC-SHA256 signature
SIGNATURE=$(echo -n "${TIMESTAMP}${PAYLOAD}" | \
    openssl dgst -sha256 -hmac "$SECRET" -hex | \
    cut -d' ' -f2)

# Send webhook
curl -X POST https://api.example.com/webhooks/deposit \
    -H "Content-Type: application/json" \
    -H "X-Webhook-Signature: $SIGNATURE" \
    -H "X-Webhook-Timestamp: $TIMESTAMP" \
    -H "X-Webhook-ID: webhook_123" \
    -d "$PAYLOAD"
```

### Automated Testing

```rust
#[test]
fn test_webhook_integration_end_to_end() {
    let env = Env::default();
    let config = create_webhook_config(&env);
    
    // Create test payload
    let payload = Bytes::from_array(&env, b"test_payload");
    let timestamp = env.ledger().timestamp();
    
    // Generate signature
    let signature = generate_test_signature(&env, &payload, &config.secret_key, timestamp);
    
    // Create request
    let request = WebhookRequest {
        payload,
        signature,
        timestamp,
        webhook_id: 1,
        source_address: None,
    };
    
    // Validate
    let result = WebhookMiddleware::validate_webhook(&env, &request, &config).unwrap();
    assert!(result.is_valid);
    
    // Verify delivery was recorded
    let delivery = WebhookMiddleware::get_delivery_record(&env, 1, 1);
    assert!(delivery.is_some());
    assert_eq!(delivery.unwrap().status, WebhookDeliveryStatus::Delivered);
}
```

## Troubleshooting Checklist

- [ ] Secret key matches between sender and receiver
- [ ] Timestamp is within tolerance window
- [ ] Payload encoding is consistent (UTF-8 vs binary)
- [ ] Signature algorithm matches configuration
- [ ] Webhook ID is globally unique
- [ ] System clocks are synchronized (NTP)
- [ ] Network latency is acceptable
- [ ] Payload size is within limits
- [ ] Replay protection is enabled in production
- [ ] Monitoring and alerting are configured

## Performance Optimization

### Caching Configuration

```rust
lazy_static::lazy_static! {
    static ref WEBHOOK_CONFIG: WebhookSecurityConfig = {
        // Load from environment once
        create_webhook_config_from_env()
    };
}
```

### Batch Processing

```rust
pub fn process_webhook_batch(
    env: &Env,
    webhooks: Vec<WebhookRequest>,
) -> Result<Vec<bool>, Error> {
    let config = &WEBHOOK_CONFIG;
    let mut results = Vec::new();
    
    for webhook in webhooks {
        let result = WebhookMiddleware::validate_webhook(env, &webhook, config)?;
        results.push(result.is_valid);
    }
    
    Ok(results)
}
```

## Security Checklist

- [ ] Use HTTPS for all webhook endpoints
- [ ] Validate webhook source IP addresses
- [ ] Implement rate limiting per source
- [ ] Rotate secrets regularly
- [ ] Monitor for suspicious patterns
- [ ] Log all validation failures
- [ ] Implement exponential backoff for retries
- [ ] Use different secrets per environment
- [ ] Never commit secrets to version control
- [ ] Implement webhook signature rotation
