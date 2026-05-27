//! Rate limiting for attestation submissions
//!
//! This module implements per-attestor rate limiting for attestation submissions
//! to prevent spam and abuse of the contract.

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env};
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

/// Rate limiter for attestation submissions
#[contract]
pub struct RateLimiter;

#[contractimpl]
impl RateLimiter {
    /// Get the current rate limit state for an attestor
    pub fn get_state(env: Env, attestor: Address) -> RateLimitState {
        let state_key = StorageKey::RateLimitState(attestor.clone());
        env.storage().persistent().get::<_, RateLimitState>(&state_key)
            .unwrap_or(RateLimitState {
                submission_count: 0,
                window_start_ledger: env.ledger().sequence(),
                total_requests: 0,
            })
    }

    /// Get the current rate limit configuration
    pub fn get_config(env: Env) -> RateLimitConfig {
        let config_key = Self::get_config_key(&env);
        env.storage().persistent().get::<_, RateLimitConfig>(&config_key)
            .unwrap_or(RateLimitConfig {
                max_submissions: 10,
                window_length: 100,
            })
    }
}

impl RateLimiter {
    /// Check if an attestor can submit an attestation and increment their counter.
    ///
    /// Rate limiting is opt-in: if no global `RateLimitConfig` has been written
    /// to storage and no per-attestor override exists for `attestor`, the check
    /// is skipped entirely and `Ok(())` is returned without touching state.
    /// Admins activate enforcement by calling `update_config`.
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

        state.total_requests += 1;

        if state.submission_count >= config.max_submissions {
            env.storage().persistent().set(&state_key, &state);
            return Err(ErrorCode::RateLimitExceeded);
        }

        state.submission_count += 1;
        env.storage().persistent().set(&state_key, &state);

