use soroban_sdk::{contracttype, Address, Env, String, Map};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InteractiveUrl {
    pub url: String,
    pub transaction_id: String,
    pub expires_at: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CallbackData {
    pub transaction_id: String,
    pub status: String,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransactionStatus {
    pub id: String,
    pub status: String,
    pub updated_at: u64,
}

pub struct InteractiveSupport;

impl InteractiveSupport {
    pub fn generate_url(env: &Env, _anchor: &Address, _token: &String, tx_id: &String) -> InteractiveUrl {
        InteractiveUrl {
            url: String::from_str(env, "https://anchor.example.com/interactive"),
            transaction_id: tx_id.clone(),
            expires_at: env.ledger().timestamp() + 1800,
        }
    }

    pub fn inject_token(env: &Env, _token: &String) -> Map<String, String> {
        let mut headers = Map::new(env);
        headers.set(String::from_str(env, "Authorization"), 
                   String::from_str(env, "Bearer token"));
        headers
    }

    pub fn handle_callback(env: &Env, tx_id: &String, status: &String) -> CallbackData {
        CallbackData {
            transaction_id: tx_id.clone(),
            status: status.clone(),
            timestamp: env.ledger().timestamp(),
        }
    }

    pub fn poll_status(env: &Env, tx_id: &String) -> TransactionStatus {
        TransactionStatus {
            id: tx_id.clone(),
            status: String::from_str(env, "pending"),
            updated_at: env.ledger().timestamp(),
        }
    }
}
