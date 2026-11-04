//! Milestone 1 acceptance tests: intake, idempotency, state machine, auth.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
    Router,
};
use rampwatch::{api, auth::hmac::sign, config::Config, AppState};
use serde_json::{json, Value};
use tower::ServiceExt;

const ACCOUNT: &str = "GBRPYHIL2CI3FNQ4BXLFMNDLFJUNPU2HY3ZMFSHONUCEOASW7QC7OX2H";

struct Harness {
    app: Router,
    state: AppState,
    api_key: String,
    secret: String,
    operator_id: String,
}

async fn call(app: &Router, req: Request<Body>) -> (StatusCode, axum::http::HeaderMap, Value) {
    let res = app.clone().oneshot(req).await.unwrap();
    let status = res.status();
    let headers = res.headers().clone();
    let bytes = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let body = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
    (status, headers, body)
}

async fn harness() -> Harness {
    let config = Config::for_tests();
    let admin = config.admin_token.clone().unwrap();
    let state = AppState::new_without_redis(config).await.unwrap();
    let app = api::router(state.clone());

    let (status, _, body) = call(
        &app,
        Request::post("/v1/admin/operators")
            .header("authorization", format!("Bearer {admin}"))
            .header("content-type", "application/json")
            .body(Body::from(
                json!({"name": "Test Wallet", "stellar_accounts": [ACCOUNT]}).to_string(),
            ))
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "{body}");

    Harness {
        app,
        state,
        api_key: body["api_key"].as_str().unwrap().to_string(),
        secret: body["hmac_secret"].as_str().unwrap().to_string(),
        operator_id: body["id"].as_str().unwrap().to_string(),
    }
}

fn event(external_id: &str, state: &str, minute: u32) -> Value {
    json!({
        "external_id": external_id,
        "attempt": 1,
        "parent_external_id": null,
        "state": state,
        "reason": null,
        "corridor": "USDC->NGN",
        "country": "NG",
        "amount_minor": 2_500_000,
        "currency": "NGN",
        "stellar_tx_hash": null,
        "memo": external_id,
        "occurred_at": format!("2026-11-04T19:{minute:02}:12Z"),
    })
}

impl Harness {
    fn signed(&self, path: &str, body: String, key: Option<&str>, secret: &str) -> Request<Body> {
        let ts = chrono::Utc::now().timestamp().to_string();
        let sig = sign(secret, &ts, body.as_bytes());
        let mut req = Request::post(path)
            .header("authorization", format!("Bearer {}", self.api_key))
            .header("content-type", "application/json")
            .header("X-RampWatch-Timestamp", ts)
            .header("X-RampWatch-Signature", sig);
        if let Some(key) = key {
            req = req.header("Idempotency-Key", key);
        }
        req.body(Body::from(body)).unwrap()
    }

    async fn send(&self, key: &str, body: &Value) -> (StatusCode, axum::http::HeaderMap, Value) {
        let req = self.signed(
            "/v1/payouts/events",
            body.to_string(),
            Some(key),
            &self.secret,
        );
        call(&self.app, req).await
    }

    async fn count(&self, sql: &str) -> i64 {
        sqlx::query_scalar(sql)
            .fetch_one(&self.state.pool)
            .await
            .unwrap()
    }
}

#[tokio::test]
async fn replayed_event_is_ignored() {
    let h = harness().await;
    let e = event("co_8f2a", "processing", 40);

    let (status, headers, first) = h.send("evt-1", &e).await;
    assert_eq!(status, StatusCode::CREATED, "{first}");
    assert!(headers.get("X-Idempotent-Replayed").is_none());

    let (status, headers, second) = h.send("evt-1", &e).await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(headers.get("X-Idempotent-Replayed").unwrap(), "true");
    assert_eq!(first, second, "a replay returns the original response");

    assert_eq!(h.count("SELECT COUNT(*) FROM payouts").await, 1);
    assert_eq!(h.count("SELECT COUNT(*) FROM payout_events").await, 1);

    // Same key, different body: rejected, nothing written.
    let (status, _, body) = h.send("evt-1", &event("co_8f2a", "completed", 41)).await;
    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(body["error"]["code"], "idempotency_conflict");
    assert_eq!(h.count("SELECT COUNT(*) FROM payout_events").await, 1);
}

