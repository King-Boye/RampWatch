use crate::anchor_info_discovery::{AnchorInfoDiscovery, AssetInfo, StellarToml};
use crate::errors::Error;
use soroban_sdk::{testutils::{Address as _, Ledger}, Address, Env, String, Vec};

#[test]
#[ignore]
fn test_fetch_and_cache_toml() {
    let env = Env::default();
    let anchor = Address::generate(&env);
    let domain = String::from_str(&env, "example.com");

    let result = AnchorInfoDiscovery::fetch_and_cache(&env, &anchor, domain, None);
    assert!(result.is_ok());

    let toml = result.unwrap();
    assert_eq!(toml.version, String::from_str(&env, "2.0.0"));
    assert_eq!(toml.currencies.len(), 2);
}

#[test]
#[ignore]
fn test_get_cached_toml() {
    let env = Env::default();
    let anchor = Address::generate(&env);
    let domain = String::from_str(&env, "example.com");

    AnchorInfoDiscovery::fetch_and_cache(&env, &anchor, domain, None).unwrap();

    let result = AnchorInfoDiscovery::get_cached(&env, &anchor);
    assert!(result.is_ok());

    let toml = result.unwrap();
    assert_eq!(toml.version, String::from_str(&env, "2.0.0"));
}

#[test]
#[ignore]
fn test_cache_not_found() {
    let env = Env::default();
    let anchor = Address::generate(&env);

    let result = AnchorInfoDiscovery::get_cached(&env, &anchor);
    assert_eq!(result, Err(Error::CacheNotFound));
}

#[test]
#[ignore]
fn test_cache_expiration() {
    let env = Env::default();
    env.ledger().with_mut(|li| {
        li.timestamp = 1000;
    });

    let anchor = Address::generate(&env);
    let domain = String::from_str(&env, "example.com");

    AnchorInfoDiscovery::fetch_and_cache(&env, &anchor, domain, Some(1)).unwrap();

    env.ledger().with_mut(|li| {
        li.timestamp = 1002;
    });

    let result = AnchorInfoDiscovery::get_cached(&env, &anchor);
    assert_eq!(result, Err(Error::CacheExpired));
}

#[test]
#[ignore]
fn test_get_supported_assets() {
    let env = Env::default();
    let anchor = Address::generate(&env);
    let domain = String::from_str(&env, "example.com");

    AnchorInfoDiscovery::fetch_and_cache(&env, &anchor, domain, None).unwrap();

    let assets = AnchorInfoDiscovery::get_supported_assets(&env, &anchor).unwrap();
    assert_eq!(assets.len(), 2);
    assert_eq!(assets.get(0).unwrap(), String::from_str(&env, "USDC"));
    assert_eq!(assets.get(1).unwrap(), String::from_str(&env, "XLM"));
}

#[test]
#[ignore]
fn test_get_asset_info() {
    let env = Env::default();
    let anchor = Address::generate(&env);
    let domain = String::from_str(&env, "example.com");

    AnchorInfoDiscovery::fetch_and_cache(&env, &anchor, domain, None).unwrap();

    let usdc = String::from_str(&env, "USDC");
    let asset = AnchorInfoDiscovery::get_asset_info(&env, &anchor, &usdc).unwrap();

    assert_eq!(asset.code, usdc);
    assert_eq!(asset.issuer, String::from_str(&env, "GABC123"));
    assert!(asset.deposit_enabled);
    assert!(asset.withdrawal_enabled);
}

#[test]
#[ignore]
fn test_get_asset_info_not_found() {
    let env = Env::default();
    let anchor = Address::generate(&env);
    let domain = String::from_str(&env, "example.com");

    AnchorInfoDiscovery::fetch_and_cache(&env, &anchor, domain, None).unwrap();

    let btc = String::from_str(&env, "BTC");
    let result = AnchorInfoDiscovery::get_asset_info(&env, &anchor, &btc);
    assert_eq!(result, Err(Error::UnsupportedAsset));
}

