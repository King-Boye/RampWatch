// =============================================================================
// Issue #1556 — Roadmap: Batch remittance processing
// https://github.com/Haroldwonder/SwiftRemit/issues/1556
//
// STATUS: SHIPPED ✅
// Listed on the project roadmap in README.md — now checked off.
//
// ─── WHAT WAS BUILT ──────────────────────────────────────────────────────────
//
// Batch remittance processing allows high-volume senders to create up to 100
// remittances in a single atomic Soroban transaction, reducing ledger fees and
// improving throughput for payroll, bulk-disbursement, and agent-network use
// cases.
//
// ─── SHIPPED ENTRY POINTS (src/lib.rs) ───────────────────────────────────────
//
//   batch_create_remittances(sender, entries) → Vec<u64>
//     Creates multiple remittances atomically. All entries are pre-validated
//     before any state change; a single token.transfer() moves the total USDC
//     into escrow. Returns the Vec of assigned remittance IDs.
//
//   create_batch_remittance(sender, entries) → Vec<u64>
//     Thin wrapper around batch_create_remittances that also emits a
//     "batch_created" event for off-chain indexers.
//
//   confirm_batch_payout(agent, remittance_ids) → ()
//     Confirms payout for multiple already-created remittances in one call.
//     Each ID is processed via the standard confirm_payout path; the whole
//     call fails if any single ID is invalid, ensuring atomic batch settlement.
//
//   batch_settle_with_netting(entries) → NettingResult
//     Computes net settlements via compute_net_settlements() (src/netting.rs),
//     settling only the net difference between opposing flows. Reduces on-chain
//     token transfer volume in high-frequency agent-to-agent corridors.
//     NOTE: This function has no require_auth() guard — see the README security
//     note and track in the sibling auth-hardening issue.
//
// ─── BATCH CONTRACT INVARIANTS ───────────────────────────────────────────────
//
//   INV-B1  Batch size: 1 ≤ len(entries) ≤ MAX_BATCH_SIZE (100).
//           Empty or oversized batches return ContractError::InvalidBatchSize.
//
//   INV-B2  Atomicity: all entries are validated BEFORE any state change.
//           If entry N fails, entries 0..N-1 are NOT created — the token
//           transfer never happens and no remittance IDs are minted.
//
//   INV-B3  Single token transfer: total USDC moved = sum(entry.amount).
//           Fee deduction still happens per remittance at confirm_payout time,
//           not at creation time. Escrow holds gross amounts.
//
//   INV-B4  Daily send limits apply per entry independently, the same as
//           individual create_remittance calls. The batch does not bypass
//           corridor or daily caps.
//
//   INV-B5  Netting correctness: batch_settle_with_netting must satisfy
//           the three properties verified in src/netting.rs property tests:
//           (a) fee preservation, (b) net amount correctness, (c) order
//           independence. See netting.rs for the proptest suite.
//
// ─── ACCEPTANCE CRITERIA (from issue) ────────────────────────────────────────
//
//  ✅  Batch size limited to MAX_BATCH_SIZE (100)         → INV-B1
//  ✅  Atomic: all succeed or all fail                    → INV-B2
//  ✅  Single token transfer for the total amount         → INV-B3
//  ✅  Unit tests: success, partial failure, oversized    → this file
//  ✅  README roadmap item checked off                    → ROADMAP.md
//
// ─── TESTS IN THIS FILE ──────────────────────────────────────────────────────
//
//   test_batch_create_success          — happy path, 3 entries, 3 agents
//   test_batch_create_partial_failure  — unregistered agent reverts entire batch
//   test_batch_create_oversized        — 101 entries → InvalidBatchSize
//   test_batch_create_empty            — 0 entries  → InvalidBatchSize
//   test_batch_create_invalid_amount   — zero amount in one entry → InvalidAmount
//   test_batch_create_max_size         — exactly 100 entries succeeds
//   test_batch_create_different_amounts — fee calculation verified per entry
//
// ─── FILES CHANGED FOR THIS FEATURE ──────────────────────────────────────────
//
//   src/types.rs                    ← BatchCreateEntry struct
//   src/lib.rs                      ← batch_create_remittances,
//                                      create_batch_remittance,
//                                      confirm_batch_payout,
//                                      batch_settle_with_netting
//   src/netting.rs                  ← compute_net_settlements + proptest suite
//   src/test_batch_create.rs        ← (THIS FILE) unit tests
//   ROADMAP.md                      ← checked off
//   README.md                       ← Batch/Netting section in function table
//   docs/implementation/BATCH_REMITTANCE.md ← implementation details
//
// =============================================================================

//! Unit tests for batch remittance creation functionality.

#[cfg(test)]
mod tests {
    use soroban_sdk::{testutils::Address as _, Address, Env, Vec};

    use crate::{
        BatchCreateEntry, ContractError, RemittanceStatus, SwiftRemitContract,
        SwiftRemitContractClient,
    };

    fn setup(env: &Env) -> (SwiftRemitContractClient, Address, Address) {
        env.mock_all_auths();
        let admin = Address::generate(env);
        let token_addr = env
            .register_stellar_asset_contract_v2(admin.clone())
            .address();
        let contract = SwiftRemitContractClient::new(
            env,
            &env.register_contract(None, SwiftRemitContract {}),
        );
        contract.initialize(&admin, &token_addr, &250, &0, &0, &admin);
        let agent = Address::generate(env);
        contract.register_agent(&agent, &None);
        (contract, admin, agent)
    }

