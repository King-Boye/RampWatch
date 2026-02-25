#[cfg(test)]
mod tests {
    use crate::sep10_auth::*;
    use soroban_sdk::{testutils::Address as _, Address, BytesN, Env, String};

    #[test]
    fn test_fetch_challenge() {
        let env = Env::default();
        let anchor = Address::generate(&env);
        let client = Address::generate(&env);

        let challenge = fetch_challenge(&env, anchor, client);
        
        assert_eq!(challenge.transaction, String::from_str(&env, "challenge_tx_xdr"));
        assert_eq!(challenge.network_passphrase, String::from_str(&env, "Test SDF Network ; September 2015"));
    }

    #[test]
    fn test_verify_signature() {
        let env = Env::default();
        let challenge = Sep10Challenge {
            transaction: String::from_str(&env, "test_tx"),
            network_passphrase: String::from_str(&env, "Test SDF Network ; September 2015"),
        };
        let signature = BytesN::from_array(&env, &[0u8; 64]);
        let public_key = BytesN::from_array(&env, &[0u8; 32]);

        assert!(verify_signature(&env, &challenge, signature, public_key));
    }
}
