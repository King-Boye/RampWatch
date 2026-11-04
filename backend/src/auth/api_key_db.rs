use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::auth::api_key_model::{
    generate_api_key, ApiKey, ApiKeyInfo, CreateApiKeyRequest, CreateApiKeyResponse,
};

/// Columns of an API key row plus its configured per-minute limit. The limit
/// lives in `api_keys_rate_limit_config` (the table the rate limiter reads);
/// keys without a row report the limiter's default.
const KEY_SELECT: &str = "SELECT k.*, COALESCE(c.limit_per_minute, 200) AS rate_limit_per_minute \
     FROM api_keys k LEFT JOIN api_keys_rate_limit_config c ON c.api_key_id = k.id";

pub struct ApiKeyDb {
    pool: SqlitePool,
}

impl ApiKeyDb {
    #[must_use]
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Creates a new API key.
    #[tracing::instrument(skip(self, req), fields(operator_id = %operator_id, key_name = %req.name))]
    pub async fn create_api_key(
        &self,
        operator_id: &str,
        req: CreateApiKeyRequest,
    ) -> Result<CreateApiKeyResponse> {
        let id = Uuid::new_v4().to_string();
        let (plain_key, prefix, key_hash) = generate_api_key();
        let scopes = req.scopes.unwrap_or_else(|| "read".to_string());
        let now = Utc::now().to_rfc3339();

        let mut tx = self.pool.begin().await.with_context(|| {
            format!(
                "Failed to begin transaction for create_api_key for operator: {}",
                operator_id
            )
        })?;

        sqlx::query(
            r"
            INSERT INTO api_keys (id, name, key_prefix, key_hash, operator_id, scopes, status, created_at, expires_at)
            VALUES ($1, $2, $3, $4, $5, $6, 'active', $7, $8)
            ",
        )
        .bind(&id)
        .bind(&req.name)
        .bind(&prefix)
        .bind(&key_hash)
        .bind(operator_id)
        .bind(&scopes)
        .bind(&now)
        .bind(&req.expires_at)
        .execute(&mut *tx)
        .await
        .with_context(|| {
            format!(
                "Failed to insert API key for operator: {}, name: {}",
                operator_id, req.name
            )
        })?;

        if let Some(limit) = req.rate_limit_per_minute {
            anyhow::ensure!(limit > 0, "rate_limit_per_minute must be positive");
            sqlx::query(
                "INSERT INTO api_keys_rate_limit_config (api_key_id, limit_per_minute) VALUES ($1, $2)",
            )
            .bind(&id)
            .bind(limit)
            .execute(&mut *tx)
            .await
            .with_context(|| format!("Failed to store rate limit for API key id: {}", id))?;
        }

        let key = sqlx::query_as::<_, ApiKey>(&format!("{KEY_SELECT} WHERE k.id = $1"))
            .bind(&id)
            .fetch_one(&mut *tx)
            .await
            .with_context(|| format!("Failed to fetch newly created API key with id: {}", id))?;

        tx.commit().await.with_context(|| {
            format!(
                "Failed to commit create_api_key transaction for operator: {}",
                operator_id
            )
        })?;

        Ok(CreateApiKeyResponse {
            key: ApiKeyInfo::from(key),
            plain_key,
        })
    }

    /// Lists all API keys for a given operator address.
    #[tracing::instrument(skip(self), fields(operator_id = %operator_id))]
    pub async fn list_api_keys(&self, operator_id: &str) -> Result<Vec<ApiKeyInfo>> {
        let keys = sqlx::query_as::<_, ApiKey>(&format!(
            "{KEY_SELECT} WHERE k.operator_id = $1 ORDER BY k.created_at DESC"
        ))
        .bind(operator_id)
        .fetch_all(&self.pool)
        .await
        .with_context(|| format!("Failed to list API keys for operator: {}", operator_id))?;
        Ok(keys.into_iter().map(ApiKeyInfo::from).collect())
    }

    /// Retrieves an API key by its ID for a specific operator address.
    #[tracing::instrument(skip(self), fields(key_id = %id, operator_id = %operator_id))]
    pub async fn get_api_key_by_id(
        &self,
        id: &str,
        operator_id: &str,
    ) -> Result<Option<ApiKeyInfo>> {
        let key = sqlx::query_as::<_, ApiKey>(&format!(
            "{KEY_SELECT} WHERE k.id = $1 AND k.operator_id = $2"
        ))
        .bind(id)
        .bind(operator_id)
        .fetch_optional(&self.pool)
        .await
        .with_context(|| {
            format!(
                "Failed to get API key id: {} for operator: {}",
                id, operator_id
            )
        })?;
        Ok(key.map(ApiKeyInfo::from))
    }

    /// Revokes an active API key.
    #[tracing::instrument(skip(self), fields(key_id = %id, operator_id = %operator_id))]
    pub async fn revoke_api_key(&self, id: &str, operator_id: &str) -> Result<bool> {
        let revoked_at = Utc::now().to_rfc3339();
        let result = sqlx::query(
            r"
                UPDATE api_keys
                SET status = 'revoked', revoked_at = $1
                WHERE id = $2 AND operator_id = $3 AND status = 'active'
                ",
        )
        .bind(&revoked_at)
        .bind(id)
        .bind(operator_id)
        .execute(&self.pool)
        .await
        .with_context(|| {
            format!(
                "Failed to revoke API key id: {} for operator: {}",
                id, operator_id
            )
        })?;

        Ok(result.rows_affected() > 0)
    }

