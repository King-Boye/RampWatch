//! Intake routes:
//! - `POST /v1/payouts/events`        one event, `Idempotency-Key` header required
//! - `POST /v1/payouts/events/batch`  up to 500 events
//! - `POST /v1/payouts/import`        CSV upload, same fields
//!
//! All three sit behind API key auth and HMAC verification.

use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    Extension, Json,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;

use crate::domain::models::{Operator, PayoutEventInput};
use crate::error::ApiError;
use crate::intake::events::{derived_key, process_event, EventOutcome};
use crate::intake::idempotency::{IDEMPOTENCY_KEY_HEADER, REPLAYED_HEADER};
use crate::intake::csv_import;
use crate::AppState;

pub const MAX_BATCH: usize = 500;

fn outcome_response(outcome: EventOutcome) -> Response {
    let replayed = outcome.replayed;
    let mut response = (outcome.status, Json(outcome.body)).into_response();
    if replayed {
        response
            .headers_mut()
            .insert(REPLAYED_HEADER, HeaderValue::from_static("true"));
    }
    response
}

fn parse_json<T: for<'de> Deserialize<'de>>(body: &[u8]) -> Result<T, ApiError> {
    serde_json::from_slice(body)
        .map_err(|e| ApiError::bad_request("invalid_json", format!("Request body is not valid: {e}")))
}

/// `POST /v1/payouts/events`
pub async fn ingest_event(
    State(state): State<AppState>,
    Extension(operator): Extension<Arc<Operator>>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, ApiError> {
    let key = headers
        .get(IDEMPOTENCY_KEY_HEADER)
        .and_then(|v| v.to_str().ok())
        .map(str::to_string)
        .ok_or_else(|| {
            ApiError::bad_request("missing_idempotency_key", "The Idempotency-Key header is required")
        })?;
    let event: PayoutEventInput = parse_json(&body)?;
    let outcome = process_event(&state.pool, &operator, &key, &event).await?;
    Ok(outcome_response(outcome))
}

#[derive(Deserialize)]
#[serde(untagged)]
enum BatchBody {
    Wrapped { events: Vec<Value> },
    Bare(Vec<Value>),
}

/// Run a list of events one by one. Each event gets its own transaction, so a
/// rejected event doesn't undo the others.
async fn run_many(
    state: &AppState,
    operator: &Operator,
    items: Vec<(usize, Result<PayoutEventInput, String>)>,
) -> Result<Value, ApiError> {
    let mut results = Vec::with_capacity(items.len());
    let (mut accepted, mut rejected, mut replayed) = (0, 0, 0);

    for (position, item) in items {
        let outcome = match item {
            Ok(event) => {
                let key = event.idempotency_key.clone().unwrap_or_else(|| derived_key(&event));
                process_event(&state.pool, operator, &key, &event).await?
            }
            Err(message) => EventOutcome {
                status: StatusCode::BAD_REQUEST,
                body: json!({"error": {"code": "invalid_event", "message": message}}),
                replayed: false,
            },
        };
        if outcome.replayed {
            replayed += 1;
        } else if outcome.status.is_success() {
            accepted += 1;
        } else {
            rejected += 1;
        }
        results.push(json!({
            "position": position,
            "status": outcome.status.as_u16(),
            "replayed": outcome.replayed,
            "body": outcome.body,
        }));
    }

    Ok(json!({
        "accepted": accepted,
        "rejected": rejected,
        "replayed": replayed,
        "results": results,
    }))
}

/// `POST /v1/payouts/events/batch`. Body: `{"events": [...]}` or a bare array.
/// Each event may carry its own `idempotency_key`; otherwise one is derived
/// from the event, so resending the same batch is safe.
pub async fn ingest_batch(
    State(state): State<AppState>,
    Extension(operator): Extension<Arc<Operator>>,
    body: Bytes,
) -> Result<Response, ApiError> {
    let events = match parse_json::<BatchBody>(&body)? {
        BatchBody::Wrapped { events } | BatchBody::Bare(events) => events,
    };
    if events.is_empty() {
        return Err(ApiError::bad_request("empty_batch", "The batch has no events"));
    }
    if events.len() > MAX_BATCH {
        return Err(ApiError::bad_request(
            "batch_too_large",
            format!("A batch can hold at most {MAX_BATCH} events"),
        ));
    }
    let items = events
        .into_iter()
        .enumerate()
        .map(|(i, v)| (i, serde_json::from_value(v).map_err(|e| e.to_string())))
        .collect();
    let summary = run_many(&state, &operator, items).await?;
    Ok((StatusCode::OK, Json(summary)).into_response())
}

/// `POST /v1/payouts/import`. Body: CSV text with a header row.
pub async fn import_csv(
    State(state): State<AppState>,
    Extension(operator): Extension<Arc<Operator>>,
    body: Bytes,
) -> Result<Response, ApiError> {
    let rows = csv_import::parse(&body);
    if rows.is_empty() {
        return Err(ApiError::bad_request("empty_csv", "The CSV has no rows"));
    }
    let summary = run_many(&state, &operator, rows).await?;
    Ok((StatusCode::OK, Json(summary)).into_response())
}
