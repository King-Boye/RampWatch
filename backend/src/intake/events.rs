//! Apply one operator event: dedupe, check the state machine, write the
//! payout and its event, and open an `illegal_transition` incident when the
//! operator's engine reports a move the state machine forbids.

use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use serde_json::{json, Value};
use sqlx::{SqliteConnection, SqlitePool};

use super::idempotency::{self, Lookup};
use crate::db::ts;
use crate::domain::models::{Operator, Payout, PayoutEventInput};
use crate::domain::retry_rules::check_resend;
use crate::domain::state::{validate_transition, PayoutState, Transition};
use crate::error::ApiError;

/// The response for one event: what the route sends back, and what a replay
/// of the same key sends again.
#[derive(Debug, Clone)]
pub struct EventOutcome {
    pub status: StatusCode,
    pub body: Value,
    pub replayed: bool,
}

impl EventOutcome {
    fn from_error(err: &ApiError) -> Self {
        Self {
            status: err.status_code(),
            body: serde_json::to_value(err.to_error_response(None)).unwrap_or(Value::Null),
            replayed: false,
        }
    }
}

/// Key for an event that came without one (batch or CSV): the same event
/// sent twice gets the same key.
#[must_use]
pub fn derived_key(event: &PayoutEventInput) -> String {
    let material = format!(
        "{}|{}|{}|{}",
        event.external_id,
        event.attempt,
        event.state,
        ts(event.occurred_at)
    );
    format!("derived:{}", idempotency::request_hash(material.as_bytes()))
}

/// Process one event in its own transaction.
pub async fn process_event(
    pool: &SqlitePool,
    operator: &Operator,
    key: &str,
    event: &PayoutEventInput,
) -> Result<EventOutcome, ApiError> {
    if !idempotency::key_is_valid(key) {
        return Ok(EventOutcome::from_error(&ApiError::bad_request(
            "invalid_idempotency_key",
            "Idempotency key must be a non-empty string of at most 255 characters",
        )));
    }
    if let Err(errors) = event.validate() {
        let mut details = std::collections::HashMap::new();
        details.insert("errors".to_string(), json!(errors));
        return Ok(EventOutcome::from_error(
            &ApiError::bad_request("validation_failed", "The event is invalid").with_details(details),
        ));
    }

    // The key is not part of what gets hashed: the same event under the same
    // key is a replay whether the key came from a header or the body.
    let mut canonical = event.clone();
    canonical.idempotency_key = None;
    let hash = idempotency::request_hash(&serde_json::to_vec(&canonical).unwrap_or_default());
    let scoped = idempotency::scoped_key(&operator.id, key);
    let now = Utc::now();

    // IMMEDIATE takes SQLite's write lock up front, so two copies of the same
    // event can't both pass the lookup.
    let mut tx = pool.begin_with("BEGIN IMMEDIATE").await?;

    match idempotency::lookup(&mut tx, &scoped, &hash, now).await? {
        Lookup::Replay { code, body } => {
            tx.commit().await?;
            return Ok(EventOutcome {
                status: StatusCode::from_u16(code).unwrap_or(StatusCode::OK),
                body: serde_json::from_str(&body).unwrap_or(Value::Null),
                replayed: true,
            });
        }
        Lookup::Conflict => {
            tx.commit().await?;
            return Ok(EventOutcome::from_error(&ApiError::conflict(
                "idempotency_conflict",
                "This Idempotency-Key was already used with a different event",
            )));
        }
        Lookup::Miss => {}
    }

    let outcome = apply(&mut tx, operator, &scoped, event, now).await?;

    let body = serde_json::to_string(&outcome.body).unwrap_or_default();
    idempotency::store(
        &mut tx,
        &scoped,
        &operator.id,
        &hash,
        outcome.status.as_u16(),
        &body,
        now,
    )
    .await?;
    tx.commit().await?;
    Ok(outcome)
}

async fn find_payout(
    conn: &mut SqliteConnection,
    operator_id: &str,
    external_id: &str,
    attempt: i64,
) -> Result<Option<Payout>, sqlx::Error> {
    sqlx::query_as::<_, Payout>(
        "SELECT * FROM payouts WHERE operator_id = ? AND external_id = ? AND attempt = ?",
    )
    .bind(operator_id)
    .bind(external_id)
    .bind(attempt)
    .fetch_optional(&mut *conn)
    .await
}

/// The payout a resend replaces: the latest earlier attempt of the parent
/// cash-out (`parent_external_id`, or the same id when the operator reuses it).
async fn find_parent(
    conn: &mut SqliteConnection,
    operator_id: &str,
    event: &PayoutEventInput,
) -> Result<Option<Payout>, sqlx::Error> {
    let parent_external = event
        .parent_external_id
        .as_deref()
        .unwrap_or(&event.external_id);
    let max_attempt = if parent_external == event.external_id {
        event.attempt - 1
    } else {
        i64::MAX
    };
    sqlx::query_as::<_, Payout>(
        "SELECT * FROM payouts WHERE operator_id = ? AND external_id = ? AND attempt <= ?
         ORDER BY attempt DESC LIMIT 1",
    )
    .bind(operator_id)
    .bind(parent_external)
    .bind(max_attempt)
    .fetch_optional(&mut *conn)
    .await
}

