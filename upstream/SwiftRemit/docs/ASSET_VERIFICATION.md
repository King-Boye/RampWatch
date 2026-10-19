# Asset Verification System

Comprehensive asset issuer verification system for SwiftRemit that validates Stellar assets against multiple trusted sources.

## Overview

The asset verification system provides a hybrid approach combining on-chain storage with off-chain verification services to ensure asset safety and build trust in the remittance platform.

## Architecture

### Components

1. **Smart Contract (Soroban)**
   - Stores verification results on-chain
   - Provides query functions for verification status
   - Validates asset safety before transactions
   - Admin-controlled verification updates

2. **Backend Service (Node.js/TypeScript)**
   - AssetVerifier service for multi-source verification
   - PostgreSQL database for caching and persistence
   - RESTful API for verification queries
   - Background job scheduler for periodic revalidation

3. **Frontend Component (React)**
   - VerificationBadge component for visual indicators
   - Detailed verification information modals
   - Warning system for suspicious assets
   - Community reporting functionality

## Verification Process

### Sources Checked

1. **Stellar Expert**
   - Asset rating and age
   - Community trust indicators
   - Score: 0-100 based on rating

2. **Stellar TOML**
   - Home domain validation
   - TOML file structure check
   - Documentation presence
   - Score: 80 if valid, 30 if partial, 0 if missing

3. **Trustline Analysis**
   - Number of accounts holding the asset
   - Score: 100 (10k+), 80 (1k+), 60 (100+), 40 (10+), 20 (<10)

4. **Transaction History**
   - Recent activity (last 30 days)
   - Historical transaction count
   - Score: 70 (recent + historical), 50 (historical only), 30 (recent only)

### Reputation Score Calculation

```
reputation_score = sum(passed_score * source_weight) / sum(passed_source_weight)
```

### Status Assignment

- **Verified**: Score ≥ 70 AND ≥ 3 sources verified
- **Suspicious**: Score < 30 OR suspicious indicators detected
- **Unverified**: All other cases

### Suspicious Indicators

- No stellar.toml file
- < 5 trustlines
- No transaction history
- Multiple community reports (≥ 5)

## Smart Contract Functions

### Admin Functions

```rust
pub fn set_asset_verification(
    env: Env,
    asset_code: String,
    issuer: Address,
    status: VerificationStatus,
    reputation_score: u32,
    trustline_count: u64,
    has_toml: bool,
) -> Result<(), ContractError>
```

Stores or updates asset verification data. Admin only.

### Query Functions

```rust
pub fn get_asset_verification(
    env: Env,
    asset_code: String,
    issuer: Address,
) -> Result<AssetVerification, ContractError>
```

Retrieves verification data for an asset.

```rust
pub fn has_asset_verification(
    env: Env,
    asset_code: String,
    issuer: Address,
) -> bool
```

Checks if verification data exists.

```rust
pub fn validate_asset_safety(
    env: Env,
    asset_code: String,
    issuer: Address,
) -> Result<(), ContractError>
```

Validates that an asset is not flagged as suspicious.

## API Endpoints

### GET /api/verification/:assetCode/:issuer

Retrieve verification status for an asset.

**Response:**
```json
{
  "asset_code": "USDC",
  "issuer": "GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN",
  "status": "verified",
  "reputation_score": 95,
  "last_verified": "2026-02-23T10:30:00Z",
  "trustline_count": 15000,
  "has_toml": true,
  "stellar_expert_verified": true,
  "community_reports": 0
}
```

### POST /api/verification/verify

Trigger new verification for an asset.

**Request:**
```json
{
  "assetCode": "USDC",
  "issuer": "GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN"
}
```

### POST /api/verification/report

Report suspicious asset.

**Request:**
```json
{
  "assetCode": "SCAM",
  "issuer": "GXXX...",
  "reason": "Phishing attempt"
}
```

### GET /api/verification/verified

List verified assets.

**Query Parameters:**
- `limit`: Max results (default: 100, max: 500)

### POST /api/verification/batch

Batch verification lookup (max 50 assets).

