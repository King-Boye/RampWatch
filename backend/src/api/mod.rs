pub mod admin;
pub mod middleware;
pub mod payouts;

use axum::{
    extract::{DefaultBodyLimit, State},
    middleware::{from_fn, from_fn_with_state},
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use std::sync::Arc;

use crate::auth::api_keys::{admin_auth_middleware, operator_auth_middleware, AuthState};
use crate::auth::hmac::{hmac_middleware, MAX_SIGNED_BODY_BYTES};
use crate::error::ApiError;
use crate::AppState;

async fn health(State(state): State<AppState>) -> Result<Json<serde_json::Value>, ApiError> {
    sqlx::query("SELECT 1").execute(&state.pool).await?;
    Ok(Json(
        json!({ "status": "ok", "version": env!("CARGO_PKG_VERSION") }),
    ))
}

/// Build the HTTP router.
pub fn router(state: AppState) -> Router {
    let auth_state = AuthState {
        pool: state.pool.clone(),
        limiter: state.limiter.clone(),
    };

    // route_layer: the last layer added runs first, so auth runs before HMAC
    // (HMAC needs the operator's secret).
    let intake = Router::new()
        .route("/v1/payouts/events", post(payouts::ingest_event))
        .route("/v1/payouts/events/batch", post(payouts::ingest_batch))
        .route("/v1/payouts/import", post(payouts::import_csv))
        .route_layer(from_fn_with_state(
            state.config.webhook_timestamp_tolerance_secs,
            hmac_middleware,
        ))
        .route_layer(from_fn_with_state(
            auth_state.clone(),
            operator_auth_middleware,
        ));

    let api_keys = crate::auth::api_key_routes::routes(Arc::new(
        crate::auth::api_key_db::ApiKeyDb::new(state.pool.clone()),
    ))
    .route_layer(from_fn_with_state(
        auth_state.clone(),
        operator_auth_middleware,
    ));

    let admin_token: Option<Arc<str>> = state.config.admin_token.as_deref().map(Arc::from);
    let admin = Router::new()
        .route("/v1/admin/operators", post(admin::create_operator))
        .route(
            "/v1/admin/operators/{id}/rotate-secret",
            post(admin::rotate_secret),
        )
        .route_layer(from_fn_with_state(admin_token, admin_auth_middleware));

    Router::new()
        .route("/health", get(health))
        .merge(intake)
        .merge(admin)
        .nest("/v1/api-keys", api_keys)
        .layer(from_fn_with_state(
            state.limiter.clone(),
            middleware::rate_limit::rate_limit_middleware,
        ))
        .layer(from_fn(middleware::request_id::request_id_middleware))
        .layer(DefaultBodyLimit::max(MAX_SIGNED_BODY_BYTES))
        .with_state(state)
}
