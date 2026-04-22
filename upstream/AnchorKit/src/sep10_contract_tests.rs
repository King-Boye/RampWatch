#![cfg(test)]

mod sep10_contract_tests {
    use ed25519_dalek::{Signer, SigningKey};
    use rand::rngs::OsRng;
    use soroban_sdk::testutils::{Address as _, Ledger, LedgerInfo};
    use soroban_sdk::{Address, Bytes, Env, String};

    use crate::contract::{AnchorKitContract, AnchorKitContractClient};
    use crate::sep10_test_util::{build_sep10_jwt, register_attestor_with_sep10};

    fn make_env() -> Env {
        let env = Env::default();
        env.mock_all_auths();
        env
    }

    fn ledger(env: &Env, ts: u64) {
        env.ledger().set(LedgerInfo {
            timestamp: ts,
            protocol_version: 21,
            sequence_number: 0,
            network_id: Default::default(),
            base_reserve: 0,
            min_persistent_entry_ttl: 4096,
            min_temp_entry_ttl: 16,
            max_entry_ttl: 6312000,
        });
    }

    #[test]
    fn contract_verify_sep10_token_succeeds() {
        let env = make_env();
        ledger(&env, 1000);
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        let issuer = Address::generate(&env);
        client.initialize(&admin);

        let sk = SigningKey::generate(&mut OsRng);
        let pk = Bytes::from_slice(&env, sk.verifying_key().as_bytes());
        client.set_sep10_jwt_verifying_key(&issuer, &pk);

        let attestor = Address::generate(&env);
        let sub = attestor.to_string();
        let mut buf = [0u8; 128];
        let len = sub.len() as usize;
        let final_len = if len > 128 { 128 } else { len };
        sub.copy_into_slice(&mut buf[..final_len]);
        let sub_std = core::str::from_utf8(&buf[..final_len]).unwrap_or("");
        let jwt = build_sep10_jwt(&sk, sub_std, 2000);
        let token = String::from_str(&env, jwt.as_str());
        client.verify_sep10_token(&token, &issuer);
    }

    #[test]
    fn contract_register_attestor_with_sep10_roundtrip() {
        let env = make_env();
        ledger(&env, 0);
        let contract_id = env.register_contract(None, AnchorKitContract);
        let client = AnchorKitContractClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        let attestor = Address::generate(&env);
        let issuer = Address::generate(&env);
        client.initialize(&admin);

        let sk = SigningKey::generate(&mut OsRng);
        register_attestor_with_sep10(&env, &client, &attestor, &issuer, &sk);
        assert!(client.is_attestor(&attestor));
    }
}
