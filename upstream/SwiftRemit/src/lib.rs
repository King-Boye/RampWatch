#![no_std]
mod debug;
mod errors;
mod events;
mod storage;
mod types;
mod validation;

use soroban_sdk::{contract, contractimpl, token, Address, Env, Vec};

pub use debug::*;
pub use errors::ContractError;
pub use events::*;
pub use storage::*;
pub use types::*;
pub use validation::*;

#[contract]
pub struct SwiftRemitContract;

#[contractimpl]
impl SwiftRemitContract {
    pub fn initialize(
        env: Env,
        admin: Address,
        usdc_token: Address,
        fee_bps: u32,
    ) -> Result<(), ContractError> {
        if has_admin(&env) {
            return Err(ContractError::AlreadyInitialized);
        }

        if fee_bps > 10000 {
            return Err(ContractError::InvalidFeeBps);
        }

        set_admin(&env, &admin);
        set_usdc_token(&env, &usdc_token);
        set_platform_fee_bps(&env, fee_bps);
        set_remittance_counter(&env, 0);
        set_accumulated_fees(&env, 0);

        log_initialize(&env, &admin, &usdc_token, fee_bps);

        Ok(())
    }

    pub fn register_agent(env: Env, agent: Address) -> Result<(), ContractError> {
        let admin = get_admin(&env)?;
        admin.require_auth();

        set_agent_registered(&env, &agent, true);
        emit_agent_registered(&env, agent.clone(), admin.clone());

        log_register_agent(&env, &agent);

        Ok(())
    }

    pub fn remove_agent(env: Env, agent: Address) -> Result<(), ContractError> {
        let admin = get_admin(&env)?;
        admin.require_auth();

        set_agent_registered(&env, &agent, false);
        emit_agent_removed(&env, agent.clone(), admin.clone());

        log_remove_agent(&env, &agent);

        Ok(())
    }

    pub fn update_fee(env: Env, fee_bps: u32) -> Result<(), ContractError> {
        let admin = get_admin(&env)?;
        admin.require_auth();

        if fee_bps > 10000 {
            return Err(ContractError::InvalidFeeBps);
        }

        set_platform_fee_bps(&env, fee_bps);
        let old_fee = get_platform_fee_bps(&env)?;
        emit_fee_updated(&env, admin.clone(), old_fee, fee_bps);

        log_update_fee(&env, fee_bps);

        Ok(())
    }

    pub fn create_remittance(
        env: Env,
        sender: Address,
        agent: Address,
        amount: i128,
        expiry: Option<u64>,
    ) -> Result<u64, ContractError> {
        sender.require_auth();

        if amount <= 0 {
            return Err(ContractError::InvalidAmount);
        }

        if !is_agent_registered(&env, &agent) {
            return Err(ContractError::AgentNotRegistered);
        }

        let fee_bps = get_platform_fee_bps(&env)?;
        let fee = amount
            .checked_mul(fee_bps as i128)
            .ok_or(ContractError::Overflow)?
            .checked_div(10000)
            .ok_or(ContractError::Overflow)?;

        let usdc_token = get_usdc_token(&env)?;
        let token_client = token::Client::new(&env, &usdc_token);
        token_client.transfer(&sender, &env.current_contract_address(), &amount);

        let counter = get_remittance_counter(&env)?;
        let remittance_id = counter.checked_add(1).ok_or(ContractError::Overflow)?;

        let remittance = Remittance {
            id: remittance_id,
            sender: sender.clone(),
            agent: agent.clone(),
            amount,
            fee,
            status: RemittanceStatus::Pending,
            expiry,
        };

        set_remittance(&env, remittance_id, &remittance);
        set_remittance_counter(&env, remittance_id);

        emit_remittance_created(&env, remittance_id, sender.clone(), agent.clone(), usdc_token.clone(), amount, fee);

        log_create_remittance(&env, remittance_id, &sender, &agent, amount, fee);

        Ok(remittance_id)
    }

