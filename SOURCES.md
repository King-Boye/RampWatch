# Sources

Every file taken or ported into RampWatch, with where it came from.

Code is taken from the **Ndifreke000** repositories. Where a piece the spec needs is missing from a Ndifreke000 fork, that piece comes from the original upstream repo, and the row says so.

| Repo | Commit used | Role |
| --- | --- | --- |
| [Ndifreke000/stellar-insights](https://github.com/Ndifreke000/stellar-insights) | `b17fcc2c5540` (2026-10-27) | Main backend donor (`backend/`) |
| [Ndifreke000/fluxapay](https://github.com/Ndifreke000/fluxapay) (fork) | `496bc70cb628` (2026-04-24) | Webhook signing, idempotency |
| [Ndifreke000/SwiftRemit](https://github.com/Ndifreke000/SwiftRemit) (fork) | `45307d657bd5` (2026-02-25) | Payout state machine |
| [Ndifreke000/AnchorKit](https://github.com/Ndifreke000/AnchorKit) (fork) | `222130af55ec` (2026-05-29) | Health score, retry, attestation (milestones 3-4) |
| [MetroLogic/fluxapay](https://github.com/MetroLogic/fluxapay) (original, gaps only) | `f8ade8ea0484` (2026-11-04) | Secondary-signature rotation, timestamp window, idempotency TTL |
| [Haroldwonder/SwiftRemit](https://github.com/Haroldwonder/SwiftRemit) (original, gaps only) | `45aec0100d73` (2026-10-25) | `Disputed` state and its edges, `retry_transaction` |

`SI` below = `https://github.com/Ndifreke000/stellar-insights/blob/b17fcc2c5540/backend/`.

## Milestone 1: skeleton and intake

| RampWatch file | Action | Origin |
| --- | --- | --- |
| `backend/src/error.rs` | Copy | [SI `src/error.rs`](https://github.com/Ndifreke000/stellar-insights/blob/b17fcc2c5540/backend/src/error.rs). Paths fixed; RPC conversion and metrics hook removed until those modules arrive; `status_code` made public. |
| `backend/src/api/middleware/request_id.rs` | Copy | [SI `src/request_id.rs`](https://github.com/Ndifreke000/stellar-insights/blob/b17fcc2c5540/backend/src/request_id.rs) |
| `backend/src/api/middleware/rate_limit.rs` | Copy | [SI `src/rate_limit.rs`](https://github.com/Ndifreke000/stellar-insights/blob/b17fcc2c5540/backend/src/rate_limit.rs). IETF `RateLimit-*` headers and Redis-with-memory-fallback kept. Removed: billing tiers, `user_subscriptions` lookups, JWT users, key-id logging hook. Key prefix `si_` → `rw_`. Fixed the key lookup query (upstream selects `*` into a struct needing `rate_limit_per_minute`, which `api_keys` lacks, so no key ever resolved). |
| `backend/src/auth/api_key_model.rs` | Copy | [SI `src/models/api_key.rs`](https://github.com/Ndifreke000/stellar-insights/blob/b17fcc2c5540/backend/src/models/api_key.rs). `wallet_address` → `operator_id`, `si_live_` → `rw_live_`, OpenAPI derive dropped. |
| `backend/src/auth/api_key_db.rs` | Copy | [SI `src/db/api_keys.rs`](https://github.com/Ndifreke000/stellar-insights/blob/b17fcc2c5540/backend/src/db/api_keys.rs). `wallet_address` → `operator_id`. |
| `backend/src/auth/api_key_routes.rs` | Adapt | [SI `src/api/api_keys.rs`](https://github.com/Ndifreke000/stellar-insights/blob/b17fcc2c5540/backend/src/api/api_keys.rs). Owner is the authenticated operator instead of a SEP-10 wallet; mounted at `/v1/api-keys`. |
| `backend/src/auth/api_keys.rs` | Adapt | Operator auth middleware built on the copied rate limiter's cached Argon2 key check ([SI `src/rate_limit.rs`](https://github.com/Ndifreke000/stellar-insights/blob/b17fcc2c5540/backend/src/rate_limit.rs) `resolve_api_key_id`). |
| `backend/src/auth/hmac.rs` | Port | Signing string `timestamp + "." + body` from [fork `webhook.service.ts` `generateWebhookSignature`](https://github.com/Ndifreke000/fluxapay/blob/496bc70cb628/fluxapay_backend/src/services/webhook.service.ts). **From the original (missing in fork):** `X-*-Signature-Secondary` rotation, `verifyWebhookTimestamp` with `WEBHOOK_TIMESTAMP_TOLERANCE_SECONDS` ([original `webhook.service.ts`](https://github.com/MetroLogic/fluxapay/blob/f8ade8ea0484/fluxapay_backend/src/services/webhook.service.ts)) and constant-time compare ([original `crypto.helper.ts`](https://github.com/MetroLogic/fluxapay/blob/f8ade8ea0484/fluxapay_backend/src/helpers/crypto.helper.ts)). Middleware shape from [SI `src/request_signing_middleware.rs`](https://github.com/Ndifreke000/stellar-insights/blob/b17fcc2c5540/backend/src/request_signing_middleware.rs). |
| `backend/src/intake/idempotency.rs` | Port | Body hash, conflict on a different body, stored-response replay from [fork `idempotency.middleware.ts`](https://github.com/Ndifreke000/fluxapay/blob/496bc70cb628/fluxapay_backend/src/middleware/idempotency.middleware.ts). **From the original (missing in fork):** 24h TTL, `X-Idempotent-Replayed` header, expired-record cleanup ([original](https://github.com/MetroLogic/fluxapay/blob/f8ade8ea0484/fluxapay_backend/src/middleware/idempotency.middleware.ts)). |
| `backend/src/domain/state.rs` | Port | Base edges (pending → processing/cancelled, processing → completed/failed) from [fork `src/transitions.rs`](https://github.com/Ndifreke000/SwiftRemit/blob/45307d657bd5/src/transitions.rs). **From the original (missing in fork):** `Disputed`, `Failed`/`Disputed` edges, processing → cancelled (refund), idempotent same-state rule ([original `transitions.rs`](https://github.com/Haroldwonder/SwiftRemit/blob/45aec0100d73/src/transitions.rs), [`types.rs`](https://github.com/Haroldwonder/SwiftRemit/blob/45aec0100d73/src/types.rs)). |
| `backend/src/domain/retry_rules.rs` | Port | **From the original (missing in fork):** [`transaction_controller.rs` `retry_transaction`](https://github.com/Haroldwonder/SwiftRemit/blob/45aec0100d73/src/transaction_controller.rs). |
| `backend/src/env_config.rs` | Copy | [SI `src/env_config.rs`](https://github.com/Ndifreke000/stellar-insights/blob/b17fcc2c5540/backend/src/env_config.rs). Required vars cut to `DATABASE_URL`; snapshot contract check now checks `REGISTRY_CONTRACT_ID`. |
| `backend/src/config.rs` | Adapt | Typed config for the spec's section 11 variables, beside the copied `env_config.rs` validation. |
| `backend/src/validation.rs` | Copy | [SI `src/validation.rs`](https://github.com/Ndifreke000/stellar-insights/blob/b17fcc2c5540/backend/src/validation.rs) |
| `backend/src/shutdown.rs` | Copy | [SI `src/shutdown.rs`](https://github.com/Ndifreke000/stellar-insights/blob/b17fcc2c5540/backend/src/shutdown.rs). Cache and websocket flush helpers removed. |
| `backend/src/db.rs` | Adapt | Pool options (busy timeout, WAL) from [SI `src/database.rs`](https://github.com/Ndifreke000/stellar-insights/blob/b17fcc2c5540/backend/src/database.rs); migrations only. |
| `backend/src/main.rs`, `lib.rs` | Adapt | [SI `src/main.rs`](https://github.com/Ndifreke000/stellar-insights/blob/b17fcc2c5540/backend/src/main.rs), [`src/lib.rs`](https://github.com/Ndifreke000/stellar-insights/blob/b17fcc2c5540/backend/src/lib.rs) (`lock_env_test`). Rewired to RampWatch modules; graceful shutdown kept. |
| `backend/migrations/0001_init.sql` | Adapt | Spec 8.1 tables, plus SI migrations [014](https://github.com/Ndifreke000/stellar-insights/blob/b17fcc2c5540/backend/migrations/014_create_api_keys.sql), [016](https://github.com/Ndifreke000/stellar-insights/blob/b17fcc2c5540/backend/migrations/016_create_telegram_subscriptions.sql), [027](https://github.com/Ndifreke000/stellar-insights/blob/b17fcc2c5540/backend/migrations/027_create_alert_rules.sql), [028](https://github.com/Ndifreke000/stellar-insights/blob/b17fcc2c5540/backend/migrations/028_create_alert_history.sql), [032](https://github.com/Ndifreke000/stellar-insights/blob/b17fcc2c5540/backend/migrations/032_create_api_keys_rate_limit_config.sql), [041](https://github.com/Ndifreke000/stellar-insights/blob/b17fcc2c5540/backend/migrations/041_create_corridor_alert_configs.sql), [042](https://github.com/Ndifreke000/stellar-insights/blob/b17fcc2c5540/backend/migrations/042_create_corridor_alert_events.sql) re-keyed to operators. |

Written new for milestone 1 (no source has them): `intake/events.rs`, `intake/csv_import.rs`, `api/payouts.rs`, `api/admin.rs`, `domain/models.rs`, `tests/intake.rs`, `scripts/send_event.sh`.
