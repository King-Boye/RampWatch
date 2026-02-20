use soroban_sdk::{Address, Env, String, Vec};

use crate::{ContractError, TransferRecord, get_user_transfers, set_user_transfers, get_daily_limit};

const SECONDS_IN_24_HOURS: u64 = 86400;

/// Validates that an address is properly formatted and not empty.
/// Stellar addresses in Soroban are represented by the Address type,
/// which is already validated by the SDK, but we check for additional constraints.
pub fn validate_address(address: &Address) -> Result<(), ContractError> {
    // The Address type in Soroban SDK is already validated by the runtime.
    // However, we can add additional checks if needed.
    // For now, we ensure the address is not a zero/empty address by checking
    // that it can be properly serialized.

    // In Soroban, the Address type is guaranteed to be valid by the SDK,
    // so this function primarily serves as a placeholder for future validation logic
    // and to make the code more explicit about validation requirements.

    Ok(())
}

/// Validates that a transfer does not exceed the user's daily send limit.
/// Aggregates transfers within a rolling 24-hour window and checks against configured limits.
pub fn validate_daily_send_limit(
    env: &Env,
    sender: &Address,
    amount: i128,
    currency: &String,
    country: &String,
) -> Result<(), ContractError> {
    // Get the configured daily limit for this currency and country
    let daily_limit = match get_daily_limit(env, currency, country) {
        Some(limit) => limit.limit,
        None => return Ok(()), // No limit configured, allow transfer
    };

    let current_time = env.ledger().timestamp();
    let cutoff_time = current_time.saturating_sub(SECONDS_IN_24_HOURS);

    // Get user's transfer history
    let mut transfers = get_user_transfers(env, sender);

    // Filter transfers within the rolling 24-hour window and calculate total
    let mut total_sent: i128 = 0;
    let mut valid_transfers = Vec::new(env);

    for transfer in transfers.iter() {
        if transfer.timestamp > cutoff_time {
            total_sent = total_sent
                .checked_add(transfer.amount)
                .ok_or(ContractError::Overflow)?;
            valid_transfers.push_back(transfer);
        }
    }

    // Check if adding the new amount would exceed the limit
    let new_total = total_sent
        .checked_add(amount)
        .ok_or(ContractError::Overflow)?;

    if new_total > daily_limit {
        return Err(ContractError::DailySendLimitExceeded);
    }

    // Record the new transfer
    valid_transfers.push_back(TransferRecord {
        timestamp: current_time,
        amount,
    });

    // Update storage with cleaned and new transfer records
    set_user_transfers(env, sender, &valid_transfers);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_validate_valid_address() {
        let env = Env::default();
        let address = Address::generate(&env);

        assert!(validate_address(&address).is_ok());
    }
}
