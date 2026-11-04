//! Typed environment configuration for the spec's section 11 variables. Startup
//! validation itself is the copied `env_config.rs` (stellar-insights).

use anyhow::{bail, Context, Result};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StellarNetwork {
    Testnet,
    Mainnet,
}

impl StellarNetwork {
    fn parse(value: &str) -> Result<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "testnet" => Ok(Self::Testnet),
            "mainnet" | "public" => Ok(Self::Mainnet),
            other => bail!("STELLAR_NETWORK must be testnet or mainnet, got '{other}'"),
        }
    }
}

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub stellar_network: StellarNetwork,
    pub horizon_url: String,
    pub soroban_rpc_url: String,
    pub registry_contract_id: Option<String>,
    pub service_secret_key: Option<String>,
    pub telegram_bot_token: Option<String>,
    pub slack_webhook_url: Option<String>,
    pub admin_token: Option<String>,
    pub report_tz: String,
    pub port: u16,
    /// Allowed age of an `X-RampWatch-Timestamp` (fluxapay default: 300s).
    pub webhook_timestamp_tolerance_secs: i64,
}

// Secrets are never printed.
impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let set = |v: &Option<String>| if v.is_some() { "<set>" } else { "<unset>" };
        f.debug_struct("Config")
            .field("database_url", &self.database_url)
            .field("stellar_network", &self.stellar_network)
            .field("horizon_url", &self.horizon_url)
            .field("soroban_rpc_url", &self.soroban_rpc_url)
            .field("registry_contract_id", &self.registry_contract_id)
            .field("service_secret_key", &set(&self.service_secret_key))
            .field("telegram_bot_token", &set(&self.telegram_bot_token))
            .field("slack_webhook_url", &set(&self.slack_webhook_url))
            .field("admin_token", &set(&self.admin_token))
            .field("report_tz", &self.report_tz)
            .field("port", &self.port)
            .field(
                "webhook_timestamp_tolerance_secs",
                &self.webhook_timestamp_tolerance_secs,
            )
            .finish()
    }
}

fn optional(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
}

fn with_default(name: &str, default: &str) -> String {
    optional(name).unwrap_or_else(|| default.to_string())
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let stellar_network = StellarNetwork::parse(&with_default("STELLAR_NETWORK", "testnet"))?;
        let (default_horizon, default_rpc) = match stellar_network {
            StellarNetwork::Testnet => (
                "https://horizon-testnet.stellar.org",
                "https://soroban-testnet.stellar.org",
            ),
            StellarNetwork::Mainnet => (
                "https://horizon.stellar.org",
                "https://soroban-rpc.mainnet.stellar.gateway.fm",
            ),
        };

        let port = with_default("PORT", "8080")
            .parse::<u16>()
            .context("PORT must be a number between 1 and 65535")?;

        let webhook_timestamp_tolerance_secs = match optional("WEBHOOK_TIMESTAMP_TOLERANCE_SECONDS")
        {
            Some(v) => v
                .parse::<i64>()
                .ok()
                .filter(|n| *n > 0)
                .context("WEBHOOK_TIMESTAMP_TOLERANCE_SECONDS must be a positive number")?,
            None => 300,
        };

        let admin_token = optional("ADMIN_TOKEN");
        if let Some(token) = &admin_token {
            if token.len() < 32 {
                bail!("ADMIN_TOKEN must be at least 32 characters");
            }
        }

        Ok(Self {
            database_url: with_default("DATABASE_URL", "sqlite://rampwatch.db"),
            stellar_network,
            horizon_url: with_default("HORIZON_URL", default_horizon),
            soroban_rpc_url: with_default("SOROBAN_RPC_URL", default_rpc),
            registry_contract_id: optional("REGISTRY_CONTRACT_ID"),
            service_secret_key: optional("SERVICE_SECRET_KEY"),
            telegram_bot_token: optional("TELEGRAM_BOT_TOKEN"),
            slack_webhook_url: optional("SLACK_WEBHOOK_URL"),
            admin_token,
            report_tz: with_default("REPORT_TZ", "Africa/Lagos"),
            port,
            webhook_timestamp_tolerance_secs,
        })
    }

    /// A config for tests: in-memory database, fixed admin token.
    #[must_use]
    pub fn for_tests() -> Self {
        Self {
            database_url: "sqlite::memory:".to_string(),
            stellar_network: StellarNetwork::Testnet,
            horizon_url: "https://horizon-testnet.stellar.org".to_string(),
            soroban_rpc_url: "https://soroban-testnet.stellar.org".to_string(),
            registry_contract_id: None,
            service_secret_key: None,
            telegram_bot_token: None,
            slack_webhook_url: None,
            admin_token: Some("test-admin-token-0123456789abcdef0123".to_string()),
            report_tz: "Africa/Lagos".to_string(),
            port: 0,
            webhook_timestamp_tolerance_secs: 300,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn network_parsing() {
        assert_eq!(
            StellarNetwork::parse("Testnet").ok(),
            Some(StellarNetwork::Testnet)
        );
        assert_eq!(
            StellarNetwork::parse("mainnet").ok(),
            Some(StellarNetwork::Mainnet)
        );
        assert!(StellarNetwork::parse("futurenet").is_err());
    }

    #[test]
    fn debug_hides_secrets() {
        let mut cfg = Config::for_tests();
        cfg.service_secret_key = Some("SDONOTPRINT".to_string());
        let printed = format!("{cfg:?}");
        assert!(!printed.contains("SDONOTPRINT"));
        assert!(!printed.contains("test-admin-token"));
    }
}
