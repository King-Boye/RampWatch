#![cfg(test)]

use crate::{SwiftRemitContract, SwiftRemitContractClient, FeeStrategy};
use soroban_sdk::{
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    token, Address, Env, IntoVal, Symbol,
};

fn create_token_contract<'a>(env: &Env, admin: &Address) -> (token::Client<'a>, token::StellarAssetClient<'a>) {
    let contract_address = env.register_stellar_asset_contract(admin.clone());
    (
        token::Client::new(env, &contract_address),
        token::StellarAssetClient::new(env, &contract_address),
    )
}

#[test]
fn test_percentage_strategy() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);
    let treasury = Address::generate(&env);

    let (token, token_admin) = create_token_contract(&env, &admin);
    token_admin.mint(&sender, &100000);

    let contract_id = env.register_contract(None, SwiftRemitContract);
    let client = SwiftRemitContractClient::new(&env, &contract_id);

    client.initialize(&admin, &token.address, &250, &0, &0, &treasury);

    // Set percentage strategy: 5%
    client.update_fee_strategy(&admin, &FeeStrategy::Percentage(500));

    client.register_agent(&agent);

    let remittance_id = client.create_remittance(&sender, &agent, &10000, &None);
    let remittance = client.get_remittance(&remittance_id);

    // Fee should be 5% of 10000 = 500
    assert_eq!(remittance.fee, 500);
}

#[test]
fn test_flat_strategy() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);
    let treasury = Address::generate(&env);

    let (token, token_admin) = create_token_contract(&env, &admin);
    token_admin.mint(&sender, &100000);

    let contract_id = env.register_contract(None, SwiftRemitContract);
    let client = SwiftRemitContractClient::new(&env, &contract_id);

    client.initialize(&admin, &token.address, &250, &0, &0, &treasury);

    // Set flat fee: 100 units
    client.update_fee_strategy(&admin, &FeeStrategy::Flat(100));

    client.register_agent(&agent);

    // Small amount
    let id1 = client.create_remittance(&sender, &agent, &1000, &None);
    assert_eq!(client.get_remittance(&id1).fee, 100);

    // Large amount - same fee
    let id2 = client.create_remittance(&sender, &agent, &50000, &None);
    assert_eq!(client.get_remittance(&id2).fee, 100);
}

#[test]
fn test_dynamic_strategy() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);
    let treasury = Address::generate(&env);

    let (token, token_admin) = create_token_contract(&env, &admin);
    token_admin.mint(&sender, &1_000_000_000_000);

    let contract_id = env.register_contract(None, SwiftRemitContract);
    let client = SwiftRemitContractClient::new(&env, &contract_id);

    client.initialize(&admin, &token.address, &250, &0, &0, &treasury);

    // Set dynamic strategy: 4% base
    client.update_fee_strategy(&admin, &FeeStrategy::Dynamic(400));

    client.register_agent(&agent);

    // Tier 1: amount < 1_000_0000000 -> full 4%
    let id1 = client.create_remittance(&sender, &agent, &5_000_000_000, &None);
    assert_eq!(client.get_remittance(&id1).fee, 200_000_000);

    // Tier 2: 1_000_0000000 <= amount < 10_000_0000000 -> 80% of base = 3.2%
    let id2 = client.create_remittance(&sender, &agent, &50_000_000_000, &None);
    assert_eq!(client.get_remittance(&id2).fee, 1_600_000_000);

    // Tier 3: amount >= 10_000_0000000 -> 60% of base = 2.4%
    let id3 = client.create_remittance(&sender, &agent, &200_000_000_000, &None);
    assert_eq!(client.get_remittance(&id3).fee, 4_800_000_000);
}

#[test]
fn test_strategy_switch_without_redeployment() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);
    let treasury = Address::generate(&env);

    let (token, token_admin) = create_token_contract(&env, &admin);
    token_admin.mint(&sender, &1_000_000_000_000);

    let contract_id = env.register_contract(None, SwiftRemitContract);
    let client = SwiftRemitContractClient::new(&env, &contract_id);

    client.initialize(&admin, &token.address, &250, &0, &0, &treasury);
    client.register_agent(&agent);

    // Start with percentage
    client.update_fee_strategy(&admin, &FeeStrategy::Percentage(250));
    let id1 = client.create_remittance(&sender, &agent, &10000, &None);
    assert_eq!(client.get_remittance(&id1).fee, 250);

    // Switch to flat
    client.update_fee_strategy(&admin, &FeeStrategy::Flat(150));
    let id2 = client.create_remittance(&sender, &agent, &10000, &None);
    assert_eq!(client.get_remittance(&id2).fee, 150);

    // Switch to dynamic: Tier 3 (>= 10_000_0000000) -> 60% of 4% = 2.4%
    client.update_fee_strategy(&admin, &FeeStrategy::Dynamic(400));
    let id3 = client.create_remittance(&sender, &agent, &200_000_000_000, &None);
    assert_eq!(client.get_remittance(&id3).fee, 4_800_000_000);
}

#[test]
fn test_get_fee_strategy() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let treasury = Address::generate(&env);

    let (token, _) = create_token_contract(&env, &admin);

    let contract_id = env.register_contract(None, SwiftRemitContract);
    let client = SwiftRemitContractClient::new(&env, &contract_id);

    client.initialize(&admin, &token.address, &250, &0, &0, &treasury);

    // Default should be Percentage(250)
    let strategy = client.get_fee_strategy();
    assert_eq!(strategy, FeeStrategy::Percentage(250));

    // Update and verify
    client.update_fee_strategy(&admin, &FeeStrategy::Flat(200));
    assert_eq!(client.get_fee_strategy(), FeeStrategy::Flat(200));
}

#[test]
fn test_backwards_compatibility() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);
    let treasury = Address::generate(&env);

    let (token, token_admin) = create_token_contract(&env, &admin);
    token_admin.mint(&sender, &100000);

    let contract_id = env.register_contract(None, SwiftRemitContract);
    let client = SwiftRemitContractClient::new(&env, &contract_id);

    // Initialize with old fee_bps parameter (250 = 2.5%)
    client.initialize(&admin, &token.address, &250, &0, &0, &treasury);
    client.register_agent(&agent);

    // Should default to Percentage strategy with 2.5%
    let id = client.create_remittance(&sender, &agent, &10000, &None);
    assert_eq!(client.get_remittance(&id).fee, 250);

    // Old update_fee should still work (updates percentage strategy)
    client.update_fee(&500); // 5%

    // Verify strategy updated to new percentage
    let strategy = client.get_fee_strategy();
    assert_eq!(strategy, FeeStrategy::Percentage(500));
}
