#![cfg(test)]

use crate::{SwiftRemitContract, SwiftRemitContractClient};
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    token, Address, Env, String,
};

fn create_token_contract<'a>(env: &Env, admin: &Address) -> token::StellarAssetClient<'a> {
    token::StellarAssetClient::new(env, &env.register_stellar_asset_contract_v2(admin.clone()).address())
}

#[test]
fn test_fx_rate_storage() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);
    let treasury = Address::generate(&env);

    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);

    let contract_id = env.register_contract(None, SwiftRemitContract);
    let contract = SwiftRemitContractClient::new(&env, &contract_id);

    // Initialize contract
    contract.initialize(&admin, &token.address, &250, &0, &50, &treasury);

    // Register agent
    contract.register_agent(&agent);

    // Mint tokens to sender
    token.mint(&sender, &10000);

    // Create remittance with FX rate
    let fx_rate = 1_2500000i128; // 1.25 (scaled by 10^7)
    let fx_provider = String::from_str(&env, "CurrencyAPI");
    
    let remittance_id = contract.create_remittance(
        &sender,
        &agent,
        &1000,
        &None,
        &Some(fx_rate),
        &Some(fx_provider.clone()),
    );

    // Retrieve remittance and verify FX rate
    let remittance = contract.get_remittance(&remittance_id);
    
    assert!(remittance.fx_rate.is_some());
    let stored_fx = remittance.fx_rate.unwrap();
    assert_eq!(stored_fx.rate, fx_rate);
    assert_eq!(stored_fx.provider, fx_provider);
    assert_eq!(stored_fx.timestamp, env.ledger().timestamp());
}

#[test]
fn test_fx_rate_optional() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);
    let treasury = Address::generate(&env);

    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);

    let contract_id = env.register_contract(None, SwiftRemitContract);
    let contract = SwiftRemitContractClient::new(&env, &contract_id);

    contract.initialize(&admin, &token.address, &250, &0, &50, &treasury);
    contract.register_agent(&agent);
    token.mint(&sender, &10000);

    // Create remittance without FX rate
    let remittance_id = contract.create_remittance(
        &sender,
        &agent,
        &1000,
        &None,
        &None,
        &None,
    );

    let remittance = contract.get_remittance(&remittance_id);
    assert!(remittance.fx_rate.is_none());
}

#[test]
fn test_fx_rate_auditability() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);
    let treasury = Address::generate(&env);

    let token_admin = Address::generate(&env);
    let token = create_token_contract(&env, &token_admin);

    let contract_id = env.register_contract(None, SwiftRemitContract);
    let contract = SwiftRemitContractClient::new(&env, &contract_id);

    contract.initialize(&admin, &token.address, &250, &0, &50, &treasury);
    contract.register_agent(&agent);
    token.mint(&sender, &10000);

    // Create remittance with FX rate
    let fx_rate = 8_5000000i128; // 0.85 (scaled by 10^7)
    let fx_provider = String::from_str(&env, "ExchangeRateAPI");
    
    let remittance_id = contract.create_remittance(
        &sender,
        &agent,
        &5000,
        &None,
        &Some(fx_rate),
        &Some(fx_provider.clone()),
    );

    // Verify FX rate is immutable and auditable
    let remittance_before = contract.get_remittance(&remittance_id);
    let fx_before = remittance_before.fx_rate.clone().unwrap();

    // Advance ledger time
    env.ledger().with_mut(|li| {
        li.timestamp = li.timestamp + 3600; // 1 hour later
    });

    // Retrieve again - FX rate should be unchanged
    let remittance_after = contract.get_remittance(&remittance_id);
    let fx_after = remittance_after.fx_rate.unwrap();

    assert_eq!(fx_before.rate, fx_after.rate);
    assert_eq!(fx_before.provider, fx_after.provider);
    assert_eq!(fx_before.timestamp, fx_after.timestamp);
}