    pub fn confirm_payout(env: Env, remittance_id: u64) -> Result<(), ContractError> {
        if is_paused(&env) {
            return Err(ContractError::ContractPaused);
        }

        let mut remittance = get_remittance(&env, remittance_id)?;

        remittance.agent.require_auth();

        if remittance.status != RemittanceStatus::Pending {
            return Err(ContractError::InvalidStatus);
        }

        // Check for duplicate settlement execution
        if has_settlement_hash(&env, remittance_id) {
            return Err(ContractError::DuplicateSettlement);
        }

        // Check if settlement has expired
        if let Some(expiry_time) = remittance.expiry {
            let current_time = env.ledger().timestamp();
            if current_time > expiry_time {
                return Err(ContractError::SettlementExpired);
            }
        }

        // Validate the agent address before transfer
        validate_address(&remittance.agent)?;

        let payout_amount = remittance
            .amount
            .checked_sub(remittance.fee)
            .ok_or(ContractError::Overflow)?;

        let usdc_token = get_usdc_token(&env)?;
        let token_client = token::Client::new(&env, &usdc_token);
        token_client.transfer(
            &env.current_contract_address(),
            &remittance.agent,
            &payout_amount,
        );

        let current_fees = get_accumulated_fees(&env)?;
        let new_fees = current_fees
            .checked_add(remittance.fee)
            .ok_or(ContractError::Overflow)?;
        set_accumulated_fees(&env, new_fees);

        remittance.status = RemittanceStatus::Completed;
        set_remittance(&env, remittance_id, &remittance);

        // Mark settlement as executed to prevent duplicates
        set_settlement_hash(&env, remittance_id);

        emit_remittance_completed(&env, remittance_id, remittance.sender.clone(), remittance.agent.clone(), usdc_token.clone(), payout_amount);
        
        // Emit settlement completed event with final executed values
        emit_settlement_completed(&env, remittance.sender.clone(), remittance.agent.clone(), usdc_token.clone(), payout_amount);

        log_confirm_payout(&env, remittance_id, payout_amount);

        Ok(())
    }

    pub fn cancel_remittance(env: Env, remittance_id: u64) -> Result<(), ContractError> {
        let mut remittance = get_remittance(&env, remittance_id)?;

        remittance.sender.require_auth();

        if remittance.status != RemittanceStatus::Pending {
            return Err(ContractError::InvalidStatus);
        }

        let usdc_token = get_usdc_token(&env)?;
        let token_client = token::Client::new(&env, &usdc_token);
        token_client.transfer(
            &env.current_contract_address(),
            &remittance.sender,
            &remittance.amount,
        );

        remittance.status = RemittanceStatus::Cancelled;
        set_remittance(&env, remittance_id, &remittance);

        emit_remittance_cancelled(&env, remittance_id, remittance.sender.clone(), remittance.agent.clone(), usdc_token.clone(), remittance.amount);

        log_cancel_remittance(&env, remittance_id);

        Ok(())
    }

    pub fn withdraw_fees(env: Env, to: Address) -> Result<(), ContractError> {
        let admin = get_admin(&env)?;
        admin.require_auth();

        // Validate the recipient address
        validate_address(&to)?;

        let fees = get_accumulated_fees(&env)?;

        if fees <= 0 {
            return Err(ContractError::NoFeesToWithdraw);
        }

        let usdc_token = get_usdc_token(&env)?;
        let token_client = token::Client::new(&env, &usdc_token);
        token_client.transfer(&env.current_contract_address(), &to, &fees);

        set_accumulated_fees(&env, 0);

        emit_fees_withdrawn(&env, admin.clone(), to.clone(), usdc_token.clone(), fees);

        log_withdraw_fees(&env, &to, fees);

        Ok(())
    }

    pub fn get_remittance(env: Env, remittance_id: u64) -> Result<Remittance, ContractError> {
        get_remittance(&env, remittance_id)
    }

    pub fn get_settlement(env: Env, id: u64) -> Result<Remittance, ContractError> {
        get_remittance(&env, id)
    }

    pub fn get_accumulated_fees(env: Env) -> Result<i128, ContractError> {
        get_accumulated_fees(&env)
    }

