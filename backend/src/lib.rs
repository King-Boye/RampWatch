//! RampWatch: health monitoring for Stellar off-ramp and payout rails.

pub mod api;
pub mod auth;
pub mod config;
pub mod db;
pub mod domain;
pub mod env_config;
pub mod error;
pub mod intake;
pub mod shutdown;
pub mod validation;

use std::sync::Arc;

use api::middleware::rate_limit::RateLimiter;
use config::Config;

/// Shared state for every handler.
#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::SqlitePool,
    pub config: Arc<Config>,
    pub limiter: Arc<RateLimiter>,
}

impl AppState {
    /// Connect the database (running migrations) and build the state.
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        let pool = db::connect(&config.database_url).await?;
        let limiter = Arc::new(RateLimiter::new_with_db(Some(pool.clone())).await?);
        Ok(Self {
            pool,
            config: Arc::new(config),
            limiter,
        })
    }

    /// Same as [`AppState::new`] but never tries Redis. For tests.
    pub async fn new_without_redis(config: Config) -> anyhow::Result<Self> {
        let pool = db::connect(&config.database_url).await?;
        let limiter = Arc::new(RateLimiter::new_memory_only(Some(pool.clone())));
        Ok(Self {
            pool,
            config: Arc::new(config),
            limiter,
        })
    }
}

/// Serialises tests that set process environment variables (copied from
/// stellar-insights `lib.rs`).
#[cfg(test)]
pub(crate) fn lock_env_test() -> std::sync::MutexGuard<'static, ()> {
    use std::sync::{Mutex, OnceLock};
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}
