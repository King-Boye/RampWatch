#![no_std]
extern crate alloc;

mod domain_validator;
mod errors;
mod response_validator;

pub use domain_validator::validate_anchor_domain;
pub use errors::Error;
pub use response_validator::{
    validate_anchor_info_response, validate_deposit_response, validate_quote_response,
    validate_withdraw_response, AnchorInfoResponse, DepositResponse, QuoteResponse,
    WithdrawResponse,
};
