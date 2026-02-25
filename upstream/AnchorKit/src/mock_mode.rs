use crate::anchor_adapter::{
    AnchorAdapter, AnchorInfo, AuthResult, DepositRequest, DepositResponse, WithdrawRequest,
    WithdrawResponse,
};
use crate::types::{QuoteData, QuoteRequest, ServiceType};
use soroban_sdk::{Address, Bytes, Env, String, Vec};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct MockAnchor {
    delays: Arc<Mutex<HashMap<String, u64>>>,
    responses: Arc<Mutex<HashMap<String, String>>>,
}

impl MockAnchor {
    pub fn new() -> Self {
        Self {
            delays: Arc::new(Mutex::new(HashMap::new())),
            responses: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn set_delay(&self, operation: &str, seconds: u64) {
        self.delays.lock().unwrap().insert(operation.to_string(), seconds);
    }

    pub fn set_response(&self, tx_id: &str, status: &str) {
        self.responses.lock().unwrap().insert(tx_id.to_string(), status.to_string());
    }

    fn simulate_delay(&self, operation: &str) {
        if let Some(&delay) = self.delays.lock().unwrap().get(operation) {
            std::thread::sleep(std::time::Duration::from_secs(delay));
        }
    }
}

impl Default for MockAnchor {
    fn default() -> Self {
        Self::new()
    }
}

impl AnchorAdapter for MockAnchor {
    fn authenticate(&self, env: &Env, anchor: &Address, _credentials: &Bytes) -> AuthResult {
        self.simulate_delay("auth");
        AuthResult {
            token: String::from_str(env, "mock_token_12345"),
            expires_at: env.ledger().timestamp() + 3600,
            anchor: anchor.clone(),
        }
    }

    fn deposit(&self, env: &Env, _auth: &AuthResult, request: &DepositRequest) -> DepositResponse {
        self.simulate_delay("deposit");
        let tx_id = format!("MOCK_DEP_{}", env.ledger().timestamp());
        let status = self.responses.lock().unwrap()
            .get(&tx_id)
            .cloned()
            .unwrap_or_else(|| "pending".to_string());

        DepositResponse {
            transaction_id: String::from_str(env, &tx_id),
            status: String::from_str(env, &status),
            deposit_address: String::from_str(env, "MOCK_ADDR_DEPOSIT"),
            expires_at: env.ledger().timestamp() + 1800,
        }
    }

    fn withdraw(&self, env: &Env, _auth: &AuthResult, request: &WithdrawRequest) -> WithdrawResponse {
        self.simulate_delay("withdraw");
        let tx_id = format!("MOCK_WD_{}", env.ledger().timestamp());
        let status = self.responses.lock().unwrap()
            .get(&tx_id)
            .cloned()
            .unwrap_or_else(|| "pending".to_string());

        WithdrawResponse {
            transaction_id: String::from_str(env, &tx_id),
            status: String::from_str(env, &status),
            estimated_completion: env.ledger().timestamp() + 600,
        }
    }

    fn get_info(&self, env: &Env, _anchor: &Address) -> AnchorInfo {
        let mut services = Vec::new(env);
        services.push_back(ServiceType::Deposit);
        services.push_back(ServiceType::Withdrawal);

        let mut assets = Vec::new(env);
        assets.push_back(String::from_str(env, "USDC"));
        assets.push_back(String::from_str(env, "EURC"));

        AnchorInfo {
            name: String::from_str(env, "Mock Anchor"),
            supported_services: services,
            supported_assets: assets,
            min_deposit: 100,
            max_deposit: 1000000,
            min_withdrawal: 100,
            max_withdrawal: 1000000,
        }
    }

    fn get_quote(&self, env: &Env, _auth: &AuthResult, request: &QuoteRequest) -> Option<QuoteData> {
        self.simulate_delay("quote");
        Some(QuoteData {
            rate: String::from_str(env, "1.0"),
            fee: 50,
            expires_at: env.ledger().timestamp() + 300,
        })
    }
}

pub struct MockWebhook {
    triggers: Arc<Mutex<Vec<String>>>,
}

impl MockWebhook {
    pub fn new() -> Self {
        Self {
            triggers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn trigger(&self, event: &str) {
        self.triggers.lock().unwrap().push(event.to_string());
    }

    pub fn get_triggers(&self) -> Vec<String> {
        self.triggers.lock().unwrap().clone()
    }

    pub fn clear(&self) {
        self.triggers.lock().unwrap().clear();
    }
}

impl Default for MockWebhook {
    fn default() -> Self {
        Self::new()
    }
}
