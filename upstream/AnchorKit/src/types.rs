use soroban_sdk::{contracttype, Address, Bytes, BytesN, String, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Attestation {
    pub id: u64,
    pub issuer: Address,
    pub subject: Address,
    pub timestamp: u64,
    pub payload_hash: BytesN<32>,
    pub signature: Bytes,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Endpoint {
    pub url: String,
    pub attestor: Address,
    pub is_active: bool,
}

/// Supported service types for anchors
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ServiceType {
    Deposits = 1,
    Withdrawals = 2,
    Quotes = 3,
    KYC = 4,
}

/// Configuration of supported services for an anchor
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnchorServices {
    pub anchor: Address,
    pub services: Vec<ServiceType>,
}
