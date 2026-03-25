//! Error types for AnchorKit

#![cfg_attr(not(test), no_std)]

use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    AttestorAlreadyRegistered = 2,
    AttestorNotRegistered = 3,
    UnauthorizedAttestor = 4,
    InvalidTimestamp = 5,
    ReplayAttack = 6,
    InvalidQuote = 7,
    InvalidServiceType = 8,
    InvalidTransactionIntent = 9,
    StaleQuote = 10,
    ComplianceNotMet = 11,
    InvalidEndpointFormat = 12,
    NoQuotesAvailable = 13,
    ServicesNotConfigured = 14,
    ValidationError = 15,
}
