//! Request signing for operator intake, and for RampWatch's own outbound
//! webhooks.
//!
//! Ported from fluxapay `services/webhook.service.ts`
//! (`generateWebhookSignature`, `verifyWebhookTimestamp`, secondary-signature
//! rotation) and `helpers/crypto.helper.ts` (`timingSafeStringEqual`), with the
//! middleware shape of stellar-insights `request_signing_middleware.rs`.
//!
//! Scheme:
//! - `X-RampWatch-Timestamp`: unix seconds or an RFC 3339 time
//! - `X-RampWatch-Signature`: hex `hmac_sha256(secret, timestamp + "." + body)`
//! - `X-RampWatch-Signature-Secondary` (optional): the same, signed with the
//!   other secret during a rotation grace window
//!
//! A request passes if any presented signature matches any secret currently
//! valid for the operator (the active secret, and the previous one until its
//! grace window ends). That covers both an operator who has switched to the
//! new secret and one who hasn't yet.

use axum::{
    body::{to_bytes, Body},
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use chrono::{DateTime, Utc};
use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;
use std::sync::Arc;

use crate::domain::models::Operator;
use crate::error::ApiError;

pub const SIGNATURE_HEADER: &str = "X-RampWatch-Signature";
pub const SECONDARY_SIGNATURE_HEADER: &str = "X-RampWatch-Signature-Secondary";
pub const TIMESTAMP_HEADER: &str = "X-RampWatch-Timestamp";

/// Clock skew allowed for timestamps slightly in the future. fluxapay rejects
/// any future timestamp; a few seconds of drift between hosts is normal.
const MAX_FUTURE_SKEW_SECS: i64 = 30;

/// Largest signed body accepted (a CSV import can be a few MB).
pub const MAX_SIGNED_BODY_BYTES: usize = 10 * 1024 * 1024;

type HmacSha256 = Hmac<Sha256>;

/// hex `hmac_sha256(secret, "{timestamp}.{body}")`
#[must_use]
pub fn sign(secret: &str, timestamp: &str, body: &[u8]) -> String {
    #[allow(clippy::expect_used)] // HMAC accepts keys of any length
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC takes any key size");
    mac.update(timestamp.as_bytes());
    mac.update(b".");
    mac.update(body);
    hex::encode(mac.finalize().into_bytes())
}

/// Constant-time byte comparison. Unequal lengths return false without
/// comparing (the length of a hex signature is public).
#[must_use]
pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

/// Parse a timestamp header: unix seconds, or RFC 3339 (fluxapay sends ISO 8601).
fn parse_timestamp(raw: &str) -> Option<DateTime<Utc>> {
    let raw = raw.trim();
    if let Ok(secs) = raw.parse::<i64>() {
        return DateTime::from_timestamp(secs, 0);
    }
    DateTime::parse_from_rfc3339(raw)
        .ok()
        .map(|t| t.with_timezone(&Utc))
}

/// fluxapay `verifyWebhookTimestamp`: reject anything older than the
/// tolerance, or further in the future than a small clock skew.
#[must_use]
pub fn timestamp_is_fresh(raw: &str, tolerance_secs: i64, now: DateTime<Utc>) -> bool {
    let Some(ts) = parse_timestamp(raw) else {
        return false;
    };
    let age = now.signed_duration_since(ts).num_seconds();
    age <= tolerance_secs && age >= -MAX_FUTURE_SKEW_SECS
}

/// Check the presented signatures against every currently valid secret.
#[must_use]
pub fn verify(
    secrets: &[&str],
    timestamp: &str,
    body: &[u8],
    presented: &[&str],
) -> bool {
    let mut ok = false;
    // Check every pair, so timing doesn't reveal which secret matched.
    for secret in secrets {
        let expected = sign(secret, timestamp, body);
        for sig in presented {
            ok |= constant_time_eq(expected.as_bytes(), sig.trim().to_ascii_lowercase().as_bytes());
        }
    }
    ok
}

/// Secrets currently valid for an operator.
#[must_use]
pub fn valid_secrets(operator: &Operator, now: DateTime<Utc>) -> Vec<&str> {
    let mut secrets = vec![operator.hmac_secret.as_str()];
    if let (Some(previous), Some(expires)) = (
        operator.hmac_secret_previous.as_deref(),
        operator.hmac_secret_previous_expires_at.as_deref(),
    ) {
        let still_valid = DateTime::parse_from_rfc3339(expires)
            .map(|t| t.with_timezone(&Utc) > now)
            .unwrap_or(false);
        if still_valid {
            secrets.push(previous);
        }
    }
    secrets
}

/// Verify the HMAC signature of an intake request. Runs after
/// `operator_auth_middleware`, which attaches the operator.
pub async fn hmac_middleware(
    State(tolerance_secs): State<i64>,
    req: Request,
    next: Next,
) -> Result<Response, ApiError> {
    let invalid = || ApiError::unauthorized("invalid_signature", "Missing or invalid request signature");

    let operator = req
        .extensions()
        .get::<Arc<Operator>>()
        .cloned()
        .ok_or_else(invalid)?;

    // Copy the headers out so no borrow of the request lives across an await.
    let (timestamp, primary, secondary) = {
        let header = |name: &str| {
            req.headers()
                .get(name)
                .and_then(|v| v.to_str().ok())
                .map(str::to_string)
        };
        (
            header(TIMESTAMP_HEADER),
            header(SIGNATURE_HEADER),
            header(SECONDARY_SIGNATURE_HEADER),
        )
    };
    let timestamp = timestamp.ok_or_else(invalid)?;
    if primary.is_none() && secondary.is_none() {
        return Err(invalid());
    }

    let now = Utc::now();
    if !timestamp_is_fresh(&timestamp, tolerance_secs, now) {
        return Err(ApiError::unauthorized(
            "stale_timestamp",
            "X-RampWatch-Timestamp is missing, malformed or outside the allowed window",
        ));
    }

    let (parts, body) = req.into_parts();
    let bytes = to_bytes(body, MAX_SIGNED_BODY_BYTES)
        .await
        .map_err(|_| ApiError::bad_request("body_too_large", "Request body is too large"))?;

    let presented: Vec<&str> = [primary.as_deref(), secondary.as_deref()]
        .into_iter()
        .flatten()
        .collect();
    if !verify(&valid_secrets(&operator, now), &timestamp, &bytes, &presented) {
        return Err(invalid());
    }

    Ok(next.run(Request::from_parts(parts, Body::from(bytes))).await)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signature_matches_fluxapay_scheme() {
        // node -e 'console.log(require("crypto").createHmac("sha256","whsec_test")
        //   .update("1730749212." + "{\"a\":1}").digest("hex"))'
        let sig = sign("whsec_test", "1730749212", br#"{"a":1}"#);
        assert_eq!(sig.len(), 64);
        assert_eq!(sig, sign("whsec_test", "1730749212", br#"{"a":1}"#));
        assert_ne!(sig, sign("whsec_other", "1730749212", br#"{"a":1}"#));
        assert_ne!(sig, sign("whsec_test", "1730749213", br#"{"a":1}"#));
    }

    #[test]
    fn verify_accepts_either_secret_during_rotation() {
        let body = b"{}";
        let old = sign("old", "1", body);
        let new = sign("new", "1", body);
        assert!(verify(&["new", "old"], "1", body, &[&old]));
        assert!(verify(&["new", "old"], "1", body, &[&new]));
        assert!(verify(&["new", "old"], "1", body, &["bad", &old]));
        assert!(!verify(&["new"], "1", body, &[&old]));
        assert!(!verify(&["new"], "1", b"{ }", &[&new]));
    }

    #[test]
    fn timestamp_window() {
        let now = Utc::now();
        let ts = |offset: i64| (now.timestamp() + offset).to_string();
        assert!(timestamp_is_fresh(&ts(0), 300, now));
        assert!(timestamp_is_fresh(&ts(-299), 300, now));
        assert!(!timestamp_is_fresh(&ts(-301), 300, now));
        assert!(timestamp_is_fresh(&ts(10), 300, now));
        assert!(!timestamp_is_fresh(&ts(120), 300, now));
        assert!(timestamp_is_fresh(&now.to_rfc3339(), 300, now));
        assert!(!timestamp_is_fresh("yesterday", 300, now));
    }

    #[test]
    fn constant_time_eq_basics() {
        assert!(constant_time_eq(b"abc", b"abc"));
        assert!(!constant_time_eq(b"abc", b"abd"));
        assert!(!constant_time_eq(b"abc", b"abcd"));
    }
}