    /// Rotates an API key: revokes the old one and creates a new one with the same settings.
    #[tracing::instrument(skip(self), fields(key_id = %id, operator_id = %operator_id))]
    pub async fn rotate_api_key(
        &self,
        id: &str,
        operator_id: &str,
    ) -> Result<Option<CreateApiKeyResponse>> {
        let mut tx = self.pool.begin().await.with_context(|| {
            format!(
                "Failed to begin API key rotation transaction for id: {}",
                id
            )
        })?;

        let existing = sqlx::query_as::<_, ApiKey>(&format!(
            "{KEY_SELECT} WHERE k.id = $1 AND k.operator_id = $2 AND k.status = 'active'"
        ))
        .bind(id)
        .bind(operator_id)
        .fetch_optional(&mut *tx)
        .await
        .with_context(|| {
            format!(
                "Failed to fetch API key id: {} for operator: {} during rotation",
                id, operator_id
            )
        })?;

        let Some(existing) = existing else {
            return Ok(None);
        };

        let revoked_at = Utc::now().to_rfc3339();
        sqlx::query(
            r"
                UPDATE api_keys
                SET status = 'revoked', revoked_at = $1
                WHERE id = $2
                ",
        )
        .bind(&revoked_at)
        .bind(&existing.id)
        .execute(&mut *tx)
        .await
        .with_context(|| {
            format!(
                "Failed to revoke existing API key during rotation for id: {}",
                existing.id
            )
        })?;

        let new_id = Uuid::new_v4().to_string();
        let (plain_key, prefix, key_hash) = generate_api_key();
        let now = Utc::now().to_rfc3339();

        sqlx::query(
            r"
                INSERT INTO api_keys (
                    id, name, key_prefix, key_hash, operator_id, scopes, status, created_at, expires_at
                )
                VALUES ($1, $2, $3, $4, $5, $6, 'active', $7, $8)
                ",
        )
        .bind(&new_id)
        .bind(&existing.name)
        .bind(&prefix)
        .bind(&key_hash)
        .bind(operator_id)
        .bind(&existing.scopes)
        .bind(&now)
        .bind(&existing.expires_at)
        .execute(&mut *tx)
        .await
        .with_context(|| {
            format!(
                "Failed to insert rotated API key for operator: {} from id: {}",
                operator_id, existing.id
            )
        })?;

        // Keep a custom per-key limit across rotation.
        sqlx::query(
            "INSERT INTO api_keys_rate_limit_config (api_key_id, limit_per_minute)
             SELECT $1, limit_per_minute FROM api_keys_rate_limit_config WHERE api_key_id = $2",
        )
        .bind(&new_id)
        .bind(&existing.id)
        .execute(&mut *tx)
        .await
        .with_context(|| {
            format!(
                "Failed to copy rate limit to rotated API key id: {}",
                new_id
            )
        })?;

        let key = sqlx::query_as::<_, ApiKey>(&format!("{KEY_SELECT} WHERE k.id = $1"))
            .bind(&new_id)
            .fetch_one(&mut *tx)
            .await
            .with_context(|| format!("Failed to fetch rotated API key with id: {}", new_id))?;

        tx.commit().await.with_context(|| {
            format!(
                "Failed to commit API key rotation transaction for id: {}",
                id
            )
        })?;

        Ok(Some(CreateApiKeyResponse {
            key: ApiKeyInfo::from(key),
            plain_key,
        }))
    }

    /// Validates an API key using the plaintext key.
    ///
    /// Looks up candidates by stored prefix (fast), then verifies each one
    /// with Argon2 (slow, intentional). This avoids a full-table scan while
    /// keeping Argon2's brute-force resistance.
    #[tracing::instrument(skip(self, plain_key))]
    pub async fn validate_api_key(&self, plain_key: &str) -> Result<Option<ApiKey>> {
        use crate::auth::api_key_model::{extract_key_prefix, verify_api_key};

        let prefix = extract_key_prefix(plain_key);

        let candidates = sqlx::query_as::<_, ApiKey>(&format!(
            "{KEY_SELECT} WHERE k.key_prefix = $1 AND k.status = 'active'"
        ))
        .bind(&prefix)
        .fetch_all(&self.pool)
        .await
        .context("Failed to query API keys by prefix")?;

        for k in candidates {
            if let Some(ref expires_at) = k.expires_at {
                match DateTime::parse_from_rfc3339(expires_at) {
                    Ok(exp) if exp < Utc::now() => continue,
                    Err(e) => {
                        tracing::warn!(
                            "API key {} has malformed expires_at '{}': {}. Treating as expired.",
                            k.id,
                            expires_at,
                            e
                        );
                        continue;
                    }
                    _ => {}
                }
            }

            if verify_api_key(plain_key, &k.key_hash) {
                // best-effort last_used_at update
                let _ = sqlx::query("UPDATE api_keys SET last_used_at = $1 WHERE id = $2")
                    .bind(Utc::now().to_rfc3339())
                    .bind(&k.id)
                    .execute(&self.pool)
                    .await;
                return Ok(Some(k));
            }
        }

        Ok(None)
    }
}
