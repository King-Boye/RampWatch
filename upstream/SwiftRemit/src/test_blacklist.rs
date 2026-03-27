#![cfg(test)]

use crate::{ContractError, SwiftRemitContract, SwiftRemitContractClient};
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, Events},
    token, Address, Env, Symbol, TryFromVal,
};

fn setup<'a>(
    env: &'a Env,
) -> (
    SwiftRemitContractClient<'a>,
    Address,
    token::StellarAssetClient<'a>,
) {
    let admin = Address::generate(env);
    let token_client = token::StellarAssetClient::new(
        env,
        &env.register_stellar_asset_contract_v2(admin.clone())
            .address(),
    );
    let contract =
        SwiftRemitContractClient::new(env, &env.register_contract(None, SwiftRemitContract {}));
    contract.initialize(&admin, &token_client.address, &250, &0, &0, &admin);
    (contract, admin, token_client)
}

#[test]
fn test_blacklist_user_requires_admin_auth() {
    let env = Env::default();
    env.mock_all_auths();

    let (contract, admin, _) = setup(&env);
    let user = Address::generate(&env);

    contract.blacklist_user(&user);

    assert!(contract.is_user_blacklisted(&user));
    assert_eq!(env.auths().len(), 1);
    assert_eq!(env.auths()[0].0, admin);
}

#[test]
fn test_blacklisted_sender_cannot_create_remittance() {
    let env = Env::default();
    env.mock_all_auths();

    let (contract, _admin, token) = setup(&env);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    token.mint(&sender, &10_000);
    contract.register_agent(&agent);
    contract.blacklist_user(&sender);

    let result = contract.try_create_remittance(&sender, &agent, &1_000, &None, &None, &None);
    assert_eq!(result, Err(Ok(ContractError::UserBlacklisted)));
}

#[test]
fn test_remove_from_blacklist_allows_remittance_again() {
    let env = Env::default();
    env.mock_all_auths();

    let (contract, admin, token) = setup(&env);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);

    token.mint(&sender, &10_000);
    contract.register_agent(&agent);
    contract.blacklist_user(&sender);
    contract.remove_from_blacklist(&sender);

    assert_eq!(env.auths().len(), 1);
    assert_eq!(env.auths()[0].0, admin);

    let remittance_id = contract.create_remittance(&sender, &agent, &1_000, &None, &None, &None);
    let remittance = contract.get_remittance(&remittance_id);

    assert_eq!(remittance.sender, sender);
    assert_eq!(env.auths().len(), 1);
    assert_eq!(env.auths()[0].0, sender);

    let events = env.events().all();
    let added = events.iter().any(|event| {
        let topic0 = event
            .1
            .get(0)
            .and_then(|topic| Symbol::try_from_val(&env, &topic).ok());
        let topic1 = event
            .1
            .get(1)
            .and_then(|topic| Symbol::try_from_val(&env, &topic).ok());

        topic0 == Some(symbol_short!("blacklist")) && topic1 == Some(symbol_short!("added"))
    });
    let removed = events.iter().any(|event| {
        let topic0 = event
            .1
            .get(0)
            .and_then(|topic| Symbol::try_from_val(&env, &topic).ok());
        let topic1 = event
            .1
            .get(1)
            .and_then(|topic| Symbol::try_from_val(&env, &topic).ok());

        topic0 == Some(symbol_short!("blacklist")) && topic1 == Some(symbol_short!("removed"))
    });

    assert!(added, "blacklist added event was not emitted");
    assert!(removed, "blacklist removed event was not emitted");
}
