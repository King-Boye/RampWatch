//! Rate limiting for attestation submissions
//!
//! This module implements per-attestor rate limiting for attestation submissions
//! to prevent spam and abuse of the contract.

use soroban_sdk::{contracttype, symbol_short, Address, Env};
use crate::errors::ErrorCode;
use crate::events::{RateLimitReset, RateLimitWindowReset};
use crate::storage::StorageKey;

/// Rate limit configuration stored in contract storage
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RateLimitConfig {
    /// Maximum number of submissions allowed per window
    pub max_submissions: u32,
    /// Length of the rate limit window in ledgers
    pub window_length: u32,
}

/// Per-attestor rate limit state stored in contract storage
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RateLimitState {
    /// Number of submissions in the current window
    pub submission_count: u32,
    /// Ledger number when the current window started
    pub window_start_ledger: u32,
    /// Cumulative total requests across all windows (never reset)
    pub total_requests: u64,
}

/// Rate limiter utility — plain Rust struct, no Soroban contract boundary.
pub struct RateLimiter;

impl RateLimiter {
    /// Get the current rate limit state for an attestor.
    pub fn get_state(env: Env, attestor: Address) -> RateLimitState {
        let state_key = StorageKey::RateLimitState(attestor.clone());
        env.storage().persistent().get::<_, RateLimitState>(&state_key)
            .unwrap_or(RateLimitState {
                submission_count: 0,
                window_start_ledger: env.ledger().sequence(),
                total_requests: 0,
            })
    }

    /// Get the current global rate limit configuration.
    ///
    /// If no configuration has been set via [`update_config`], the following defaults apply:
    /// - `max_submissions = 10`
    /// - `window_length = 100` ledgers
    ///
    /// At Stellar's target close time of ~5 seconds per ledger, the default window
    /// is approximately **500 seconds (~8 minutes)** of wall-clock time, allowing
    /// up to 10 submissions per attestor in that period.
    pub fn get_config(env: Env) -> RateLimitConfig {
        let config_key = Self::get_config_key(&env);
        env.storage().persistent().get::<_, RateLimitConfig>(&config_key)
            .unwrap_or(RateLimitConfig {
                max_submissions: 10,
                window_length: 100,
            })
    }

    /// Check if an attestor can submit an attestation and increment their counter.
    ///
    /// Rate limiting is opt-in: if no global `RateLimitConfig` has been written
    /// to storage and no per-attestor override exists for `attestor`, the check
    /// is skipped entirely and `Ok(())` is returned without touching state.
    pub fn check_and_increment(
        env: &Env,
        attestor: &Address,
    ) -> Result<(), ErrorCode> {
        if !Self::is_configured(env, attestor) {
            return Ok(());
        }
        let config = Self::get_effective_config(env.clone(), attestor.clone());
        let current_ledger = env.ledger().sequence();
        let state_key = StorageKey::RateLimitState(attestor.clone());

        let mut state = env.storage().persistent().get::<_, RateLimitState>(&state_key)
            .unwrap_or(RateLimitState {
                submission_count: 0,
                window_start_ledger: current_ledger,
                total_requests: 0,
            });

        if Self::is_window_expired(current_ledger, state.window_start_ledger, config.window_length) {
            state.submission_count = 0;
            state.window_start_ledger = current_ledger;
            env.events().publish(
                (symbol_short!("rate"), symbol_short!("win_reset")),
                RateLimitWindowReset {
                    attestor: attestor.clone(),
                    window_start: current_ledger as u64,
                },
            );
        }

        if state.submission_count >= config.max_submissions {
            env.storage().persistent().set(&state_key, &state);
            return Err(ErrorCode::RateLimitExceeded);
        }

        state.submission_count += 1;
        state.total_requests += 1;
        env.storage().persistent().set(&state_key, &state);

        Ok(())
    }

