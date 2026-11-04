//! `POST /v1/admin/operators`: create an operator. The API key and HMAC
//! secret are returned once and can't be read back.
//! `POST /v1/admin/operators/{id}/rotate-secret`: start an HMAC secret
//! rotation (fluxapay's rotate flow): the old secret keeps working for 24h.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::{Duration, Utc};
use serde::Deserialize;
use serde_json::json;

use crate::auth::api_key_db::ApiKeyDb;
use crate::auth::api_key_model::CreateApiKeyRequest;
use crate::db::{now_str, ts};
use crate::domain::models::is_stellar_account;
use crate::error::ApiError;
use crate::AppState;

/// How long the previous HMAC secret stays valid after a rotation.
pub const ROTATION_GRACE_HOURS: i64 = 24;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateOperatorRequest {
    pub name: String,
    pub stellar_accounts: Vec<String>,
    #[serde(default)]
    pub telegram_chat_id: Option<String>,
    #[serde(default)]
    pub stuck_after_secs: Option<i64>,
}

/// fluxapay `generateWebhookSecret` format: `whsec_` + random hex.
fn generate_secret() -> String {
    format!(
        "whsec_{}{}",
        uuid::Uuid::new_v4().simple(),
        uuid::Uuid::new_v4().simple()
    )
}

pub async fn create_operator(
    State(state): State<AppState>,
    Json(req): Json<CreateOperatorRequest>,
) -> Result<Response, ApiError> {
    let name = req.name.trim();
    if name.is_empty() || name.len() > 100 {
        return Err(ApiError::bad_request("invalid_name", "name must be 1 to 100 characters"));
    }
    if req.stellar_accounts.is_empty() {
        return Err(ApiError::bad_request(
            "invalid_accounts",
            "stellar_accounts needs at least one distribution account",
        ));
    }
    if let Some(bad) = req.stellar_accounts.iter().find(|a| !is_stellar_account(a)) {
        return Err(ApiError::bad_request(
            "invalid_accounts",
            format!("'{bad}' is not a Stellar account id (G...)"),
        ));
    }
    let stuck_after_secs = req.stuck_after_secs.unwrap_or(900);
    if !(60..=86_400).contains(&stuck_after_secs) {
        return Err(ApiError::bad_request(
            "invalid_stuck_after_secs",
            "stuck_after_secs must be between 60 and 86400",
        ));
    }

    let id = uuid::Uuid::new_v4().to_string();
    let secret = generate_secret();
    let accounts = serde_json::to_string(&req.stellar_accounts).unwrap_or_else(|_| "[]".into());

    sqlx::query(
        "INSERT INTO operators (id, name, stellar_accounts, hmac_secret, telegram_chat_id, stuck_after_secs, created_at)
         VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(name)
    .bind(&accounts)
    .bind(&secret)
    .bind(&req.telegram_chat_id)
    .bind(stuck_after_secs)
    .bind(now_str())
    .execute(&state.pool)
    .await?;

    let key_request = CreateApiKeyRequest {
        name: format!("{name} intake"),
        scopes: Some("write".to_string()),
        rate_limit_per_minute: None,
        expires_at: None,
    };
    let api_key = match ApiKeyDb::new(state.pool.clone()).create_api_key(&id, key_request).await {
        Ok(created) => created.plain_key,
        Err(e) => {
            // Don't leave an operator nobody can authenticate as.
            let _ = sqlx::query("DELETE FROM operators WHERE id = ?")
                .bind(&id)
                .execute(&state.pool)
                .await;
            return Err(e.into());
        }
    };

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "id": id,
            "name": name,
            "stellar_accounts": req.stellar_accounts,
            "stuck_after_secs": stuck_after_secs,
            "api_key": api_key,
            "hmac_secret": secret,
            "note": "Store api_key and hmac_secret now. They are not shown again."
        })),
    )
        .into_response())
}

pub async fn rotate_secret(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Response, ApiError> {
    let new_secret = generate_secret();
    let expires = ts(Utc::now() + Duration::hours(ROTATION_GRACE_HOURS));
    let updated = sqlx::query(
        "UPDATE operators SET hmac_secret_previous = hmac_secret,
           hmac_secret_previous_expires_at = ?, hmac_secret = ? WHERE id = ?",
    )
    .bind(&expires)
    .bind(&new_secret)
    .bind(&id)
    .execute(&state.pool)
    .await?;
    if updated.rows_affected() == 0 {
        return Err(ApiError::not_found("operator_not_found", "Operator not found"));
    }
    Ok(Json(json!({
        "id": id,
        "hmac_secret": new_secret,
        "previous_secret_valid_until": expires,
        "instructions": "Sign with the new secret. Until the time above, requests signed with the old secret (in either signature header) are still accepted."
    }))
    .into_response())
}
