use soroban_sdk::{contracttype, Address, Env, String, Vec};

use crate::Error;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetConfig {
    pub anchor: Address,
    pub supported_assets: Vec<String>, // Asset codes (e.g., "USDC", "BTC")
}

pub struct AssetValidator;

impl AssetValidator {
    pub fn set_supported_assets(env: &Env, anchor: &Address, assets: Vec<String>) {
        let config = AssetConfig {
            anchor: anchor.clone(),
            supported_assets: assets,
        };
        let key = (soroban_sdk::symbol_short!("ASSETS"), anchor);
        env.storage().persistent().set(&key, &config);
        env.storage().persistent().extend_ttl(&key, 7776000, 7776000); // 90 days
    }

    pub fn get_supported_assets(env: &Env, anchor: &Address) -> Option<Vec<String>> {
        let key = (soroban_sdk::symbol_short!("ASSETS"), anchor);
        let config: Option<AssetConfig> = env.storage().persistent().get(&key);
        config.map(|c| c.supported_assets)
    }

    pub fn is_asset_supported(env: &Env, anchor: &Address, asset: &String) -> bool {
        if let Some(assets) = Self::get_supported_assets(env, anchor) {
            assets.contains(asset)
        } else {
            false
        }
    }

    pub fn validate_asset_pair(
        env: &Env,
        anchor: &Address,
        base_asset: &String,
        quote_asset: &String,
    ) -> Result<(), Error> {
        let assets = Self::get_supported_assets(env, anchor)
            .ok_or(Error::ServicesNotConfigured)?;

        if !assets.contains(base_asset) {
            return Err(Error::InvalidServiceType);
        }

        if !assets.contains(quote_asset) {
            return Err(Error::InvalidServiceType);
        }

        Ok(())
    }
}
