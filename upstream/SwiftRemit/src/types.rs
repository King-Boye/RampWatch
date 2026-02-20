use soroban_sdk::{contracttype, Address};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RemittanceStatus {
    Pending,
    Authorized,
    Settled,
    Finalized,
    Failed,
}

impl RemittanceStatus {
    pub fn can_transition_to(&self, next: &RemittanceStatus) -> bool {
        match (self, next) {
            (RemittanceStatus::Pending, RemittanceStatus::Authorized) => true,
            (RemittanceStatus::Pending, RemittanceStatus::Failed) => true,

            (RemittanceStatus::Authorized, RemittanceStatus::Settled) => true,
            (RemittanceStatus::Authorized, RemittanceStatus::Failed) => true,

            (RemittanceStatus::Settled, RemittanceStatus::Finalized) => true,

            // Allow transitions to Failed from any non-terminal state

            _ => false,
        }
    }
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