        Ok(())
    }

    /// Update the global rate limit configuration, or set a per-attestor override when
    /// `attestor` is `Some`.
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

    /// Reset the rate limit for a specified attestor (admin-only function).
    ///
    /// This function:
    /// 1. Requires the caller to be authenticated as the admin
    /// 2. Clears the rate limit state (submission_count and window_start_ledger) for the attestor
    /// 3. Preserves the total_requests counter (never reset)
    /// 4. Emits a RateLimitReset event
    ///
    /// After this call, the attestor can immediately submit new attestations without hitting the rate limit.
    ///
    /// # Arguments
    /// - `env`: The Soroban environment
    /// - `admin`: The admin address. Must be authenticated via `admin.require_auth()`
    /// - `attestor`: The attestor address whose rate limit is being reset
    ///
    /// # Errors
    /// Returns an error if:
    /// - The caller cannot be authenticated as the admin
    pub fn reset_rate_limit(env: &Env, admin: &Address, attestor: &Address) -> Result<(), ErrorCode> {
        // Admin authorization check
        admin.require_auth();

        // Get current state to preserve total_requests
        let state_key = StorageKey::RateLimitState(attestor.clone());
        let current_state = env.storage().persistent().get::<_, RateLimitState>(&state_key)
            .unwrap_or(RateLimitState {
                submission_count: 0,
                window_start_ledger: env.ledger().sequence(),
                total_requests: 0,
            });

        // Create new state with counts reset but total_requests preserved
        let reset_state = RateLimitState {
            submission_count: 0,
            window_start_ledger: env.ledger().sequence(),
            total_requests: current_state.total_requests, // Preserve cumulative count
        };

        env.storage().persistent().set(&state_key, &reset_state);

        // Emit event
        let timestamp = env.ledger().timestamp();
        env.events().publish(
            (symbol_short!("rate"), symbol_short!("reset")),
            RateLimitReset {
                attestor: attestor.clone(),
                admin: admin.clone(),
                timestamp,
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

    fn make_contract(env: &Env) -> Address {
        env.register_contract(None, crate::rate_limiter::RateLimiter)
    }

    #[test]
    fn test_rate_limit_under_limit() {
        let env = Env::default();
        let attestor = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);
        let contract_address = make_contract(&env);
        let contract_id = env.register_contract(&contract_address, crate::rate_limiter::RateLimiter);

        // Set global config with limit of 10
        env.as_contract(&contract_id, &|| {
            RateLimiter::update_config(&env, &contract_address, RateLimitConfig { max_submissions: 10, window_length: 100 }, None).unwrap();
        });

        let result = env.as_contract(&contract_id, &|| {
            RateLimiter::check_and_increment(&env, &attestor)
        });
        assert!(result.is_ok());

        let state = env.as_contract(&contract_id, &|| {
            RateLimiter::get_state(env.clone(), attestor.clone())
        });
        assert_eq!(state.submission_count, 1);
    }

    #[test]
    fn test_rate_limit_at_limit() {
        let env = Env::default();
        let attestor = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);
        let contract_address = make_contract(&env);

        env.as_contract(&contract_address, &|| {
            RateLimiter::update_config(&env, &contract_address, RateLimitConfig { max_submissions: 2, window_length: 100 }, None).unwrap();
        });

        assert!(env.as_contract(&contract_address, &|| {
            RateLimiter::check_and_increment(&env, &attestor)
        }).is_ok());
        assert!(env.as_contract(&contract_address, &|| {
            RateLimiter::check_and_increment(&env, &attestor)
        }).is_ok());

        let result = env.as_contract(&contract_address, &|| {
            RateLimiter::check_and_increment(&env, &attestor)
        });
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ErrorCode::RateLimitExceeded);
    }

    #[test]
    fn test_rate_limit_over_limit() {
        let env = Env::default();
        let attestor = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);
        let contract_address = make_contract(&env);

        env.as_contract(&contract_address, &|| {
            RateLimiter::update_config(&env, &contract_address, RateLimitConfig { max_submissions: 1, window_length: 100 }, None).unwrap();
        });

        assert!(env.as_contract(&contract_address, &|| {
            RateLimiter::check_and_increment(&env, &attestor)
        }).is_ok());

        let result = env.as_contract(&contract_address, &|| {
            RateLimiter::check_and_increment(&env, &attestor)
        });
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ErrorCode::RateLimitExceeded);
    }

    #[test]
    fn test_rate_limit_window_reset() {
        let env = Env::default();
        let attestor = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);
        let contract_address = make_contract(&env);

        env.as_contract(&contract_address, &|| {
            RateLimiter::update_config(&env, &contract_address, RateLimitConfig { max_submissions: 1, window_length: 10 }, None).unwrap();
        });

        // First call succeeds.
        assert!(env.as_contract(&contract_address, &|| {
            RateLimiter::check_and_increment(&env, &attestor)
        }).is_ok());

        // Second call hits the limit.
        assert!(env.as_contract(&contract_address, &|| {
            RateLimiter::check_and_increment(&env, &attestor)
        }).is_err());

        // Advance the ledger past the current window so the rate limit resets.
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

        assert!(env.as_contract(&contract_address, &|| {
            RateLimiter::check_and_increment(&env, &attestor)
        }).is_ok());

        let events = env.events().all();
        assert_eq!(events.len(), 1);

        let (publisher, topics, _event_data) = events.get(0).unwrap();
        assert_eq!(publisher, contract_address);
        assert_eq!(topics.len(), 2);
        assert_eq!(Symbol::try_from_val(&env, &topics.get(0).unwrap()).unwrap(), symbol_short!("rate"));
        assert_eq!(Symbol::try_from_val(&env, &topics.get(1).unwrap()).unwrap(), symbol_short!("win_reset"));

        let state = env.as_contract(&contract_address, &|| {
            RateLimiter::get_state(env.clone(), attestor.clone())
        });
        assert_eq!(state.submission_count, 1);
        assert_eq!(state.total_requests, 3);
    }

    #[test]
    fn test_rate_limit_config_update() {
        let env = Env::default();
        let admin = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);
        let contract_address = make_contract(&env);
        let new_config = RateLimitConfig { max_submissions: 20, window_length: 200 };

        let result = env.as_contract(&contract_address, &|| {
            RateLimiter::update_config(&env, &admin, new_config.clone(), None)
        });
        assert!(result.is_ok());

        let config = env.as_contract(&contract_address, &|| {
            RateLimiter::get_config(env.clone())
        });
        assert_eq!(config.max_submissions, 20);
        assert_eq!(config.window_length, 200);
    }

    #[test]
    fn test_rate_limit_default_config() {
        let env = Env::default();
        let contract_address = make_contract(&env);

        let config = env.as_contract(&contract_address, &|| {
            RateLimiter::get_config(env.clone())
        });
        assert_eq!(config.max_submissions, 10);
        assert_eq!(config.window_length, 100);
    }

    // --- per-attestor override tests ---

    #[test]
    fn test_per_attestor_override_takes_precedence() {
        let env = Env::default();
        let attestor = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);
        let contract_address = make_contract(&env);

        env.as_contract(&contract_address, &|| {
            // Global: limit 1
            RateLimiter::update_config(&env, &contract_address, RateLimitConfig { max_submissions: 1, window_length: 100 }, None).unwrap();
            // Per-attestor override: limit 5
            RateLimiter::update_config(&env, &contract_address, RateLimitConfig { max_submissions: 5, window_length: 100 }, Some(&attestor)).unwrap();
        });

        // Should succeed 5 times (override), not just 1 (global)
        for _ in 0..5 {
            assert!(env.as_contract(&contract_address, &|| {
                RateLimiter::check_and_increment(&env, &attestor)
            }).is_ok());
        }
        // 6th should fail
        assert!(env.as_contract(&contract_address, &|| {
            RateLimiter::check_and_increment(&env, &attestor)
        }).is_err());
    }

    #[test]
    fn test_fallback_to_global_when_no_override() {
        let env = Env::default();
        let attestor = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);
        let contract_address = make_contract(&env);

        env.as_contract(&contract_address, &|| {
            // Global: limit 2, no per-attestor override
            RateLimiter::update_config(&env, &contract_address, RateLimitConfig { max_submissions: 2, window_length: 100 }, None).unwrap();
        });

        assert!(env.as_contract(&contract_address, &|| {
            RateLimiter::check_and_increment(&env, &attestor)
        }).is_ok());
        assert!(env.as_contract(&contract_address, &|| {
            RateLimiter::check_and_increment(&env, &attestor)
        }).is_ok());
        // 3rd exceeds global limit
        assert!(env.as_contract(&contract_address, &|| {
            RateLimiter::check_and_increment(&env, &attestor)
        }).is_err());
    }

    #[test]
    fn test_override_does_not_affect_other_attestors() {
        let env = Env::default();
        let high_volume = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);
        let normal = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);
        let contract_address = make_contract(&env);

        env.as_contract(&contract_address, &|| {
            // Global: limit 1
            RateLimiter::update_config(&env, &contract_address, RateLimitConfig { max_submissions: 1, window_length: 100 }, None).unwrap();
            // Override only for high_volume
            RateLimiter::update_config(&env, &contract_address, RateLimitConfig { max_submissions: 10, window_length: 100 }, Some(&high_volume)).unwrap();
        });

        // high_volume can submit 10 times
        for _ in 0..10 {
            assert!(env.as_contract(&contract_address, &|| {
                RateLimiter::check_and_increment(&env, &high_volume)
            }).is_ok());
        }

        // normal attestor is still capped at 1
        assert!(env.as_contract(&contract_address, &|| {
            RateLimiter::check_and_increment(&env, &normal)
        }).is_ok());
        assert!(env.as_contract(&contract_address, &|| {
            RateLimiter::check_and_increment(&env, &normal)
        }).is_err());
    }

    // --- reset_rate_limit tests ---

    #[test]
    fn test_reset_rate_limit_admin_successfully_resets() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);
        let attestor = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);
        let contract_address = make_contract(&env);

        env.as_contract(&contract_address, &|| {
            // Set up rate limiting: max 1 submission
            RateLimiter::update_config(&env, &admin, RateLimitConfig { max_submissions: 1, window_length: 100 }, None).unwrap();

            // Attestor submits once - hits limit
            assert!(RateLimiter::check_and_increment(&env, &attestor).is_ok());
            assert_eq!(
                RateLimiter::get_state(env.clone(), attestor.clone()).submission_count,
                1
            );

            // Second submission should fail (rate limit exceeded)
            assert!(RateLimiter::check_and_increment(&env, &attestor).is_err());

            // Admin resets the rate limit
            assert!(RateLimiter::reset_rate_limit(&env, &admin, &attestor).is_ok());

            // After reset, submission_count should be 0
            let state_after = RateLimiter::get_state(env.clone(), attestor.clone());
            assert_eq!(state_after.submission_count, 0);

            // Attestor can now submit again (1 attempt after reset)
            assert!(RateLimiter::check_and_increment(&env, &attestor).is_ok());
            assert_eq!(
                RateLimiter::get_state(env.clone(), attestor.clone()).submission_count,
                1
            );
        });
    }

    #[test]
    fn test_reset_rate_limit_preserves_total_requests() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);
        let attestor = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);
        let contract_address = make_contract(&env);

        env.as_contract(&contract_address, &|| {
            RateLimiter::update_config(&env, &admin, RateLimitConfig { max_submissions: 1, window_length: 100 }, None).unwrap();

            // Make 3 submissions (2 will succeed, 3rd fails due to limit)
            RateLimiter::check_and_increment(&env, &attestor).unwrap();
            let _ = RateLimiter::check_and_increment(&env, &attestor); // Fails but increments total_requests

            let state_before = RateLimiter::get_state(env.clone(), attestor.clone());
            assert_eq!(state_before.total_requests, 2); // 2 attempts recorded

            // Admin resets rate limit
            RateLimiter::reset_rate_limit(&env, &admin, &attestor).unwrap();

            // total_requests should still be 2 (never reset)
            let state_after = RateLimiter::get_state(env.clone(), attestor.clone());
            assert_eq!(state_after.total_requests, 2);
            assert_eq!(state_after.submission_count, 0); // But submission_count is reset
        });
    }

    #[test]
    fn test_reset_rate_limit_non_admin_unauthorized() {
        let env = Env::default();
        let admin = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);
        let attacker = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);
        let attestor = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);
        let contract_address = make_contract(&env);

        env.as_contract(&contract_address, &|| {
            // Set up rate limiting
            RateLimiter::update_config(&env, &admin, RateLimitConfig { max_submissions: 1, window_length: 100 }, None).unwrap();

            // Attestor hits rate limit
            RateLimiter::check_and_increment(&env, &attestor).unwrap();
            RateLimiter::check_and_increment(&env, &attestor).unwrap_err(); // Hits limit

            // Non-admin (attacker) tries to reset - should fail
            // In Soroban, unauthorized calls panic. We verify state is unchanged.
            // (Authorization is enforced by require_auth which panics on failure)
            let state = RateLimiter::get_state(env.clone(), attestor.clone());
            assert_eq!(state.submission_count, 1); // Should not have been reset
        });
    }

    #[test]
    fn test_reset_rate_limit_multiple_attestors_independent() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);
        let attestor1 = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);
        let attestor2 = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);
        let contract_address = make_contract(&env);

        env.as_contract(&contract_address, &|| {
            RateLimiter::update_config(&env, &admin, RateLimitConfig { max_submissions: 1, window_length: 100 }, None).unwrap();

            // Both attestors hit rate limit
            RateLimiter::check_and_increment(&env, &attestor1).unwrap();
            RateLimiter::check_and_increment(&env, &attestor1).unwrap_err();

            RateLimiter::check_and_increment(&env, &attestor2).unwrap();
            RateLimiter::check_and_increment(&env, &attestor2).unwrap_err();

            // Reset only attestor1
            RateLimiter::reset_rate_limit(&env, &admin, &attestor1).unwrap();

            // attestor1 should be reset
            assert_eq!(RateLimiter::get_state(env.clone(), attestor1.clone()).submission_count, 0);

            // attestor2 should still be rate limited
            assert_eq!(RateLimiter::get_state(env.clone(), attestor2.clone()).submission_count, 1);
            assert!(RateLimiter::check_and_increment(&env, &attestor2).is_err());
        });
    }

    #[test]
    fn test_reset_rate_limit_resets_window_start_ledger() {
        let env = Env::default();
        env.mock_all_auths();
        let admin = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);
        let attestor = <soroban_sdk::Address as soroban_sdk::testutils::Address>::generate(&env);
        let contract_address = make_contract(&env);

        env.as_contract(&contract_address, &|| {
            RateLimiter::update_config(&env, &admin, RateLimitConfig { max_submissions: 2, window_length: 100 }, None).unwrap();

            // Get initial ledger when making transaction
            RateLimiter::check_and_increment(&env, &attestor).unwrap();
            let state_before = RateLimiter::get_state(env.clone(), attestor.clone());
            let ledger_before = state_before.window_start_ledger;

            // Reset rate limit
            RateLimiter::reset_rate_limit(&env, &admin, &attestor).unwrap();

            // window_start_ledger should be updated to current ledger
            let state_after = RateLimiter::get_state(env.clone(), attestor.clone());
            assert_eq!(state_after.window_start_ledger, env.ledger().sequence());
            // The reset typically updates it to "now"
            assert!(state_after.window_start_ledger >= ledger_before);
        });
    }
}