**Request:**
```json
{
  "assets": [
    { "assetCode": "USDC", "issuer": "GA5Z..." },
    { "assetCode": "BTC", "issuer": "GBXX..." }
  ]
}
```

## Frontend Usage

### Basic Usage

```tsx
import { VerificationBadge } from './components/VerificationBadge';

function AssetDisplay() {
  return (
    <div>
      <h3>USDC</h3>
      <VerificationBadge
        assetCode="USDC"
        issuer="GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN"
        apiUrl="http://localhost:3000"
      />
    </div>
  );
}
```

### With Warning Callback

```tsx
<VerificationBadge
  assetCode="UNKNOWN"
  issuer="GXXX..."
  onWarning={(verification) => {
    console.warn('Asset warning:', verification);
    // Show custom warning UI
  }}
/>
```

## Database Schema

```sql
CREATE TABLE verified_assets (
  id SERIAL PRIMARY KEY,
  asset_code VARCHAR(12) NOT NULL,
  issuer VARCHAR(56) NOT NULL,
  status VARCHAR(20) NOT NULL,
  reputation_score INTEGER NOT NULL CHECK (reputation_score >= 0 AND reputation_score <= 100),
  last_verified TIMESTAMP NOT NULL DEFAULT NOW(),
  trustline_count BIGINT NOT NULL DEFAULT 0,
  has_toml BOOLEAN NOT NULL DEFAULT FALSE,
  stellar_expert_verified BOOLEAN DEFAULT FALSE,
  toml_data JSONB,
  community_reports INTEGER DEFAULT 0,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
  UNIQUE(asset_code, issuer)
);
```

## Security Features

### Input Validation

- Asset code: Max 12 characters
- Issuer: Exactly 56 characters (Stellar address format)
- Reputation score: 0-100 range enforced
- Report reason: Max 500 characters

### Rate Limiting

- 100 requests per 15 minutes per IP
- Configurable via environment variables

### Protection Against Abuse

- Unique constraint on (asset_code, issuer)
- Community report threshold (5 reports → suspicious)
- Admin-only on-chain updates
- Request timeouts and retries

### Safe HTTP Clients

- 5-second timeout per request
- 3 retry attempts with exponential backoff
- Error handling for malformed responses
- Graceful degradation on source failures

## Background Jobs

### Periodic Revalidation

- Runs every 6 hours
- Revalidates assets older than 24 hours
- Updates both database and on-chain storage
- Rate-limited to 1 asset per second

## Configuration

### Environment Variables

```bash
# Database
DATABASE_URL=postgresql://user:password@localhost:5432/swiftremit

# Stellar
STELLAR_NETWORK=testnet
HORIZON_URL=https://horizon-testnet.stellar.org
CONTRACT_ID=CXXX...
ADMIN_SECRET_KEY=SXXX...

# API
PORT=3000
NODE_ENV=development

# Rate Limiting
RATE_LIMIT_WINDOW_MS=900000
RATE_LIMIT_MAX_REQUESTS=100

# Verification
VERIFICATION_INTERVAL_HOURS=24
MIN_TRUSTLINE_COUNT=10
MIN_REPUTATION_SCORE=50
```

## Deployment

### Backend Service

```bash
cd backend
npm install
cp .env.example .env
# Edit .env with your configuration
npm run build
npm start
```

### Database Setup

```bash
# PostgreSQL must be running
psql -U postgres -c "CREATE DATABASE swiftremit;"
# Tables are created automatically on first run
```

### Smart Contract

```bash
# Build contract
cargo build --target wasm32-unknown-unknown --release
stellar contract optimize --wasm target/wasm32-unknown-unknown/release/swiftremit.wasm

# Deploy
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/swiftremit.optimized.wasm \
  --source deployer \
  --network testnet
```

## Testing

### Backend Tests

```bash
cd backend
npm test
```

### Smart Contract Tests

```bash
cargo test
```

### Integration Tests

```bash
# Start backend service
cd backend && npm start

# Run integration tests
npm run test:integration
```

## Error Handling

### Contract Errors

