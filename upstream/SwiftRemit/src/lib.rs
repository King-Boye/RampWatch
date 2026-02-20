//! SwiftRemit - A Soroban smart contract for cross-border remittance services.
//!
//! This contract enables secure, fee-based remittance transactions between senders and agents,
//! with built-in duplicate settlement protection and expiry mechanisms.

#![no_std]

mod errors;
mod events;
mod storage;
mod types;
mod validation;

use soroban_sdk::{contract, contractimpl, token, Address, Env};

pub use errors::ContractError;
pub use events::*;
pub use storage::*;
pub use types::*;
pub use validation::*;

/// The main SwiftRemit contract for managing cross-border remittances.
///
/// This contract handles the complete lifecycle of remittance transactions including:
/// - Agent registration and management
/// - Remittance creation with automatic fee calculation
/// - Settlement confirmation with duplicate protection
/// - Cancellation and refund processing
/// - Platform fee collection and withdrawal
#[contract]
pub struct SwiftRemitContract;

#[contractimpl]
impl SwiftRemitContract {
    /// Initializes the contract with admin, token, and fee configuration.
    ///
    /// This function can only be called once. It sets up the contract's core parameters
    /// and initializes all counters and accumulators to zero.
    ///
    /// # Arguments
    ///
    /// * `env` - The contract execution environment
    /// * `admin` - Address that will have administrative privileges
    /// * `usdc_token` - Address of the USDC token contract used for transactions
    /// * `fee_bps` - Platform fee in basis points (1 bps = 0.01%, max 10000 = 100%)
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Contract successfully initialized
    /// * `Err(ContractError::AlreadyInitialized)` - Contract was already initialized
    /// * `Err(ContractError::InvalidFeeBps)` - Fee exceeds maximum allowed (10000 bps)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// contract.initialize(env, admin_addr, usdc_addr, 250); // 2.5% fee
    /// ```
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