    pub fn is_agent_registered(env: Env, agent: Address) -> bool {
        is_agent_registered(&env, &agent)
    }

    pub fn get_platform_fee_bps(env: Env) -> Result<u32, ContractError> {
        get_platform_fee_bps(&env)
    }

    pub fn pause(env: Env) -> Result<(), ContractError> {
        let admin = get_admin(&env)?;
        admin.require_auth();

        set_paused(&env, true);
        emit_paused(&env, admin);

        Ok(())
    }

    pub fn unpause(env: Env) -> Result<(), ContractError> {
        let admin = get_admin(&env)?;
        admin.require_auth();

        set_paused(&env, false);
        emit_unpaused(&env, admin);

        Ok(())
    }

    pub fn is_paused(env: Env) -> bool {
        is_paused(&env)
    }

    /// Process multiple settlements in a single transaction.
    /// 
    /// This function provides atomic batch processing of settlements:
    /// - All entries are validated before any state changes are made
    /// - If any entry fails validation, the entire batch fails (no partial state writes)
    /// - This ensures atomic execution (all succeed or all fail)
    ///
    /// # Arguments
    /// * `settlements` - Vector of BatchSettlementEntry containing remittance IDs to settle
    ///
    /// # Returns
    /// * `BatchSettlementResult` - Contains list of successfully settled remittance IDs
    ///
    /// # Errors
    /// * `EmptyBatchSettlement` - If the batch is empty
    /// * `BatchTooLarge` - If the batch exceeds MAX_BATCH_SIZE entries
    /// * `BatchValidationFailed` - If any entry fails validation
    /// * `ContractPaused` - If the contract is paused
    pub fn batch_settle(
        env: Env,
        settlements: Vec<BatchSettlementEntry>,
    ) -> Result<BatchSettlementResult, ContractError> {
        // Check if contract is paused
        if is_paused(&env) {
            return Err(ContractError::ContractPaused);
        }

        // Validate batch is not empty
        if settlements.is_empty() {
            return Err(ContractError::EmptyBatchSettlement);
        }

        // Validate batch size
        let batch_size = settlements.len();
        if batch_size > MAX_BATCH_SIZE {
            return Err(ContractError::BatchTooLarge);
        }

        // Emit batch started event
        emit_batch_settlement_started(&env, batch_size);

        // PHASE 1: Validate ALL entries before any state changes
        // This ensures atomic execution - we fail fast if any entry is invalid
        let mut validated_remittances: Vec<Remittance> = Vec::new(&env);
        
        // Use a temporary vector to track seen IDs for duplicate detection within batch
        let mut seen_ids: Vec<u64> = Vec::new(&env);

        for i in 0..settlements.len() {
            let entry = settlements.get(i).unwrap();
            let remittance_id = entry.remittance_id;

            // Check for duplicate IDs within the same batch
            let mut is_duplicate = false;
            for j in 0..seen_ids.len() {
                if seen_ids.get(j).unwrap() == remittance_id {
                    is_duplicate = true;
                    break;
                }
            }
            if is_duplicate {
                emit_batch_settlement_failed(&env, 16); // BatchValidationFailed
                return Err(ContractError::BatchValidationFailed);
            }
            seen_ids.push_back(remittance_id);

            // Get and validate the remittance
            let remittance = get_remittance(&env, remittance_id)?;

            // Validate remittance status is Pending
            if remittance.status != RemittanceStatus::Pending {
                emit_batch_settlement_failed(&env, 16); // BatchValidationFailed
                return Err(ContractError::BatchValidationFailed);
            }

            // Check for duplicate settlement (already settled)
            if has_settlement_hash(&env, remittance_id) {
                emit_batch_settlement_failed(&env, 16); // BatchValidationFailed
                return Err(ContractError::BatchValidationFailed);
            }

            // Check if settlement has expired
            if let Some(expiry_time) = remittance.expiry {
                let current_time = env.ledger().timestamp();
                if current_time > expiry_time {
                    emit_batch_settlement_failed(&env, 16); // BatchValidationFailed
                    return Err(ContractError::BatchValidationFailed);
                }
            }

            // Validate the agent address
            validate_address(&remittance.agent)?;

            // Store validated remittance for phase 2
            validated_remittances.push_back(remittance);
        }

        // PHASE 2: Execute all settlements
        // Only reached if ALL validations passed - atomic execution
        let usdc_token = get_usdc_token(&env)?;
        let token_client = token::Client::new(&env, &usdc_token);
        let mut settled_ids: Vec<u64> = Vec::new(&env);
        let mut total_fees: i128 = 0;

        for i in 0..validated_remittances.len() {
            let mut remittance = validated_remittances.get(i).unwrap();
            let remittance_id = remittance.id;

            // Calculate payout amount
            let payout_amount = remittance
                .amount
                .checked_sub(remittance.fee)
                .ok_or(ContractError::Overflow)?;

            // Transfer tokens to agent
            token_client.transfer(
                &env.current_contract_address(),
                &remittance.agent,
                &payout_amount,
            );

            // Accumulate fees
            total_fees = total_fees
                .checked_add(remittance.fee)
                .ok_or(ContractError::Overflow)?;

            // Update remittance status
            remittance.status = RemittanceStatus::Completed;
            set_remittance(&env, remittance_id, &remittance);

            // Mark settlement as executed to prevent duplicates
            set_settlement_hash(&env, remittance_id);

            // Emit individual settlement completed event
            emit_remittance_completed(
                &env,
                remittance_id,
                remittance.sender.clone(),
                remittance.agent.clone(),
                usdc_token.clone(),
                payout_amount,
            );

            emit_settlement_completed(
                &env,
                remittance.sender.clone(),
                remittance.agent.clone(),
                usdc_token.clone(),
                payout_amount,
            );

            settled_ids.push_back(remittance_id);
        }

        // Update accumulated fees
        let current_fees = get_accumulated_fees(&env)?;
        let new_fees = current_fees
            .checked_add(total_fees)
            .ok_or(ContractError::Overflow)?;
        set_accumulated_fees(&env, new_fees);

        // Emit batch completed event
        emit_batch_settlement_completed(&env, settled_ids.len() as u32, 0);

        log_batch_settlement(&env, settled_ids.len() as u32, 0);

        Ok(BatchSettlementResult { settled_ids })
    }
}

