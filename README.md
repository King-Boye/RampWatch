# RampWatch

[![CI](https://github.com/King-Boye/RampWatch/actions/workflows/ci.yml/badge.svg)](https://github.com/King-Boye/RampWatch/actions/workflows/ci.yml) [![Security](https://github.com/King-Boye/RampWatch/actions/workflows/security.yml/badge.svg)](https://github.com/King-Boye/RampWatch/actions/workflows/security.yml) [![CodeQL](https://github.com/King-Boye/RampWatch/actions/workflows/codeql.yml/badge.svg)](https://github.com/King-Boye/RampWatch/actions/workflows/codeql.yml)

Health monitoring for teams that run their own off-ramp or payout rails on Stellar. Operators send every cash-out state change; RampWatch checks it against the payout state machine and the Stellar ledger, flags stuck payouts and risky resends, scores each corridor, and anchors a weekly report hash on-chain.

Workspace:

- `backend/`: Rust / Axum API (SQLite via sqlx)
- `contracts/registry/`: Soroban report registry (milestone 4)

## Status

| Milestone | State |
| --- | --- |
| M1 Skeleton and intake | Done |
| M2 Ledger matching | Next |
| M3 Detectors and alerts | |
| M4 Reports and registry contract | |
| M5 Learned baselines | After pilot start |

## Run

```bash
cp .env.example .env        # set ADMIN_TOKEN (32+ chars)
cargo run -p rampwatch-backend
```

Create an operator (the API key and HMAC secret are shown once):

```bash
curl -X POST localhost:8080/v1/admin/operators \
  -H "Authorization: Bearer $ADMIN_TOKEN" -H 'content-type: application/json' \
  -d '{"name":"Demo Wallet","stellar_accounts":["G..."]}'
```

Send a signed event:

```bash
RAMPWATCH_API_KEY=rw_live_... RAMPWATCH_HMAC_SECRET=whsec_... scripts/send_event.sh processing co_8f2a
```

## Intake API

All intake routes need `Authorization: Bearer <api_key>` and an HMAC signature:

- `X-RampWatch-Timestamp`: unix seconds (or RFC 3339), within 300s
- `X-RampWatch-Signature`: hex `hmac_sha256(secret, timestamp + "." + body)`
- `X-RampWatch-Signature-Secondary`: optional, during a secret rotation

| Method | Path | |
| --- | --- | --- |
| POST | `/v1/payouts/events` | One event. `Idempotency-Key` header required. A replay returns the first response with `X-Idempotent-Replayed: true`. |
| POST | `/v1/payouts/events/batch` | Up to 500 events (`{"events":[...]}`), each processed on its own |
| POST | `/v1/payouts/import` | CSV with a header row, same fields |
| GET/POST/DELETE | `/v1/api-keys`, `/v1/api-keys/{id}`, `/v1/api-keys/{id}/rotate` | Operator manages its own keys |
| POST | `/v1/admin/operators` | Create operator (admin token) |
| POST | `/v1/admin/operators/{id}/rotate-secret` | New HMAC secret; old one valid 24h |
| GET | `/health` | Liveness |

Illegal state changes get `409 invalid_transition` and open an `illegal_transition` incident.

## Test

```bash
cargo build && cargo test
```

Every pull request must pass CI and the security scans (dependency audit, OSV, ClamAV malware scan, gitleaks, dependency review, CodeQL) before it can merge; see [SECURITY.md](SECURITY.md). Dependabot keeps crates and Actions up to date and auto-merges minor and patch updates once checks pass.

See [SOURCES.md](SOURCES.md) for where each file came from.