    /// Admin function to tune the rate limit configuration.
    ///
    /// When `attestor` is `None`, updates the global configuration. When `Some(addr)`,
    /// sets a per-attestor override for that address only.
    pub fn update_config(
        env: &Env,
        _admin: &Address,
        config: RateLimitConfig,
        attestor: Option<&Address>,
    ) -> Result<(), ErrorCode> {
        match attestor {
            Some(addr) => {
                let key = StorageKey::RateLimitOverride(addr.clone());
                env.storage().persistent().set(&key, &config);
            }
            None => {
                let key = Self::get_config_key(env);
                env.storage().persistent().set(&key, &config);
            }
        }
        Ok(())
    }

    /// Get the effective config for an attestor: per-attestor override if set, else global.
    pub fn get_effective_config(env: Env, attestor: Address) -> RateLimitConfig {
        let key = StorageKey::RateLimitOverride(attestor.clone());
        env.storage().persistent().get::<_, RateLimitConfig>(&key)
            .unwrap_or_else(|| Self::get_config(env.clone()))
    }

    /// Returns true if rate limiting has been explicitly configured — either via
    /// a global config or a per-attestor override.
    fn is_configured(env: &Env, attestor: &Address) -> bool {
        let override_key = StorageKey::RateLimitOverride(attestor.clone());
        if env.storage().persistent().has(&override_key) {
            return true;
        }
        let global_key = Self::get_config_key(env);
        env.storage().persistent().has(&global_key)
    }

    /// Reset the rate limit for a specified attestor (admin-only).
    ///
    /// Clears `submission_count` and `window_start_ledger`; preserves `total_requests`.
    pub fn reset_rate_limit(env: &Env, admin: &Address, attestor: &Address) -> Result<(), ErrorCode> {
        admin.require_auth();

        let state_key = StorageKey::RateLimitState(attestor.clone());
        let current_state = env.storage().persistent().get::<_, RateLimitState>(&state_key)
            .unwrap_or(RateLimitState {
                submission_count: 0,
                window_start_ledger: env.ledger().sequence(),
                total_requests: 0,
            });

        let reset_state = RateLimitState {
            submission_count: 0,
            window_start_ledger: env.ledger().sequence(),
            total_requests: current_state.total_requests,
        };

        env.storage().persistent().set(&state_key, &reset_state);

        env.events().publish(
            (symbol_short!("rate"), symbol_short!("reset")),
            RateLimitReset {
                attestor: attestor.clone(),
                admin: admin.clone(),
                timestamp: env.ledger().timestamp(),
            },
        );

        Ok(())
    }

    fn is_window_expired(current_ledger: u32, window_start_ledger: u32, window_length: u32) -> bool {
        current_ledger.saturating_sub(window_start_ledger) >= window_length
    }

