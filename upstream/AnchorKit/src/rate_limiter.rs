use soroban_sdk::{contracttype, Address, Env};

use crate::Error;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RateLimitStrategy {
    FixedWindow,
    TokenBucket,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RateLimitConfig {
    pub strategy: RateLimitStrategy,
    pub max_requests: u32,
    pub window_seconds: u64,
    pub refill_rate: u32, // tokens per second (for token bucket)
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
struct RateLimitState {
    pub requests: u32,
    pub window_start: u64,
    pub tokens: u32,
    pub last_refill: u64,
}

pub struct RateLimiter;

impl RateLimiter {
    pub fn check_and_update(
        env: &Env,
        anchor: &Address,
        config: &RateLimitConfig,
    ) -> Result<(), Error> {
        let now = env.ledger().timestamp();
        let mut state = Self::get_state(env, anchor).unwrap_or(RateLimitState {
            requests: 0,
            window_start: now,
            tokens: config.max_requests,
            last_refill: now,
        });

        match config.strategy {
            RateLimitStrategy::FixedWindow => {
                if now >= state.window_start + config.window_seconds {
                    state.requests = 0;
                    state.window_start = now;
                }

                if state.requests >= config.max_requests {
                    return Err(Error::RateLimitExceeded);
                }

                state.requests += 1;
            }
            RateLimitStrategy::TokenBucket => {
                let elapsed = now.saturating_sub(state.last_refill);
                let new_tokens = (elapsed * config.refill_rate as u64) as u32;
                state.tokens = (state.tokens + new_tokens).min(config.max_requests);
                state.last_refill = now;

                if state.tokens == 0 {
                    return Err(Error::RateLimitExceeded);
                }

                state.tokens -= 1;
            }
        }

        Self::set_state(env, anchor, &state);
        Ok(())
    }

    fn get_state(env: &Env, anchor: &Address) -> Option<RateLimitState> {
        let key = (soroban_sdk::symbol_short!("RATELIM"), anchor);
        env.storage().temporary().get(&key)
    }

    fn set_state(env: &Env, anchor: &Address, state: &RateLimitState) {
        let key = (soroban_sdk::symbol_short!("RATELIM"), anchor);
        env.storage().temporary().set(&key, state);
        env.storage().temporary().extend_ttl(&key, 17280, 17280); // 1 day
    }
}