#[tokio::test]
async fn illegal_transition_gets_409_and_an_incident() {
    let h = harness().await;
    assert_eq!(
        h.send("a", &event("co_1", "processing", 40)).await.0,
        StatusCode::CREATED
    );
    assert_eq!(
        h.send("b", &event("co_1", "completed", 45)).await.0,
        StatusCode::OK
    );

    let (status, _, body) = h.send("c", &event("co_1", "pending", 46)).await;
    assert_eq!(status, StatusCode::CONFLICT, "{body}");
    assert_eq!(body["error"]["code"], "invalid_transition");
    assert_eq!(body["error"]["details"]["from_state"], "completed");

    let state: String = sqlx::query_scalar("SELECT state FROM payouts WHERE external_id = 'co_1'")
        .fetch_one(&h.state.pool)
        .await
        .unwrap();
    assert_eq!(
        state, "completed",
        "the illegal event did not change the payout"
    );
    assert_eq!(
        h.count("SELECT COUNT(*) FROM incidents WHERE kind = 'illegal_transition'")
            .await,
        1
    );

    // A new payout can't start as completed either.
    let (status, _, _) = h.send("d", &event("co_2", "completed", 47)).await;
    assert_eq!(status, StatusCode::CONFLICT);

    // Replaying the rejected event returns the same 409 without opening
    // another incident: still one for co_1 and one for co_2.
    let (status, headers, _) = h.send("c", &event("co_1", "pending", 46)).await;
    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(headers.get("X-Idempotent-Replayed").unwrap(), "true");
    assert_eq!(
        h.count("SELECT COUNT(*) FROM incidents WHERE kind = 'illegal_transition'")
            .await,
        2
    );
}

#[tokio::test]
async fn bad_signature_gets_401() {
    let h = harness().await;
    let body = event("co_9", "pending", 40).to_string();

    // Wrong secret.
    let req = h.signed(
        "/v1/payouts/events",
        body.clone(),
        Some("k1"),
        "whsec_wrong",
    );
    let (status, _, res) = call(&h.app, req).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(res["error"]["code"], "invalid_signature");

    // Body changed after signing.
    let mut req = h.signed("/v1/payouts/events", body.clone(), Some("k2"), &h.secret);
    *req.body_mut() = Body::from(body.replace("2500000", "9500000"));
    assert_eq!(call(&h.app, req).await.0, StatusCode::UNAUTHORIZED);

    // Missing signature.
    let req = Request::post("/v1/payouts/events")
        .header("authorization", format!("Bearer {}", h.api_key))
        .header(
            "X-RampWatch-Timestamp",
            chrono::Utc::now().timestamp().to_string(),
        )
        .header("Idempotency-Key", "k3")
        .body(Body::from(body.clone()))
        .unwrap();
    assert_eq!(call(&h.app, req).await.0, StatusCode::UNAUTHORIZED);

    // Stale timestamp, correctly signed.
    let old = (chrono::Utc::now().timestamp() - 600).to_string();
    let req = Request::post("/v1/payouts/events")
        .header("authorization", format!("Bearer {}", h.api_key))
        .header("X-RampWatch-Timestamp", &old)
        .header(
            "X-RampWatch-Signature",
            sign(&h.secret, &old, body.as_bytes()),
        )
        .header("Idempotency-Key", "k4")
        .body(Body::from(body.clone()))
        .unwrap();
    let (status, _, res) = call(&h.app, req).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(res["error"]["code"], "stale_timestamp");

    // Bad API key.
    let mut req = h.signed("/v1/payouts/events", body, Some("k5"), &h.secret);
    req.headers_mut().insert(
        "authorization",
        "Bearer rw_live_00000000000000000000000000000000"
            .parse()
            .unwrap(),
    );
    assert_eq!(call(&h.app, req).await.0, StatusCode::UNAUTHORIZED);

    assert_eq!(h.count("SELECT COUNT(*) FROM payouts").await, 0);
}

#[tokio::test]
async fn missing_idempotency_key_is_rejected() {
    let h = harness().await;
    let req = h.signed(
        "/v1/payouts/events",
        event("co_3", "pending", 40).to_string(),
        None,
        &h.secret,
    );
    let (status, _, body) = call(&h.app, req).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], "missing_idempotency_key");
}

#[tokio::test]
async fn batch_is_per_event_and_safe_to_resend() {
    let h = harness().await;
    let batch = json!({"events": [
        event("co_b1", "processing", 40),
        event("co_b1", "completed", 42),
        event("co_b2", "failed", 43),
    ]})
    .to_string();

    let (status, _, body) = call(
        &h.app,
        h.signed("/v1/payouts/events/batch", batch.clone(), None, &h.secret),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["accepted"], 2);
    assert_eq!(body["rejected"], 1);
    assert_eq!(body["results"][2]["status"], 409);

    let (_, _, again) = call(
        &h.app,
        h.signed("/v1/payouts/events/batch", batch, None, &h.secret),
    )
    .await;
    assert_eq!(again["replayed"], 3);
    assert_eq!(h.count("SELECT COUNT(*) FROM payout_events").await, 2);
}

