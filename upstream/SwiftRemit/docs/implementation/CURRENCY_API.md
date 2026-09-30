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

**Alias resolution (#1547):** the `:code` segment also accepts any registered
alias for a currency (e.g. `DOLLAR`, `DOLLARS`, `NAIRA`).  Aliases are matched
case-insensitively.  The canonical currency object is always returned, so the
response is identical whether you request `USD` or `DOLLAR`.

**Parameters:**
- `code` - Currency code (e.g., "USD", "EUR") **or** a registered alias (e.g., "DOLLAR", "EURO")

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

### GET /api/currencies/:from/rates/:to

Returns the current exchange rate from one currency to another (#1545).

**Parameters:**
- `from` - Source currency code or alias (e.g., "USD", "DOLLAR")
- `to` - Target currency code or alias (e.g., "NGN", "NAIRA")

Rates are cached for 5 minutes.  When `EXCHANGE_RATE_API_URL` is set the
service fetches live rates from the configured provider; otherwise a set of
static seed rates (USD-pivoted) is used so development environments work
without credentials.

**Response:**
```json
{
  "success": true,
  "data": {
    "from": "USD",
    "to": "NGN",
    "rate": 1600.000000,
    "fetched_at": "2026-09-30T10:00:00.000Z"
  },
  "timestamp": "2026-09-30T10:00:00.000Z"
}
```

**Error Response (404):**
```json
{
  "success": false,
  "error": {
    "message": "Currency not found: XYZ",
    "code": "CURRENCY_NOT_FOUND"
  },
  "timestamp": "2026-09-30T10:00:00.000Z"
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

# #1545 — Currency conversion rates
# When set, conversion rate requests are proxied to this URL.
# Expected interface: GET {EXCHANGE_RATE_API_URL}?from=USD&to=EUR → { "rate": number }
# When unset, a static USD-pivoted seed table is used (suitable for dev/test).
EXCHANGE_RATE_API_URL=
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
- [x] Currency conversion rates — implemented: `GET /api/currencies/:from/rates/:to`. Closes #1545.
- [ ] Historical currency data
- [x] Currency aliases (e.g., "DOLLAR" → "USD") — implemented: `aliases` field on `Currency`; `GET /api/currencies/:code` resolves by alias. Closes #1547.
- [ ] Localized currency names
- [ ] Currency grouping (fiat, crypto, etc.)
- [ ] Admin API for currency management
- [ ] Webhook notifications on config changes
- [ ] GraphQL endpoint
- [ ] Currency validation endpoint
- [ ] Bulk currency operations

---

<!--
=============================================================================
Issue #1546 — Currency API: Historical currency data
https://github.com/Haroldwonder/SwiftRemit/issues/1546

PROBLEM
-------
The current API serves only the present state of currency configuration.
There is no way for clients, auditors, or the compliance team to query what
decimal_precision or symbol was in effect for a given currency at a past
point in time. This matters for retroactive invoice generation and audit trails.

PROPOSED IMPLEMENTATION
-----------------------
Add a versioned history store that snapshots the full CurrencyConfig
whenever a currency is created, updated, or deleted (via the admin API
from issue #1550).

Data model:

  interface CurrencyHistoryEntry {
    code: string;
    snapshot: CurrencyConfig;        // full config at this point in time
    changed_at: string;              // ISO 8601 timestamp
    changed_by: string;              // admin actor (JWT sub)
    change_type: 'created' | 'updated' | 'deleted';
    previous_snapshot?: CurrencyConfig; // null for 'created'
  }

Storage options (in order of implementation simplicity):
  1. Append-only JSONL file: api/data/currency_history.jsonl
  2. PostgreSQL table: currency_config_history
  3. Redis sorted set keyed by (code, timestamp)

Option 1 requires no new infrastructure and is appropriate for early
implementation; migrate to option 2 when the service adopts a DB.

New endpoints:

  GET /api/currencies/:code/history
    Query params:
      from  (ISO date, optional)
      to    (ISO date, optional)
      limit (integer, default 50, max 200)
    Returns: paginated list of CurrencyHistoryEntry sorted by changed_at desc.

  GET /api/currencies/:code/history/:timestamp
    Returns the CurrencyConfig that was active at the given timestamp.
    Algorithm: find the latest entry with changed_at <= timestamp.

ACCEPTANCE CRITERIA
-------------------
  ✅ Every admin mutation (create/update/delete) writes a history entry
  ✅ GET /api/currencies/:code/history returns entries in reverse chronological order
  ✅ GET /api/currencies/:code/history/:timestamp returns the config active at that moment
  ✅ History entries survive service restarts (persistent storage)
  ✅ Unit tests: history records written, point-in-time lookup returns correct snapshot

FILES TO ADD/MODIFY
-------------------
  api/src/services/currency-history.service.ts  ← new history store service
  api/src/routes/currencies.ts                  ← add /history routes
  api/data/currency_history.jsonl               ← new append-only history file
  api/src/__tests__/currency-history.test.ts    ← new test file

=============================================================================

Issue #1548 — Currency API: Localized currency names
https://github.com/Haroldwonder/SwiftRemit/issues/1548

PROBLEM
-------
The `name` field in CurrencyConfig is a single English string
(e.g. "United States Dollar"). SwiftRemit serves users across Africa,
Asia, and Latin America where the UI is localized. A Nigerian user sees
"United States Dollar" even when the UI language is set to Hausa or Igbo.

PROPOSED IMPLEMENTATION
-----------------------
Extend the CurrencyConfig schema with an optional `localized_names` map:

  interface CurrencyConfig {
    code: string;
    symbol: string;
    decimal_precision: number;
    name: string;                          // English fallback (required)
    localized_names?: Record<string, string>; // locale → name
  }

Example in currencies.json:

  {
    "code": "USD",
    "symbol": "$",
    "decimal_precision": 2,
    "name": "United States Dollar",
    "localized_names": {
      "fr": "Dollar américain",
      "es": "Dólar estadounidense",
      "sw": "Dola ya Marekani",
      "ha": "Dalar Amurka",
      "yo": "Dola Amẹrika"
    }
  }

API behavior:

  GET /api/currencies?locale=fr
    Returns currencies with `display_name` set to localized_names["fr"]
    if present, falling back to `name` if not.

  GET /api/currencies/:code?locale=sw
    Returns single currency with `display_name` in Swahili.

The `locale` query parameter follows BCP 47 (e.g. "fr", "sw", "en-NG").
Lookup tries exact match first, then language subtag only (e.g. "en-NG" → "en").

ACCEPTANCE CRITERIA
-------------------
  ✅ CurrencyConfig schema extended with optional localized_names map
  ✅ locale query param accepted on both GET /api/currencies endpoints
  ✅ display_name field in response reflects active locale with English fallback
  ✅ Joi validation schema updated to allow localized_names
  ✅ Unit tests: locale lookup, fallback to English, unknown locale

FILES TO MODIFY
---------------
  api/config/currencies.json          ← add localized_names to default currencies
  api/src/config.ts                   ← extend Joi schema, add getLocalizedName()
  api/src/routes/currencies.ts        ← pass locale param, set display_name
  api/src/__tests__/config.test.ts    ← tests for localized_names validation
  api/src/__tests__/routes.test.ts    ← tests for locale query parameter

=============================================================================

Issue #1549 — Currency API: Currency grouping (fiat, crypto, etc.)
https://github.com/Haroldwonder/SwiftRemit/issues/1549

PROBLEM
-------
The API returns all currencies in a flat list with no way to filter or
group by type. As the platform adds more digital assets (USDC, BTC, ETH)
alongside fiat currencies, clients need a structured way to display
"Send with crypto" vs "Send with fiat" sections in the UI.

PROPOSED IMPLEMENTATION
-----------------------
Add a `group` field to CurrencyConfig:

  type CurrencyGroup = 'fiat' | 'crypto' | 'stablecoin' | 'commodity' | 'other';

  interface CurrencyConfig {
    code: string;
    symbol: string;
    decimal_precision: number;
    name: string;
    group: CurrencyGroup;   // required for all currencies
    localized_names?: Record<string, string>;
  }

Default groupings for existing currencies:
  fiat:        USD, EUR, GBP, JPY, NGN, KES, GHS, ZAR, INR, PHP
  stablecoin:  USDC

New endpoints:

  GET /api/currencies/groups
    Returns all available group names:
      { "groups": ["fiat", "stablecoin"] }

  GET /api/currencies?group=fiat
    Returns only currencies in the "fiat" group.
    Can be combined with ?locale= for localized fiat names.

  GET /api/currencies?group=stablecoin
    Returns only stablecoins.

The existing GET /api/currencies (no group filter) continues to return all
currencies unchanged — no breaking change.

ACCEPTANCE CRITERIA
-------------------
  ✅ group field added to CurrencyConfig schema (required, validated enum)
  ✅ All 11 default currencies assigned correct group values
  ✅ GET /api/currencies?group=fiat filters correctly
  ✅ GET /api/currencies/groups returns available groups
  ✅ Combinable with ?locale= parameter (from issue #1548)
  ✅ Joi validation rejects unknown group values
  ✅ Unit and integration tests for group filtering

FILES TO MODIFY
---------------
  api/config/currencies.json      ← add group field to all currencies
  api/src/config.ts               ← extend Joi schema, add group to Currency type
  api/src/routes/currencies.ts    ← add group filter and /groups endpoint
  api/src/__tests__/config.test.ts   ← tests for group validation
  api/src/__tests__/routes.test.ts   ← tests for group filter query param

=============================================================================

Issue #1552 — Currency API: GraphQL endpoint
https://github.com/Haroldwonder/SwiftRemit/issues/1552

PROBLEM
-------
The REST API works well for simple lookups but requires multiple round trips
when a client needs a filtered, paginated, localized list of currencies in a
specific group. GraphQL lets the client specify exactly what fields and filters
it needs in one request, reducing over-fetching and under-fetching.

PROPOSED IMPLEMENTATION
-----------------------
Add a GraphQL endpoint alongside the existing REST API (additive, not replacing).

Schema:

  type Currency {
    code: String!
    symbol: String!
    decimal_precision: Int!
    name: String!
    display_name: String        # locale-aware name (requires locale arg)
    group: CurrencyGroup!
    localized_names: JSON       # raw map for advanced clients
  }

  enum CurrencyGroup {
    fiat
    crypto
    stablecoin
    commodity
    other
  }

  type Query {
    currencies(
      group: CurrencyGroup
      locale: String
      codes: [String]           # filter by specific codes
    ): [Currency!]!

    currency(code: String!, locale: String): Currency
  }

Endpoint: POST /api/graphql (standard GraphQL over HTTP)
         GET  /api/graphql (GraphiQL IDE in development mode only)

Example query:

  query FiatCurrencies {
    currencies(group: fiat, locale: "fr") {
      code
      symbol
      display_name
      decimal_precision
    }
  }

Implementation using graphql-js + express-graphql or graphql-yoga:

  npm install graphql graphql-yoga   # or express-graphql

The resolvers read from the same in-memory CurrencyConfig store used by
the REST routes — no separate data source, no duplication of business logic.

Security:
  - Depth limit: max 3 levels (currencies have no nested relations)
  - Complexity limit: max 100 (prevents field-explosion attacks)
  - Rate limiting: same 100 req/15min as REST endpoints
  - Auth: read-only, no mutations (mutations go through admin REST API)
  - Introspection disabled in production (enabled in development only)

DEPENDENCY ON #1548 AND #1549
------------------------------
GraphQL locale argument requires the localized_names field (#1548).
GraphQL group filter requires the group field (#1549).
Implement #1548 and #1549 first, then add the GraphQL layer on top.

ACCEPTANCE CRITERIA
-------------------
  ✅ POST /api/graphql handles currencies and currency queries
  ✅ group and locale args filter/localize results correctly
  ✅ Depth and complexity limits enforced
  ✅ Introspection disabled in production
  ✅ GraphiQL available in development mode
  ✅ Existing REST endpoints unchanged (additive change)
  ✅ Unit tests for each resolver
  ✅ Integration test: GraphQL query returns same data as equivalent REST call

FILES TO ADD/MODIFY
-------------------
  api/src/graphql/schema.ts           ← new GraphQL type definitions
  api/src/graphql/resolvers.ts        ← new resolvers (delegate to config store)
  api/src/graphql/index.ts            ← new graphql-yoga or express-graphql setup
  api/src/app.ts                      ← mount /api/graphql endpoint
  api/src/__tests__/graphql.test.ts   ← new GraphQL test file
  api/package.json                    ← add graphql dependency

IMPLEMENTATION ORDER
---------------------
  1. #1548 (localized_names)   — extends CurrencyConfig schema
  2. #1549 (currency groups)   — adds group field
  3. #1550 (admin API)         — adds mutation capability
  4. #1551 (webhooks)          — depends on #1550
  5. #1552 (GraphQL)           — wraps everything above in a query layer

=============================================================================
-->


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