- `AssetNotFound (13)`: Asset not in verification database
- `InvalidReputationScore (14)`: Score not in 0-100 range
- `SuspiciousAsset (15)`: Asset flagged as suspicious

### API Errors

- `400`: Invalid input parameters
- `404`: Asset verification not found
- `429`: Rate limit exceeded
- `500`: Internal server error

## Performance Considerations

- Database indexes on (asset_code, issuer), status, last_verified
- Verification results cached in database
- On-chain storage for critical data only
- Batch API for multiple asset lookups
- Background jobs prevent blocking user requests

## Future Enhancements

- [ ] Machine learning for fraud detection
- [ ] Integration with additional anchor registries
- [ ] Real-time event streaming for verification updates
- [ ] Multi-network support (mainnet, testnet)
- [ ] Advanced analytics dashboard
- [ ] Automated dispute resolution
- [ ] Reputation decay over time
- [ ] Weighted scoring based on source reliability

## Support

For issues or questions:
- GitHub Issues: [Report issues](https://github.com/yourusername/swiftremit/issues)
- Documentation: See this file and inline code comments
- Stellar Discord: https://discord.gg/stellar

## Network-Aware Verification and Scoring Details

The following operational details previously appeared only in the root guide.

### Supported Networks

| Network  | Description                                             |
| -------- | ------------------------------------------------------- |
| mainnet  | Production network with real assets and live metadata.  |
| testnet  | Test network with sandbox assets and staging metadata.  |

## Configuration

The active network is selected through configuration. Verification resolves the
network first, then loads the asset registry and metadata source associated with
that network.

```
ASSET_VERIFICATION_NETWORK=mainnet   # or: testnet
```

When no network is configured, verification defaults to `mainnet` to preserve
existing behavior.

## Verification Flow

1. Resolve the active network (`mainnet` or `testnet`).
2. Load the asset registry for that network.
3. Verify each asset against the network-specific metadata source.
4. Report results, tagging every result with the network it was verified on.

## Network-Specific Behavior

- **Asset registry:** each network maintains its own registry of known assets.
- **Metadata source:** mainnet uses production metadata; testnet uses staging
  metadata.
- **Contract addresses:** addresses are resolved per network and are never
  shared across networks.
- **Result reporting:** verification results include the network so that
  mainnet and testnet results are never conflated.

## Reputation Decay

Cached verification results degrade over time so that stale data does not
permanently lock an asset into a high-reputation state.  The decay model is
**linear**: a result keeps its full score when fresh and reaches zero after
2 × `DECAY_HALF_LIFE` (currently 48 hours).

```
decayed_score = score × max(0, 1 − age / (2 × half_life))
```

The stored score is never mutated — decay is applied on read so the original
verification result remains available for audit purposes.  When the decayed
score crosses a status boundary the displayed status updates accordingly:

| Decayed score | Status      |
| ------------- | ----------- |
| ≥ 70 (+ ≥ 3 verified sources) | `verified`   |
| < 30 or suspicious indicators  | `suspicious` |
| otherwise                      | `unverified` |

The `verified_at` ISO-8601 timestamp on every `VerificationResult` is the
anchor used to compute age.

## Weighted Scoring

Each verification source is assigned a **reliability weight** that reflects
its quality as a signal.  The final `reputation_score` is a weighted average
of the scores from sources that individually passed their verification check.

| Source               | Weight | Rationale                                         |
| -------------------- | ------ | ------------------------------------------------- |
| Stellar Expert       | 0.40   | Curated community rating; highest signal quality  |
| Stellar TOML         | 0.30   | Issuer self-attestation with on-chain anchor      |
| Trustline Analysis   | 0.20   | On-chain adoption; gameable at low cost           |
| Transaction History  | 0.10   | Activity proxy; least discriminating signal       |

Weights are normalised internally.  A source that fails its check contributes
`score = 0` but is **excluded** from the weighted average so it does not drag
down a result where other sources could not be reached.

The `reliability_weight` field is exposed on each `VerificationSource` object
in the API response so consumers can see how the final score was computed.
