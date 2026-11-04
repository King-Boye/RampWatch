-- RampWatch initial schema.
-- Core tables follow spec section 8.1. Alert, Telegram and API key tables are
-- adapted from stellar-insights backend migrations 014, 016, 027, 028, 032, 041, 042,
-- re-keyed from users/wallets to operators.
--
-- All timestamps are RFC 3339 UTC strings with millisecond precision
-- (e.g. 2026-11-04T19:40:12.000Z), so they sort and compare as text.

CREATE TABLE operators (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  stellar_accounts TEXT NOT NULL,          -- JSON array of distribution accounts
  -- The spec names this column hmac_secret_hash. HMAC verification needs the
  -- secret itself (a hash of it can't check a signature), so the raw secret is
  -- stored, as fluxapay does with webhook_secret.
  hmac_secret TEXT NOT NULL,
  -- Previous secret, still accepted until it expires (rotation grace window).
  hmac_secret_previous TEXT,
  hmac_secret_previous_expires_at TEXT,
  telegram_chat_id TEXT,
  stuck_after_secs INTEGER NOT NULL DEFAULT 900,
  created_at TEXT NOT NULL
);

CREATE TABLE payouts (
  id TEXT PRIMARY KEY,                     -- rampwatch id
  operator_id TEXT NOT NULL REFERENCES operators(id),
  external_id TEXT NOT NULL,               -- operator's cash-out id
  corridor TEXT NOT NULL,                  -- e.g. USDC->NGN
  country TEXT NOT NULL,                   -- NG | GH | KE | RW
  amount_minor INTEGER NOT NULL,
  currency TEXT NOT NULL,
  state TEXT NOT NULL,                     -- pending | processing | completed | failed | disputed | cancelled
  attempt INTEGER NOT NULL DEFAULT 1,
  parent_payout_id TEXT REFERENCES payouts(id),
  stellar_tx_hash TEXT,
  memo TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  UNIQUE (operator_id, external_id, attempt)
);
CREATE INDEX idx_payouts_operator_state ON payouts (operator_id, state, updated_at);
CREATE INDEX idx_payouts_parent ON payouts (parent_payout_id);
CREATE INDEX idx_payouts_tx_hash ON payouts (stellar_tx_hash);
CREATE INDEX idx_payouts_memo ON payouts (operator_id, memo);

CREATE TABLE payout_events (
  id TEXT PRIMARY KEY,
  payout_id TEXT NOT NULL REFERENCES payouts(id),
  from_state TEXT,
  to_state TEXT NOT NULL,
  reason TEXT,
  occurred_at TEXT NOT NULL,
  received_at TEXT NOT NULL,
  idempotency_key TEXT NOT NULL UNIQUE     -- "<operator_id>:<key>"
);
CREATE INDEX idx_payout_events_payout ON payout_events (payout_id, occurred_at);

CREATE TABLE ledger_legs (
  tx_hash TEXT PRIMARY KEY,
  operator_id TEXT NOT NULL REFERENCES operators(id),
  from_account TEXT,
  to_account TEXT,
  asset TEXT,
  amount TEXT,
  memo TEXT,
  ledger INTEGER,
  closed_at TEXT,
  successful INTEGER NOT NULL,
  matched_payout_id TEXT REFERENCES payouts(id)
);
CREATE INDEX idx_ledger_legs_match ON ledger_legs (operator_id, memo, to_account);
CREATE INDEX idx_ledger_legs_payout ON ledger_legs (matched_payout_id);

CREATE TABLE incidents (
  id TEXT PRIMARY KEY,
  operator_id TEXT NOT NULL REFERENCES operators(id),
  kind TEXT NOT NULL,                      -- see spec 8.6
  severity TEXT NOT NULL,                  -- info | warning | critical
  corridor TEXT,
  payout_id TEXT,
  detail TEXT NOT NULL,
  opened_at TEXT NOT NULL,
  resolved_at TEXT
);
CREATE INDEX idx_incidents_open ON incidents (operator_id, resolved_at, opened_at);
CREATE INDEX idx_incidents_kind ON incidents (operator_id, kind, payout_id);

CREATE TABLE corridor_metrics (
  operator_id TEXT NOT NULL,
  corridor TEXT NOT NULL,
  bucket_start TEXT NOT NULL,
  total INTEGER,
  completed INTEGER,
  failed INTEGER,
  stuck INTEGER,
  p50_settle_secs INTEGER,
  p95_settle_secs INTEGER,
  health_score INTEGER,
  PRIMARY KEY (operator_id, corridor, bucket_start)
);

CREATE TABLE reports (
  id TEXT PRIMARY KEY,
  operator_id TEXT NOT NULL REFERENCES operators(id),
  week_start TEXT NOT NULL,
  body_json TEXT NOT NULL,
  sha256 TEXT NOT NULL,
  anchored_tx_hash TEXT,
  created_at TEXT NOT NULL,
  UNIQUE (operator_id, week_start)
);

-- Idempotency (ported from fluxapay IdempotencyRecord). Keyed per operator.
CREATE TABLE idempotency_records (
  scoped_key TEXT PRIMARY KEY,             -- "<operator_id>:<Idempotency-Key>"
  operator_id TEXT NOT NULL REFERENCES operators(id),
  request_hash TEXT NOT NULL,              -- sha256 of the canonical request
  response_code INTEGER NOT NULL,
  response_body TEXT NOT NULL,
  created_at TEXT NOT NULL
);
CREATE INDEX idx_idempotency_created ON idempotency_records (created_at);

-- Adapted from stellar-insights 014_create_api_keys + 032 (per-key limits).
-- Columns match the copied ApiKey model; wallet_address became operator_id.
-- Admin issues an operator's first key; the operator can rotate or revoke it.
CREATE TABLE api_keys (
  id TEXT PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  key_prefix TEXT NOT NULL,
  key_hash TEXT NOT NULL UNIQUE,           -- Argon2id PHC string
  operator_id TEXT NOT NULL REFERENCES operators(id),
  scopes TEXT NOT NULL DEFAULT 'write',
  status TEXT NOT NULL DEFAULT 'active',   -- active | revoked
  created_at TEXT NOT NULL DEFAULT (datetime('now')),
  last_used_at TEXT,
  expires_at TEXT,
  revoked_at TEXT
);
CREATE INDEX idx_api_keys_key_prefix ON api_keys (key_prefix, status);
CREATE INDEX idx_api_keys_operator ON api_keys (operator_id, status);

CREATE TABLE api_keys_rate_limit_config (
  api_key_id TEXT PRIMARY KEY NOT NULL REFERENCES api_keys(id) ON DELETE CASCADE,
  limit_per_minute INTEGER NOT NULL DEFAULT 60,
  created_at TEXT NOT NULL DEFAULT (datetime('now')),
  updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Adapted from stellar-insights 016_create_telegram_subscriptions.
CREATE TABLE telegram_subscriptions (
  id TEXT PRIMARY KEY,
  operator_id TEXT NOT NULL REFERENCES operators(id),
  chat_id INTEGER NOT NULL,
  chat_type TEXT NOT NULL DEFAULT 'private',
  chat_title TEXT,
  username TEXT,
  subscribed_at TEXT NOT NULL,
  is_active INTEGER NOT NULL DEFAULT 1,
  alert_types TEXT NOT NULL DEFAULT 'all',
  last_alert_sent_at TEXT,
  UNIQUE (operator_id, chat_id)
);

-- Adapted from stellar-insights 027_create_alert_rules.
CREATE TABLE alert_rules (
  id TEXT PRIMARY KEY,
  operator_id TEXT NOT NULL REFERENCES operators(id) ON DELETE CASCADE,
  corridor TEXT,
  alert_kind TEXT NOT NULL,
  notify_telegram INTEGER NOT NULL DEFAULT 1,
  notify_slack INTEGER NOT NULL DEFAULT 1,
  notify_webhook INTEGER NOT NULL DEFAULT 1,
  is_active INTEGER NOT NULL DEFAULT 1,
  snoozed_until TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
CREATE INDEX idx_alert_rules_operator ON alert_rules (operator_id, alert_kind);

-- Adapted from stellar-insights 028_create_alert_history: one row per delivery.
CREATE TABLE alert_history (
  id TEXT PRIMARY KEY,
  operator_id TEXT NOT NULL REFERENCES operators(id) ON DELETE CASCADE,
  incident_id TEXT REFERENCES incidents(id),
  channel TEXT NOT NULL,                   -- telegram | slack | webhook
  alert_kind TEXT NOT NULL,
  severity TEXT NOT NULL,
  message TEXT NOT NULL,
  delivered INTEGER NOT NULL DEFAULT 0,
  error TEXT,
  triggered_at TEXT NOT NULL
);
CREATE INDEX idx_alert_history_operator ON alert_history (operator_id, triggered_at DESC);
CREATE UNIQUE INDEX idx_alert_history_once ON alert_history (incident_id, channel) WHERE incident_id IS NOT NULL;

-- Adapted from stellar-insights 041_create_corridor_alert_configs.
CREATE TABLE corridor_alert_configs (
  id TEXT PRIMARY KEY,
  operator_id TEXT NOT NULL REFERENCES operators(id) ON DELETE CASCADE,
  corridor TEXT,                           -- NULL = all corridors
  degraded_below INTEGER NOT NULL DEFAULT 85,
  down_below INTEGER NOT NULL DEFAULT 60,
  failure_rate_threshold REAL NOT NULL DEFAULT 0.20,
  cooldown_seconds INTEGER NOT NULL DEFAULT 300,
  is_active INTEGER NOT NULL DEFAULT 1,
  last_triggered_at TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
CREATE INDEX idx_corridor_alert_configs_operator ON corridor_alert_configs (operator_id, corridor);

-- Adapted from stellar-insights 042_create_corridor_alert_events: corridor status
-- changes (healthy -> degraded -> down -> recovered).
CREATE TABLE corridor_alert_events (
  id TEXT PRIMARY KEY,
  operator_id TEXT NOT NULL REFERENCES operators(id) ON DELETE CASCADE,
  corridor TEXT NOT NULL,
  alert_kind TEXT NOT NULL,                -- corridor_degraded | corridor_down | corridor_recovered
  severity TEXT NOT NULL DEFAULT 'warning',
  message TEXT NOT NULL,
  old_value REAL,
  new_value REAL,
  threshold_value REAL,
  triggered_at TEXT NOT NULL
);
CREATE INDEX idx_corridor_alert_events_operator ON corridor_alert_events (operator_id, corridor, triggered_at DESC);