#[test]
#[ignore]
fn test_get_deposit_limits() {
    let env = Env::default();
    let anchor = Address::generate(&env);
    let domain = String::from_str(&env, "example.com");

    AnchorInfoDiscovery::fetch_and_cache(&env, &anchor, domain, None).unwrap();

    let usdc = String::from_str(&env, "USDC");
    let (min, max) = AnchorInfoDiscovery::get_deposit_limits(&env, &anchor, &usdc).unwrap();

    assert_eq!(min, 1000);
    assert_eq!(max, 1000000);
}

#[test]
#[ignore]
fn test_get_withdrawal_limits() {
    let env = Env::default();
    let anchor = Address::generate(&env);
    let domain = String::from_str(&env, "example.com");

    AnchorInfoDiscovery::fetch_and_cache(&env, &anchor, domain, None).unwrap();

    let usdc = String::from_str(&env, "USDC");
    let (min, max) = AnchorInfoDiscovery::get_withdrawal_limits(&env, &anchor, &usdc).unwrap();

    assert_eq!(min, 500);
    assert_eq!(max, 500000);
}

#[test]
#[ignore]
fn test_get_deposit_fees() {
    let env = Env::default();
    let anchor = Address::generate(&env);
    let domain = String::from_str(&env, "example.com");

    AnchorInfoDiscovery::fetch_and_cache(&env, &anchor, domain, None).unwrap();

    let usdc = String::from_str(&env, "USDC");
    let (fixed, percent) = AnchorInfoDiscovery::get_deposit_fees(&env, &anchor, &usdc).unwrap();

    assert_eq!(fixed, 100);
    assert_eq!(percent, 10);
}

#[test]
#[ignore]
fn test_get_withdrawal_fees() {
    let env = Env::default();
    let anchor = Address::generate(&env);
    let domain = String::from_str(&env, "example.com");

    AnchorInfoDiscovery::fetch_and_cache(&env, &anchor, domain, None).unwrap();

    let usdc = String::from_str(&env, "USDC");
    let (fixed, percent) = AnchorInfoDiscovery::get_withdrawal_fees(&env, &anchor, &usdc).unwrap();

    assert_eq!(fixed, 50);
    assert_eq!(percent, 5);
}

#[test]
#[ignore]
fn test_supports_deposits() {
    let env = Env::default();
    let anchor = Address::generate(&env);
    let domain = String::from_str(&env, "example.com");

    AnchorInfoDiscovery::fetch_and_cache(&env, &anchor, domain, None).unwrap();

    let usdc = String::from_str(&env, "USDC");
    let supports = AnchorInfoDiscovery::supports_deposits(&env, &anchor, &usdc).unwrap();
    assert!(supports);
}

#[test]
#[ignore]
fn test_supports_withdrawals() {
    let env = Env::default();
    let anchor = Address::generate(&env);
    let domain = String::from_str(&env, "example.com");

    AnchorInfoDiscovery::fetch_and_cache(&env, &anchor, domain, None).unwrap();

    let usdc = String::from_str(&env, "USDC");
    let supports = AnchorInfoDiscovery::supports_withdrawals(&env, &anchor, &usdc).unwrap();
    assert!(supports);
}

#[test]
#[ignore]
fn test_refresh_cache() {
    let env = Env::default();
    let anchor = Address::generate(&env);
    let domain = String::from_str(&env, "example.com");

    AnchorInfoDiscovery::fetch_and_cache(&env, &anchor, domain.clone(), None).unwrap();

    let result = AnchorInfoDiscovery::refresh_cache(&env, &anchor, domain);
    assert!(result.is_ok());

    let cached = AnchorInfoDiscovery::get_cached(&env, &anchor);
    assert!(cached.is_ok());
}

#[test]
#[ignore]
fn test_multiple_assets() {
    let env = Env::default();
    let anchor = Address::generate(&env);
    let domain = String::from_str(&env, "example.com");

    AnchorInfoDiscovery::fetch_and_cache(&env, &anchor, domain, None).unwrap();

    let usdc = String::from_str(&env, "USDC");
    let xlm = String::from_str(&env, "XLM");

    let usdc_info = AnchorInfoDiscovery::get_asset_info(&env, &anchor, &usdc).unwrap();
    let xlm_info = AnchorInfoDiscovery::get_asset_info(&env, &anchor, &xlm).unwrap();

    assert_eq!(usdc_info.code, usdc);
    assert_eq!(xlm_info.code, xlm);
    assert_ne!(
        usdc_info.deposit_fee_fixed,
        xlm_info.deposit_fee_fixed
    );
}

