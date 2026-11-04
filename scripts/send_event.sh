#!/usr/bin/env bash
# Send one signed cash-out event to RampWatch.
#
#   RAMPWATCH_API_KEY=rw_live_... RAMPWATCH_HMAC_SECRET=whsec_... \
#     scripts/send_event.sh [state] [external_id]
#
# To see a replay, send the same event twice with the same key and time:
#   export IDEMPOTENCY_KEY=demo-1 OCCURRED_AT=2026-11-04T19:40:12Z
#   scripts/send_event.sh; scripts/send_event.sh   # 2nd has X-Idempotent-Replayed: true
# Env: RAMPWATCH_URL (default http://localhost:8080), IDEMPOTENCY_KEY, OCCURRED_AT.
set -euo pipefail

: "${RAMPWATCH_API_KEY:?set RAMPWATCH_API_KEY (from POST /v1/admin/operators)}"
: "${RAMPWATCH_HMAC_SECRET:?set RAMPWATCH_HMAC_SECRET (from POST /v1/admin/operators)}"
URL="${RAMPWATCH_URL:-http://localhost:8080}"
STATE="${1:-processing}"
EXTERNAL_ID="${2:-co_8f2a}"
KEY="${IDEMPOTENCY_KEY:-$EXTERNAL_ID-$STATE}"
NOW="${OCCURRED_AT:-$(date -u +%Y-%m-%dT%H:%M:%SZ)}"

BODY=$(cat <<JSON
{"external_id":"$EXTERNAL_ID","attempt":1,"parent_external_id":null,"state":"$STATE","reason":null,"corridor":"USDC->NGN","country":"NG","amount_minor":2500000,"currency":"NGN","stellar_tx_hash":null,"memo":"$EXTERNAL_ID","occurred_at":"$NOW"}
JSON
)

TS="$(date -u +%s)"
# hex hmac_sha256(secret, timestamp + "." + body)
SIG="$(printf '%s.%s' "$TS" "$BODY" | openssl dgst -sha256 -hmac "$RAMPWATCH_HMAC_SECRET" -hex | sed 's/^.*= //')"

curl -sS -i -X POST "$URL/v1/payouts/events" \
  -H "Authorization: Bearer $RAMPWATCH_API_KEY" \
  -H "Content-Type: application/json" \
  -H "Idempotency-Key: $KEY" \
  -H "X-RampWatch-Timestamp: $TS" \
  -H "X-RampWatch-Signature: $SIG" \
  --data "$BODY"
echo
