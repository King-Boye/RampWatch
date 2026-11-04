//! Rows and request bodies.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::state::PayoutState;

/// Countries RampWatch watches at launch.
pub const COUNTRIES: [&str; 4] = ["NG", "GH", "KE", "RW"];

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Operator {
    pub id: String,
    pub name: String,
    /// JSON array of distribution accounts.
    pub stellar_accounts: String,
    pub hmac_secret: String,
    pub hmac_secret_previous: Option<String>,
    pub hmac_secret_previous_expires_at: Option<String>,
    pub telegram_chat_id: Option<String>,
    pub stuck_after_secs: i64,
    pub created_at: String,
}

impl Operator {
    #[must_use]
    pub fn accounts(&self) -> Vec<String> {
        serde_json::from_str(&self.stellar_accounts).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Payout {
    pub id: String,
    pub operator_id: String,
    pub external_id: String,
    pub corridor: String,
    pub country: String,
    pub amount_minor: i64,
    pub currency: String,
    pub state: String,
    pub attempt: i64,
    pub parent_payout_id: Option<String>,
    pub stellar_tx_hash: Option<String>,
    pub memo: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// One cash-out state change, as the operator sends it (spec section 10).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PayoutEventInput {
    pub external_id: String,
    #[serde(default = "default_attempt")]
    pub attempt: i64,
    #[serde(default)]
    pub parent_external_id: Option<String>,
    pub state: PayoutState,
    #[serde(default)]
    pub reason: Option<String>,
    pub corridor: String,
    pub country: String,
    pub amount_minor: i64,
    pub currency: String,
    #[serde(default)]
    pub stellar_tx_hash: Option<String>,
    #[serde(default)]
    pub memo: Option<String>,
    pub occurred_at: DateTime<Utc>,
    /// Per-event dedupe key for batch and CSV intake. The single-event route
    /// takes it from the `Idempotency-Key` header instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
}

const fn default_attempt() -> i64 {
    1
}

fn non_empty(value: &str, field: &str, max: usize, errors: &mut Vec<String>) {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        errors.push(format!("{field} is required"));
    } else if trimmed.len() > max {
        errors.push(format!("{field} must be at most {max} characters"));
    }
}

impl PayoutEventInput {
    /// Field checks. Returns every problem at once.
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        non_empty(&self.external_id, "external_id", 128, &mut errors);
        non_empty(&self.corridor, "corridor", 32, &mut errors);
        non_empty(&self.currency, "currency", 12, &mut errors);
        if !COUNTRIES.contains(&self.country.as_str()) {
            errors.push(format!("country must be one of {}", COUNTRIES.join(", ")));
        }
        if self.attempt < 1 {
            errors.push("attempt must be 1 or more".to_string());
        }
        if self.amount_minor <= 0 {
            errors.push("amount_minor must be greater than 0".to_string());
        }
        if let Some(parent) = &self.parent_external_id {
            non_empty(parent, "parent_external_id", 128, &mut errors);
        }
        if let Some(hash) = &self.stellar_tx_hash {
            if hash.len() != 64 || !hash.bytes().all(|b| b.is_ascii_hexdigit()) {
                errors.push("stellar_tx_hash must be 64 hex characters".to_string());
            }
        }
        if self.memo.as_ref().is_some_and(|m| m.len() > 64) {
            errors.push("memo must be at most 64 characters".to_string());
        }
        if self.reason.as_ref().is_some_and(|r| r.len() > 500) {
            errors.push("reason must be at most 500 characters".to_string());
        }
        if self.occurred_at > Utc::now() + chrono::Duration::minutes(5) {
            errors.push("occurred_at is in the future".to_string());
        }
        if let Some(key) = &self.idempotency_key {
            non_empty(key, "idempotency_key", 255, &mut errors);
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

/// Is this a valid Stellar account id (G..., 56 chars, base32)?
#[must_use]
pub fn is_stellar_account(s: &str) -> bool {
    s.len() == 56
        && s.starts_with('G')
        && s.bytes().all(|b| b.is_ascii_uppercase() || (b'2'..=b'7').contains(&b))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn event() -> PayoutEventInput {
        serde_json::from_value(serde_json::json!({
            "external_id": "co_8f2a",
            "attempt": 1,
            "parent_external_id": null,
            "state": "processing",
            "reason": null,
            "corridor": "USDC->NGN",
            "country": "NG",
            "amount_minor": 2_500_000,
            "currency": "NGN",
            "stellar_tx_hash": null,
            "memo": "co_8f2a",
            "occurred_at": "2026-11-04T19:40:12Z"
        }))
        .unwrap()
    }

    #[test]
    fn spec_example_parses_and_validates() {
        let e = event();
        assert_eq!(e.state, PayoutState::Processing);
        assert!(e.validate().is_ok());
    }

    #[test]
    fn collects_every_error() {
        let mut e = event();
        e.country = "US".into();
        e.amount_minor = 0;
        e.stellar_tx_hash = Some("abc".into());
        assert_eq!(e.validate().unwrap_err().len(), 3);
    }

    #[test]
    fn rejects_unknown_fields() {
        let bad = serde_json::json!({"external_id": "x", "state": "pending", "nope": 1});
        assert!(serde_json::from_value::<PayoutEventInput>(bad).is_err());
    }

    #[test]
    fn stellar_account_check() {
        assert!(is_stellar_account("GBRPYHIL2CI3FNQ4BXLFMNDLFJUNPU2HY3ZMFSHONUCEOASW7QC7OX2H"));
        assert!(!is_stellar_account("SBRPYHIL2CI3FNQ4BXLFMNDLFJUNPU2HY3ZMFSHONUCEOASW7QC7OX2H"));
        assert!(!is_stellar_account("GABC"));
    }
}