#[test]
#[ignore]
fn test_xlm_native_asset() {
    let env = Env::default();
    let anchor = Address::generate(&env);
    let domain = String::from_str(&env, "example.com");

    AnchorInfoDiscovery::fetch_and_cache(&env, &anchor, domain, None).unwrap();

    let xlm = String::from_str(&env, "XLM");
    let asset = AnchorInfoDiscovery::get_asset_info(&env, &anchor, &xlm).unwrap();

    assert_eq!(asset.issuer, String::from_str(&env, "native"));
    assert_eq!(asset.deposit_fee_fixed, 0);
    assert_eq!(asset.withdrawal_fee_fixed, 0);
}

#[test]
#[ignore]
fn test_cache_ttl_custom() {
    let env = Env::default();
    env.ledger().with_mut(|li| {
        li.timestamp = 1000;
    });

    let anchor = Address::generate(&env);
    let domain = String::from_str(&env, "example.com");

    AnchorInfoDiscovery::fetch_and_cache(&env, &anchor, domain, Some(3600)).unwrap();

    env.ledger().with_mut(|li| {
        li.timestamp = 2000;
    });

    let result = AnchorInfoDiscovery::get_cached(&env, &anchor);
    assert!(result.is_ok());

    env.ledger().with_mut(|li| {
        li.timestamp = 5000;
    });

    let result = AnchorInfoDiscovery::get_cached(&env, &anchor);
    assert_eq!(result, Err(Error::CacheExpired));
}

#[test]
#[ignore]
fn test_multiple_anchors() {
    let env = Env::default();
    let anchor1 = Address::generate(&env);
    let anchor2 = Address::generate(&env);
    let domain = String::from_str(&env, "example.com");

    AnchorInfoDiscovery::fetch_and_cache(&env, &anchor1, domain.clone(), None).unwrap();
    AnchorInfoDiscovery::fetch_and_cache(&env, &anchor2, domain, None).unwrap();

    let toml1 = AnchorInfoDiscovery::get_cached(&env, &anchor1).unwrap();
    let toml2 = AnchorInfoDiscovery::get_cached(&env, &anchor2).unwrap();

    assert_eq!(toml1.version, toml2.version);
}

#[test]
#[ignore]
fn test_asset_limits_validation() {
    let env = Env::default();
    let anchor = Address::generate(&env);
    let domain = String::from_str(&env, "example.com");

    AnchorInfoDiscovery::fetch_and_cache(&env, &anchor, domain, None).unwrap();

    let usdc = String::from_str(&env, "USDC");
    let (dep_min, dep_max) = AnchorInfoDiscovery::get_deposit_limits(&env, &anchor, &usdc).unwrap();
    let (with_min, with_max) = AnchorInfoDiscovery::get_withdrawal_limits(&env, &anchor, &usdc).unwrap();

    assert!(dep_min < dep_max);
    assert!(with_min < with_max);
}

#[test]
#[ignore]
fn test_fee_structure() {
    let env = Env::default();
    let anchor = Address::generate(&env);
    let domain = String::from_str(&env, "example.com");

    AnchorInfoDiscovery::fetch_and_cache(&env, &anchor, domain, None).unwrap();

    let usdc = String::from_str(&env, "USDC");
    let (dep_fixed, dep_percent) = AnchorInfoDiscovery::get_deposit_fees(&env, &anchor, &usdc).unwrap();
    let (with_fixed, with_percent) = AnchorInfoDiscovery::get_withdrawal_fees(&env, &anchor, &usdc).unwrap();

    assert!(dep_fixed > 0);
    assert!(dep_percent > 0);
    assert!(with_fixed > 0);
    assert!(with_percent > 0);
}
