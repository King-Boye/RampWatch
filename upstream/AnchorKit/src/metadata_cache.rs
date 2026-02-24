use soroban_sdk::{contracttype, Address, Env, String};

use crate::{types::AnchorMetadata, Error};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CachedMetadata {
    pub metadata: AnchorMetadata,
    pub cached_at: u64,
    pub ttl_seconds: u64,
}

impl CachedMetadata {
    pub fn is_expired(&self, current_time: u64) -> bool {
        current_time >= self.cached_at + self.ttl_seconds
    }
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CachedCapabilities {
    pub toml_url: String,
    pub capabilities: String, // JSON string of capabilities
    pub cached_at: u64,
    pub ttl_seconds: u64,
}

impl CachedCapabilities {
    pub fn is_expired(&self, current_time: u64) -> bool {
        current_time >= self.cached_at + self.ttl_seconds
    }
}

pub struct MetadataCache;

impl MetadataCache {
    pub fn set_metadata(env: &Env, anchor: &Address, metadata: &AnchorMetadata, ttl: u64) {
        let cached = CachedMetadata {
            metadata: metadata.clone(),
            cached_at: env.ledger().timestamp(),
            ttl_seconds: ttl,
        };
        let key = (soroban_sdk::symbol_short!("METACACHE"), anchor);
        env.storage().temporary().set(&key, &cached);
        env.storage().temporary().extend_ttl(&key, ttl as u32, ttl as u32);
    }

    pub fn get_metadata(env: &Env, anchor: &Address) -> Result<AnchorMetadata, Error> {
        let key = (soroban_sdk::symbol_short!("METACACHE"), anchor);
        let cached: Option<CachedMetadata> = env.storage().temporary().get(&key);
        
        match cached {
            Some(c) => {
                if c.is_expired(env.ledger().timestamp()) {
                    Err(Error::CacheExpired)
                } else {
                    Ok(c.metadata)
                }
            }
            None => Err(Error::CacheNotFound),
        }
    }

    pub fn invalidate_metadata(env: &Env, anchor: &Address) {
        let key = (soroban_sdk::symbol_short!("METACACHE"), anchor);
        env.storage().temporary().remove(&key);
    }

    pub fn set_capabilities(env: &Env, anchor: &Address, toml_url: String, capabilities: String, ttl: u64) {
        let cached = CachedCapabilities {
            toml_url,
            capabilities,
            cached_at: env.ledger().timestamp(),
            ttl_seconds: ttl,
        };
        let key = (soroban_sdk::symbol_short!("CAPCACHE"), anchor);
        env.storage().temporary().set(&key, &cached);
        env.storage().temporary().extend_ttl(&key, ttl as u32, ttl as u32);
    }

    pub fn get_capabilities(env: &Env, anchor: &Address) -> Result<CachedCapabilities, Error> {
        let key = (soroban_sdk::symbol_short!("CAPCACHE"), anchor);
        let cached: Option<CachedCapabilities> = env.storage().temporary().get(&key);
        
        match cached {
            Some(c) => {
                if c.is_expired(env.ledger().timestamp()) {
                    Err(Error::CacheExpired)
                } else {
                    Ok(c)
                }
            }
            None => Err(Error::CacheNotFound),
        }
    }

    pub fn invalidate_capabilities(env: &Env, anchor: &Address) {
        let key = (soroban_sdk::symbol_short!("CAPCACHE"), anchor);
        env.storage().temporary().remove(&key);
    }
}