#[cfg(test)]
mod batch_settlement_tests {
    use crate::{SwiftRemitContract, SwiftRemitContractClient, BatchSettlementEntry, BatchSettlementResult, RemittanceStatus, MAX_BATCH_SIZE};
    use soroban_sdk::{testutils::Address as _, token, Address, Env};

    fn create_token_contract<'a>(env: &Env, admin: &Address) -> token::StellarAssetClient<'a> {
        // Create a dummy token for testing
        let token_contract = env.register_stellar_asset_contract_v2(admin.clone());
        // Get the address from the registered contract
        let token_addr = token_contract.address();
        token::StellarAssetClient::new(env, &token_addr)
    }

    fn create_swiftremit_contract<'a>(env: &Env) -> SwiftRemitContractClient<'a> {
        SwiftRemitContractClient::new(env, &env.register_contract(None, SwiftRemitContract {}))
    }

    #[test]
    fn test_batch_settle_success() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let token_admin = Address::generate(&env);
        let token = create_token_contract(&env, &token_admin);
        let sender = Address::generate(&env);
        let agent = Address::generate(&env);

        // Fund sender with enough tokens for multiple remittances
        token.mint(&sender, &10000);

        let contract = create_swiftremit_contract(&env);
        contract.initialize(&admin, &token.address, &250);
        contract.register_agent(&agent);

        // Create multiple remittances
        let remittance_id_1 = contract.create_remittance(&sender, &agent, &1000, &None);
        let remittance_id_2 = contract.create_remittance(&sender, &agent, &2000, &None);
        let remittance_id_3 = contract.create_remittance(&sender, &agent, &3000, &None);

        // Create batch settlement entries using Vec
        let mut entries = crate::Vec::new(&env);
        entries.push_back(BatchSettlementEntry { remittance_id: remittance_id_1 });
        entries.push_back(BatchSettlementEntry { remittance_id: remittance_id_2 });
        entries.push_back(BatchSettlementEntry { remittance_id: remittance_id_3 });