async fn open_illegal_transition_incident(
    conn: &mut SqliteConnection,
    operator: &Operator,
    event: &PayoutEventInput,
    payout_id: Option<&str>,
    message: &str,
    now: DateTime<Utc>,
) -> Result<String, sqlx::Error> {
    let id = uuid::Uuid::new_v4().to_string();
    let detail = json!({
        "message": message,
        "external_id": event.external_id,
        "attempt": event.attempt,
        "to_state": event.state,
        "occurred_at": ts(event.occurred_at),
    });
    sqlx::query(
        "INSERT INTO incidents (id, operator_id, kind, severity, corridor, payout_id, detail, opened_at)
         VALUES (?, ?, 'illegal_transition', 'warning', ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&operator.id)
    .bind(&event.corridor)
    .bind(payout_id)
    .bind(detail.to_string())
    .bind(ts(now))
    .execute(&mut *conn)
    .await?;
    Ok(id)
}

async fn apply(
    conn: &mut SqliteConnection,
    operator: &Operator,
    scoped_key: &str,
    event: &PayoutEventInput,
    now: DateTime<Utc>,
) -> Result<EventOutcome, ApiError> {
    let existing = find_payout(conn, &operator.id, &event.external_id, event.attempt).await?;
    let from_state = existing
        .as_ref()
        .map(|p| p.state.parse::<PayoutState>())
        .transpose()
        .map_err(|e| ApiError::internal("corrupt_state", e))?;

    let transition = match validate_transition(from_state, event.state) {
        Ok(t) => t,
        Err(invalid) => {
            let message = invalid.to_string();
            let payout_id = existing.as_ref().map(|p| p.id.as_str());
            let incident_id =
                open_illegal_transition_incident(conn, operator, event, payout_id, &message, now)
                    .await?;
            let mut details = std::collections::HashMap::new();
            details.insert("from_state".to_string(), json!(from_state));
            details.insert("to_state".to_string(), json!(event.state));
            details.insert("payout_id".to_string(), json!(payout_id));
            details.insert("incident_id".to_string(), json!(incident_id));
            return Ok(EventOutcome::from_error(
                &ApiError::conflict("invalid_transition", message).with_details(details),
            ));
        }
    };

    let occurred = ts(event.occurred_at);
    let (payout_id, created, resend) = match existing {
        Some(payout) => {
            if transition == Transition::Changed {
                sqlx::query(
                    "UPDATE payouts SET state = ?, updated_at = MAX(updated_at, ?),
                       stellar_tx_hash = COALESCE(?, stellar_tx_hash), memo = COALESCE(?, memo)
                     WHERE id = ?",
                )
                .bind(event.state.as_str())
                .bind(&occurred)
                .bind(&event.stellar_tx_hash)
                .bind(&event.memo)
                .bind(&payout.id)
                .execute(&mut *conn)
                .await?;
            } else {
                sqlx::query(
                    "UPDATE payouts SET stellar_tx_hash = COALESCE(?, stellar_tx_hash),
                       memo = COALESCE(?, memo) WHERE id = ?",
                )
                .bind(&event.stellar_tx_hash)
                .bind(&event.memo)
                .bind(&payout.id)
                .execute(&mut *conn)
                .await?;
            }
            (payout.id, false, None)
        }
        None => {
            let (parent_id, resend) = if event.attempt > 1 || event.parent_external_id.is_some() {
                match find_parent(conn, &operator.id, event).await? {
                    Some(parent) => {
                        let parent_state = parent
                            .state
                            .parse::<PayoutState>()
                            .map_err(|e| ApiError::internal("corrupt_state", e))?;
                        let verdict = check_resend(parent_state, parent.attempt, event.attempt);
                        (Some(parent.id), Some(json!(verdict)))
                    }
                    None => (None, Some(json!("parent_unknown"))),
                }
            } else {
                (None, None)
            };

            let id = uuid::Uuid::new_v4().to_string();
            sqlx::query(
                "INSERT INTO payouts (id, operator_id, external_id, corridor, country, amount_minor,
                   currency, state, attempt, parent_payout_id, stellar_tx_hash, memo, created_at, updated_at)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(&id)
            .bind(&operator.id)
            .bind(&event.external_id)
            .bind(&event.corridor)
            .bind(&event.country)
            .bind(event.amount_minor)
            .bind(&event.currency)
            .bind(event.state.as_str())
            .bind(event.attempt)
            .bind(&parent_id)
            .bind(&event.stellar_tx_hash)
            .bind(&event.memo)
            .bind(&occurred)
            .bind(&occurred)
            .execute(&mut *conn)
            .await?;
            (id, true, resend)
        }
    };

    let event_id = uuid::Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO payout_events (id, payout_id, from_state, to_state, reason, occurred_at, received_at, idempotency_key)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&event_id)
    .bind(&payout_id)
    .bind(from_state.map(PayoutState::as_str))
    .bind(event.state.as_str())
    .bind(&event.reason)
    .bind(&occurred)
    .bind(ts(now))
    .bind(scoped_key)
    .execute(&mut *conn)
    .await?;

    let mut body = json!({
        "event_id": event_id,
        "payout_id": payout_id,
        "external_id": event.external_id,
        "attempt": event.attempt,
        "previous_state": from_state,
        "state": event.state,
        "changed": transition == Transition::Changed,
    });
    if let Some(resend) = resend {
        body["resend"] = resend;
    }
    // A resend whose parent never failed is still accepted (it already
    // happened); the verdict tells the operator now, and the double-pay
    // detector follows up once ledger data is in.

    Ok(EventOutcome {
        status: if created { StatusCode::CREATED } else { StatusCode::OK },
        body,
        replayed: false,
    })
}
