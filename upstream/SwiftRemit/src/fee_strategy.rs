//! Fee strategy module for flexible fee calculation.
//!
//! Supports multiple fee strategies that can be configured at runtime:
//! - Percentage: Fee based on percentage of amount (basis points)
//! - Flat: Fixed fee regardless of amount
//! - Dynamic: Fee varies based on amount tiers

use soroban_sdk::{contracttype, Env};
use crate::ContractError;

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

/// Calculate fee based on configured strategy
pub fn calculate_fee(env: &Env, strategy: &FeeStrategy, amount: i128) -> Result<i128, ContractError> {
    match strategy {
        FeeStrategy::Percentage(bps) => {
            if *bps > 10000 {
                return Err(ContractError::InvalidFeeBps);
            }
            amount
                .checked_mul(*bps as i128)
                .ok_or(ContractError::Overflow)?
                .checked_div(10000)
                .ok_or(ContractError::Overflow)
        }
        FeeStrategy::Flat(fee) => {
            if *fee < 0 {
                return Err(ContractError::InvalidAmount);
            }
            Ok(*fee)
        }
        FeeStrategy::Dynamic(base_bps) => {
            // Tiered: <1000 = base_bps, 1000-10000 = base_bps/2, >10000 = base_bps/4
            let bps = if amount < 1000 {
                *base_bps
            } else if amount < 10000 {
                base_bps / 2
            } else {
                base_bps / 4
            };
            
            if bps > 10000 {
                return Err(ContractError::InvalidFeeBps);
            }
            
            amount
                .checked_mul(bps as i128)
                .ok_or(ContractError::Overflow)?
                .checked_div(10000)
                .ok_or(ContractError::Overflow)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_percentage_strategy() {
        let env = Env::default();
        let strategy = FeeStrategy::Percentage(250); // 2.5%
        assert_eq!(calculate_fee(&env, &strategy, 10000).unwrap(), 250);
    }

    #[test]
    fn test_flat_strategy() {
        let env = Env::default();
        let strategy = FeeStrategy::Flat(100);
        assert_eq!(calculate_fee(&env, &strategy, 10000).unwrap(), 100);
        assert_eq!(calculate_fee(&env, &strategy, 1000).unwrap(), 100);
    }

    #[test]
    fn test_dynamic_strategy() {
        let env = Env::default();
        let strategy = FeeStrategy::Dynamic(400); // 4% base
        
        // <1000: 4%
        assert_eq!(calculate_fee(&env, &strategy, 500).unwrap(), 20);
        // 1000-10000: 2%
        assert_eq!(calculate_fee(&env, &strategy, 5000).unwrap(), 100);
        // >10000: 1%
        assert_eq!(calculate_fee(&env, &strategy, 20000).unwrap(), 200);
    }
}
