//! State transition validation for the SwiftRemit contract.
//!
//! This module implements a structured transaction state machine that enforces
//! strict, deterministic state transitions to prevent inconsistent transfer statuses.
//!
//! # State Machine
//!
//! ```
//! INITIATED → SUBMITTED → PENDING_ANCHOR → COMPLETED
//!                                        ↘ FAILED
//! ```
//!
//! # Rules
//!
//! 1. All transitions must be explicitly validated before execution
//! 2. Terminal states (COMPLETED, FAILED) cannot transition to any other state
//! 3. Invalid transitions are rejected with explicit errors (no panics)
//! 4. State updates are atomic to prevent partial writes
//! 5. Repeated submissions are idempotent (same state → same state is allowed)

use crate::types::RemittanceStatus;
use crate::errors::ContractError;
use soroban_sdk::Env;

/// Validates if a state transition is allowed.
///
/// This is the centralized validation function that enforces the state machine rules.
/// All state changes must go through this validation to ensure consistency.
///
/// # Arguments
///
/// * `from` - Current status of the remittance
/// * `to` - Target status to transition to
///
/// # Returns
///
/// * `Ok(())` - Transition is valid and allowed
/// * `Err(ContractError::InvalidStateTransition)` - Transition is invalid
///
/// # State Transition Rules
///
/// ## From INITIATED
/// - Can transition to: SUBMITTED, FAILED
/// - Cannot transition to: PENDING_ANCHOR, COMPLETED
///
/// ## From SUBMITTED
/// - Can transition to: PENDING_ANCHOR, FAILED
/// - Cannot transition to: INITIATED, COMPLETED
///
/// ## From PENDING_ANCHOR
/// - Can transition to: COMPLETED, FAILED
/// - Cannot transition to: INITIATED, SUBMITTED
///
/// ## From COMPLETED (Terminal)
/// - Cannot transition to any state
///
/// ## From FAILED (Terminal)
/// - Cannot transition to any state
///
/// # Examples
///
/// ```ignore
/// // Valid transition
/// validate_transition(&RemittanceStatus::Initiated, &RemittanceStatus::Submitted)?;
///
/// // Invalid transition - will return error
/// validate_transition(&RemittanceStatus::Initiated, &RemittanceStatus::Completed)?;
///
/// // Terminal state - will return error
/// validate_transition(&RemittanceStatus::Completed, &RemittanceStatus::Failed)?;
/// ```
pub fn validate_transition(
    from: &RemittanceStatus,
    to: &RemittanceStatus,
) -> Result<(), ContractError> {
    // Idempotent: Allow same state → same state (for retry scenarios)
    if from == to {
        return Ok(());
    }

    // Use the can_transition_to method from RemittanceStatus
    if from.can_transition_to(to) {
        Ok(())
    } else {
        Err(ContractError::InvalidStateTransition)
    }
}

/// Atomically updates the remittance status with validation.
///
/// This function ensures that:
/// 1. The transition is valid according to state machine rules
/// 2. The update is atomic (all or nothing)
/// 3. Storage integrity is maintained
///
/// # Arguments
///
/// * `env` - The contract execution environment
/// * `remittance` - Mutable reference to the remittance to update
/// * `new_status` - The target status to transition to
///
/// # Returns
///
/// * `Ok(())` - Status updated successfully
/// * `Err(ContractError::InvalidStateTransition)` - Transition is invalid
///
/// # Guarantees
///
/// - Atomic: Either the status is updated or an error is returned (no partial updates)
/// - Validated: All transitions are validated before execution
/// - Deterministic: Same input always produces same result
/// - Idempotent: Repeated calls with same status are safe
pub fn transition_status(
    env: &Env,
    remittance: &mut crate::Remittance,
    new_status: RemittanceStatus,
) -> Result<(), ContractError> {
    // Validate the transition
    validate_transition(&remittance.status, &new_status)?;
    
    // Log transition for debugging (only in test/debug builds)
    log_transition(env, remittance.id, &remittance.status, &new_status);
    
    // Atomically update the status
    remittance.status = new_status;
    
    Ok(())
}

