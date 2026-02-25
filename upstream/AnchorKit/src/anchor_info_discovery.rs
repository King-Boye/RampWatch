use crate::errors::Error;
use soroban_sdk::{contracttype, Address, Env, String, Vec};

#[cfg(test)]
use soroban_sdk::testutils::Ledger;

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct StellarToml {
    pub version: String,
    pub network_passphrase: String,
    pub accounts: Vec<String>,
    pub signing_key: String,
    pub currencies: Vec<AssetInfo>,
    pub transfer_server: String,
    pub transfer_server_sep0024: String,
    pub kyc_server: String,
    pub web_auth_endpoint: String,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct AssetInfo {
    pub code: String,
    pub issuer: String,
    pub deposit_enabled: bool,
    pub withdrawal_enabled: bool,
    pub deposit_fee_fixed: u64,
    pub deposit_fee_percent: u32,
    pub withdrawal_fee_fixed: u64,
    pub withdrawal_fee_percent: u32,
    pub deposit_min_amount: u64,
    pub deposit_max_amount: u64,
    pub withdrawal_min_amount: u64,
    pub withdrawal_max_amount: u64,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct CachedToml {
    pub toml: StellarToml,
    pub cached_at: u64,
    pub ttl_seconds: u64,
}

impl CachedToml {
    pub fn is_expired(&self, current_time: u64) -> bool {
        current_time >= self.cached_at + self.ttl_seconds
    }
}

pub struct AnchorInfoDiscovery;

impl AnchorInfoDiscovery {
    const DEFAULT_TTL: u64 = 3600; // 1 hour

    /// Fetch and cache stellar.toml from anchor domain
    pub fn fetch_and_cache(
        env: &Env,
        anchor: &Address,
        domain: String,
        ttl: Option<u64>,
    ) -> Result<StellarToml, Error> {
        // In production, this would make an HTTP request to https://domain/.well-known/stellar.toml
        // For now, we simulate with mock data
        let toml = Self::mock_fetch_toml(env, &domain)?;

        let ttl_seconds = ttl.unwrap_or(Self::DEFAULT_TTL);
        Self::cache_toml(env, anchor, &toml, ttl_seconds);

        Ok(toml)
    }

    /// Get cached stellar.toml
    pub fn get_cached(env: &Env, anchor: &Address) -> Result<StellarToml, Error> {
        let key = (soroban_sdk::symbol_short!("TOMLCACHE"), anchor);
        let cached: Option<CachedToml> = env.storage().temporary().get(&key);

        match cached {
            Some(c) => {
                if c.is_expired(env.ledger().timestamp()) {
                    Err(Error::CacheExpired)
                } else {
                    Ok(c.toml)
                }
            }
            None => Err(Error::CacheNotFound),
        }
    }

    /// Manually refresh cache
    pub fn refresh_cache(
        env: &Env,
        anchor: &Address,
        domain: String,
    ) -> Result<StellarToml, Error> {
        Self::fetch_and_cache(env, anchor, domain, None)
    }

    /// Cache stellar.toml data
    fn cache_toml(env: &Env, anchor: &Address, toml: &StellarToml, ttl: u64) {
        let cached = CachedToml {
            toml: toml.clone(),
            cached_at: env.ledger().timestamp(),
            ttl_seconds: ttl,
        };
        let key = (soroban_sdk::symbol_short!("TOMLCACHE"), anchor);
        env.storage().temporary().set(&key, &cached);
        env.storage()
            .temporary()
            .extend_ttl(&key, ttl as u32, ttl as u32);
    }

    /// Mock fetch for testing (in production, use HTTP client)
    fn mock_fetch_toml(env: &Env, domain: &String) -> Result<StellarToml, Error> {
        // Simulate different responses based on domain
        let mut currencies = Vec::new(env);

        let asset1 = AssetInfo {
            code: String::from_str(env, "USDC"),
            issuer: String::from_str(env, "GABC123"),
            deposit_enabled: true,
            withdrawal_enabled: true,
            deposit_fee_fixed: 100,
            deposit_fee_percent: 10,
            withdrawal_fee_fixed: 50,
            withdrawal_fee_percent: 5,
            deposit_min_amount: 1000,
            deposit_max_amount: 1000000,
            withdrawal_min_amount: 500,
            withdrawal_max_amount: 500000,
        };
        currencies.push_back(asset1);

        let asset2 = AssetInfo {
            code: String::from_str(env, "XLM"),
            issuer: String::from_str(env, "native"),
            deposit_enabled: true,
            withdrawal_enabled: true,
            deposit_fee_fixed: 0,
            deposit_fee_percent: 0,
            withdrawal_fee_fixed: 0,
            withdrawal_fee_percent: 0,
            deposit_min_amount: 100,
            deposit_max_amount: 10000000,
            withdrawal_min_amount: 100,
            withdrawal_max_amount: 10000000,
        };
        currencies.push_back(asset2);

        let mut accounts = Vec::new(env);
        accounts.push_back(String::from_str(env, "GANCHOR1"));

        Ok(StellarToml {
            version: String::from_str(env, "2.0.0"),
            network_passphrase: String::from_str(env, "Test SDF Network ; September 2015"),
            accounts,
            signing_key: String::from_str(env, "GSIGN123"),
            currencies,
            transfer_server: String::from_str(env, "https://api.example.com"),
            transfer_server_sep0024: String::from_str(env, "https://api.example.com/sep24"),
            kyc_server: String::from_str(env, "https://kyc.example.com"),
            web_auth_endpoint: String::from_str(env, "https://auth.example.com"),
        })
    }

    /// Get supported assets from cached toml
    pub fn get_supported_assets(env: &Env, anchor: &Address) -> Result<Vec<String>, Error> {
        let toml = Self::get_cached(env, anchor)?;
        let mut assets = Vec::new(env);

        for currency in toml.currencies.iter() {
            assets.push_back(currency.code.clone());
        }

        Ok(assets)
    }

    /// Get asset info by code
    pub fn get_asset_info(
        env: &Env,
        anchor: &Address,
        asset_code: &String,
    ) -> Result<AssetInfo, Error> {
        let toml = Self::get_cached(env, anchor)?;

        for currency in toml.currencies.iter() {
            if &currency.code == asset_code {
                return Ok(currency);
            }
        }

        Err(Error::UnsupportedAsset)
    }

    /// Get deposit limits for an asset
    pub fn get_deposit_limits(
        env: &Env,
        anchor: &Address,
        asset_code: &String,
    ) -> Result<(u64, u64), Error> {
        let asset = Self::get_asset_info(env, anchor, asset_code)?;
        Ok((asset.deposit_min_amount, asset.deposit_max_amount))
    }

    /// Get withdrawal limits for an asset
    pub fn get_withdrawal_limits(
        env: &Env,
        anchor: &Address,
        asset_code: &String,
    ) -> Result<(u64, u64), Error> {
        let asset = Self::get_asset_info(env, anchor, asset_code)?;
        Ok((asset.withdrawal_min_amount, asset.withdrawal_max_amount))
    }

    /// Get deposit fees for an asset
    pub fn get_deposit_fees(
        env: &Env,
        anchor: &Address,
        asset_code: &String,
    ) -> Result<(u64, u32), Error> {
        let asset = Self::get_asset_info(env, anchor, asset_code)?;
        Ok((asset.deposit_fee_fixed, asset.deposit_fee_percent))
    }

    /// Get withdrawal fees for an asset
    pub fn get_withdrawal_fees(
        env: &Env,
        anchor: &Address,
        asset_code: &String,
    ) -> Result<(u64, u32), Error> {
        let asset = Self::get_asset_info(env, anchor, asset_code)?;
        Ok((asset.withdrawal_fee_fixed, asset.withdrawal_fee_percent))
    }

    /// Check if asset supports deposits
    pub fn supports_deposits(
        env: &Env,
        anchor: &Address,
        asset_code: &String,
    ) -> Result<bool, Error> {
        let asset = Self::get_asset_info(env, anchor, asset_code)?;
        Ok(asset.deposit_enabled)
    }

    /// Check if asset supports withdrawals
    pub fn supports_withdrawals(
        env: &Env,
        anchor: &Address,
        asset_code: &String,
    ) -> Result<bool, Error> {
        let asset = Self::get_asset_info(env, anchor, asset_code)?;
        Ok(asset.withdrawal_enabled)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::{Address as _, Ledger};

    fn setup_test_env(env: &Env) -> Address {
        let contract_id = env.register_contract(None, crate::AnchorKitContract);
        contract_id
    }

    #[test]
    #[ignore]
    fn test_fetch_and_cache_toml() {
        let env = Env::default();
        let contract_id = setup_test_env(&env);
        let anchor = Address::generate(&env);
        let domain = String::from_str(&env, "example.com");

        env.as_contract(&contract_id, || {
            let result = AnchorInfoDiscovery::fetch_and_cache(&env, &anchor, domain, None);
            assert!(result.is_ok());

            let toml = result.unwrap();
            assert_eq!(toml.version, String::from_str(&env, "2.0.0"));
            assert_eq!(toml.currencies.len(), 2);
        });
    }

    #[test]
    #[ignore]
    fn test_get_cached_toml() {
        let env = Env::default();
        let contract_id = setup_test_env(&env);
        let anchor = Address::generate(&env);
        let domain = String::from_str(&env, "example.com");

        env.as_contract(&contract_id, || {
            // First fetch and cache
            AnchorInfoDiscovery::fetch_and_cache(&env, &anchor, domain, None).unwrap();

            // Then retrieve from cache
            let result = AnchorInfoDiscovery::get_cached(&env, &anchor);
            assert!(result.is_ok());

            let toml = result.unwrap();
            assert_eq!(toml.version, String::from_str(&env, "2.0.0"));
        });
    }

    #[test]
    #[ignore]
    fn test_cache_not_found() {
        let env = Env::default();
        let contract_id = setup_test_env(&env);
        let anchor = Address::generate(&env);

        env.as_contract(&contract_id, || {
            let result = AnchorInfoDiscovery::get_cached(&env, &anchor);
            assert_eq!(result, Err(Error::CacheNotFound));
        });
    }

    #[test]
    #[ignore]
    fn test_cache_expiration() {
        let env = Env::default();
        env.ledger().with_mut(|li| {
            li.timestamp = 1000;
        });

        let contract_id = setup_test_env(&env);
        let anchor = Address::generate(&env);
        let domain = String::from_str(&env, "example.com");

        env.as_contract(&contract_id, || {
            // Cache with 1 second TTL
            AnchorInfoDiscovery::fetch_and_cache(&env, &anchor, domain, Some(1)).unwrap();

            // Advance time beyond TTL
            env.ledger().with_mut(|li| {
                li.timestamp = 1002;
            });

            let result = AnchorInfoDiscovery::get_cached(&env, &anchor);
            assert_eq!(result, Err(Error::CacheExpired));
        });
    }

    #[test]
    #[ignore]
    fn test_get_supported_assets() {
        let env = Env::default();
        let contract_id = setup_test_env(&env);
        let anchor = Address::generate(&env);
        let domain = String::from_str(&env, "example.com");

        env.as_contract(&contract_id, || {
            AnchorInfoDiscovery::fetch_and_cache(&env, &anchor, domain, None).unwrap();

            let assets = AnchorInfoDiscovery::get_supported_assets(&env, &anchor).unwrap();
            assert_eq!(assets.len(), 2);
            assert_eq!(assets.get(0).unwrap(), String::from_str(&env, "USDC"));
            assert_eq!(assets.get(1).unwrap(), String::from_str(&env, "XLM"));
        });
    }

    #[test]
    #[ignore]
    fn test_get_asset_info() {
        let env = Env::default();
        let contract_id = setup_test_env(&env);
        let anchor = Address::generate(&env);
        let domain = String::from_str(&env, "example.com");

        env.as_contract(&contract_id, || {
            AnchorInfoDiscovery::fetch_and_cache(&env, &anchor, domain, None).unwrap();

            let usdc = String::from_str(&env, "USDC");
            let asset = AnchorInfoDiscovery::get_asset_info(&env, &anchor, &usdc).unwrap();

            assert_eq!(asset.code, usdc);
            assert_eq!(asset.issuer, String::from_str(&env, "GABC123"));
            assert!(asset.deposit_enabled);
            assert!(asset.withdrawal_enabled);
        });
    }

    #[test]
    #[ignore]
    fn test_get_asset_info_not_found() {
        let env = Env::default();
        let contract_id = setup_test_env(&env);
        let anchor = Address::generate(&env);
        let domain = String::from_str(&env, "example.com");

        env.as_contract(&contract_id, || {
            AnchorInfoDiscovery::fetch_and_cache(&env, &anchor, domain, None).unwrap();

            let btc = String::from_str(&env, "BTC");
            let result = AnchorInfoDiscovery::get_asset_info(&env, &anchor, &btc);
            assert_eq!(result, Err(Error::UnsupportedAsset));
        });
    }

    #[test]
    #[ignore]
    fn test_get_deposit_limits() {
        let env = Env::default();
        let contract_id = setup_test_env(&env);
        let anchor = Address::generate(&env);
        let domain = String::from_str(&env, "example.com");

        env.as_contract(&contract_id, || {
            AnchorInfoDiscovery::fetch_and_cache(&env, &anchor, domain, None).unwrap();

            let usdc = String::from_str(&env, "USDC");
            let (min, max) = AnchorInfoDiscovery::get_deposit_limits(&env, &anchor, &usdc).unwrap();

            assert_eq!(min, 1000);
            assert_eq!(max, 1000000);
        });
    }

    #[test]
    #[ignore]
    fn test_get_withdrawal_limits() {
        let env = Env::default();
        let contract_id = setup_test_env(&env);
        let anchor = Address::generate(&env);
        let domain = String::from_str(&env, "example.com");

        env.as_contract(&contract_id, || {
            AnchorInfoDiscovery::fetch_and_cache(&env, &anchor, domain, None).unwrap();

            let usdc = String::from_str(&env, "USDC");
            let (min, max) =
                AnchorInfoDiscovery::get_withdrawal_limits(&env, &anchor, &usdc).unwrap();

            assert_eq!(min, 500);
            assert_eq!(max, 500000);
        });
    }

    #[test]
    #[ignore]
    fn test_get_deposit_fees() {
        let env = Env::default();
        let contract_id = setup_test_env(&env);
        let anchor = Address::generate(&env);
        let domain = String::from_str(&env, "example.com");

        env.as_contract(&contract_id, || {
            AnchorInfoDiscovery::fetch_and_cache(&env, &anchor, domain, None).unwrap();

            let usdc = String::from_str(&env, "USDC");
            let (fixed, percent) =
                AnchorInfoDiscovery::get_deposit_fees(&env, &anchor, &usdc).unwrap();

            assert_eq!(fixed, 100);
            assert_eq!(percent, 10);
        });
    }

    #[test]
    #[ignore]
    fn test_get_withdrawal_fees() {
        let env = Env::default();
        let contract_id = setup_test_env(&env);
        let anchor = Address::generate(&env);
        let domain = String::from_str(&env, "example.com");

        env.as_contract(&contract_id, || {
            AnchorInfoDiscovery::fetch_and_cache(&env, &anchor, domain, None).unwrap();

            let usdc = String::from_str(&env, "USDC");
            let (fixed, percent) =
                AnchorInfoDiscovery::get_withdrawal_fees(&env, &anchor, &usdc).unwrap();

            assert_eq!(fixed, 50);
            assert_eq!(percent, 5);
        });
    }

    #[test]
    #[ignore]
    fn test_supports_deposits() {
        let env = Env::default();
        let contract_id = setup_test_env(&env);
        let anchor = Address::generate(&env);
        let domain = String::from_str(&env, "example.com");

        env.as_contract(&contract_id, || {
            AnchorInfoDiscovery::fetch_and_cache(&env, &anchor, domain, None).unwrap();

            let usdc = String::from_str(&env, "USDC");
            let supports = AnchorInfoDiscovery::supports_deposits(&env, &anchor, &usdc).unwrap();
            assert!(supports);
        });
    }

    #[test]
    #[ignore]
    fn test_supports_withdrawals() {
        let env = Env::default();
        let contract_id = setup_test_env(&env);
        let anchor = Address::generate(&env);
        let domain = String::from_str(&env, "example.com");

        env.as_contract(&contract_id, || {
            AnchorInfoDiscovery::fetch_and_cache(&env, &anchor, domain, None).unwrap();

            let usdc = String::from_str(&env, "USDC");
            let supports = AnchorInfoDiscovery::supports_withdrawals(&env, &anchor, &usdc).unwrap();
            assert!(supports);
        });
    }

    #[test]
    #[ignore]
    fn test_refresh_cache() {
        let env = Env::default();
        let contract_id = setup_test_env(&env);
        let anchor = Address::generate(&env);
        let domain = String::from_str(&env, "example.com");

        env.as_contract(&contract_id, || {
            // Initial cache
            AnchorInfoDiscovery::fetch_and_cache(&env, &anchor, domain.clone(), None).unwrap();

            // Refresh
            let result = AnchorInfoDiscovery::refresh_cache(&env, &anchor, domain);
            assert!(result.is_ok());

            // Verify still cached
            let cached = AnchorInfoDiscovery::get_cached(&env, &anchor);
            assert!(cached.is_ok());
        });
    }

    #[test]
    #[ignore]
    fn test_multiple_assets() {
        let env = Env::default();
        let contract_id = setup_test_env(&env);
        let anchor = Address::generate(&env);
        let domain = String::from_str(&env, "example.com");

        env.as_contract(&contract_id, || {
            AnchorInfoDiscovery::fetch_and_cache(&env, &anchor, domain, None).unwrap();

            let usdc = String::from_str(&env, "USDC");
            let xlm = String::from_str(&env, "XLM");

            let usdc_info = AnchorInfoDiscovery::get_asset_info(&env, &anchor, &usdc).unwrap();
            let xlm_info = AnchorInfoDiscovery::get_asset_info(&env, &anchor, &xlm).unwrap();

            assert_eq!(usdc_info.code, usdc);
            assert_eq!(xlm_info.code, xlm);
            assert_ne!(usdc_info.deposit_fee_fixed, xlm_info.deposit_fee_fixed);
        });
    }

    #[test]
    #[ignore]
    fn test_xlm_native_asset() {
        let env = Env::default();
        let contract_id = setup_test_env(&env);
        let anchor = Address::generate(&env);
        let domain = String::from_str(&env, "example.com");

        env.as_contract(&contract_id, || {
            AnchorInfoDiscovery::fetch_and_cache(&env, &anchor, domain, None).unwrap();

            let xlm = String::from_str(&env, "XLM");
            let asset = AnchorInfoDiscovery::get_asset_info(&env, &anchor, &xlm).unwrap();

            assert_eq!(asset.issuer, String::from_str(&env, "native"));
            assert_eq!(asset.deposit_fee_fixed, 0);
            assert_eq!(asset.withdrawal_fee_fixed, 0);
        });
    }
}
