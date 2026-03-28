# Webhook Listener & Verification Module

Secure webhook endpoint for receiving and processing anchor callbacks with signature verification, replay attack prevention, and transaction state management.

## Features

- ✅ **Signature Verification**: Supports both Stellar keypair signatures and HMAC-SHA256
- ✅ **Replay Attack Prevention**: Timestamp validation and nonce tracking
- ✅ **Transaction State Management**: Enforced state transitions with validation
- ✅ **Suspicious Activity Detection**: Pattern-based anomaly detection
- ✅ **Comprehensive Logging**: All webhooks logged with verification status
- ✅ **Multi-Event Support**: Deposit, withdrawal, and KYC updates

## Architecture

### Components

1. **WebhookVerifier** (`webhook-verifier.ts`)
   - Signature verification (Stellar keypair or HMAC)
   - Timestamp validation (5-minute window)
   - Nonce tracking for replay prevention

2. **WebhookLogger** (`webhook-logger.ts`)
   - Webhook logging with verification status
   - Suspicious activity detection
   - Pattern analysis (duplicates, failed verifications)

3. **TransactionStateManager** (`transaction-state.ts`)
   - State transition validation
   - Transaction updates with history tracking
   - KYC status management

4. **WebhookHandler** (`webhook-handler.ts`)
   - Main endpoint handler
   - Request routing
   - Error handling

## API Endpoint

### POST /webhooks/anchor

Receives webhook callbacks from anchors.

**Required Headers:**
```
x-signature: <base64-signature or hex-hmac>
x-timestamp: <ISO-8601 timestamp>
x-nonce: <unique-request-id>
x-anchor-id: <anchor-identifier>
```

**Deposit Update Payload:**
```json
{
  "event_type": "deposit_update",
  "transaction_id": "abc123",
  "status": "pending_stellar",
  "status_eta": 300,
  "amount_in": "100.00",
  "amount_out": "99.50",
  "amount_fee": "0.50",
  "stellar_transaction_id": "xyz789",
  "message": "Processing deposit"
}
```

**Withdrawal Update Payload:**
```json
{
  "event_type": "withdrawal_update",
  "transaction_id": "def456",
  "status": "pending_external",
  "external_transaction_id": "bank-tx-123",
  "amount_out": "95.00"
}
```

**KYC Update Payload:**
```json
{
  "event_type": "kyc_update",
  "transaction_id": "ghi789",
  "kyc_status": "approved",
  "kyc_fields": {
    "first_name": "John",
    "last_name": "Doe"
  }
}
```

**Response:**
```json
{
  "success": true,
  "processing_time_ms": 45
}
```

## Security Features

### 1. Signature Verification

**Stellar Keypair Method:**
```typescript
// Anchor signs payload with private key
const signature = keypair.sign(Buffer.from(payload)).toString('base64');

// Server verifies with anchor's public key
verifier.verifySignature(payload, signature, anchorPublicKey);
```

**HMAC Method:**
```typescript
// Anchor creates HMAC with shared secret
const signature = crypto.createHmac('sha256', secret)
  .update(payload)
  .digest('hex');

// Server verifies with same secret
verifier.verifyHMAC(payload, signature, secret);
```

### 2. Replay Attack Prevention

**Timestamp Validation:**
- Webhooks must be sent within 5-minute window
- Rejects both old and future-dated requests

**Nonce Tracking:**
- Each request requires unique nonce
- Duplicate nonces rejected immediately
- Nonces cleared after replay window expires

### 3. State Transition Validation

**Deposit Flow:**
```
pending_user_transfer_start → pending_anchor → pending_stellar → completed
                           ↓                  ↓                 ↓
                         expired            error            error
```

**Withdrawal Flow:**
```
pending_user_transfer_start → pending_anchor → pending_external → completed
                           ↓                  ↓                   ↓
                         expired            error               error
```

Invalid transitions are rejected with error.

### 4. Suspicious Activity Detection

Automatically flags:
- Multiple webhooks for same transaction (>3 in 5 minutes)
- High rate of failed verifications (>10 in 1 hour)
- Invalid signatures
- Replay attempts

## Database Schema

```sql
-- Webhook logs
CREATE TABLE webhook_logs (
  id UUID PRIMARY KEY,
  anchor_id VARCHAR(255),
  transaction_id VARCHAR(255),
  event_type VARCHAR(50),
  payload JSONB,
  verified BOOLEAN,
  received_at TIMESTAMP
);

-- Suspicious activity
CREATE TABLE suspicious_webhooks (
  id UUID PRIMARY KEY,
  webhook_id UUID,
  anchor_id VARCHAR(255),
  reason TEXT,
  payload JSONB,
  detected_at TIMESTAMP
);

-- Transactions
CREATE TABLE transactions (
  id UUID PRIMARY KEY,
  transaction_id VARCHAR(255) UNIQUE,
  anchor_id VARCHAR(255),
  kind VARCHAR(20),
  status VARCHAR(50),
  amount_in DECIMAL(20, 7),
  amount_out DECIMAL(20, 7),
  stellar_transaction_id VARCHAR(64),
  external_transaction_id VARCHAR(255)
);
```

## Setup

