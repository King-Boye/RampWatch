# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **#925** Automated mainnet deployment checklist CI (`mainnet-checklist.yml`) enforcing WASM build, full test suite, no TODO/FIXME in security-critical files, CHANGELOG update, and Clippy strict; `deploy-mainnet.yml` workflow blocks on checklist passing. Manual checklist items documented in `PRODUCTION_READINESS_CHECKLIST.md`.
- **#933** Cargo-fuzz targets for `validate_amount`, corridor validation, and `validate_evidence_hash` (recipient hash), with seed corpus and a 30-second CI job (`fuzz-ci.yml`).
- **#938** Content Security Policy enforced on the frontend: strict CSP in Vite dev server, `<meta>` tag injection for production builds, HTTP response headers in `vercel.json` and `_headers`. Staging uses `Content-Security-Policy-Report-Only` first before production enforcement.
- **#940** Input sanitization applied to all user-supplied string fields across backend API endpoints (`api.ts`, `routes/compliance.ts`) and API service routes (`agents`, `auth`, `anchors`). Parameterized query audit confirmed across all DB-touching code paths.

### Fixed
- Dark mode support with CSS custom properties and theme toggle component
- Correlation ID propagation from API through to webhook delivery
- CHANGELOG.md following Keep a Changelog format
- Automated release workflow with GitHub Actions

### Security
- All free-form string fields in `backend/src/api.ts` and `backend/src/routes/compliance.ts` now pass through `sanitizeInput` (XSS library) before storage.
- API service (`api/src/routes/`) sanitizes `name`, `payout_address`, `userId`, and anchor provider string fields via a new `sanitizeInput`/`sanitizeObject` utility.
- `validateAssetParams` middleware now mutates `req.body` with sanitized `assetCode` and `issuer` before passing to route handlers.

### Fixed (previous)
- `withdraw_integrator_fees` correctly returns `NoFeesToWithdraw` when balance is zero

## [1.0.0] - 2024-01-15

### Added
- Escrow-based remittance system with USDC on Stellar/Soroban
- Agent network registration and management
- Automated fee collection and withdrawal
- Lifecycle state management (Pending, Processing, Completed, Cancelled)
- Role-based access control for all operations
- Comprehensive event emission for off-chain monitoring
- Cancellation support with full refund capability
- Admin controls for platform fee management
- Daily send limits per currency/country with rolling 24h windows
- Off-chain proof commitments with optional validation
- Asset verification via Stellar Expert API and stellar.toml
- Circuit breaker for emergency pause functionality
- Rate limiting and abuse protection
- Webhook system with HMAC signature verification
- Webhook delivery retry with exponential backoff
- Dead-letter queue for failed webhook deliveries
- KYC integration with anchor services
- FX rate caching and currency conversion API
- Transaction state machine with enforced transitions
- Health check endpoints for monitoring
- OpenAPI documentation
- Property-based testing for fee calculations
- Integration tests for contract upgrade scenarios
- Frontend React application with Stellar wallet integration
- TypeScript SDK for contract interaction
- PostgreSQL backend for off-chain data
- Docker containerization for all services
- CI/CD pipeline with GitHub Actions

### Security
- HMAC-SHA256 webhook signature verification
- XSS sanitization for user inputs
- Admin audit logging
- Blacklist functionality for malicious actors
- Token whitelist for approved assets

