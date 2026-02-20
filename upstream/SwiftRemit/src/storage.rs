//! Storage management for the SwiftRemit contract.
//!
//! This module provides functions for reading and writing contract state,
//! including configuration, remittance records, agent registration, and fee tracking.
//! Uses both instance storage (contract-level config) and persistent storage
//! (per-entity data).

use soroban_sdk::{contracttype, Address, Env};

use crate::{ContractError, Remittance};

/// Storage keys for the SwiftRemit contract.
/// 
/// Storage Layout:
/// - Instance storage: Contract-level configuration and state (Admin, UsdcToken, PlatformFeeBps, 
///   RemittanceCounter, AccumulatedFees)
/// - Persistent storage: Per-entity data that needs long-term retention (Remittance records, 
///   AgentRegistered status)
#[contracttype]
#[derive(Clone)]
enum DataKey {
    // === Contract Configuration ===
    // Core contract settings stored in instance storage
    
    /// Contract administrator address with privileged access
    Admin,
    
    /// USDC token contract address used for all remittance transactions
    UsdcToken,
    
    /// Platform fee in basis points (1 bps = 0.01%)
    PlatformFeeBps,
    
    // === Remittance Management ===
    // Keys for tracking and storing remittance transactions
    
    /// Global counter for generating unique remittance IDs
    RemittanceCounter,
    
    /// Individual remittance record indexed by ID (persistent storage)
    Remittance(u64),
    
    // === Agent Management ===
    // Keys for tracking registered agents
    
    /// Agent registration status indexed by agent address (persistent storage)
    AgentRegistered(Address),
    
    // === Fee Tracking ===
    // Keys for managing platform fees
    
    /// Total accumulated platform fees awaiting withdrawal
    AccumulatedFees,
    
    // === Settlement Deduplication ===
    // Keys for preventing duplicate settlement execution
    
    /// Settlement hash for duplicate detection (persistent storage)
    SettlementHash(u64),
}

/// Checks if the contract has an admin configured.
///
/// # Arguments
///
/// * `env` - The contract execution environment
///
/// # Returns
///
/// * `true` - Admin is configured
/// * `false` - Admin is not configured (contract not initialized)
pub fn has_admin(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Admin)
}

/// Sets the contract administrator address.
///
/// # Arguments
///
/// * `env` - The contract execution environment
/// * `admin` - Address to set as admin
pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
}

/// Retrieves the contract administrator address.
///
/// # Arguments
///
/// * `env` - The contract execution environment
///
/// # Returns
///
/// * `Ok(Address)` - The admin address
/// * `Err(ContractError::NotInitialized)` - Contract not initialized
pub fn get_admin(env: &Env) -> Result<Address, ContractError> {
    env.storage()
        .instance()
        .get(&DataKey::Admin)
        .ok_or(ContractError::NotInitialized)
}

/// Sets the USDC token contract address.
///
/// # Arguments
///
/// * `env` - The contract execution environment
/// * `token` - Address of the USDC token contract
pub fn set_usdc_token(env: &Env, token: &Address) {
    env.storage().instance().set(&DataKey::UsdcToken, token);
}

/// Retrieves the USDC token contract address.
///
/// # Arguments
///
/// * `env` - The contract execution environment
///
/// # Returns
///
/// * `Ok(Address)` - The USDC token contract address
/// * `Err(ContractError::NotInitialized)` - Contract not initialized
pub fn get_usdc_token(env: &Env) -> Result<Address, ContractError> {
    env.storage()
        .instance()
        .get(&DataKey::UsdcToken)
        .ok_or(ContractError::NotInitialized)
}

/// Sets the platform fee rate.
///
/// # Arguments
///
/// * `env` - The contract execution environment
/// * `fee_bps` - Fee in basis points (1 bps = 0.01%)
pub fn set_platform_fee_bps(env: &Env, fee_bps: u32) {
    env.storage()
        .instance()
        .set(&DataKey::PlatformFeeBps, &fee_bps);
}

/// Retrieves the platform fee rate.
///
/// # Arguments
///
/// * `env` - The contract execution environment
///
/// # Returns
///
/// * `Ok(u32)` - Fee in basis points
/// * `Err(ContractError::NotInitialized)` - Contract not initialized
pub fn get_platform_fee_bps(env: &Env) -> Result<u32, ContractError> {
    env.storage()
        .instance()
        .get(&DataKey::PlatformFeeBps)
        .ok_or(ContractError::NotInitialized)
}

