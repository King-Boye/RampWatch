#![no_std]
extern crate alloc;

mod domain_validator;
mod errors;
mod rate_limiter;
mod response_validator;
mod transaction_state_tracker;

pub use domain_validator::validate_anchor_domain;
pub use errors::{AnchorKitError, ErrorCode};

/// Backward-compatible alias. Prefer [`AnchorKitError`] for new code.
pub use errors::Error;
pub use rate_limiter::{RateLimiter, RateLimitConfig, RateLimitState};
pub use response_validator::{
    validate_anchor_info_response, validate_deposit_response, validate_quote_response,
    validate_withdraw_response, AnchorInfoResponse, DepositResponse, QuoteResponse,
    WithdrawResponse,
};

#[cfg(test)]
mod transaction_state_tracker_tests;
