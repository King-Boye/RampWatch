//! Operator authentication. Keys are owned by an operator instead of a SEP-10
//! wallet. Hashing and the `rw_live_` format come from the copied
//! `api_key_model` (Argon2id, prefix lookup), the key-management handlers are
//! in `api_key_routes` (copied from stellar-insights `api/api_keys.rs`), and
//! verification reuses the copied rate limiter's cached lookup so a key is not
//! re-hashed on every request.

use axum::{
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::Response,
};
use sqlx::SqlitePool;
use std::sync::Arc;

use crate::api::middleware::rate_limit::RateLimiter;
use crate::domain::models::Operator;
use crate::error::ApiError;

#[derive(Clone)]
pub struct AuthState {
    pub pool: SqlitePool,
    pub limiter: Arc<RateLimiter>,
}

/// Require `Authorization: Bearer <api_key>` and attach the [`Operator`].
pub async fn operator_auth_middleware(
    State(state): State<AuthState>,
    mut req: Request,
    next: Next,
) -> Result<Response, ApiError> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(str::trim)
        .ok_or_else(|| {
            ApiError::unauthorized(
                "missing_api_key",
                "Authorization: Bearer <api_key> is required",
            )
        })?;

    let key_id = state
        .limiter
        .resolve_api_key_id(token)
        .await
        .ok_or_else(|| {
            ApiError::unauthorized("invalid_api_key", "API key is invalid or revoked")
        })?;

    let operator = sqlx::query_as::<_, Operator>(
        "SELECT o.* FROM operators o JOIN api_keys k ON k.operator_id = o.id
         WHERE k.id = ? AND k.status = 'active'",
    )
    .bind(&key_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| ApiError::unauthorized("invalid_api_key", "API key is invalid or revoked"))?;

    req.extensions_mut().insert(Arc::new(operator));
    Ok(next.run(req).await)
}

/// Require `Authorization: Bearer <ADMIN_TOKEN>` (constant-time compare).
pub async fn admin_auth_middleware(
    State(admin_token): State<Option<Arc<str>>>,
    req: Request,
    next: Next,
) -> Result<Response, ApiError> {
    let Some(expected) = admin_token else {
        return Err(ApiError::forbidden(
            "admin_disabled",
            "ADMIN_TOKEN is not configured",
        ));
    };
    let presented = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(str::trim)
        .unwrap_or_default();

    if !super::hmac::constant_time_eq(presented.as_bytes(), expected.as_bytes()) {
        return Err(ApiError::unauthorized(
            "invalid_admin_token",
            "Admin token is invalid",
        ));
    }
    Ok(next.run(req).await)
}