/// Checks if a status is terminal (cannot transition further).
///
/// # Arguments
///
/// * `status` - The status to check
///
/// # Returns
///
/// * `true` - Status is terminal (COMPLETED or FAILED)
/// * `false` - Status is non-terminal
pub fn is_terminal_status(status: &RemittanceStatus) -> bool {
    status.is_terminal()
}

/// Gets the list of valid next states for a given status.
///
/// # Arguments
///
/// * `status` - The current status
///
/// # Returns
///
/// Vector of valid next states (empty for terminal states)
pub fn get_valid_next_states(status: &RemittanceStatus) -> soroban_sdk::Vec<RemittanceStatus> {
    let env = Env::default();
    let mut result = soroban_sdk::Vec::new(&env);
    
    for next_status in status.next_valid_states() {
        result.push_back(next_status);
    }
    
    result
}

/// Logs a state transition for debugging purposes.
///
/// This function only logs in test/debug builds and has no effect in production.
///
/// # Arguments
///
/// * `env` - The contract execution environment
/// * `remittance_id` - ID of the remittance being transitioned
/// * `from` - Current status
/// * `to` - Target status
fn log_transition(env: &Env, remittance_id: u64, from: &RemittanceStatus, to: &RemittanceStatus) {
    #[cfg(any(test, feature = "testutils"))]
    {
        use crate::debug::log_info;
        log_info(
            env,
            &format!(
                "Remittance {} transition: {:?} → {:?}",
                remittance_id, from, to
            ),
        );
    }
    
    // Suppress unused variable warnings in production
    #[cfg(not(any(test, feature = "testutils")))]
    {
        let _ = (env, remittance_id, from, to);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ═══════════════════════════════════════════════════════════════════════════
    // Valid Transition Tests
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_valid_transition_initiated_to_submitted() {
        assert!(validate_transition(
            &RemittanceStatus::Initiated,
            &RemittanceStatus::Submitted
        )
        .is_ok());
    }

    #[test]
    fn test_valid_transition_initiated_to_failed() {
        assert!(validate_transition(
            &RemittanceStatus::Initiated,
            &RemittanceStatus::Failed
        )
        .is_ok());
    }

    #[test]
    fn test_valid_transition_submitted_to_pending_anchor() {
        assert!(validate_transition(
            &RemittanceStatus::Submitted,
            &RemittanceStatus::PendingAnchor
        )
        .is_ok());
    }

    #[test]
    fn test_valid_transition_submitted_to_failed() {
        assert!(validate_transition(
            &RemittanceStatus::Submitted,
            &RemittanceStatus::Failed
        )
        .is_ok());
    }

    #[test]
    fn test_valid_transition_pending_anchor_to_completed() {
        assert!(validate_transition(
            &RemittanceStatus::PendingAnchor,
            &RemittanceStatus::Completed
        )
        .is_ok());
    }

    #[test]
    fn test_valid_transition_pending_anchor_to_failed() {
        assert!(validate_transition(
            &RemittanceStatus::PendingAnchor,
            &RemittanceStatus::Failed
        )
        .is_ok());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Idempotent Transition Tests (Same State → Same State)
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_idempotent_transition_initiated() {
        assert!(validate_transition(
            &RemittanceStatus::Initiated,
            &RemittanceStatus::Initiated
        )
        .is_ok());
    }

    #[test]
    fn test_idempotent_transition_submitted() {
        assert!(validate_transition(
            &RemittanceStatus::Submitted,
            &RemittanceStatus::Submitted
        )
        .is_ok());
    }

    #[test]
    fn test_idempotent_transition_pending_anchor() {
        assert!(validate_transition(
            &RemittanceStatus::PendingAnchor,
            &RemittanceStatus::PendingAnchor
        )
        .is_ok());
    }

    #[test]
    fn test_idempotent_transition_completed() {
        assert!(validate_transition(
            &RemittanceStatus::Completed,
            &RemittanceStatus::Completed
        )
        .is_ok());
    }

    #[test]
    fn test_idempotent_transition_failed() {
        assert!(validate_transition(
            &RemittanceStatus::Failed,
            &RemittanceStatus::Failed
        )
        .is_ok());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Invalid Transition Tests - From INITIATED
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_invalid_transition_initiated_to_pending_anchor() {
        let result = validate_transition(
            &RemittanceStatus::Initiated,
            &RemittanceStatus::PendingAnchor,
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ContractError::InvalidStateTransition);
    }

    #[test]
    fn test_invalid_transition_initiated_to_completed() {
        let result = validate_transition(
            &RemittanceStatus::Initiated,
            &RemittanceStatus::Completed,
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ContractError::InvalidStateTransition);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Invalid Transition Tests - From SUBMITTED
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_invalid_transition_submitted_to_initiated() {
        let result = validate_transition(
            &RemittanceStatus::Submitted,
            &RemittanceStatus::Initiated,
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ContractError::InvalidStateTransition);
    }

    #[test]
    fn test_invalid_transition_submitted_to_completed() {
        let result = validate_transition(
            &RemittanceStatus::Submitted,
            &RemittanceStatus::Completed,
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ContractError::InvalidStateTransition);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Invalid Transition Tests - From PENDING_ANCHOR
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_invalid_transition_pending_anchor_to_initiated() {
        let result = validate_transition(
            &RemittanceStatus::PendingAnchor,
            &RemittanceStatus::Initiated,
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ContractError::InvalidStateTransition);
    }

    #[test]
    fn test_invalid_transition_pending_anchor_to_submitted() {
        let result = validate_transition(
            &RemittanceStatus::PendingAnchor,
            &RemittanceStatus::Submitted,
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ContractError::InvalidStateTransition);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Terminal State Protection Tests - COMPLETED
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_terminal_completed_cannot_transition_to_initiated() {
        let result = validate_transition(
            &RemittanceStatus::Completed,
            &RemittanceStatus::Initiated,
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ContractError::InvalidStateTransition);
    }

    #[test]
    fn test_terminal_completed_cannot_transition_to_submitted() {
        let result = validate_transition(
            &RemittanceStatus::Completed,
            &RemittanceStatus::Submitted,
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ContractError::InvalidStateTransition);
    }

    #[test]
    fn test_terminal_completed_cannot_transition_to_pending_anchor() {
        let result = validate_transition(
            &RemittanceStatus::Completed,
            &RemittanceStatus::PendingAnchor,
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ContractError::InvalidStateTransition);
    }

    #[test]
    fn test_terminal_completed_cannot_transition_to_failed() {
        let result = validate_transition(
            &RemittanceStatus::Completed,
            &RemittanceStatus::Failed,
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ContractError::InvalidStateTransition);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Terminal State Protection Tests - FAILED
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_terminal_failed_cannot_transition_to_initiated() {
        let result = validate_transition(
            &RemittanceStatus::Failed,
            &RemittanceStatus::Initiated,
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ContractError::InvalidStateTransition);
    }

    #[test]
    fn test_terminal_failed_cannot_transition_to_submitted() {
        let result = validate_transition(
            &RemittanceStatus::Failed,
            &RemittanceStatus::Submitted,
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ContractError::InvalidStateTransition);
    }

    #[test]
    fn test_terminal_failed_cannot_transition_to_pending_anchor() {
        let result = validate_transition(
            &RemittanceStatus::Failed,
            &RemittanceStatus::PendingAnchor,
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ContractError::InvalidStateTransition);
    }

    #[test]
    fn test_terminal_failed_cannot_transition_to_completed() {
        let result = validate_transition(
            &RemittanceStatus::Failed,
            &RemittanceStatus::Completed,
        );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ContractError::InvalidStateTransition);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Terminal Status Check Tests
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_is_terminal_status_completed() {
        assert!(is_terminal_status(&RemittanceStatus::Completed));
    }

    #[test]
    fn test_is_terminal_status_failed() {
        assert!(is_terminal_status(&RemittanceStatus::Failed));
    }

    #[test]
    fn test_is_not_terminal_status_initiated() {
        assert!(!is_terminal_status(&RemittanceStatus::Initiated));
    }

    #[test]
    fn test_is_not_terminal_status_submitted() {
        assert!(!is_terminal_status(&RemittanceStatus::Submitted));
    }

    #[test]
    fn test_is_not_terminal_status_pending_anchor() {
        assert!(!is_terminal_status(&RemittanceStatus::PendingAnchor));
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Valid Next States Tests
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_valid_next_states_from_initiated() {
        let next_states = get_valid_next_states(&RemittanceStatus::Initiated);
        assert_eq!(next_states.len(), 2);
        assert!(next_states.contains(&RemittanceStatus::Submitted));
        assert!(next_states.contains(&RemittanceStatus::Failed));
    }

    #[test]
    fn test_valid_next_states_from_submitted() {
        let next_states = get_valid_next_states(&RemittanceStatus::Submitted);
        assert_eq!(next_states.len(), 2);
        assert!(next_states.contains(&RemittanceStatus::PendingAnchor));
        assert!(next_states.contains(&RemittanceStatus::Failed));
    }

    #[test]
    fn test_valid_next_states_from_pending_anchor() {
        let next_states = get_valid_next_states(&RemittanceStatus::PendingAnchor);
        assert_eq!(next_states.len(), 2);
        assert!(next_states.contains(&RemittanceStatus::Completed));
        assert!(next_states.contains(&RemittanceStatus::Failed));
    }

    #[test]
    fn test_valid_next_states_from_completed() {
        let next_states = get_valid_next_states(&RemittanceStatus::Completed);
        assert_eq!(next_states.len(), 0); // Terminal state has no valid transitions
    }

    #[test]
    fn test_valid_next_states_from_failed() {
        let next_states = get_valid_next_states(&RemittanceStatus::Failed);
        assert_eq!(next_states.len(), 0); // Terminal state has no valid transitions
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // Atomic Transition Tests
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_transition_status_valid() {
        let env = Env::default();
        let sender = soroban_sdk::Address::generate(&env);
        let agent = soroban_sdk::Address::generate(&env);
        
        let mut remittance = crate::Remittance {
            id: 1,
            sender,
            agent,
            amount: 100,
            fee: 2,
            status: RemittanceStatus::Initiated,
            expiry: None,
        };

        let result = transition_status(&env, &mut remittance, RemittanceStatus::Submitted);
        assert!(result.is_ok());
        assert_eq!(remittance.status, RemittanceStatus::Submitted);
    }

    #[test]
    fn test_transition_status_invalid() {
        let env = Env::default();
        let sender = soroban_sdk::Address::generate(&env);
        let agent = soroban_sdk::Address::generate(&env);
        
        let mut remittance = crate::Remittance {
            id: 1,
            sender,
            agent,
            amount: 100,
            fee: 2,
            status: RemittanceStatus::Initiated,
            expiry: None,
        };

        let result = transition_status(&env, &mut remittance, RemittanceStatus::Completed);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ContractError::InvalidStateTransition);
        // Status should remain unchanged after failed transition
        assert_eq!(remittance.status, RemittanceStatus::Initiated);
    }

    #[test]
    fn test_transition_status_idempotent() {
        let env = Env::default();
        let sender = soroban_sdk::Address::generate(&env);
        let agent = soroban_sdk::Address::generate(&env);
        
        let mut remittance = crate::Remittance {
            id: 1,
            sender,
            agent,
            amount: 100,
            fee: 2,
            status: RemittanceStatus::Submitted,
            expiry: None,
        };

        // Transitioning to same state should succeed (idempotent)
        let result = transition_status(&env, &mut remittance, RemittanceStatus::Submitted);
        assert!(result.is_ok());
        assert_eq!(remittance.status, RemittanceStatus::Submitted);
    }
}
