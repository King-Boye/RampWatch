use soroban_sdk::{contracttype, Address, Env, Vec};

use crate::{types::QuoteData, Error};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FallbackConfig {
    pub anchor_order: Vec<Address>, // Ordered list of anchors to try
    pub max_retries: u32,
    pub failure_threshold: u32, // Failures before marking anchor as down
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnchorFailureState {
    pub anchor: Address,
    pub failure_count: u32,
    pub last_failure: u64,
    pub is_down: bool,
}

pub struct FallbackSelector;

impl FallbackSelector {
    pub fn set_config(env: &Env, config: &FallbackConfig) {
        let key = soroban_sdk::symbol_short!("FBCONFIG");
        env.storage().persistent().set(&key, config);
        env.storage().persistent().extend_ttl(&key, 7776000, 7776000); // 90 days
    }

    pub fn get_config(env: &Env) -> Option<FallbackConfig> {
        let key = soroban_sdk::symbol_short!("FBCONFIG");
        env.storage().persistent().get(&key)
    }

    pub fn record_failure(env: &Env, anchor: &Address, threshold: u32) {
        let key = (soroban_sdk::symbol_short!("FBFAIL"), anchor);
        let mut state: AnchorFailureState = env
            .storage()
            .temporary()
            .get(&key)
            .unwrap_or(AnchorFailureState {
                anchor: anchor.clone(),
                failure_count: 0,
                last_failure: 0,
                is_down: false,
            });

        state.failure_count += 1;
        state.last_failure = env.ledger().timestamp();
        state.is_down = state.failure_count >= threshold;

        env.storage().temporary().set(&key, &state);
        env.storage().temporary().extend_ttl(&key, 17280, 17280); // 1 day
    }

    pub fn record_success(env: &Env, anchor: &Address) {
        let key = (soroban_sdk::symbol_short!("FBFAIL"), anchor);
        env.storage().temporary().remove(&key);
    }

    pub fn get_failure_state(env: &Env, anchor: &Address) -> Option<AnchorFailureState> {
        let key = (soroban_sdk::symbol_short!("FBFAIL"), anchor);
        env.storage().temporary().get(&key)
    }

    pub fn is_anchor_available(env: &Env, anchor: &Address) -> bool {
        if let Some(state) = Self::get_failure_state(env, anchor) {
            !state.is_down
        } else {
            true
        }
    }

    pub fn select_next_anchor(
        env: &Env,
        config: &FallbackConfig,
        failed_anchor: Option<&Address>,
    ) -> Result<Address, Error> {
        let mut start_index = 0;

        // Find where to start in the fallback order
        if let Some(failed) = failed_anchor {
            for i in 0..config.anchor_order.len() {
                if let Some(addr) = config.anchor_order.get(i) {
                    if addr == *failed {
                        start_index = i + 1;
                        break;
                    }
                }
            }
        }

        // Try anchors in order
        for i in start_index..config.anchor_order.len() {
            if let Some(anchor) = config.anchor_order.get(i) {
                if Self::is_anchor_available(env, &anchor) {
                    return Ok(anchor);
                }
            }
        }

        Err(Error::AnchorMetadataNotFound)
    }
}
