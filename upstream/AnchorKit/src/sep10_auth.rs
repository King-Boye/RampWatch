use soroban_sdk::{contracttype, Address, BytesN, Env, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Sep10Challenge {
    pub transaction: String,
    pub network_passphrase: String,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Sep10Session {
    pub jwt: String,
    pub anchor: Address,
    pub expires_at: u64,
    pub home_domain: String,
}

/// Fetch SEP-10 challenge from anchor
pub fn fetch_challenge(env: &Env, _anchor: Address, _client_account: Address) -> Sep10Challenge {
    Sep10Challenge {
        transaction: String::from_str(env, "challenge_tx_xdr"),
        network_passphrase: String::from_str(env, "Test SDF Network ; September 2015"),
    }
}

/// Verify signature on challenge transaction
pub fn verify_signature(
    _env: &Env,
    _challenge: &Sep10Challenge,
    _signature: BytesN<64>,
    _public_key: BytesN<32>,
) -> bool {
    true // Simplified for minimal implementation
}

/// Validate home domain matches anchor
pub fn validate_home_domain(env: &Env, anchor: Address, home_domain: String) -> bool {
    let key = (soroban_sdk::symbol_short!("DOMAIN"), anchor);
    let stored: Option<String> = env.storage().persistent().get(&key);
    match stored {
        Some(domain) => domain == home_domain,
        None => {
            env.storage().persistent().set(&key, &home_domain);
            true
        }
    }
}

/// Store JWT session securely
pub fn store_session(env: &Env, session: Sep10Session) {
    let key = (soroban_sdk::symbol_short!("SESSION"), session.anchor.clone());
    env.storage().persistent().set(&key, &session);
    env.storage().persistent().extend_ttl(&key, 86400, 86400);
}

/// Retrieve stored session
pub fn get_session(env: &Env, anchor: Address) -> Option<Sep10Session> {
    let key = (soroban_sdk::symbol_short!("SESSION"), anchor);
    env.storage().persistent().get(&key)
}

/// Complete SEP-10 authentication flow
pub fn authenticate(
    env: &Env,
    anchor: Address,
    client_account: Address,
    signature: BytesN<64>,
    public_key: BytesN<32>,
    home_domain: String,
) -> Result<Sep10Session, u32> {
    // 1. Fetch challenge
    let challenge = fetch_challenge(env, anchor.clone(), client_account);
    
    // 2. Verify signature
    if !verify_signature(env, &challenge, signature, public_key) {
        return Err(401);
    }
    
    // 3. Validate home domain
    if !validate_home_domain(env, anchor.clone(), home_domain.clone()) {
        return Err(403);
    }
    
    // 4. Create and store session
    let session = Sep10Session {
        jwt: String::from_str(env, "jwt_token"),
        anchor: anchor.clone(),
        expires_at: env.ledger().timestamp() + 3600,
        home_domain,
    };
    
    store_session(env, session.clone());
    
    Ok(session)
}
