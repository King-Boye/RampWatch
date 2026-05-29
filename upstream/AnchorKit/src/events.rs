use soroban_sdk::{contracttype, Address, Bytes, String};

#[contracttype]
#[derive(Clone)]
pub struct SessionCreatedEvent {
    pub session_id: u64,
    pub initiator: Address,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone)]
pub struct QuoteSubmitEvent {
    pub quote_id: u64,
    pub anchor: Address,
    pub base_asset: String,
    pub quote_asset: String,
    pub rate: u64,
    pub valid_until: u64,
}

#[contracttype]
#[derive(Clone)]
pub struct QuoteReceivedEvent {
    pub quote_id: u64,
    pub receiver: Address,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone)]
pub struct AuditLogEvent {
    pub log_id: u64,
    pub session_id: u64,
    pub operation_index: u64,
    pub operation_type: String,
    pub status: String,
}

#[contracttype]
#[derive(Clone)]
pub struct AttestEvent {
    pub payload_hash: Bytes,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone)]
pub struct AuditLogPruned {
    pub pruned_count: u64,
    pub new_offset: u64,
}

#[contracttype]
#[derive(Clone)]
pub struct EndpointUpdated {
    pub attestor: Address,
    pub endpoint: String,
}

#[contracttype]
#[derive(Clone)]
pub struct AnchorDeactivated {
    pub anchor: Address,
    pub failure_count: u32,
    pub threshold: u32,
}

#[contracttype]
#[derive(Clone)]
pub struct RateLimitReset {
    pub attestor: Address,
    pub admin: Address,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone)]
pub struct RateLimitWindowReset {
    pub attestor: Address,
    pub window_start: u64,
}

#[contracttype]
#[derive(Clone)]
pub struct RoutingDecisionEvent {
    pub anchor: Address,
    pub strategy: String,
    pub quote_id: u64,
    pub ledger_sequence: u32,
}

#[contracttype]
#[derive(Clone)]
pub struct QuoteExpiredEvent {
    pub anchor: Address,
    pub quote_id: u64,
    pub valid_until: u64,
}