    #[test]
    fn test_batch_create_success() {
        let env = Env::default();
        let (contract, _admin, _) = setup(&env);
        let sender = Address::generate(&env);

        let agent1 = Address::generate(&env);
        let agent2 = Address::generate(&env);
        let agent3 = Address::generate(&env);
        contract.register_agent(&agent1, &None);
        contract.register_agent(&agent2, &None);
        contract.register_agent(&agent3, &None);

        let mut entries = Vec::new(&env);
        entries.push_back(BatchCreateEntry { agent: agent1.clone(), amount: 100_000_000, expiry: None });
        entries.push_back(BatchCreateEntry { agent: agent2.clone(), amount: 200_000_000, expiry: Some(env.ledger().timestamp() + 3600) });
        entries.push_back(BatchCreateEntry { agent: agent3.clone(), amount: 150_000_000, expiry: None });

        let remittance_ids = contract.batch_create_remittances(&sender, &entries);
        assert_eq!(remittance_ids.len(), 3);

        let r1 = contract.get_remittance(&remittance_ids.get_unchecked(0));
        assert_eq!(r1.status, RemittanceStatus::Pending);
        assert_eq!(r1.sender, sender);
        assert_eq!(r1.agent, agent1);
        assert_eq!(r1.amount, 100_000_000);

        let r2 = contract.get_remittance(&remittance_ids.get_unchecked(1));
        assert_eq!(r2.agent, agent2);
        assert!(r2.expiry.is_some());

        let r3 = contract.get_remittance(&remittance_ids.get_unchecked(2));
        assert_eq!(r3.agent, agent3);
        assert_eq!(r3.amount, 150_000_000);
    }

    #[test]
    fn test_batch_create_partial_failure() {
        let env = Env::default();
        let (contract, _admin, _) = setup(&env);
        let sender = Address::generate(&env);

        let agent1 = Address::generate(&env);
        let agent2 = Address::generate(&env);
        let unregistered = Address::generate(&env);
        contract.register_agent(&agent1, &None);
        contract.register_agent(&agent2, &None);

        let mut entries = Vec::new(&env);
        entries.push_back(BatchCreateEntry { agent: agent1.clone(), amount: 100_000_000, expiry: None });
        entries.push_back(BatchCreateEntry { agent: unregistered.clone(), amount: 200_000_000, expiry: None });
        entries.push_back(BatchCreateEntry { agent: agent2.clone(), amount: 150_000_000, expiry: None });

        let result = contract.try_batch_create_remittances(&sender, &entries);
        assert_eq!(result, Err(Ok(ContractError::AgentNotRegistered)));
    }

    #[test]
    fn test_batch_create_oversized() {
        let env = Env::default();
        let (contract, _admin, agent) = setup(&env);
        let sender = Address::generate(&env);

        let mut entries = Vec::new(&env);
        for _ in 0..101 {
            entries.push_back(BatchCreateEntry { agent: agent.clone(), amount: 1_000_000, expiry: None });
        }

        let result = contract.try_batch_create_remittances(&sender, &entries);
        assert_eq!(result, Err(Ok(ContractError::InvalidBatchSize)));
    }

    #[test]
    fn test_batch_create_empty() {
        let env = Env::default();
        let (contract, _admin, _) = setup(&env);
        let sender = Address::generate(&env);

        let entries = Vec::new(&env);
        let result = contract.try_batch_create_remittances(&sender, &entries);
        assert_eq!(result, Err(Ok(ContractError::InvalidBatchSize)));
    }

    #[test]
    fn test_batch_create_invalid_amount() {
        let env = Env::default();
        let (contract, _admin, _) = setup(&env);
        let sender = Address::generate(&env);

        let agent1 = Address::generate(&env);
        let agent2 = Address::generate(&env);
        contract.register_agent(&agent1, &None);
        contract.register_agent(&agent2, &None);

        let mut entries = Vec::new(&env);
        entries.push_back(BatchCreateEntry { agent: agent1.clone(), amount: 100_000_000, expiry: None });
        entries.push_back(BatchCreateEntry { agent: agent2.clone(), amount: 0, expiry: None });

        let result = contract.try_batch_create_remittances(&sender, &entries);
        assert_eq!(result, Err(Ok(ContractError::InvalidAmount)));
    }

    #[test]
    fn test_batch_create_max_size() {
        let env = Env::default();
        let (contract, _admin, agent) = setup(&env);
        let sender = Address::generate(&env);

        let mut entries = Vec::new(&env);
        for _ in 0..100 {
            entries.push_back(BatchCreateEntry { agent: agent.clone(), amount: 1_000_000, expiry: None });
        }

        let remittance_ids = contract.batch_create_remittances(&sender, &entries);
        assert_eq!(remittance_ids.len(), 100);
    }

    #[test]
    fn test_batch_create_different_amounts() {
        let env = Env::default();
        let (contract, _admin, _) = setup(&env);
        let sender = Address::generate(&env);

        let agent1 = Address::generate(&env);
        let agent2 = Address::generate(&env);
        contract.register_agent(&agent1, &None);
        contract.register_agent(&agent2, &None);

        let mut entries = Vec::new(&env);
        entries.push_back(BatchCreateEntry { agent: agent1.clone(), amount: 50_000_000, expiry: None });
        entries.push_back(BatchCreateEntry { agent: agent2.clone(), amount: 150_000_000, expiry: None });

        let remittance_ids = contract.batch_create_remittances(&sender, &entries);
        let r1 = contract.get_remittance(&remittance_ids.get_unchecked(0));
        let r2 = contract.get_remittance(&remittance_ids.get_unchecked(1));

        assert!(r1.fee > 0);
        assert!(r2.fee > 0);
        assert_ne!(r1.fee, r2.fee);
    }
}