#[tokio::test]
async fn resend_links_to_its_parent() {
    let h = harness().await;
    h.send("p1", &event("co_r", "processing", 40)).await;
    h.send("p2", &event("co_r", "failed", 45)).await;

    let mut resend = event("co_r", "processing", 50);
    resend["attempt"] = json!(2);
    let (status, _, body) = h.send("p3", &resend).await;
    assert_eq!(status, StatusCode::CREATED, "{body}");
    assert_eq!(body["resend"], "allowed");

    // A resend of a payout that never failed is accepted but flagged.
    h.send("q1", &event("co_s", "processing", 40)).await;
    let mut early = event("co_s", "processing", 41);
    early["attempt"] = json!(2);
    let (_, _, body) = h.send("q2", &early).await;
    assert_eq!(body["resend"], "parent_not_failed");

    assert_eq!(
        h.count("SELECT COUNT(*) FROM payouts WHERE parent_payout_id IS NOT NULL")
            .await,
        2
    );
}

#[tokio::test]
async fn csv_import() {
    let h = harness().await;
    let csv = "\
external_id,attempt,parent_external_id,state,reason,corridor,country,amount_minor,currency,stellar_tx_hash,memo,occurred_at
co_c1,1,,processing,,USDC->GHS,GH,150000,GHS,,co_c1,2026-11-04T19:40:12Z
co_c1,1,,completed,,USDC->GHS,GH,150000,GHS,,co_c1,2026-11-04T19:44:12Z
co_c2,1,,teleported,,USDC->GHS,GH,150000,GHS,,co_c2,2026-11-04T19:44:12Z
";
    let mut req = h.signed("/v1/payouts/import", csv.to_string(), None, &h.secret);
    req.headers_mut()
        .insert("content-type", "text/csv".parse().unwrap());
    let (status, _, body) = call(&h.app, req).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["accepted"], 2);
    assert_eq!(body["rejected"], 1);
    assert_eq!(body["results"][2]["position"], 4);
}

#[tokio::test]
async fn old_secret_still_works_during_rotation() {
    let h = harness().await;
    let admin = h.state.config.admin_token.clone().unwrap();
    let (status, _, body) = call(
        &h.app,
        Request::post(format!(
            "/v1/admin/operators/{}/rotate-secret",
            h.operator_id
        ))
        .header("authorization", format!("Bearer {admin}"))
        .body(Body::empty())
        .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let new_secret = body["hmac_secret"].as_str().unwrap().to_string();

    // Old secret in the secondary header only.
    let payload = event("co_rot", "pending", 40).to_string();
    let ts = chrono::Utc::now().timestamp().to_string();
    let req = Request::post("/v1/payouts/events")
        .header("authorization", format!("Bearer {}", h.api_key))
        .header("Idempotency-Key", "rot-1")
        .header("X-RampWatch-Timestamp", &ts)
        .header(
            "X-RampWatch-Signature-Secondary",
            sign(&h.secret, &ts, payload.as_bytes()),
        )
        .body(Body::from(payload))
        .unwrap();
    assert_eq!(call(&h.app, req).await.0, StatusCode::CREATED);

    // New secret in the primary header.
    let req = h.signed(
        "/v1/payouts/events",
        event("co_rot2", "pending", 41).to_string(),
        Some("rot-2"),
        &new_secret,
    );
    assert_eq!(call(&h.app, req).await.0, StatusCode::CREATED);
}

#[tokio::test]
async fn operator_can_rotate_its_api_key() {
    let h = harness().await;
    let auth = format!("Bearer {}", h.api_key);
    let (status, _, list) = call(
        &h.app,
        Request::get("/v1/api-keys")
            .header("authorization", &auth)
            .body(Body::empty())
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{list}");
    let key_id = list["keys"][0]["id"].as_str().unwrap().to_string();
    assert!(list["keys"][0]["key_prefix"]
        .as_str()
        .unwrap()
        .starts_with("rw_live_"));

    let (status, _, rotated) = call(
        &h.app,
        Request::post(format!("/v1/api-keys/{key_id}/rotate"))
            .header("authorization", &auth)
            .body(Body::empty())
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{rotated}");
    let new_key = rotated["plain_key"].as_str().unwrap();

    let (status, _, _) = call(
        &h.app,
        Request::get("/v1/api-keys")
            .header("authorization", format!("Bearer {new_key}"))
            .body(Body::empty())
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn admin_routes_need_the_admin_token() {
    let h = harness().await;
    let (status, _, _) = call(
        &h.app,
        Request::post("/v1/admin/operators")
            .header("authorization", "Bearer nope")
            .header("content-type", "application/json")
            .body(Body::from(
                json!({"name": "x", "stellar_accounts": [ACCOUNT]}).to_string(),
            ))
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}