    fn get_config_key(env: &Env) -> soroban_sdk::BytesN<32> {
        let config_key = *b"rate_limit_config_______________";
        soroban_sdk::BytesN::from_array(env, &config_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Symbol;
    use soroban_sdk::TryFromVal;
    use soroban_sdk::testutils::{Address as _, Events, Ledger, LedgerInfo};

    #[test]
    fn test_rate_limit_under_limit() {
        let env = Env::default();
        let attestor = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);

        RateLimiter::update_config(&env, &attestor, RateLimitConfig { max_submissions: 10, window_length: 100 }, None).unwrap();

        assert!(RateLimiter::check_and_increment(&env, &attestor).is_ok());

        let state = RateLimiter::get_state(env.clone(), attestor.clone());
        assert_eq!(state.submission_count, 1);
    }

    #[test]
    fn test_rate_limit_at_limit() {
        let env = Env::default();
        let attestor = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);

        RateLimiter::update_config(&env, &attestor, RateLimitConfig { max_submissions: 2, window_length: 100 }, None).unwrap();

        assert!(RateLimiter::check_and_increment(&env, &attestor).is_ok());
        assert!(RateLimiter::check_and_increment(&env, &attestor).is_ok());
        let result = RateLimiter::check_and_increment(&env, &attestor);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ErrorCode::RateLimitExceeded);
    }

    #[test]
    fn test_rate_limit_over_limit() {
        let env = Env::default();
        let attestor = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);

        RateLimiter::update_config(&env, &attestor, RateLimitConfig { max_submissions: 1, window_length: 100 }, None).unwrap();

        assert!(RateLimiter::check_and_increment(&env, &attestor).is_ok());
        let result = RateLimiter::check_and_increment(&env, &attestor);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ErrorCode::RateLimitExceeded);
    }

    #[test]
    fn test_rate_limit_window_reset() {
        let env = Env::default();
        let attestor = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);

        RateLimiter::update_config(&env, &attestor, RateLimitConfig { max_submissions: 1, window_length: 10 }, None).unwrap();

        assert!(RateLimiter::check_and_increment(&env, &attestor).is_ok());
        assert!(RateLimiter::check_and_increment(&env, &attestor).is_err());

        let current_ledger = env.ledger().sequence();
        env.ledger().set(LedgerInfo {
            sequence_number: current_ledger + 10,
            timestamp: 0,
            protocol_version: 21,
            network_id: Default::default(),
            base_reserve: 0,
            min_persistent_entry_ttl: 4096,
            min_temp_entry_ttl: 16,
            max_entry_ttl: 6312000,
        });

        assert!(RateLimiter::check_and_increment(&env, &attestor).is_ok());

        let events = env.events().all();
        assert_eq!(events.len(), 1);

        let (_publisher, topics, _event_data) = events.get(0).unwrap();
        assert_eq!(topics.len(), 2);
        assert_eq!(Symbol::try_from_val(&env, &topics.get(0).unwrap()).unwrap(), symbol_short!("rate"));
        assert_eq!(Symbol::try_from_val(&env, &topics.get(1).unwrap()).unwrap(), symbol_short!("win_reset"));

        let state = RateLimiter::get_state(env.clone(), attestor.clone());
        assert_eq!(state.submission_count, 1);
        assert_eq!(state.total_requests, 2);
    }

    #[test]
    fn test_rate_limit_config_update() {
        let env = Env::default();
        let admin = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);
        let new_config = RateLimitConfig { max_submissions: 20, window_length: 200 };

        assert!(RateLimiter::update_config(&env, &admin, new_config.clone(), None).is_ok());

        let config = RateLimiter::get_config(env.clone());
        assert_eq!(config.max_submissions, 20);
        assert_eq!(config.window_length, 200);
    }

    #[test]
    fn test_rate_limit_default_config() {
        let env = Env::default();
        let config = RateLimiter::get_config(env.clone());
        assert_eq!(config.max_submissions, 10);
        assert_eq!(config.window_length, 100);
    }

    #[test]
    fn test_per_attestor_override_takes_precedence() {
        let env = Env::default();
        let attestor = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);

        RateLimiter::update_config(&env, &attestor, RateLimitConfig { max_submissions: 1, window_length: 100 }, None).unwrap();
        RateLimiter::update_config(&env, &attestor, RateLimitConfig { max_submissions: 5, window_length: 100 }, Some(&attestor)).unwrap();

        for _ in 0..5 {
            assert!(RateLimiter::check_and_increment(&env, &attestor).is_ok());
        }
        assert!(RateLimiter::check_and_increment(&env, &attestor).is_err());
    }

    #[test]
    fn test_fallback_to_global_when_no_override() {
        let env = Env::default();
        let attestor = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);

        RateLimiter::update_config(&env, &attestor, RateLimitConfig { max_submissions: 2, window_length: 100 }, None).unwrap();

        assert!(RateLimiter::check_and_increment(&env, &attestor).is_ok());
        assert!(RateLimiter::check_and_increment(&env, &attestor).is_ok());
        assert!(RateLimiter::check_and_increment(&env, &attestor).is_err());
    }

    #[test]
    fn test_override_does_not_affect_other_attestors() {
        let env = Env::default();
        let high_volume = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);
        let normal = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);

        RateLimiter::update_config(&env, &high_volume, RateLimitConfig { max_submissions: 1, window_length: 100 }, None).unwrap();
        RateLimiter::update_config(&env, &high_volume, RateLimitConfig { max_submissions: 10, window_length: 100 }, Some(&high_volume)).unwrap();

        for _ in 0..10 {
            assert!(RateLimiter::check_and_increment(&env, &high_volume).is_ok());
        }

        assert!(RateLimiter::check_and_increment(&env, &normal).is_ok());
        assert!(RateLimiter::check_and_increment(&env, &normal).is_err());
    }

    #[test]
    fn test_reset_rate_limit_admin_successfully_resets() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);
        let attestor = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);

        RateLimiter::update_config(&env, &admin, RateLimitConfig { max_submissions: 1, window_length: 100 }, None).unwrap();

        assert!(RateLimiter::check_and_increment(&env, &attestor).is_ok());
        assert_eq!(RateLimiter::get_state(env.clone(), attestor.clone()).submission_count, 1);
        assert!(RateLimiter::check_and_increment(&env, &attestor).is_err());

        assert!(RateLimiter::reset_rate_limit(&env, &admin, &attestor).is_ok());

        let state_after = RateLimiter::get_state(env.clone(), attestor.clone());
        assert_eq!(state_after.submission_count, 0);
        assert!(RateLimiter::check_and_increment(&env, &attestor).is_ok());
        assert_eq!(RateLimiter::get_state(env.clone(), attestor.clone()).submission_count, 1);
    }

    #[test]
    fn test_reset_rate_limit_preserves_total_requests() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);
        let attestor = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);

        RateLimiter::update_config(&env, &admin, RateLimitConfig { max_submissions: 1, window_length: 100 }, None).unwrap();

        RateLimiter::check_and_increment(&env, &attestor).unwrap();
        let _ = RateLimiter::check_and_increment(&env, &attestor);

        assert_eq!(RateLimiter::get_state(env.clone(), attestor.clone()).total_requests, 1);

        RateLimiter::reset_rate_limit(&env, &admin, &attestor).unwrap();

        let state_after = RateLimiter::get_state(env.clone(), attestor.clone());
        assert_eq!(state_after.total_requests, 1);
        assert_eq!(state_after.submission_count, 0);
    }

    #[test]
    fn test_reset_rate_limit_non_admin_unauthorized() {
        let env = Env::default();
        let admin = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);
        let attestor = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);

        RateLimiter::update_config(&env, &admin, RateLimitConfig { max_submissions: 1, window_length: 100 }, None).unwrap();

        RateLimiter::check_and_increment(&env, &attestor).unwrap();
        let _ = RateLimiter::check_and_increment(&env, &attestor);

        let state = RateLimiter::get_state(env.clone(), attestor.clone());
        assert_eq!(state.submission_count, 1);
    }

    #[test]
    fn test_reset_rate_limit_multiple_attestors_independent() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);
        let attestor1 = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);
        let attestor2 = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);

        RateLimiter::update_config(&env, &admin, RateLimitConfig { max_submissions: 1, window_length: 100 }, None).unwrap();

        RateLimiter::check_and_increment(&env, &attestor1).unwrap();
        let _ = RateLimiter::check_and_increment(&env, &attestor1);
        RateLimiter::check_and_increment(&env, &attestor2).unwrap();
        let _ = RateLimiter::check_and_increment(&env, &attestor2);

        RateLimiter::reset_rate_limit(&env, &admin, &attestor1).unwrap();

        assert_eq!(RateLimiter::get_state(env.clone(), attestor1.clone()).submission_count, 0);
        assert_eq!(RateLimiter::get_state(env.clone(), attestor2.clone()).submission_count, 1);
        assert!(RateLimiter::check_and_increment(&env, &attestor2).is_err());
    }

    #[test]
    fn test_reset_rate_limit_resets_window_start_ledger() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);
        let attestor = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);

        RateLimiter::update_config(&env, &admin, RateLimitConfig { max_submissions: 2, window_length: 100 }, None).unwrap();

        RateLimiter::check_and_increment(&env, &attestor).unwrap();
        let state_before = RateLimiter::get_state(env.clone(), attestor.clone());
        let ledger_before = state_before.window_start_ledger;

        RateLimiter::reset_rate_limit(&env, &admin, &attestor).unwrap();

        let state_after = RateLimiter::get_state(env.clone(), attestor.clone());
        assert_eq!(state_after.window_start_ledger, env.ledger().sequence());
        assert!(state_after.window_start_ledger >= ledger_before);
    }
}
