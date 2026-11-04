//! RampWatch API server. Startup and graceful shutdown follow
//! stellar-insights `main.rs`, rewired to RampWatch modules.

use std::net::SocketAddr;
use std::time::{Duration, Instant};

use rampwatch::{api, config::Config, env_config, shutdown, AppState};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let dotenv_status = env_config::load_dotenv()?;
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,sqlx=warn")),
        )
        .init();
    env_config::log_dotenv_status(&dotenv_status);
    env_config::validate_env()?;

    let config = Config::from_env()?;
    tracing::info!(?config, "starting RampWatch");
    if config.admin_token.is_none() {
        tracing::warn!("ADMIN_TOKEN is not set: /v1/admin routes are disabled");
    }

    let port = config.port;
    let state = AppState::new(config).await?;
    let pool = state.pool.clone();
    let app = api::router(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("listening on {addr}");

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown::shutdown_signal())
    .await?;

    let started = Instant::now();
    shutdown::shutdown_database(pool, Duration::from_secs(10)).await;
    shutdown::log_shutdown_summary(started);
    Ok(())
}
