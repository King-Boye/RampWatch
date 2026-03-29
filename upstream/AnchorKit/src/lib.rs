#![no_std]
extern crate alloc;

mod domain_validator;
mod errors;
mod rate_limiter;
mod response_validator;
mod transaction_state_tracker;
pub mod sep6;
pub mod contract;

pub use domain_validator::validate_anchor_domain;
pub use errors::{AnchorKitError, ErrorCode};

/// Backward-compatible alias. Prefer [`AnchorKitError`] for new code.
pub use errors::Error;
pub use rate_limiter::{RateLimiter, RateLimitConfig, RateLimitState};
pub use response_validator::{
    validate_anchor_info_response, validate_deposit_response, validate_quote_response,
    validate_withdraw_response, AnchorInfoResponse, DepositResponse as ValidatorDepositResponse,
    QuoteResponse, WithdrawResponse,
};

#[cfg(test)]
mod transaction_state_tracker_tests;
pub use sep6::{
    fetch_transaction_status, initiate_deposit, initiate_withdrawal, DepositResponse,
    RawDepositResponse, RawTransactionResponse, RawWithdrawalResponse, TransactionKind,
    TransactionStatus, TransactionStatusResponse, WithdrawalResponse,
};
pub use contract::AnchorKitContract;

#[cfg(test)]
mod request_id_tests;

#[cfg(test)]
mod tracing_span_tests;

#[cfg(test)]
mod metadata_cache_tests;

#[cfg(test)]
mod streaming_flow_tests;

#[cfg(test)]
mod webhook_middleware_tests;

#[cfg(test)]
mod session_tests;

#[cfg(test)]
mod anchor_info_discovery_tests;

#[cfg(test)]
mod routing_tests;

#[cfg(test)]
mod capability_detection_tests;
