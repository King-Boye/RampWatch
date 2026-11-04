//! Idempotency store. Ported from fluxapay
//! `middleware/idempotency.middleware.ts` (`idempotencyMiddleware`,
//! `storeIdempotentResponse`, `cleanupExpiredIdempotencyRecords`).
//!
//! A key maps to the hash of the request and the response that was sent. For
//! 24 hours a replay with the same body gets the original response back, and a
//! reuse of the key with a different body is rejected. Keys are scoped per
//! operator. Unlike the fluxapay middleware, lookups and stores run inside the
//! caller's transaction, so a crash can't leave a processed event without its
//! record.

use chrono::{DateTime, Duration, Utc};
use sha2::{Digest, Sha256};
use sqlx::SqliteConnection;

use crate::db::ts;

pub const IDEMPOTENCY_TTL_HOURS: i64 = 24;
pub const MAX_KEY_LEN: usize = 255;

/// Header the operator sends.
pub const IDEMPOTENCY_KEY_HEADER: &str = "Idempotency-Key";
/// Header set on a replayed response (as fluxapay does).
pub const REPLAYED_HEADER: &str = "X-Idempotent-Replayed";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lookup {
    /// Never seen (or expired): process the request.
    Miss,
    /// Seen with the same body: send this response again.
    Replay { code: u16, body: String },
    /// Seen with a different body.
    Conflict,
}

/// `"<operator_id>:<key>"`
#[must_use]
pub fn scoped_key(operator_id: &str, key: &str) -> String {
    format!("{operator_id}:{key}")
}

/// Hex sha256 of a request's canonical bytes.
#[must_use]
pub fn request_hash(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

/// Validate a key the way fluxapay does: non-empty, at most 255 characters.
#[must_use]
pub fn key_is_valid(key: &str) -> bool {
    !key.trim().is_empty() && key.len() <= MAX_KEY_LEN
}

fn expiry_cutoff(now: DateTime<Utc>) -> String {
    ts(now - Duration::hours(IDEMPOTENCY_TTL_HOURS))
}

pub async fn lookup(
    conn: &mut SqliteConnection,
    scoped_key: &str,
    request_hash: &str,
    now: DateTime<Utc>,
) -> Result<Lookup, sqlx::Error> {
    let row: Option<(String, i64, String, String)> = sqlx::query_as(
        "SELECT request_hash, response_code, response_body, created_at
         FROM idempotency_records WHERE scoped_key = ?",
    )
    .bind(scoped_key)
    .fetch_optional(&mut *conn)
    .await?;

    let Some((stored_hash, code, body, created_at)) = row else {
        return Ok(Lookup::Miss);
    };

    if created_at < expiry_cutoff(now) {
        sqlx::query("DELETE FROM idempotency_records WHERE scoped_key = ?")
            .bind(scoped_key)
            .execute(&mut *conn)
            .await?;
        return Ok(Lookup::Miss);
    }

    if stored_hash != request_hash {
        return Ok(Lookup::Conflict);
    }

    Ok(Lookup::Replay {
        code: u16::try_from(code).unwrap_or(200),
        body,
    })
}

pub async fn store(
    conn: &mut SqliteConnection,
    scoped_key: &str,
    operator_id: &str,
    request_hash: &str,
    code: u16,
    body: &str,
    now: DateTime<Utc>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO idempotency_records
           (scoped_key, operator_id, request_hash, response_code, response_body, created_at)
         VALUES (?, ?, ?, ?, ?, ?)
         ON CONFLICT (scoped_key) DO UPDATE SET
           request_hash = excluded.request_hash,
           response_code = excluded.response_code,
           response_body = excluded.response_body,
           created_at = excluded.created_at",
    )
    .bind(scoped_key)
    .bind(operator_id)
    .bind(request_hash)
    .bind(i64::from(code))
    .bind(body)
    .bind(ts(now))
    .execute(&mut *conn)
    .await?;
    Ok(())
}

/// Delete records older than the TTL. Run periodically by the scheduler.
pub async fn purge_expired(pool: &sqlx::SqlitePool) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM idempotency_records WHERE created_at < ?")
        .bind(expiry_cutoff(Utc::now()))
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_rules() {
        assert!(key_is_valid("abc"));
        assert!(!key_is_valid(""));
        assert!(!key_is_valid("   "));
        assert!(!key_is_valid(&"k".repeat(256)));
        assert!(key_is_valid(&"k".repeat(255)));
    }

    #[test]
    fn hash_is_stable() {
        assert_eq!(request_hash(b"x"), request_hash(b"x"));
        assert_ne!(request_hash(b"x"), request_hash(b"y"));
    }

    #[tokio::test]
    async fn replay_conflict_and_expiry() {
        let pool = crate::db::connect("sqlite::memory:").await.unwrap();
        sqlx::query(
            "INSERT INTO operators (id, name, stellar_accounts, hmac_secret, created_at)
             VALUES ('op', 'Op', '[]', 's', '2026-01-01T00:00:00.000Z')",
        )
        .execute(&pool)
        .await
        .unwrap();
        let mut conn = pool.acquire().await.unwrap();
        let now = Utc::now();

        assert_eq!(
            lookup(&mut conn, "op:k", "h1", now).await.unwrap(),
            Lookup::Miss
        );
        store(&mut conn, "op:k", "op", "h1", 201, "{}", now)
            .await
            .unwrap();
        assert_eq!(
            lookup(&mut conn, "op:k", "h1", now).await.unwrap(),
            Lookup::Replay {
                code: 201,
                body: "{}".into()
            }
        );
        assert_eq!(
            lookup(&mut conn, "op:k", "h2", now).await.unwrap(),
            Lookup::Conflict
        );

        let later = now + Duration::hours(IDEMPOTENCY_TTL_HOURS + 1);
        assert_eq!(
            lookup(&mut conn, "op:k", "h1", later).await.unwrap(),
            Lookup::Miss
        );
    }
}
