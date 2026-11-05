<p align="center">
  <img src="docs/assets/hero.svg" alt="RampWatch: health monitoring for off-ramp and payout rails on Stellar" width="100%">
</p>

<p align="center">
  <a href="https://github.com/King-Boye/RampWatch/actions/workflows/ci.yml"><img src="https://github.com/King-Boye/RampWatch/actions/workflows/ci.yml/badge.svg" alt="CI"></a>
  <a href="https://github.com/King-Boye/RampWatch/actions/workflows/security.yml"><img src="https://github.com/King-Boye/RampWatch/actions/workflows/security.yml/badge.svg" alt="Security"></a>
  <a href="https://github.com/King-Boye/RampWatch/actions/workflows/codeql.yml"><img src="https://github.com/King-Boye/RampWatch/actions/workflows/codeql.yml/badge.svg" alt="CodeQL"></a>
  <img src="https://img.shields.io/badge/rust-stable-orange?logo=rust" alt="Rust stable">
  <img src="https://img.shields.io/badge/Stellar-Soroban-7B61FF" alt="Stellar Soroban">
</p>

RampWatch watches the health of teams that run their own off-ramp or payout rails on Stellar: wallets with their own liquidity, remittance apps and anchors. The operator sends every cash-out state change. RampWatch checks it against the payout state machine and the Stellar ledger, and tells the operator:

- whether each corridor (NG, GH, KE, RW to start) is **up, degraded or down**
- which cash-outs are **stuck**, and for how long
- which resends were safe, and which may have **paid the user twice**
- a **0 to 100 health score** per corridor, plus a weekly report whose hash is **anchored on Stellar**

## How it works

<p align="center">
  <img src="docs/assets/flow.svg" alt="Pipeline: Operator and Stellar feed Intake and the Ledger watcher, which write to the Store; Detectors read the Store and drive Alerts, the Weekly report and Anchoring" width="100%">
</p>

```mermaid
flowchart LR
    subgraph OP["Operator"]
        ENG["Off-ramp engine"]
    end
    subgraph NET["Stellar network"]
        HZ["Horizon / Soroban RPC"]
    end

    ENG -- "webhook or CSV" --> IN["Intake<br/>API key · HMAC · Idempotency-Key"]
    HZ -- "payments to operator accounts" --> LW["Ledger watcher<br/>one poller · match by memo / tx hash"]
    IN --> ST[("Store<br/>SQLite via sqlx")]
    LW --> ST
    ST --> DET["Detectors<br/>stuck · retry / double-pay · corridor<br/>anomaly · health score · reserve"]
    DET --> AL["Alerts<br/>Telegram · Slack · webhooks"]
    DET --> RP["Weekly report<br/>canonical JSON"]
    RP --> AN["Anchoring<br/>sha256 → Soroban registry"]

    classDef built fill:#0f766e,stroke:#2dd4bf,color:#fff
    classDef planned fill:#1e293b,stroke:#64748b,color:#e2e8f0,stroke-dasharray: 5 5
    class IN,ST built
    class LW,DET,AL,RP,AN planned
```

Solid boxes are built; dashed boxes arrive in milestones 2 to 4.

## Payout state machine

Every event is checked against this machine, ported from SwiftRemit. An illegal move is rejected with `409 invalid_transition` and opens an `illegal_transition` incident, because it says something about the operator's engine.

```mermaid
stateDiagram-v2
    direction LR
    [*] --> pending
    [*] --> processing
    pending --> processing
    pending --> cancelled
    processing --> completed
    processing --> failed
    processing --> cancelled : refunded
    failed --> disputed
    failed --> cancelled
    disputed --> completed
    disputed --> cancelled
    completed --> [*]
    cancelled --> [*]

    note right of failed
        A resend is a new payout row
        with attempt + 1 and a link
        to its parent, never a reset.
    end note
```

## What happens to one event

```mermaid
sequenceDiagram
    autonumber
    participant Op as Operator engine
    participant API as RampWatch API
    participant DB as SQLite

    Op->>API: POST /v1/payouts/events<br/>Bearer key · X-RampWatch-Signature · Idempotency-Key
    API->>API: Check API key (Argon2id, cached 60s)
    API->>API: Check HMAC and timestamp (within 300s)
    opt bad key, signature or timestamp
        API-->>Op: 401
    end
    API->>DB: BEGIN IMMEDIATE · look up Idempotency-Key
    alt key seen with the same body
        DB-->>API: stored response
        API-->>Op: same response + X-Idempotent-Replayed: true
    else key seen with a different body
        API-->>Op: 409 idempotency_conflict
    else new key
        API->>API: check the state machine
        alt illegal move
            API->>DB: open illegal_transition incident
            API-->>Op: 409 invalid_transition
        else allowed
            API->>DB: write payout + event · store response
            API-->>Op: 201 created / 200 updated
        end
    end
```

## Corridor health

