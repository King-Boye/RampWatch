//! SQLite pool and migrations. Adapted from stellar-insights `backend/src/database.rs`
//! (pool options only; its queries are not used here).

use anyhow::{Context, Result};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous};
use sqlx::SqlitePool;
use std::str::FromStr;
use std::time::Duration;

/// How long a connection waits for SQLite's single write lock before failing.
/// SQLite's default is 0, so concurrent writers would fail at once with
/// "database is locked".
const BUSY_TIMEOUT: Duration = Duration::from_secs(5);

fn connect_options(database_url: &str) -> Result<SqliteConnectOptions> {
    let opts = SqliteConnectOptions::from_str(database_url)
        .context("Failed to parse DATABASE_URL for SQLite connection")?
        .create_if_missing(true)
        .foreign_keys(true)
        .busy_timeout(BUSY_TIMEOUT);

    // WAL lets readers run while the intake path writes. In-memory databases
    // don't support it.
    if database_url.contains(":memory:") {
        Ok(opts)
    } else {
        Ok(opts
            .journal_mode(SqliteJournalMode::Wal)
            .synchronous(SqliteSynchronous::Normal))
    }
}

/// Open the pool and run migrations.
pub async fn connect(database_url: &str) -> Result<SqlitePool> {
    let in_memory = database_url.contains(":memory:");
    let pool = SqlitePoolOptions::new()
        // Each in-memory connection is its own database, so tests use one.
        .max_connections(if in_memory { 1 } else { 10 })
        .min_connections(1)
        .acquire_timeout(Duration::from_secs(10))
        // Keep the single in-memory connection alive for the whole test.
        .idle_timeout(if in_memory { None } else { Some(Duration::from_secs(600)) })
        .max_lifetime(if in_memory { None } else { Some(Duration::from_secs(1800)) })
        .connect_with(connect_options(database_url)?)
        .await
        .context("Failed to create SQLite connection pool")?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .context("Failed to run database migrations")?;

    Ok(pool)
}

/// Current time as the RFC 3339 string format used in every table.
#[must_use]
pub fn now_str() -> String {
    ts(chrono::Utc::now())
}

/// Format a timestamp the way every table stores it.
#[must_use]
pub fn ts(t: chrono::DateTime<chrono::Utc>) -> String {
    t.to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
}
