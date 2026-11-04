use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::sync::Arc;

use crate::auth::api_key_db::ApiKeyDb;
use crate::auth::api_key_model::CreateApiKeyRequest;
use crate::domain::models::Operator;
use crate::validation::ValidatedJson;

/// The key owner is the operator proven by the caller's current API key,
/// which `operator_auth_middleware` attaches to the request.
fn operator_id(operator: Option<Extension<Arc<Operator>>>) -> Result<String, ApiKeyError> {
    operator
        .map(|Extension(op)| op.id.clone())
        .ok_or_else(|| ApiKeyError::Unauthorized("An operator API key is required".to_string()))
}

/// POST /v1/api-keys - Create a new API key
pub async fn create_api_key(
    State(db): State<Arc<ApiKeyDb>>,
    operator: Option<Extension<Arc<Operator>>>,
    ValidatedJson(req): ValidatedJson<CreateApiKeyRequest>,
) -> Result<Response, ApiKeyError> {
    let operator_id = operator_id(operator)?;

    let name = req.name.trim();
    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == ' ')
    {
        return Err(ApiKeyError::BadRequest(
            "Key name may only contain letters, digits, spaces, hyphens, and underscores"
                .to_string(),
        ));
    }

    let response = db
        .create_api_key(&operator_id, req)
        .await
        .map_err(|e| ApiKeyError::ServerError(e.to_string()))?;

    Ok((StatusCode::CREATED, Json(json!(response))).into_response())
}

/// GET /v1/api-keys - List all API keys for the authenticated user
pub async fn list_api_keys(
    State(db): State<Arc<ApiKeyDb>>,
    operator: Option<Extension<Arc<Operator>>>,
) -> Result<Response, ApiKeyError> {
    let operator_id = operator_id(operator)?;

    let keys = db
        .list_api_keys(&operator_id)
        .await
        .map_err(|e| ApiKeyError::ServerError(e.to_string()))?;

    Ok((StatusCode::OK, Json(json!({ "keys": keys }))).into_response())
}

/// GET /v1/api-keys/{id} - Get a specific API key by ID
pub async fn get_api_key(
    State(db): State<Arc<ApiKeyDb>>,
    operator: Option<Extension<Arc<Operator>>>,
    Path(id): Path<String>,
) -> Result<Response, ApiKeyError> {
    let operator_id = operator_id(operator)?;

    let key = db
        .get_api_key_by_id(&id, &operator_id)
        .await
        .map_err(|e| ApiKeyError::ServerError(e.to_string()))?;

    match key {
        Some(k) => Ok((StatusCode::OK, Json(json!(k))).into_response()),
        None => Err(ApiKeyError::NotFound("API key not found".to_string())),
    }
}

/// POST /v1/api-keys/{id}/rotate - Rotate an API key
pub async fn rotate_api_key(
    State(db): State<Arc<ApiKeyDb>>,
    operator: Option<Extension<Arc<Operator>>>,
    Path(id): Path<String>,
) -> Result<Response, ApiKeyError> {
    let operator_id = operator_id(operator)?;

    let response = db
        .rotate_api_key(&id, &operator_id)
        .await
        .map_err(|e| ApiKeyError::ServerError(e.to_string()))?;

    match response {
        Some(r) => Ok((StatusCode::OK, Json(json!(r))).into_response()),
        None => Err(ApiKeyError::NotFound(
            "API key not found or already revoked".to_string(),
        )),
    }
}

/// DELETE /v1/api-keys/{id} - Revoke an API key
pub async fn revoke_api_key(
    State(db): State<Arc<ApiKeyDb>>,
    operator: Option<Extension<Arc<Operator>>>,
    Path(id): Path<String>,
) -> Result<Response, ApiKeyError> {
    let operator_id = operator_id(operator)?;

    let revoked = db
        .revoke_api_key(&id, &operator_id)
        .await
        .map_err(|e| ApiKeyError::ServerError(e.to_string()))?;

    if revoked {
        Ok((
            StatusCode::OK,
            Json(json!({ "message": "API key revoked successfully" })),
        )
            .into_response())
    } else {
        Err(ApiKeyError::NotFound(
            "API key not found or already revoked".to_string(),
        ))
    }
}

#[derive(Debug)]
pub enum ApiKeyError {
    NotFound(String),
    BadRequest(String),
    Unauthorized(String),
    ServerError(String),
}

impl IntoResponse for ApiKeyError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            Self::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            Self::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            Self::ServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

/// API key management for the calling operator. Mounted behind
/// `operator_auth_middleware` (see `api::router`).
pub fn routes<S: Clone + Send + Sync + 'static>(db: Arc<ApiKeyDb>) -> axum::Router<S> {
    use axum::routing::{get, post};
    axum::Router::new()
        .route("/", get(list_api_keys).post(create_api_key))
        .route("/{id}", get(get_api_key).delete(revoke_api_key))
        .route("/{id}/rotate", post(rotate_api_key))
        .with_state(db)
}