/// Sets the remittance counter for ID generation.
///
/// # Arguments
///
/// * `env` - The contract execution environment
/// * `counter` - Current counter value
pub fn set_remittance_counter(env: &Env, counter: u64) {
    env.storage()
        .instance()
        .set(&DataKey::RemittanceCounter, &counter);
}

/// Retrieves the current remittance counter.
///
/// # Arguments
///
/// * `env` - The contract execution environment
///
/// # Returns
///
/// * `Ok(u64)` - Current counter value
/// * `Err(ContractError::NotInitialized)` - Contract not initialized
pub fn get_remittance_counter(env: &Env) -> Result<u64, ContractError> {
    env.storage()
        .instance()
        .get(&DataKey::RemittanceCounter)
        .ok_or(ContractError::NotInitialized)
}

/// Stores a remittance record.
///
/// # Arguments
///
/// * `env` - The contract execution environment
/// * `id` - Remittance ID
/// * `remittance` - Remittance record to store
pub fn set_remittance(env: &Env, id: u64, remittance: &Remittance) {
    env.storage()
        .persistent()
        .set(&DataKey::Remittance(id), remittance);
}

/// Retrieves a remittance record by ID.
///
/// # Arguments
///
/// * `env` - The contract execution environment
/// * `id` - Remittance ID to retrieve
///
/// # Returns
///
/// * `Ok(Remittance)` - The remittance record
/// * `Err(ContractError::RemittanceNotFound)` - Remittance does not exist
pub fn get_remittance(env: &Env, id: u64) -> Result<Remittance, ContractError> {
    env.storage()
        .persistent()
        .get(&DataKey::Remittance(id))
        .ok_or(ContractError::RemittanceNotFound)
}

/// Sets an agent's registration status.
///
/// # Arguments
///
/// * `env` - The contract execution environment
/// * `agent` - Agent address
/// * `registered` - Registration status (true = registered, false = removed)
pub fn set_agent_registered(env: &Env, agent: &Address, registered: bool) {
    env.storage()
        .persistent()
        .set(&DataKey::AgentRegistered(agent.clone()), &registered);
}

/// Checks if an address is registered as an agent.
///
/// # Arguments
///
/// * `env` - The contract execution environment
/// * `agent` - Agent address to check
///
/// # Returns
///
/// * `true` - Address is registered
/// * `false` - Address is not registered
pub fn is_agent_registered(env: &Env, agent: &Address) -> bool {
    env.storage()
        .persistent()
        .get(&DataKey::AgentRegistered(agent.clone()))
        .unwrap_or(false)
}

/// Sets the accumulated platform fees.
///
/// # Arguments
///
/// * `env` - The contract execution environment
/// * `fees` - Total accumulated fees
pub fn set_accumulated_fees(env: &Env, fees: i128) {
    env.storage()
        .instance()
        .set(&DataKey::AccumulatedFees, &fees);
}

/// Retrieves the accumulated platform fees.
///
/// # Arguments
///
/// * `env` - The contract execution environment
///
/// # Returns
///
/// * `Ok(i128)` - Total accumulated fees
/// * `Err(ContractError::NotInitialized)` - Contract not initialized
pub fn get_accumulated_fees(env: &Env) -> Result<i128, ContractError> {
    env.storage()
        .instance()
        .get(&DataKey::AccumulatedFees)
        .ok_or(ContractError::NotInitialized)
}

/// Checks if a settlement hash exists for duplicate detection.
///
/// # Arguments
///
/// * `env` - The contract execution environment
/// * `remittance_id` - Remittance ID to check
///
/// # Returns
///
/// * `true` - Settlement has been executed
/// * `false` - Settlement has not been executed
pub fn has_settlement_hash(env: &Env, remittance_id: u64) -> bool {
    env.storage()
        .persistent()
        .has(&DataKey::SettlementHash(remittance_id))
}

/// Marks a settlement as executed for duplicate prevention.
///
/// # Arguments
///
/// * `env` - The contract execution environment
/// * `remittance_id` - Remittance ID to mark as settled
pub fn set_settlement_hash(env: &Env, remittance_id: u64) {
    env.storage()
        .persistent()
        .set(&DataKey::SettlementHash(remittance_id), &true);
}
