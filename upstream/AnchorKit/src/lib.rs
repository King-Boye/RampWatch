#![no_std]
extern crate alloc;

mod domain_validator;
mod errors;
pub mod sep6;

pub use domain_validator::validate_anchor_domain;
pub use errors::Error;
pub use sep6::{
    fetch_transaction_status, initiate_deposit, initiate_withdrawal, DepositResponse,
    RawDepositResponse, RawTransactionResponse, RawWithdrawalResponse, TransactionKind,
    TransactionStatus, TransactionStatusResponse, WithdrawalResponse,
};