### 1. Install Dependencies

```bash
cd backend
npm install
```

### 2. Database Migration

```bash
psql -U postgres -d swiftremit < migrations/webhook_schema.sql
```

### 3. Register Anchor

```sql
INSERT INTO anchors (id, name, public_key, webhook_secret, home_domain)
VALUES (
  'anchor1',
  'Example Anchor',
  'GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX',
  'optional-hmac-secret',
  'anchor.example.com'
);
```

### 4. Configure Webhook Handler

```typescript
import express from 'express';
import { Pool } from 'pg';
import { WebhookHandler } from './webhook-handler';

const app = express();
const pool = new Pool({
  connectionString: process.env.DATABASE_URL
});

const webhookHandler = new WebhookHandler(pool);
webhookHandler.setupRoutes(app);

app.listen(3000);
```

## Usage Examples

### Sending Webhook (Anchor Side)

```typescript
import { Keypair } from '@stellar/stellar-sdk';

const keypair = Keypair.fromSecret('SXXXXXXX...');
const payload = JSON.stringify({
  event_type: 'deposit_update',
  transaction_id: 'tx123',
  status: 'completed'
});

const signature = keypair.sign(Buffer.from(payload)).toString('base64');

await fetch('https://swiftremit.com/webhooks/anchor', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
    'x-signature': signature,
    'x-timestamp': new Date().toISOString(),
    'x-nonce': crypto.randomUUID(),
    'x-anchor-id': 'anchor1'
  },
  body: payload
});
```

### Querying Webhook Logs

```sql
-- Recent webhooks
SELECT * FROM webhook_logs 
ORDER BY received_at DESC 
LIMIT 100;

-- Failed verifications
SELECT * FROM webhook_logs 
WHERE verified = false 
ORDER BY received_at DESC;

-- Suspicious activity
SELECT * FROM suspicious_webhooks 
WHERE investigated = false 
ORDER BY detected_at DESC;
```

## Testing

```bash
npm test
```

**Test Coverage:**
- ✅ Signature verification (valid/invalid)
- ✅ HMAC verification
- ✅ Timestamp validation (recent/old/future/invalid)
- ✅ Nonce validation (new/duplicate)
- ✅ State transition validation (deposits/withdrawals)
- ✅ Error recovery flows

## Monitoring

### Key Metrics

1. **Webhook Success Rate**: `verified = true / total`
2. **Average Processing Time**: `AVG(processing_time_ms)`
3. **Suspicious Activity Rate**: `suspicious_webhooks / webhook_logs`
4. **State Transition Errors**: Failed validation attempts

### Alerts

Set up alerts for:
- Verification failure rate > 5%
- Suspicious activity detected
- Processing time > 1000ms
- Invalid state transitions

## Security Best Practices

1. **Use HTTPS**: Always use TLS for webhook endpoints
2. **Rotate Secrets**: Regularly rotate HMAC secrets
3. **Monitor Logs**: Review suspicious activity daily
4. **Rate Limiting**: Implement per-anchor rate limits
5. **IP Whitelisting**: Restrict to known anchor IPs
6. **Audit Trail**: Maintain complete webhook history

## Middleware Enforcement

As of version 2.0, HMAC signature verification is enforced via Express middleware on all `/webhooks/*` routes. This ensures that:

- Every inbound webhook request is verified before processing
- No handler can accidentally skip verification
- Invalid/missing signatures return 401 immediately

### Middleware Usage

```typescript
import { createWebhookVerificationMiddleware } from './webhook-middleware';

// Apply to your Express app
const verifyWebhook = createWebhookVerificationMiddleware({
  timestampWindowSeconds: 300,  // 5 minute window
  requireSignature: true,
  getAnchorSecret: async (anchorId) => {
    // Fetch from your database or config
    return getSecretFromDb(anchorId);
  },
});

app.use('/webhooks', verifyWebhook);
```

### Required Headers

All webhook requests must include:

| Header | Required | Description |
|--------|----------|-------------|
| x-anchor-id | Yes | Anchor identifier |
| x-signature | Yes* | HMAC-SHA256 signature |
| x-timestamp | No** | ISO-8601 timestamp |
| x-nonce | No** | Unique request ID |

*Required unless `requireSignature: false` in middleware options
**Recommended for replay attack prevention

### Testing Verification

Run the unit tests to verify middleware correctly rejects tampered payloads:

```bash
cd backend
npm test -- webhook-middleware.test.ts
```

Test coverage includes:
- ✅ Valid signatures accepted
- ✅ Invalid signatures rejected
- ✅ Tampered payloads rejected
- ✅ Missing signatures rejected (when required)
- ✅ Expired timestamps rejected
- ✅ Duplicate nonces rejected (replay attack)

## Troubleshooting

### Signature Verification Fails

- Verify anchor public key is correct
- Check payload encoding (UTF-8)
- Ensure signature is base64 encoded
- Verify HMAC secret matches

### Timestamp Validation Fails

- Check server time synchronization (NTP)
- Verify timezone handling
- Adjust replay window if needed

### State Transition Rejected

- Review current transaction status
- Check transition rules in `transaction-state.ts`
- Verify transaction exists in database

## License

MIT
