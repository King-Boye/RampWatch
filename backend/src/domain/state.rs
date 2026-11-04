//! Payout state machine (spec 8.2).
//!
//! Ported from Haroldwonder/SwiftRemit `src/transitions.rs` and the
//! `RemittanceStatus` enum in `src/types.rs`, including the `Disputed` state.
//! Two differences from the contract, both from the spec:
//! - `processing -> failed` is allowed (the contract only reaches `Failed`
//!   through an agent call outside `get_valid_next_states`).
//! - A payout can be first seen in `processing`, because operators often only
//!   report once the cash-out is in flight.
//!
//! As in SwiftRemit, a same-state transition is idempotent and always allowed.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PayoutState {
    Pending,
    Processing,
    Completed,
    Failed,
    Disputed,
    Cancelled,
}

impl PayoutState {
    pub const ALL: [Self; 6] = [
        Self::Pending,
        Self::Processing,
        Self::Completed,
        Self::Failed,
        Self::Disputed,
        Self::Cancelled,
    ];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Processing => "processing",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Disputed => "disputed",
            Self::Cancelled => "cancelled",
        }
    }

    /// Only `completed` and `cancelled` are terminal. `failed` and `disputed`
    /// still move on (to a dispute, a refund or a resolved payout).
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Cancelled)
    }

    /// States a payout may enter from this one.
    #[must_use]
    pub const fn valid_next_states(self) -> &'static [Self] {
        match self {
            Self::Pending => &[Self::Processing, Self::Cancelled],
            // cancelled here means refunded
            Self::Processing => &[Self::Completed, Self::Failed, Self::Cancelled],
            Self::Failed => &[Self::Disputed, Self::Cancelled],
            Self::Disputed => &[Self::Completed, Self::Cancelled],
            Self::Completed | Self::Cancelled => &[],
        }
    }

    /// States a payout may be first seen in.
    #[must_use]
    pub const fn initial_states() -> &'static [Self] {
        &[Self::Pending, Self::Processing]
    }

    #[must_use]
    pub fn can_transition_to(self, to: Self) -> bool {
        self.valid_next_states().contains(&to)
    }
}

impl fmt::Display for PayoutState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for PayoutState {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|state| state.as_str() == s)
            .ok_or_else(|| format!("unknown payout state '{s}'"))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidTransition {
    pub from: Option<PayoutState>,
    pub to: PayoutState,
}

impl fmt::Display for InvalidTransition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.from {
            Some(from) => write!(f, "cannot move a payout from {from} to {}", self.to),
            None => write!(f, "a new payout cannot start in {}", self.to),
        }
    }
}

/// The outcome of a valid transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transition {
    /// The payout moves to a new state.
    Changed,
    /// Same state as before: recorded, nothing changes (SwiftRemit's
    /// idempotent same-state rule).
    Unchanged,
}

/// Check a transition. `from` is `None` for a payout seen for the first time.
pub fn validate_transition(
    from: Option<PayoutState>,
    to: PayoutState,
) -> Result<Transition, InvalidTransition> {
    match from {
        None if PayoutState::initial_states().contains(&to) => Ok(Transition::Changed),
        Some(from) if from == to => Ok(Transition::Unchanged),
        Some(from) if from.can_transition_to(to) => Ok(Transition::Changed),
        _ => Err(InvalidTransition { from, to }),
    }
}

#[cfg(test)]
mod tests {
    use super::PayoutState::*;
    use super::*;

    #[test]
    fn spec_table_allows_exactly_these_edges() {
        let allowed = [
            (Pending, Processing),
            (Pending, Cancelled),
            (Processing, Completed),
            (Processing, Failed),
            (Processing, Cancelled),
            (Failed, Disputed),
            (Failed, Cancelled),
            (Disputed, Completed),
            (Disputed, Cancelled),
        ];
        for from in PayoutState::ALL {
            for to in PayoutState::ALL {
                let expected = from == to || allowed.contains(&(from, to));
                assert_eq!(
                    validate_transition(Some(from), to).is_ok(),
                    expected,
                    "{from} -> {to}"
                );
            }
        }
    }

    #[test]
    fn new_payout_starts_pending_or_processing() {
        assert_eq!(validate_transition(None, Pending), Ok(Transition::Changed));
        assert_eq!(validate_transition(None, Processing), Ok(Transition::Changed));
        for to in [Completed, Failed, Disputed, Cancelled] {
            assert!(validate_transition(None, to).is_err(), "new payout in {to}");
        }
    }

    #[test]
    fn same_state_is_idempotent() {
        for s in PayoutState::ALL {
            assert_eq!(validate_transition(Some(s), s), Ok(Transition::Unchanged));
        }
    }

    #[test]
    fn terminal_states() {
        assert!(Completed.is_terminal());
        assert!(Cancelled.is_terminal());
        for s in [Pending, Processing, Failed, Disputed] {
            assert!(!s.is_terminal());
        }
        assert!(Completed.valid_next_states().is_empty());
        assert!(Cancelled.valid_next_states().is_empty());
    }

    #[test]
    fn completed_cannot_go_back() {
        let err = validate_transition(Some(Completed), Pending).unwrap_err();
        assert_eq!(err.to_string(), "cannot move a payout from completed to pending");
    }

    #[test]
    fn parse_round_trip() {
        for s in PayoutState::ALL {
            assert_eq!(s.as_str().parse::<PayoutState>(), Ok(s));
        }
        assert!("refunded".parse::<PayoutState>().is_err());
    }
}
