use soroban_sdk::{contracttype, Address, BytesN, Env, String, Vec};

/// Represents a single API call in the request history
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApiCallRecord {
    pub call_id: u64,
    pub request_id: BytesN<16>,
    pub operation: String,
    pub caller: Address,
    pub timestamp: u64,
    pub status: ApiCallStatus,
    pub duration_ms: u64,
    pub error_code: Option<u32>,
}

/// Status of an API call
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, Copy)]
#[repr(u32)]
pub enum ApiCallStatus {
    Success = 1,
    Failed = 2,
    Pending = 3,
}

/// Detailed information about an API call
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApiCallDetails {
    pub record: ApiCallRecord,
    pub target_address: Option<Address>,
    pub amount: Option<u64>,
    pub result_data: Option<String>,
}

/// Request history panel data
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequestHistoryPanel {
    pub recent_calls: Vec<ApiCallRecord>,
    pub total_calls: u64,
    pub success_count: u64,
    pub failed_count: u64,
    pub last_updated: u64,
}

impl ApiCallRecord {
    pub fn new(
        env: &Env,
        call_id: u64,
        request_id: BytesN<16>,
        operation: String,
        caller: Address,
        status: ApiCallStatus,
        duration_ms: u64,
    ) -> Self {
        Self {
            call_id,
            request_id,
            operation,
            caller,
            timestamp: env.ledger().timestamp(),
            status,
            duration_ms,
            error_code: None,
        }
    }

    pub fn with_error(mut self, error_code: u32) -> Self {
        self.error_code = Some(error_code);
        self
    }
}

pub struct RequestHistory;

impl RequestHistory {
    const MAX_HISTORY_SIZE: u32 = 100;

    /// Record a new API call
    pub fn record_call(env: &Env, record: &ApiCallRecord) {
        let call_id = record.call_id;
        
        // Store the individual record
        let key = (soroban_sdk::symbol_short!("CALL"), call_id);
        env.storage().temporary().set(&key, record);
        env.storage().temporary().extend_ttl(&key, 17280, 17280); // 1 day

        // Add to recent calls list
        Self::add_to_recent_list(env, call_id);

        // Update statistics
        Self::update_statistics(env, &record.status);
    }

    /// Get a specific API call record by ID
    pub fn get_call(env: &Env, call_id: u64) -> Option<ApiCallRecord> {
        let key = (soroban_sdk::symbol_short!("CALL"), call_id);
        env.storage().temporary().get(&key)
    }

    /// Get recent API calls (up to limit)
    pub fn get_recent_calls(env: &Env, limit: u32) -> Vec<ApiCallRecord> {
        let recent_ids = Self::get_recent_list(env);
        let mut calls: Vec<ApiCallRecord> = Vec::new(env);
        
        let max_items = limit.min(recent_ids.len());
        for i in 0..max_items {
            if let Some(call_id) = recent_ids.get(i) {
                if let Some(record) = Self::get_call(env, call_id) {
                    calls.push_back(record);
                }
            }
        }
        
        calls
    }

    /// Get request history panel data
    pub fn get_panel_data(env: &Env, limit: u32) -> RequestHistoryPanel {
        let recent_calls = Self::get_recent_calls(env, limit);
        let stats = Self::get_statistics(env);
        
        RequestHistoryPanel {
            recent_calls,
            total_calls: stats.0,
            success_count: stats.1,
            failed_count: stats.2,
            last_updated: env.ledger().timestamp(),
        }
    }

    /// Store detailed information about an API call
    pub fn store_call_details(env: &Env, details: &ApiCallDetails) {
        let key = (soroban_sdk::symbol_short!("DETAIL"), details.record.call_id);
        env.storage().temporary().set(&key, details);
        env.storage().temporary().extend_ttl(&key, 17280, 17280); // 1 day
    }

    /// Get detailed information about an API call
    pub fn get_call_details(env: &Env, call_id: u64) -> Option<ApiCallDetails> {
        let key = (soroban_sdk::symbol_short!("DETAIL"), call_id);
        env.storage().temporary().get(&key)
    }

    /// Get next call ID
    pub fn get_next_call_id(env: &Env) -> u64 {
        let key = soroban_sdk::symbol_short!("CALL_CTR");
        let counter: u64 = env.storage().persistent().get(&key).unwrap_or(0);
        env.storage().persistent().set(&key, &(counter + 1));
        counter
    }

    // Private helper methods

    fn add_to_recent_list(env: &Env, call_id: u64) {
        let key = soroban_sdk::symbol_short!("RECENT");
        let mut recent: Vec<u64> = env.storage().temporary().get(&key).unwrap_or(Vec::new(env));
        
        // Add to front of list
        recent.push_front(call_id);
        
        // Trim to max size
        while recent.len() > Self::MAX_HISTORY_SIZE {
            recent.pop_back();
        }
        
        env.storage().temporary().set(&key, &recent);
        env.storage().temporary().extend_ttl(&key, 17280, 17280); // 1 day
    }

    fn get_recent_list(env: &Env) -> Vec<u64> {
        let key = soroban_sdk::symbol_short!("RECENT");
        env.storage().temporary().get(&key).unwrap_or(Vec::new(env))
    }

    fn update_statistics(env: &Env, status: &ApiCallStatus) {
        let key = soroban_sdk::symbol_short!("STATS");
        let stats: (u64, u64, u64) = env.storage().temporary().get(&key).unwrap_or((0, 0, 0));
        
        let new_stats = match status {
            ApiCallStatus::Success => (stats.0 + 1, stats.1 + 1, stats.2),
            ApiCallStatus::Failed => (stats.0 + 1, stats.1, stats.2 + 1),
            ApiCallStatus::Pending => (stats.0 + 1, stats.1, stats.2),
        };
        
        env.storage().temporary().set(&key, &new_stats);
        env.storage().temporary().extend_ttl(&key, 17280, 17280); // 1 day
    }

    fn get_statistics(env: &Env) -> (u64, u64, u64) {
        let key = soroban_sdk::symbol_short!("STATS");
        env.storage().temporary().get(&key).unwrap_or((0, 0, 0))
    }
}
