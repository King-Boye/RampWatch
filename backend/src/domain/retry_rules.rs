//! Resend rules, ported from Haroldwonder/SwiftRemit
//! `src/transaction_controller.rs` `retry_transaction`: only a rolled-back
//! (failed) transaction may be retried, and a retry starts again from the
//! initial state with its own counter.
//!
//! In RampWatch a resend is a new payout row (`attempt + 1`,
//! `parent_payout_id` set), never a reset of the old one. These rules tell the
//! operator whether a resend they already made was allowed. The double-pay and
//! blind-retry checks that also need ledger data live in `detect::retry`.

use super::state::PayoutState;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ResendVerdict {
    /// The parent failed, so a resend is allowed.
    Allowed,
    /// The parent never reached `failed`: it may still settle, so the resend
    /// risks paying twice.
    ParentNotFailed,
    /// The attempt number doesn't follow the parent's.
    AttemptOutOfOrder,
}

/// Was a resend with `attempt` allowed, given its parent's state and attempt?
#[must_use]
pub const fn check_resend(
    parent_state: PayoutState,
    parent_attempt: i64,
    attempt: i64,
) -> ResendVerdict {
    if attempt != parent_attempt + 1 {
        return ResendVerdict::AttemptOutOfOrder;
    }
    match parent_state {
        PayoutState::Failed => ResendVerdict::Allowed,
        _ => ResendVerdict::ParentNotFailed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_failed_parent_may_be_resent() {
        assert_eq!(check_resend(PayoutState::Failed, 1, 2), ResendVerdict::Allowed);
        for s in [
            PayoutState::Pending,
            PayoutState::Processing,
            PayoutState::Completed,
            PayoutState::Disputed,
            PayoutState::Cancelled,
        ] {
            assert_eq!(check_resend(s, 1, 2), ResendVerdict::ParentNotFailed, "{s}");
        }
    }

    #[test]
    fn attempts_must_follow_on() {
        assert_eq!(
            check_resend(PayoutState::Failed, 1, 3),
            ResendVerdict::AttemptOutOfOrder
        );
        assert_eq!(
            check_resend(PayoutState::Failed, 2, 2),
            ResendVerdict::AttemptOutOfOrder
        );
    }
}
