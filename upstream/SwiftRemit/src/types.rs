use soroban_sdk::{contracttype, Address, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RemittanceStatus {
    Pending,
    Completed,
    Cancelled,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Remittance {
    pub id: u64,
    pub sender: Address,
    pub agent: Address,
    pub amount: i128,
    pub fee: i128,
    pub status: RemittanceStatus,
    pub expiry: Option<u64>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DailyLimit {
    pub currency: String,
    pub country: String,
    pub limit: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransferRecord {
    pub timestamp: u64,
    pub amount: i128,
}
