use soroban_sdk::{contracttype, Address, Bytes, Env, String, Vec};
use crate::types::{ServiceType, QuoteRequest, QuoteData};

/// Authentication result containing credentials and session info
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthResult {
    pub token: String,
    pub expires_at: u64,
    pub anchor: Address,
}

/// Deposit request parameters
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DepositRequest {
    pub asset: String,
    pub amount: u64,
    pub destination: Address,
    pub memo: Option<String>,
}

/// Deposit response with transaction details
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DepositResponse {
    pub transaction_id: String,
    pub status: String,
    pub deposit_address: String,
    pub expires_at: u64,
}

/// Withdrawal request parameters
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WithdrawRequest {
    pub asset: String,
    pub amount: u64,
    pub destination: String,
    pub memo: Option<String>,
}

/// Withdrawal response with transaction details
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WithdrawResponse {
    pub transaction_id: String,
    pub status: String,
    pub estimated_completion: u64,
}

/// Anchor information and capabilities
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnchorInfo {
    pub name: String,
    pub supported_services: Vec<ServiceType>,
    pub supported_assets: Vec<String>,
    pub min_deposit: u64,
    pub max_deposit: u64,
    pub min_withdrawal: u64,
    pub max_withdrawal: u64,
}

/// Unified interface for anchor integrations
#[allow(dead_code)]
pub trait AnchorAdapter {
    /// Authenticate with the anchor and obtain credentials
    fn authenticate(&self, env: &Env, anchor: &Address, credentials: &Bytes) -> AuthResult;
    
    /// Initiate a deposit transaction
    fn deposit(&self, env: &Env, auth: &AuthResult, request: &DepositRequest) -> DepositResponse;
    
    /// Initiate a withdrawal transaction
    fn withdraw(&self, env: &Env, auth: &AuthResult, request: &WithdrawRequest) -> WithdrawResponse;
    
    /// Get anchor information and capabilities
    fn get_info(&self, env: &Env, anchor: &Address) -> AnchorInfo;
    
    /// Get a quote for an exchange (optional, returns None if not supported)
    fn get_quote(&self, env: &Env, auth: &AuthResult, request: &QuoteRequest) -> Option<QuoteData>;
}
