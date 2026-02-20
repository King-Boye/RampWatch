# SwiftRemit Event Audit — Issue #33

## Audit Table

| Function | Event Topics | Data Payload | Issues |
|---|---|---|---|
| emit_remittance_created | `("created",)` | `(remittance_id: u64, sender: Address, agent: Address, amount: i128, fee: i128)` | No schema version; no timestamp; no namespace topic; no token/asset field; actor named `sender` inconsistent with `to` elsewhere |
| emit_remittance_completed | `("completed",)` | `(remittance_id: u64, agent: Address, payout_amount: i128)` | No schema version; no timestamp; `sender` absent — can't link to originator without external lookup; no token field; `payout_amount` naming inconsistent with `amount` |
| emit_remittance_cancelled | `("cancelled",)` | `(remittance_id: u64, sender: Address, refund_amount: i128)` | No schema version; no timestamp; `agent` absent; no token field; `refund_amount` is a third distinct field name alongside `amount` and `payout_amount` |
| emit_agent_registered | `("agent_reg",)` | `agent: Address` (bare scalar) | No schema version; no timestamp; abbreviated topic inconsistent with full-word topics elsewhere; bare scalar instead of tuple; no `registered_by` admin field |
| emit_agent_removed | `("agent_rem",)` | `agent: Address` (bare scalar) | No schema version; no timestamp; abbreviated topic; bare scalar instead of tuple; no `removed_by` or reason/status field |
| emit_fee_updated | `("fee_upd",)` | `fee_bps: u32` (bare scalar) | No schema version; no timestamp; no `previous_fee_bps` — indexers must store prior state themselves to compute delta; no `updated_by` field; bare scalar |
| emit_fees_withdrawn | `("fees_with",)` | `(to: Address, amount: i128)` | No schema version; no timestamp; recipient named `to` — inconsistent with `sender`/`agent` elsewhere; no token field; no `withdrawn_by` admin field |

## Cross-Cutting Issues

- No event carries a schema version — if the payload shape changes, historical indexers break silently
- No event includes a timestamp or ledger sequence — both available via `env.ledger().timestamp()` and `env.ledger().sequence()`
- All events use a single-element topic tuple — the standard Soroban pattern is `(namespace, action)` e.g. `("remittance", "created")` so indexers can filter by category
- No event records which token/asset is involved — indexers must query contract state separately
- Actor field naming is inconsistent: `sender`, `agent`, `to` are all used with no convention
- Three different amount field names: `amount`, `payout_amount`, `refund_amount` — no shared naming schema
- Some payloads are bare scalars, others are tuples — breaks any generic deserializer
- Topic abbreviations (`agent_reg`, `agent_rem`, `fee_upd`, `fees_with`) are cryptic and inconsistently styled vs full-word topics like `created`, `completed`, `cancelled`