        Ok(())
    }

    /// Registers a new agent authorized to receive remittance payouts.
    ///
    /// Only the contract admin can register agents. Registered agents can confirm
    /// payouts for remittances assigned to them.
    ///
    /// # Arguments
    ///
    /// * `env` - The contract execution environment
    /// * `agent` - Address to register as an authorized agent
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Agent successfully registered
    /// * `Err(ContractError::NotInitialized)` - Contract not initialized
    ///
    /// # Authorization
    ///
    /// Requires authentication from the contract admin.
    pub fn register_agent(env: Env, agent: Address) -> Result<(), ContractError> {
        let admin = get_admin(&env)?;
        admin.require_auth();

        set_agent_registered(&env, &agent, true);
        emit_agent_registered(&env, agent);

        Ok(())
    }

    /// Removes an agent's authorization to receive remittance payouts.
    ///
    /// Only the contract admin can remove agents. Removed agents cannot confirm
    /// new payouts, but existing remittances assigned to them remain valid.
    ///
    /// # Arguments
    ///
    /// * `env` - The contract execution environment
    /// * `agent` - Address of the agent to remove
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Agent successfully removed
    /// * `Err(ContractError::NotInitialized)` - Contract not initialized
    ///
    /// # Authorization
    ///
    /// Requires authentication from the contract admin.
    pub fn remove_agent(env: Env, agent: Address) -> Result<(), ContractError> {
        let admin = get_admin(&env)?;
        admin.require_auth();

        set_agent_registered(&env, &agent, false);
        emit_agent_removed(&env, agent);

        Ok(())
    }

    /// Updates the platform fee rate.
    ///
    /// Only the contract admin can update the fee. The new fee applies to all
    /// remittances created after the update.
    ///
    /// # Arguments
    ///
    /// * `env` - The contract execution environment
    /// * `fee_bps` - New platform fee in basis points (1 bps = 0.01%, max 10000 = 100%)
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Fee successfully updated
    /// * `Err(ContractError::NotInitialized)` - Contract not initialized
    /// * `Err(ContractError::InvalidFeeBps)` - Fee exceeds maximum allowed (10000 bps)
    ///
    /// # Authorization
    ///
    /// Requires authentication from the contract admin.
    pub fn update_fee(env: Env, fee_bps: u32) -> Result<(), ContractError> {
        let admin = get_admin(&env)?;
        admin.require_auth();

        if fee_bps > 10000 {
            return Err(ContractError::InvalidFeeBps);
        }

        set_platform_fee_bps(&env, fee_bps);
        emit_fee_updated(&env, fee_bps);

        Ok(())
    }

    /// Creates a new remittance transaction.
    ///
    /// Transfers the specified amount from the sender to the contract, calculates
    /// the platform fee, and creates a pending remittance record. The agent can later
    /// confirm the payout to receive the amount minus fees.
    ///
    /// # Arguments
    ///
    /// * `env` - The contract execution environment
    /// * `sender` - Address initiating the remittance
    /// * `agent` - Address of the registered agent who will receive the payout
    /// * `amount` - Amount to remit in USDC (must be positive)
    /// * `expiry` - Optional expiry timestamp (seconds since epoch) after which settlement fails
    ///
    /// # Returns
    ///
    /// * `Ok(remittance_id)` - Unique ID of the created remittance
    /// * `Err(ContractError::InvalidAmount)` - Amount is zero or negative
    /// * `Err(ContractError::AgentNotRegistered)` - Specified agent is not registered
    /// * `Err(ContractError::Overflow)` - Arithmetic overflow in fee calculation
    /// * `Err(ContractError::NotInitialized)` - Contract not initialized
    ///
    /// # Authorization
    ///
    /// Requires authentication from the sender address.
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
        let remittance_id = counter
            .checked_add(1)
            .ok_or(ContractError::Overflow)?;

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

        emit_remittance_created(&env, remittance_id, sender, agent, amount, fee);

        Ok(remittance_id)
    }

    /// Confirms a remittance payout to the agent.
    ///
    /// Transfers the remittance amount (minus platform fee) to the agent and marks
    /// the remittance as completed. Includes duplicate settlement protection and
    /// expiry validation.
    ///
    /// # Arguments
    ///
    /// * `env` - The contract execution environment
    /// * `remittance_id` - ID of the remittance to confirm
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Payout successfully confirmed and transferred
    /// * `Err(ContractError::RemittanceNotFound)` - Remittance ID does not exist
    /// * `Err(ContractError::InvalidStatus)` - Remittance is not in Pending status
    /// * `Err(ContractError::DuplicateSettlement)` - Settlement already executed
    /// * `Err(ContractError::SettlementExpired)` - Current time exceeds expiry timestamp
    /// * `Err(ContractError::InvalidAddress)` - Agent address validation failed
    /// * `Err(ContractError::Overflow)` - Arithmetic overflow in payout calculation
    ///
    /// # Authorization
    ///
    /// Requires authentication from the agent address assigned to the remittance.
    pub fn confirm_payout(env: Env, remittance_id: u64) -> Result<(), ContractError> {
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

        emit_remittance_completed(&env, remittance_id, remittance.agent, payout_amount);

        Ok(())
    }

    /// Cancels a pending remittance and refunds the sender.
    ///
    /// Returns the full remittance amount to the sender and marks the remittance
    /// as cancelled. Can only be called by the original sender.
    ///
    /// # Arguments
    ///
    /// * `env` - The contract execution environment
    /// * `remittance_id` - ID of the remittance to cancel
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Remittance successfully cancelled and refunded
    /// * `Err(ContractError::RemittanceNotFound)` - Remittance ID does not exist
    /// * `Err(ContractError::InvalidStatus)` - Remittance is not in Pending status
    ///
    /// # Authorization
    ///
    /// Requires authentication from the sender address who created the remittance.
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

        emit_remittance_cancelled(&env, remittance_id, remittance.sender, remittance.amount);

        Ok(())
    }

    /// Withdraws accumulated platform fees to a specified address.
    ///
    /// Transfers all accumulated fees to the recipient address and resets the
    /// fee counter to zero. Only the contract admin can withdraw fees.
    ///
    /// # Arguments
    ///
    /// * `env` - The contract execution environment
    /// * `to` - Address to receive the withdrawn fees
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Fees successfully withdrawn
    /// * `Err(ContractError::NotInitialized)` - Contract not initialized
    /// * `Err(ContractError::NoFeesToWithdraw)` - No fees available (balance is zero or negative)
    /// * `Err(ContractError::InvalidAddress)` - Recipient address validation failed
    ///
    /// # Authorization
    ///
    /// Requires authentication from the contract admin.
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

        emit_fees_withdrawn(&env, to, fees);

        Ok(())
    }

    /// Retrieves a remittance record by ID.
    ///
    /// # Arguments
    ///
    /// * `env` - The contract execution environment
    /// * `remittance_id` - ID of the remittance to retrieve
    ///
    /// # Returns
    ///
    /// * `Ok(Remittance)` - The remittance record
    /// * `Err(ContractError::RemittanceNotFound)` - Remittance ID does not exist
    pub fn get_remittance(env: Env, remittance_id: u64) -> Result<Remittance, ContractError> {
        get_remittance(&env, remittance_id)
    }

    /// Retrieves the total accumulated platform fees.
    ///
    /// # Arguments
    ///
    /// * `env` - The contract execution environment
    ///
    /// # Returns
    ///
    /// * `Ok(i128)` - Total accumulated fees in USDC
    /// * `Err(ContractError::NotInitialized)` - Contract not initialized
    pub fn get_accumulated_fees(env: Env) -> Result<i128, ContractError> {
        get_accumulated_fees(&env)
    }

    /// Checks if an address is registered as an agent.
    ///
    /// # Arguments
    ///
    /// * `env` - The contract execution environment
    /// * `agent` - Address to check
    ///
    /// # Returns
    ///
    /// * `true` - Address is a registered agent
    /// * `false` - Address is not registered
    pub fn is_agent_registered(env: Env, agent: Address) -> bool {
        is_agent_registered(&env, &agent)
    }

    /// Retrieves the current platform fee rate.
    ///
    /// # Arguments
    ///
    /// * `env` - The contract execution environment
    ///
    /// # Returns
    ///
    /// * `Ok(u32)` - Platform fee in basis points (1 bps = 0.01%)
    /// * `Err(ContractError::NotInitialized)` - Contract not initialized
    pub fn get_platform_fee_bps(env: Env) -> Result<u32, ContractError> {
        get_platform_fee_bps(&env)
    }
}
