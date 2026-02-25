//! Fee strategy module for flexible fee calculation.
//!
//! Supports multiple fee strategies that can be configured at runtime:
//! - Percentage: Fee based on percentage of amount (basis points)
//! - Flat: Fixed fee regardless of amount
//! - Dynamic: Fee varies based on amount tiers
//!
//! Note: Fee calculation logic has been moved to fee_service.rs for centralization.
//! This module only defines the FeeStrategy enum.

use soroban_sdk::contracttype;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FeeStrategy {
    /// Percentage-based fee (basis points)
    Percentage(u32),
    /// Flat fee amount
    Flat(i128),
    /// Dynamic tiered fee: (threshold, fee_bps)
    Dynamic(u32),
}