<p align="center">
  <img src="docs/assets/corridors.svg" alt="Example corridor health board: NG 92 healthy, GH 87 healthy, KE 71 degraded, RW 54 down" width="100%">
</p>

Each corridor gets a score from 0 to 100, ported from AnchorKit's anchor health score with reputation replaced by measured reliability:

```mermaid
pie showData title Health score weights
    "Uptime (5-min buckets with a completion and < 20% failures)" : 40
    "Success rate (completed / completed + failed, 7 days)" : 35
    "Speed (p50 settle time: ≤5 min 100 → over 1 h 20)" : 25
```

| Score | Status |
| --- | --- |
| 85 to 100 | 🟢 healthy |
| 60 to 84 | 🟡 degraded |
| below 60 | 🔴 down |

## Resends and double pays

The detectors (milestone 3) look at every resend chain:

```mermaid
flowchart TD
    R["Resend arrives<br/>attempt > 1"] --> P{"Parent failed?"}
    P -- no --> NF["parent_not_failed<br/>flagged in the intake response"]
    P -- yes --> L{"Parent has a successful<br/>ledger leg?"}
    L -- yes --> DP["🔴 possible_double_pay<br/>critical, both tx hashes"]
    L -- no --> T{"Resent within 10 min<br/>before the ledger was checked?"}
    T -- yes --> UR["🟡 unverified_retry<br/>warning"]
    T -- no --> OK["🟢 safe resend"]

    classDef bad fill:#7f1d1d,stroke:#f87171,color:#fff
    classDef warn fill:#78350f,stroke:#fbbf24,color:#fff
    classDef good fill:#064e3b,stroke:#34d399,color:#fff
    class DP bad
    class UR,NF warn
    class OK good
```

## Data model

```mermaid
erDiagram
    OPERATORS ||--o{ PAYOUTS : sends
    OPERATORS ||--o{ API_KEYS : "authenticates with"
    OPERATORS ||--o{ INCIDENTS : has
    OPERATORS ||--o{ REPORTS : "gets weekly"
    PAYOUTS ||--o{ PAYOUT_EVENTS : "moves through"
    PAYOUTS |o--o{ PAYOUTS : "resend of"
    PAYOUTS |o--o{ LEDGER_LEGS : "matched to"

    OPERATORS {
        text id PK
        text name
        text stellar_accounts "JSON array"
        int stuck_after_secs
    }
    PAYOUTS {
        text id PK
        text external_id "operator's cash-out id"
        text corridor "e.g. USDC->NGN"
        text state
        int attempt
        text parent_payout_id FK
        text stellar_tx_hash
    }
    PAYOUT_EVENTS {
        text id PK
        text from_state
        text to_state
        text idempotency_key UK
    }
    LEDGER_LEGS {
        text tx_hash PK
        text memo
        text amount
        int successful
    }
    INCIDENTS {
        text id PK
        text kind
        text severity
        text resolved_at
    }
    REPORTS {
        text id PK
        text week_start
        text sha256
        text anchored_tx_hash
    }
```

## Status

```mermaid
flowchart LR
    M1["✅ M1<br/>Skeleton and intake"] --> M2["🔜 M2<br/>Ledger matching"] --> M3["M3<br/>Detectors and alerts"] --> M4["M4<br/>Reports and registry contract"] --> M5["M5<br/>Learned baselines<br/>after pilot start"]

    classDef done fill:#0f766e,stroke:#2dd4bf,color:#fff
    classDef next fill:#1e3a8a,stroke:#60a5fa,color:#fff
    classDef todo fill:#1e293b,stroke:#64748b,color:#e2e8f0,stroke-dasharray: 5 5
    class M1 done
    class M2 next
    class M3,M4,M5 todo
```

Workspace:

- `backend/`: Rust / Axum API (SQLite via sqlx)
- `contracts/registry/`: Soroban report registry, crate `rampwatch-registry` (milestone 4)

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

## Test

```bash
cargo build && cargo test
```

## Checks before every merge

```mermaid
flowchart LR
    PR["Pull request<br/>or Dependabot update"] --> CI["CI<br/>format · clippy · build · test"]
    PR --> SEC["Security<br/>cargo-deny · OSV · ClamAV<br/>gitleaks · dependency review"]
    PR --> CQ["CodeQL<br/>Rust + Actions"]
    CI --> G{"All 10 required<br/>checks green?"}
    SEC --> G
    CQ --> G
    G -- yes --> M["Merge to main"]
    G -- no --> B["Blocked until fixed"]
    D["Dependabot<br/>minor / patch"] -. "auto-merge" .-> G

    classDef ok fill:#064e3b,stroke:#34d399,color:#fff
    classDef stop fill:#7f1d1d,stroke:#f87171,color:#fff
    class M ok
    class B stop
```

Dependency, malware and CodeQL scans also rerun every Monday. Details in [SECURITY.md](SECURITY.md).

