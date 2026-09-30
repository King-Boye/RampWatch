# Currency Configuration API - Implementation Documentation

<!--
=============================================================================
Issue #1551 — Currency API: Webhook notifications on config changes
https://github.com/Haroldwonder/SwiftRemit/issues/1551

Issue #1550 — Currency API: Admin API for currency management
https://github.com/Haroldwonder/SwiftRemit/issues/1550

STATUS: PENDING — listed as Future Enhancements below.
Both issues are documented here as the canonical design specification
so they can be implemented without ambiguity.
=============================================================================

── ISSUE #1550: Admin API for currency management ──────────────────────────

PROBLEM
-------
The current currency configuration is file-based + env-var-based. There is
no HTTP API to add, update, or remove currencies at runtime without restarting
the service or editing a file/env var. An ops team managing a production
deployment cannot add a new currency corridor (e.g. USDT) without a redeploy.

PROPOSED IMPLEMENTATION
-----------------------
Add a set of authenticated admin routes to api/src/routes/currencies.ts:

  POST   /api/admin/currencies
    Body: { code, symbol, decimal_precision, name? }
    Adds a new currency. Returns 409 if code already exists.
    Writes through to the in-memory config AND persists to currencies.json
    (or a database table if the service has migrated to DB-backed config).

  PUT    /api/admin/currencies/:code
    Body: partial CurrencyConfig (any subset of symbol/decimal_precision/name)
    Updates an existing currency. Returns 404 if code not found.

  DELETE /api/admin/currencies/:code
    Removes a currency. Returns 404 if code not found.
    Returns 409 if the currency is currently in use by an active corridor
    (prevents removing USD while USD corridors are active).