        // Execute batch settlement
        let result: BatchSettlementResult = contract.batch_settle(&entries);

        // Verify all settlements were processed
        assert_eq!(result.settled_ids.len(), 3);
        assert!(result.settled_ids.contains(&remittance_id_1));
        assert!(result.settled_ids.contains(&remittance_id_2));
        assert!(result.settled_ids.contains(&remittance_id_3));

        // Verify remittance statuses
        let rem1 = contract.get_remittance(&remittance_id_1);
        let rem2 = contract.get_remittance(&remittance_id_2);
        let rem3 = contract.get_remittance(&remittance_id_3);
        assert_eq!(rem1.status, RemittanceStatus::Completed);
        assert_eq!(rem2.status, RemittanceStatus::Completed);
        assert_eq!(rem3.status, RemittanceStatus::Completed);
    }

    #[test]
    #[should_panic(expected = "14")]
    fn test_batch_settle_empty_batch() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let token_admin = Address::generate(&env);
        let token = create_token_contract(&env, &token_admin);
        let agent = Address::generate(&env);

        let contract = create_swiftremit_contract(&env);
        contract.initialize(&admin, &token.address, &250);
        contract.register_agent(&agent);

        // Try to settle with empty batch
        let entries: crate::Vec<BatchSettlementEntry> = crate::Vec::new(&env);
        contract.batch_settle(&entries);
    }

    #[test]
    fn test_batch_settle_max_size_allowed() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let token_admin = Address::generate(&env);
        let token = create_token_contract(&env, &token_admin);
        let sender = Address::generate(&env);
        let agent = Address::generate(&env);

        // Fund sender with enough tokens for max batch
        let total_amount = 1000 * (MAX_BATCH_SIZE as i128);
        token.mint(&sender, &(total_amount + 1000));

        let contract = create_swiftremit_contract(&env);
        contract.initialize(&admin, &token.address, &250);
        contract.register_agent(&agent);

        // Create MAX_BATCH_SIZE remittances
        let mut entries: crate::Vec<BatchSettlementEntry> = crate::Vec::new(&env);
        for i in 0..MAX_BATCH_SIZE {
            let remittance_id = contract.create_remittance(&sender, &agent, &1000, &None);
            entries.push_back(BatchSettlementEntry { remittance_id });
        }

        // This should succeed (exactly at max size)
        let result: BatchSettlementResult = contract.batch_settle(&entries);
        assert_eq!(result.settled_ids.len(), MAX_BATCH_SIZE);
    }

    #[test]
    #[should_panic(expected = "15")]
    fn test_batch_settle_exceeds_max_size() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let token_admin = Address::generate(&env);
        let token = create_token_contract(&env, &token_admin);
        let sender = Address::generate(&env);
        let agent = Address::generate(&env);

        // Fund sender with enough tokens
        token.mint(&sender, &1000000);

        let contract = create_swiftremit_contract(&env);
        contract.initialize(&admin, &token.address, &250);
        contract.register_agent(&agent);

        // Create more remittances than MAX_BATCH_SIZE (50)
        let mut entries: crate::Vec<BatchSettlementEntry> = crate::Vec::new(&env);
        for _i in 0..51 {
            let remittance_id = contract.create_remittance(&sender, &agent, &1000, &None);
            entries.push_back(BatchSettlementEntry { remittance_id });
        }

        contract.batch_settle(&entries);
    }

    #[test]
    #[should_panic(expected = "16")]
    fn test_batch_settle_invalid_remittance() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let token_admin = Address::generate(&env);
        let token = create_token_contract(&env, &token_admin);
        let sender = Address::generate(&env);
        let agent = Address::generate(&env);

        token.mint(&sender, &10000);

        let contract = create_swiftremit_contract(&env);
        contract.initialize(&admin, &token.address, &250);
        contract.register_agent(&agent);

        // Create a valid remittance
        let remittance_id = contract.create_remittance(&sender, &agent, &1000, &None);

        // Try to batch settle with an invalid remittance ID (999)
        let mut entries: crate::Vec<BatchSettlementEntry> = crate::Vec::new(&env);
        entries.push_back(BatchSettlementEntry { remittance_id });
        entries.push_back(BatchSettlementEntry { remittance_id: 999 });

        contract.batch_settle(&entries);
    }

    #[test]
    #[should_panic(expected = "16")]
    fn test_batch_settle_duplicate_ids() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let token_admin = Address::generate(&env);
        let token = create_token_contract(&env, &token_admin);
        let sender = Address::generate(&env);
        let agent = Address::generate(&env);

        token.mint(&sender, &10000);

        let contract = create_swiftremit_contract(&env);
        contract.initialize(&admin, &token.address, &250);
        contract.register_agent(&agent);

        let remittance_id = contract.create_remittance(&sender, &agent, &1000, &None);

        // Try to batch settle with duplicate IDs
        let mut entries: crate::Vec<BatchSettlementEntry> = crate::Vec::new(&env);
        entries.push_back(BatchSettlementEntry { remittance_id });
        entries.push_back(BatchSettlementEntry { remittance_id });

        contract.batch_settle(&entries);
    }

    #[test]
    #[should_panic(expected = "16")]
    fn test_batch_settle_already_completed() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let token_admin = Address::generate(&env);
        let token = create_token_contract(&env, &token_admin);
        let sender = Address::generate(&env);
        let agent = Address::generate(&env);

        token.mint(&sender, &10000);

        let contract = create_swiftremit_contract(&env);
        contract.initialize(&admin, &token.address, &250);
        contract.register_agent(&agent);

        // Create and complete a remittance
        let remittance_id = contract.create_remittance(&sender, &agent, &1000, &None);
        contract.confirm_payout(&remittance_id);

        // Try to batch settle an already completed remittance
        let mut entries: crate::Vec<BatchSettlementEntry> = crate::Vec::new(&env);
        entries.push_back(BatchSettlementEntry { remittance_id });

        contract.batch_settle(&entries);
    }

    #[test]
    #[should_panic(expected = "13")]
    fn test_batch_settle_when_paused() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let token_admin = Address::generate(&env);
        let token = create_token_contract(&env, &token_admin);
        let sender = Address::generate(&env);
        let agent = Address::generate(&env);

        token.mint(&sender, &10000);

        let contract = create_swiftremit_contract(&env);
        contract.initialize(&admin, &token.address, &250);
        contract.register_agent(&agent);

        let remittance_id = contract.create_remittance(&sender, &agent, &1000, &None);

        // Pause the contract
        contract.pause();

        // Try to batch settle
        let mut entries: crate::Vec<BatchSettlementEntry> = crate::Vec::new(&env);
        entries.push_back(BatchSettlementEntry { remittance_id });

        contract.batch_settle(&entries);
    }

    // Stress tests
    
    #[test]
    fn test_batch_settle_stress_10_settlements() {
        // Stress test with 10 simultaneous settlements
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let token_admin = Address::generate(&env);
        let token = create_token_contract(&env, &token_admin);
        let sender = Address::generate(&env);
        let agent = Address::generate(&env);

        // Fund sender with enough tokens
        token.mint(&sender, &100000);

        let contract = create_swiftremit_contract(&env);
        contract.initialize(&admin, &token.address, &250);
        contract.register_agent(&agent);

        // Create 10 remittances
        let mut entries: crate::Vec<BatchSettlementEntry> = crate::Vec::new(&env);
        for i in 0..10 {
            let remittance_id = contract.create_remittance(&sender, &agent, &(1000 * ((i + 1) as i128)), &None);
            entries.push_back(BatchSettlementEntry { remittance_id });
        }

        // Execute batch settlement
        let result: BatchSettlementResult = contract.batch_settle(&entries);

        // Verify all settlements were processed
        assert_eq!(result.settled_ids.len(), 10);

        // Verify accumulated fees
        let fees = contract.get_accumulated_fees();
        // Total amount: 1000 + 2000 + ... + 10000 = 55000
        // Fee: 2.5% = 1375
        assert_eq!(fees, 1375);
    }

    #[test]
    fn test_batch_settle_stress_50_settlements() {
        // Stress test with 50 simultaneous settlements
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let token_admin = Address::generate(&env);
        let token = create_token_contract(&env, &token_admin);
        let sender = Address::generate(&env);
        let agent = Address::generate(&env);

        // Fund sender with enough tokens
        token.mint(&sender, &1000000);

        let contract = create_swiftremit_contract(&env);
        contract.initialize(&admin, &token.address, &250);
        contract.register_agent(&agent);

        // Create 50 remittances
        let mut entries: crate::Vec<BatchSettlementEntry> = crate::Vec::new(&env);
        for _i in 0..50 {
            let remittance_id = contract.create_remittance(&sender, &agent, &1000, &None);
            entries.push_back(BatchSettlementEntry { remittance_id });
        }

        // Execute batch settlement
        let result: BatchSettlementResult = contract.batch_settle(&entries);

        // Verify all settlements were processed
        assert_eq!(result.settled_ids.len(), 50);

        // Verify accumulated fees: 50 * 1000 * 0.025 = 1250
        let fees = contract.get_accumulated_fees();
        assert_eq!(fees, 1250);
    }

    #[test]
    fn test_batch_settle_stress_max_size() {
        // Stress test with maximum batch size (100 settlements)
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let token_admin = Address::generate(&env);
        let token = create_token_contract(&env, &token_admin);
        let sender = Address::generate(&env);
        let agent = Address::generate(&env);

        // Fund sender with enough tokens
        token.mint(&sender, &10000000);

        let contract = create_swiftremit_contract(&env);
        contract.initialize(&admin, &token.address, &250);
        contract.register_agent(&agent);

        // Create 100 remittances (MAX_BATCH_SIZE)
        let mut entries: crate::Vec<BatchSettlementEntry> = crate::Vec::new(&env);
        for _i in 0..MAX_BATCH_SIZE {
            let remittance_id = contract.create_remittance(&sender, &agent, &1000, &None);
            entries.push_back(BatchSettlementEntry { remittance_id });
        }

        // Execute batch settlement
        let result: BatchSettlementResult = contract.batch_settle(&entries);

        // Verify all settlements were processed
        assert_eq!(result.settled_ids.len(), MAX_BATCH_SIZE);

        // Verify accumulated fees: 50 * 1000 * 0.025 = 1250
        let fees = contract.get_accumulated_fees();
        assert_eq!(fees, 1250);
    }

    #[test]
    fn test_batch_settle_multiple_batches() {
        // Test processing multiple batches sequentially
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let token_admin = Address::generate(&env);
        let token = create_token_contract(&env, &token_admin);
        let sender = Address::generate(&env);
        let agent = Address::generate(&env);

        // Fund sender with enough tokens
        token.mint(&sender, &100000);

        let contract = create_swiftremit_contract(&env);
        contract.initialize(&admin, &token.address, &250);
        contract.register_agent(&agent);

        // First batch - 5 remittances
        let mut entries1: crate::Vec<BatchSettlementEntry> = crate::Vec::new(&env);
        for _i in 0..5 {
            let remittance_id = contract.create_remittance(&sender, &agent, &1000, &None);
            entries1.push_back(BatchSettlementEntry { remittance_id });
        }
        let result1: BatchSettlementResult = contract.batch_settle(&entries1);
        assert_eq!(result1.settled_ids.len(), 5);

        // Second batch - 5 more remittances
        let mut entries2: crate::Vec<BatchSettlementEntry> = crate::Vec::new(&env);
        for _i in 0..5 {
            let remittance_id = contract.create_remittance(&sender, &agent, &1000, &None);
            entries2.push_back(BatchSettlementEntry { remittance_id });
        }
        let result2: BatchSettlementResult = contract.batch_settle(&entries2);
        assert_eq!(result2.settled_ids.len(), 5);

        // Verify total accumulated fees: 10 * 1000 * 0.025 = 250
        let fees = contract.get_accumulated_fees();
        assert_eq!(fees, 250);
    }
}
