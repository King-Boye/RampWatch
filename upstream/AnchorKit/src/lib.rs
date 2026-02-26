#![no_std]
extern crate alloc;

mod domain_validator;
mod errors;

pub use domain_validator::validate_anchor_domain;
pub use errors::Error;
