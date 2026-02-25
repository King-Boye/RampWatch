use crate::types::ServiceType;
use soroban_sdk::{contracttype, symbol_short, Address, BytesN, Env, String, Vec};

// --- EXISTING ATTESTOR EVENTS ---

pub struct AttestorAdded;
impl AttestorAdded {
    pub fn publish(env: &Env, attestor: &Address) {
        env.events().publish(
            (symbol_short!("attestor"), symbol_short!("added"), attestor),
            (),
        );
    }
}

pub struct AttestorRemoved;
impl AttestorRemoved {
    pub fn publish(env: &Env, attestor: &Address) {
        env.events().publish(
            (
                symbol_short!("attestor"),
                symbol_short!("removed"),
                attestor,
            ),
            (),
        );
    }
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AttestationRecordedData {
    pub timestamp: u64,
    pub payload_hash: BytesN<32>,
}

pub struct AttestationRecorded;
impl AttestationRecorded {
    pub fn publish(
        env: &Env,
        id: u64,
        subject: &Address,
        timestamp: u64,
        payload_hash: BytesN<32>,
    ) {
        env.events().publish(
            (
                symbol_short!("attest"),
                symbol_short!("recorded"),
                id,
                subject,
            ),
            AttestationRecordedData {
                timestamp,
                payload_hash,
            },
        );
    }
}

// --- CONFIGURATION EVENTS ---

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EndpointConfigured {
    pub attestor: Address,
    pub url: String,
}

impl EndpointConfigured {
    pub fn publish(&self, env: &Env) {
        env.events().publish(
            (symbol_short!("endpoint"), symbol_short!("config")),
            self.clone(),
        );
    }
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EndpointRemoved {
    pub attestor: Address,
}

impl EndpointRemoved {
    pub fn publish(&self, env: &Env) {
        env.events().publish(
            (symbol_short!("endpoint"), symbol_short!("removed")),
            self.clone(),
        );
    }
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ServicesConfigured {
    pub anchor: Address,
    pub services: Vec<ServiceType>,
}

impl ServicesConfigured {
    pub fn publish(&self, env: &Env) {
        env.events().publish(
            (symbol_short!("services"), symbol_short!("config")),
            self.clone(),
        );
    }
}

// --- QUOTE & SESSION EVENTS ---

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuoteSubmitted {
    pub anchor: Address,
    pub quote_id: u64,
    pub base_asset: String,
    pub quote_asset: String,
    pub rate: u64,
    pub valid_until: u64,
}

impl QuoteSubmitted {
    pub fn publish(
        env: &Env,
        anchor: &Address,
        quote_id: u64,
        base_asset: &String,
        quote_asset: &String,
        rate: u64,
        valid_until: u64,
    ) {
        env.events().publish(
            (symbol_short!("quote"), symbol_short!("submit"), quote_id),
            QuoteSubmitted {
                anchor: anchor.clone(),
                quote_id,
                base_asset: base_asset.clone(),
                quote_asset: quote_asset.clone(),
                rate,
                valid_until,
            },
        );
    }
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionCreated {
    pub session_id: u64,
    pub initiator: Address,
    pub timestamp: u64,
}

impl SessionCreated {
    pub fn publish(env: &Env, session_id: u64, initiator: &Address, timestamp: u64) {
        env.events().publish(
            (
                symbol_short!("session"),
                symbol_short!("created"),
                session_id,
            ),
            SessionCreated {
                session_id,
                initiator: initiator.clone(),
                timestamp,
            },
        );
    }
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationLogged {
    pub log_id: u64,
    pub session_id: u64,
    pub operation_index: u64,
    pub operation_type: String,
    pub status: String,
}

impl OperationLogged {
    pub fn publish(
        env: &Env,
        log_id: u64,
        session_id: u64,
        operation_index: u64,
        operation_type: &String,
        status: &String,
    ) {
        env.events().publish(
            (symbol_short!("audit"), symbol_short!("logged"), log_id),
            OperationLogged {
                log_id,
                session_id,
                operation_index,
                operation_type: operation_type.clone(),
                status: status.clone(),
            },
        );
    }
}

// --- NEW LIFECYCLE EVENTS ---

/// Event emitted when an app or user receives/fetches a specific quote.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuoteReceived {
    pub quote_id: u64,
    pub receiver: Address,
    pub timestamp: u64,
}

impl QuoteReceived {
    pub fn publish(env: &Env, quote_id: u64, receiver: &Address, timestamp: u64) {
        env.events().publish(
            (symbol_short!("quote"), symbol_short!("received"), quote_id),
            QuoteReceived {
                quote_id,
                receiver: receiver.clone(),
                timestamp,
            },
        );
    }
}

/// Event emitted when a transfer operation starts.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransferInitiated {
    pub transfer_id: u64,
    pub sender: Address,
    pub destination: Address,
    pub amount: i128,
}

impl TransferInitiated {
    pub fn publish(
        env: &Env,
        transfer_id: u64,
        sender: &Address,
        destination: &Address,
        amount: i128,
    ) {
        env.events().publish(
            (
                symbol_short!("transfer"),
                symbol_short!("init"),
                transfer_id,
            ),
            TransferInitiated {
                transfer_id,
                sender: sender.clone(),
                destination: destination.clone(),
                amount,
            },
        );
    }
}

/// Event emitted when the settlement of a transfer is finalized.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SettlementConfirmed {
    pub transfer_id: u64,
    pub settlement_ref: BytesN<32>,
    pub timestamp: u64,
}

impl SettlementConfirmed {
    pub fn publish(env: &Env, transfer_id: u64, settlement_ref: BytesN<32>, timestamp: u64) {
        env.events().publish(
            (
                symbol_short!("settle"),
                symbol_short!("confirm"),
                transfer_id,
            ),
            SettlementConfirmed {
                transfer_id,
                settlement_ref,
                timestamp,
            },
        );
    }
}

// --- RATE LIMIT EVENTS ---

/// Event emitted when a 429 rate limit response is detected
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RateLimitEncountered {
    pub source: u32,          // RateLimitSource as u32
    pub retry_after_seconds: u64,
    pub remaining: Option<u32>,
    pub reset_at: Option<u64>,
    pub attempt: u32,
    pub timestamp: u64,
}

impl RateLimitEncountered {
    pub fn publish(
        env: &Env,
        source: u32,
        retry_after_seconds: u64,
        remaining: Option<u32>,
        reset_at: Option<u64>,
        attempt: u32,
    ) {
        env.events().publish(
            (
                symbol_short!("rate"),
                symbol_short!("limit"),
                attempt,
            ),
            RateLimitEncountered {
                source,
                retry_after_seconds,
                remaining,
                reset_at,
                attempt,
                timestamp: env.ledger().timestamp(),
            },
        );
    }
}

/// Event emitted when backoff is applied due to rate limiting
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RateLimitBackoff {
    pub delay_ms: u64,
    pub attempt: u32,
    pub timestamp: u64,
}

impl RateLimitBackoff {
    pub fn publish(env: &Env, delay_ms: u64, attempt: u32) {
        env.events().publish(
            (
                symbol_short!("rate"),
                symbol_short!("backoff"),
                attempt,
            ),
            RateLimitBackoff {
                delay_ms,
                attempt,
                timestamp: env.ledger().timestamp(),
            },
        );
    }
}

/// Event emitted when a retry succeeds after rate limiting
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RateLimitRecovered {
    pub attempts_made: u32,
    pub total_backoff_ms: u64,
    pub timestamp: u64,
}

impl RateLimitRecovered {
    pub fn publish(env: &Env, attempts_made: u32, total_backoff_ms: u64) {
        env.events().publish(
            (
                symbol_short!("rate"),
                symbol_short!("recover"),
            ),
            RateLimitRecovered {
                attempts_made,
                total_backoff_ms,
                timestamp: env.ledger().timestamp(),
            },
        );
    }
}
