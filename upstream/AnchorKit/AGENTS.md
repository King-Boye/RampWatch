#897 transport.rs tests construct HealthStatus with a nonexistent last_check field
Repo Avatar
Haroldwonder/AnchorKit
Problem
Three tests in src/transport.rs construct HealthStatus struct literals that include a last_check: 1000 field. The actual HealthStatus definition in src/types.rs only has four fields — anchor, latency_ms, failure_count, availability_percent — and has no last_check field. This produces a compile error (E0560: struct has no field named last_check) in the transport test module.

Location
src/transport.rs:330, 362, 421 — HealthStatus { ..., last_check: 1000 }
src/types.rs:477-484 — actual struct definition, no last_check field
Why it matters
Combined with the QuoteData import error, transport.rs's test module cannot compile at all. This strongly suggests HealthStatus was refactored (a last_check field removed) without updating the consumers in transport.rs, and no CI gate caught the drift.

Suggested fix
Either remove last_check: 1000 from the three test literals to match the current HealthStatus shape, or reintroduce a last_check: u64 field on HealthStatus in types.rs if health-check recency tracking is still an intended feature (the field name suggests it was deliberately designed in, then dropped from the struct but not from call sites).

#898 rate_limiter.rs defines get_rate_limit_status twice in the same impl block — compile error
Repo Avatar
Haroldwonder/AnchorKit
Problem
impl RateLimiter in src/rate_limiter.rs defines the method get_rate_limit_status(env: Env, attestor: Address) -> RateLimitStatus twice, with identical signatures and near-identical bodies. Rust does not allow duplicate inherent method definitions with the same name in one impl block; this is a hard compile error (E0592: duplicate definitions with name "get_rate_limit_status").

Location
src/rate_limiter.rs:175-190 (first definition)
src/rate_limiter.rs:206-225 (second, near-identical definition)
Why it matters
This breaks compilation of the whole crate (rate_limiter is mod rate_limiter; in lib.rs, unconditionally included). It looks like a merge artifact — likely two contributor PRs both added the same accessor independently and the conflict wasn't resolved before merging (consistent with the repo's most recent commit being "post-merge cleanup after contributor PRs").

Suggested fix
Delete one of the two duplicate definitions (they are functionally identical). Keep the second, slightly more explicit version (line 206) and remove the first (line 175), or vice versa — then re-run the full test suite to confirm no callers depended on subtle differences between the two copies.


#900 RetryConfig::delay_for_attempt can panic via overflow / division-by-zero when the capped delay reaches u64::MAX
Repo Avatar
Haroldwonder/AnchorKit
Problem
delay_for_attempt computes jitter_seed % (capped + 1) where capped = raw.min(self.max_delay_ms). If capped equals u64::MAX, then capped + 1 overflows: in debug builds this panics with "attempt to add with overflow"; in release builds (where arithmetic overflow checks are typically off) it wraps to 0, and the subsequent jitter_seed % 0 panics unconditionally with "attempt to calculate the remainder with a divisor of zero".

This is reachable with a legitimately constructible config: RetryConfig::new performs no upper-bound validation on base_delay_ms or max_delay_ms (it only asserts max_attempts >= 1). A caller who sets max_delay_ms: u64::MAX — a natural way to express "uncap the delay", mirroring the exact u64::MAX-as-sentinel convention already used for budget_ms in this same struct (see Default impl, line 28) — combined with any base_delay_ms/backoff_multiplier large enough that raw also saturates to u64::MAX, will panic on the very first retry.

Location
src/retry.rs:82-88 (delay_for_attempt, specifically line 87: jitter_seed % (capped + 1))
src/retry.rs:33-49 (RetryConfig::new — no bounds validation on base_delay_ms/max_delay_ms)
Why it matters
A config value that looks reasonable by analogy to budget_ms: u64::MAX ("disable the cap") crashes the retry loop instead of behaving as "no cap", turning a config mistake into a panic/DoS in whatever process embeds this SDK.

Suggested fix
Use capped.checked_add(1).map(|m| jitter_seed % m).unwrap_or(jitter_seed) (or clamp max_delay_ms to u64::MAX - 1 in new/Default), and add a regression test with max_delay_ms: u64::MAX.

#888 get_fee_estimate doesn't bound-check fee_percent_bps from anchor-supplied AnchorFeeData
Repo Avatar
Haroldwonder/AnchorKit
Problem
pub fn get_fee_estimate(asset_code: &str, amount: u64, operation: FeeOperation, fee_data: &AnchorFeeData) -> Result<FeeEstimate, Error> {
    if !is_valid_asset_code(asset_code) { ... }
    if amount == 0 { return Err(Error::invalid_amount()); }
    let _ = operation;
    let percent_fee = (amount as u128 * fee_data.fee_percent_bps as u128 / 10_000) as u64;
    let total_fee = fee_data.fee_fixed.saturating_add(percent_fee);
    Ok(FeeEstimate { total_fee, fee_fixed: fee_data.fee_fixed, fee_percent_bps: fee_data.fee_percent_bps })
}
Location
src/sep6.rs:634-650

Why it matters
fee_percent_bps is documented as basis points ("e.g. 150 = 1.50%", src/sep6.rs:618-619), implying a sane range of 0..=10_000 (0–100%). Nothing here rejects values above 10_000. Every other numeric/string field this file normalizes from anchor responses is validated somewhere in this module (asset codes, amounts, Stellar addresses) — this is the one piece of anchor-controlled fee data that flows straight through unchecked. A misconfigured or malicious anchor reporting fee_percent_bps = 50_000 (500%) produces a total_fee larger than amount, which downstream code computing amount - total_fee (a very natural next step for a caller building a net-payout figure) would either panic (unsigned underflow in debug builds) or wrap silently (in release), depending on how the caller does the subtraction — a bug entirely traceable back to this unchecked field.

Suggested fix
Validate fee_data.fee_percent_bps <= 10_000 and return Err(Error::with_context(ErrorCode::ValidationError, ...)) otherwise, consistent with how the other fields in this file are treated as untrusted anchor input.