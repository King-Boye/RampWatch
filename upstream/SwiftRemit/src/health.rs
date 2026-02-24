use soroban_sdk::contracttype;

/// Health check response for contract monitoring.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HealthStatus {
    pub operational: bool,
    pub timestamp: u64,
    pub initialized: bool,
}
