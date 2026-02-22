use crate::types::RemittanceStatus;
use crate::errors::ContractError;

/// Validates if a state transition is allowed.
/// Returns Ok(()) if valid, Err(ContractError::InvalidStatus) if invalid.
pub fn validate_transition(
    from: &RemittanceStatus,
    to: &RemittanceStatus,
) -> Result<(), ContractError> {
    match (from, to) {
        // From Pending
        (RemittanceStatus::Pending, RemittanceStatus::Processing) => Ok(()),
        (RemittanceStatus::Pending, RemittanceStatus::Cancelled) => Ok(()),
        
        // From Processing
        (RemittanceStatus::Processing, RemittanceStatus::Completed) => Ok(()),
        (RemittanceStatus::Processing, RemittanceStatus::Failed) => Ok(()),
        
        // Terminal states cannot transition
        (RemittanceStatus::Completed, _) => Err(ContractError::InvalidStatus),
        (RemittanceStatus::Cancelled, _) => Err(ContractError::InvalidStatus),
        (RemittanceStatus::Failed, _) => Err(ContractError::InvalidStatus),
        
        // All other transitions are invalid
        _ => Err(ContractError::InvalidStatus),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_transitions() {
        assert!(validate_transition(&RemittanceStatus::Pending, &RemittanceStatus::Processing).is_ok());
        assert!(validate_transition(&RemittanceStatus::Pending, &RemittanceStatus::Cancelled).is_ok());
        assert!(validate_transition(&RemittanceStatus::Processing, &RemittanceStatus::Completed).is_ok());
        assert!(validate_transition(&RemittanceStatus::Processing, &RemittanceStatus::Failed).is_ok());
    }

    #[test]
    fn test_invalid_transitions_from_pending() {
        assert!(validate_transition(&RemittanceStatus::Pending, &RemittanceStatus::Completed).is_err());
        assert!(validate_transition(&RemittanceStatus::Pending, &RemittanceStatus::Failed).is_err());
    }

    #[test]
    fn test_invalid_transitions_from_processing() {
        assert!(validate_transition(&RemittanceStatus::Processing, &RemittanceStatus::Pending).is_err());
        assert!(validate_transition(&RemittanceStatus::Processing, &RemittanceStatus::Cancelled).is_err());
    }

    #[test]
    fn test_terminal_states_cannot_transition() {
        assert!(validate_transition(&RemittanceStatus::Completed, &RemittanceStatus::Pending).is_err());
        assert!(validate_transition(&RemittanceStatus::Completed, &RemittanceStatus::Processing).is_err());
        assert!(validate_transition(&RemittanceStatus::Cancelled, &RemittanceStatus::Pending).is_err());
        assert!(validate_transition(&RemittanceStatus::Failed, &RemittanceStatus::Processing).is_err());
    }
}