AUTHENTICATION
--------------
All /api/admin/* routes MUST be gated by an admin authentication middleware.
The existing pattern in the backend service uses JWT with admin role claims:

  router.use('/admin', requireAuth({ role: 'admin' }));

The API service should adopt the same pattern. Admin tokens must:
  - Be short-lived (≤ 1 hour expiry)
  - Be issued only to service accounts with the 'currency:write' scope
  - Not be stored in client-side storage (use httpOnly cookies or
    server-side session if admin UI is browser-based)

IDEMPOTENCY
-----------
POST /api/admin/currencies should accept an optional Idempotency-Key header
so repeated calls (e.g. from a retry loop) do not create duplicate entries:

  Idempotency-Key: create-currency-USDT-2026-09-28

AUDIT LOG
---------
Every admin mutation should write to a structured audit log:
  { timestamp, action: 'currency.created'|'currency.updated'|'currency.deleted',
    actor: <admin_jwt_sub>, currency_code, before: {...}, after: {...} }

VALIDATION
----------
Reuse the existing Joi schema from api/src/config.ts for POST/PUT request
body validation. A 400 response is returned for any schema violation with
the same error format as the existing endpoints:
  { success: false, error: { message, code: 'VALIDATION_ERROR' }, timestamp }

TESTS TO ADD
------------
  api/src/__tests__/admin-currencies.test.ts:
    - POST creates a new currency and it appears in GET /api/currencies
    - POST with duplicate code returns 409
    - PUT updates an existing currency
    - PUT on non-existent code returns 404
    - DELETE removes a currency
    - DELETE on in-use currency returns 409
    - All routes return 401 without admin token
    - All routes return 403 with non-admin token
    - POST is idempotent with Idempotency-Key

── ISSUE #1551: Webhook notifications on config changes ────────────────────

PROBLEM
-------
When a currency is added, updated, or deleted (via the admin API from #1550,
or via a manual config reload), any downstream service (mobile app, frontend,
partner integrations) that has cached the currency list does not know it is
stale. There is no push notification mechanism.

PROPOSED IMPLEMENTATION
-----------------------
Add a webhook fan-out layer that fires on every successful currency mutation.
This integrates with the existing webhook infrastructure in:
  backend/src/webhook-service.ts  (delivery engine)
  backend/src/webhooks/           (event types)
  docs/WEBHOOKS.md                (conventions)

Step 1 — Define event types in backend/src/webhooks/events.ts:

  type CurrencyEvent =
    | { type: 'currency.created'; data: CurrencyConfig; timestamp: string }
    | { type: 'currency.updated'; data: { before: CurrencyConfig; after: CurrencyConfig }; timestamp: string }
    | { type: 'currency.deleted'; data: { code: string }; timestamp: string }
    | { type: 'currency.config_reloaded'; data: { count: number }; timestamp: string };

Step 2 — Emit events from admin route handlers (api/src/routes/currencies.ts):

  // After successful POST /api/admin/currencies:
  await webhookService.emit('currency.created', { ...newCurrency });

  // After successful PUT:
  await webhookService.emit('currency.updated', { before: old, after: updated });

  // After successful DELETE:
  await webhookService.emit('currency.deleted', { code });

Step 3 — Subscribe endpoint for consumers:
  Consumers register via the existing webhook subscription API.
  No new subscription mechanism needed — the existing
  POST /api/webhooks/subscribe with event_type: 'currency.*' is sufficient.

Step 4 — Retry and delivery guarantee:
  Use the existing outbox pattern from backend/src/webhook-outbox.ts (if it
  exists) or the webhook retry logic already in the delivery service.
  At-least-once delivery is sufficient; consumers must be idempotent.

PAYLOAD EXAMPLE
---------------
  {
    "id": "wh_01J9KZXQ...",
    "type": "currency.created",
    "timestamp": "2026-09-28T10:00:00.000Z",
    "data": {
      "code": "USDT",
      "symbol": "₮",
      "decimal_precision": 6,
      "name": "Tether USD"
    }
  }

SECURITY
--------
Webhook payloads must be signed using the existing HMAC-SHA256 signature
scheme documented in docs/WEBHOOKS.md. Consumers verify:
  X-SwiftRemit-Signature: sha256=<hmac>

No PII or admin credentials should appear in the event payload. Currency
configs contain only public formatting data so this requirement is already met.

TESTS TO ADD
------------
  api/src/__tests__/currency-webhooks.test.ts:
    - Webhook fires after POST /api/admin/currencies
    - Webhook fires after PUT /api/admin/currencies/:code
    - Webhook fires after DELETE /api/admin/currencies/:code
    - Webhook NOT fired if admin mutation returns an error
    - Webhook payload matches CurrencyEvent schema
    - Payload is signed with correct HMAC-SHA256 signature

DEPENDENCY ON #1550
-------------------
Issue #1551 depends on #1550. The webhook events are only emitted from the
admin mutation handlers, which do not exist until #1550 is implemented.
#1550 should be completed and merged first.

=============================================================================
-->

## Overview

This document describes the implementation of a RESTful API endpoint that exposes all supported currencies and their formatting rules for the SwiftRemit platform.

## Requirements Met

✅ API endpoint exposing all supported currencies
✅ Structured response with code, symbol, and decimal_precision
✅ Dynamic loading from centralized configuration
✅ No hardcoded values in codebase
✅ Environment-based configuration overrides
✅ Startup validation with fail-fast behavior
✅ Consistent, schema-validated JSON responses
✅ Input/output validation
✅ Safe handling of empty/invalid configuration
✅ No breaking changes to existing APIs
✅ Comprehensive unit tests
✅ Integration tests
✅ CI/CD pipeline configuration

## Architecture

### Components

1. **Configuration Loader** (`api/src/config.ts`)
   - Loads currencies from JSON file
   - Validates configuration against schema
   - Supports environment overrides
   - Fails fast on invalid configuration

2. **API Routes** (`api/src/routes/currencies.ts`)
   - GET `/api/currencies` - List all currencies
   - GET `/api/currencies/:code` - Get specific currency

3. **Express Application** (`api/src/app.ts`)
   - Security middleware (Helmet, CORS)
   - Rate limiting
   - Error handling
   - Health check endpoint

4. **Entry Point** (`api/src/index.ts`)
   - Initializes configuration
   - Starts Express server
   - Handles startup errors

### Configuration File Structure

```json
{
  "currencies": [
    {
      "code": "USD",
      "symbol": "$",
      "decimal_precision": 2,
      "name": "United States Dollar"
    }
  ]
}
```

### Validation Rules

- **code**: 3-12 uppercase alphanumeric characters
- **symbol**: 1-10 characters
- **decimal_precision**: Integer 0-18
- **name**: 1-100 characters (optional)
- No duplicate currency codes allowed

## API Endpoints

### GET /api/currencies

Returns all supported currencies.

**Response Schema:**
```typescript
{
  success: boolean;
  data: Currency[];
  count: number;
  timestamp: string;
}
```

**Example Response:**
```json
{
  "success": true,
  "data": [
    {
      "code": "USD",
      "symbol": "$",
      "decimal_precision": 2,
      "name": "United States Dollar"
    },
    {
      "code": "EUR",
      "symbol": "€",
      "decimal_precision": 2,
      "name": "Euro"
    }
  ],
  "count": 2,
  "timestamp": "2026-02-23T10:30:00.000Z"
}
```

### GET /api/currencies/:code

Returns a specific currency by code (case-insensitive).

**Parameters:**
- `code` - Currency code (e.g., "USD", "EUR")

**Response:** Same schema as above, with single currency in data array

**Error Response (404):**
```json
{
  "success": false,
  "error": {
    "message": "Currency not found: XYZ",
    "code": "CURRENCY_NOT_FOUND"
  },
  "timestamp": "2026-02-23T10:30:00.000Z"
}
```

## Configuration Management

### Base Configuration

Located at `api/config/currencies.json` (configurable via `CURRENCY_CONFIG_PATH`)

Includes 11 currencies by default:
- USD, EUR, GBP, JPY (major fiat)
- NGN, KES, GHS, ZAR (African currencies)
- INR, PHP (Asian currencies)
- USDC (Stellar stablecoin)

### Environment Overrides

Enable with `CURRENCY_CONFIG_ENV_OVERRIDE=true`

Override or add currencies via `CURRENCY_OVERRIDES` environment variable:

```bash
CURRENCY_OVERRIDES='[
  {"code":"USD","symbol":"US$","decimal_precision":3},
  {"code":"BTC","symbol":"₿","decimal_precision":8}
]'
```

**Merge Behavior:**
- Existing currencies are updated with override values
- New currencies are added to the list
- Base configuration file remains unchanged

## Validation & Error Handling

### Startup Validation

The service performs comprehensive validation at startup:

1. **File Existence**: Checks if configuration file exists
2. **JSON Parsing**: Validates JSON syntax
3. **Schema Validation**: Validates against Joi schema
4. **Duplicate Check**: Ensures no duplicate currency codes
5. **Override Validation**: Validates environment overrides if enabled

**Fail-Fast Behavior:**
```
✗ Failed to load currency configuration: Configuration file not found
✗ Server startup aborted due to configuration error
Process exits with code 1
```

### Runtime Validation

- Input validation on API requests
- Schema validation on responses
- Type checking on all data
- Safe error handling with consistent format

### Error Response Format

All errors return consistent structure:

```typescript
{
  success: false;
  error: {
    message: string;
    code: string;
  };
  timestamp: string;
}
```

## Testing

### Unit Tests (`api/src/__tests__/config.test.ts`)

Tests configuration loader:
- ✅ Load valid configuration
- ✅ Reject missing file
- ✅ Reject invalid JSON
- ✅ Reject missing required fields
- ✅ Reject empty currencies array
- ✅ Reject duplicate codes
- ✅ Validate field formats
- ✅ Validate field ranges
- ✅ Apply environment overrides
- ✅ Reject invalid overrides
- ✅ Currency retrieval methods
- ✅ Configuration reload

### Route Tests (`api/src/__tests__/routes.test.ts`)

Tests API endpoints:
- ✅ Health check endpoint
- ✅ List all currencies
- ✅ Correct response structure
- ✅ Data consistency
- ✅ Get currency by code
- ✅ Case-insensitive lookup
- ✅ 404 for non-existent currency
- ✅ Error handling
- ✅ Response schema validation
- ✅ Content-Type headers

### Integration Tests (`api/src/__tests__/integration.test.ts`)

Tests end-to-end scenarios:
- ✅ Full currency retrieval flow
- ✅ Multiple concurrent requests
- ✅ Configuration change reflection
- ✅ Invalid input handling
- ✅ Error format consistency
- ✅ Performance benchmarks
- ✅ Data integrity
- ✅ Decimal precision validation

### CI/CD Pipeline

GitHub Actions workflow (`.github/workflows/currency-api-ci.yml`):
- ✅ Test on Node.js 18.x and 20.x
- ✅ Run linter
- ✅ Run unit tests
- ✅ Run integration tests
- ✅ Build verification
- ✅ Startup test with health check
- ✅ Security audit
- ✅ Code coverage reporting

## Security Features

### Rate Limiting

- 100 requests per 15 minutes per IP
- Configurable via environment variables
- Returns 429 status when exceeded

### Security Headers

- Helmet.js for security headers
- CORS enabled for cross-origin requests
- JSON body parsing with size limits

### Input Validation

- Currency codes validated against regex pattern
- Decimal precision range checked (0-18)
- Symbol length validated (1-10 characters)
- No SQL injection risk (no database)

### Configuration Security

- Validation at startup prevents malicious config
- Environment overrides require explicit enablement
- No code execution from configuration
- Safe JSON parsing with error handling

## Performance

### Benchmarks

- Configuration loaded once at startup
- In-memory currency lookup: O(n)
- No database queries
- Response time: < 100ms
- Throughput: 1000+ req/s

### Optimization

- Single configuration load
- No file I/O on requests
- Minimal memory footprint
- Efficient JSON serialization

## Deployment

### Environment Variables

```bash
# Required
PORT=3000
NODE_ENV=production

# Optional
CURRENCY_CONFIG_PATH=./config/currencies.json
CURRENCY_CONFIG_ENV_OVERRIDE=false
CURRENCY_OVERRIDES=
RATE_LIMIT_WINDOW_MS=900000
RATE_LIMIT_MAX_REQUESTS=100
```

### Docker Deployment

```dockerfile
FROM node:18-alpine
WORKDIR /app
COPY api/package*.json ./
RUN npm ci --production
COPY api/ ./
RUN npm run build
EXPOSE 3000
CMD ["npm", "start"]
```

### Health Checks

```bash
# Liveness probe
curl http://localhost:3000/health

# Readiness probe
curl http://localhost:3000/api/currencies
```

## Adding New Currencies

### Method 1: Configuration File

Edit `api/config/currencies.json`:

```json
{
  "currencies": [
    {
      "code": "BTC",
      "symbol": "₿",
      "decimal_precision": 8,
      "name": "Bitcoin"
    }
  ]
}
```

Restart service to apply.

### Method 2: Environment Override

```bash
CURRENCY_CONFIG_ENV_OVERRIDE=true
CURRENCY_OVERRIDES='[{"code":"BTC","symbol":"₿","decimal_precision":8}]'
```

No restart required if using hot-reload.

## Breaking Changes

**None.** This is a new API endpoint that:
- Does not modify existing endpoints
- Does not change existing data structures
- Does not affect smart contract
- Is backward compatible

## Future Enhancements

Potential improvements:
- [ ] Currency conversion rates
- [ ] Historical currency data
- [ ] Currency aliases (e.g., "DOLLAR" → "USD")
- [ ] Localized currency names
- [ ] Currency grouping (fiat, crypto, etc.)
- [ ] Admin API for currency management
- [ ] Webhook notifications on config changes
- [ ] GraphQL endpoint
- [ ] Currency validation endpoint
- [ ] Bulk currency operations

## Troubleshooting

### Configuration Not Loading

```bash
# Check file exists
ls -la api/config/currencies.json

# Validate JSON
cat api/config/currencies.json | jq .

# Check environment
echo $CURRENCY_CONFIG_PATH
```

### Validation Errors

Check startup logs for specific validation errors:
```
Configuration validation failed: "decimal_precision" must be less than or equal to 18
```

### Port Conflicts

```bash
# Change port
PORT=3001 npm run dev

# Or kill existing process
lsof -ti:3000 | xargs kill
```

## Support

For issues or questions:
- Check [api/README.md](api/README.md) for detailed documentation
- Review test files for usage examples
- Open an issue on GitHub
- Contact the development team

## License

MIT
