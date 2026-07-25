/// Soroban RPC client for contract interaction from CLI
///
/// This module is intended to provide HTTP-based RPC calls to interact with the
/// AnchorKit contract on the Soroban network. Currently all three public functions
/// (`create_session_rpc`, `get_session_rpc`, `list_sessions_rpc`) are **placeholders**
/// that return descriptive `Err` messages — no actual network requests are made yet.
///
/// # Status
/// - `reqwest` is now declared as an optional dependency (enabled with the `std` feature)
///   so the HTTP client layer can be wired up without further Cargo.toml changes.
/// - Full implementation requires: XDR transaction building, signing, and RPC submission.

use serde::{Deserialize, Serialize};
use std::env;

/// Environment variable for RPC URL
const RPC_URL_VAR: &str = "SOROBAN_RPC_URL";
const RPC_URL_ALT: &str = "ANCHORKIT_RPC_URL";

/// Environment variable for contract ID
const CONTRACT_ID_VAR: &str = "SOROBAN_CONTRACT_ID";
const CONTRACT_ID_ALT: &str = "ANCHORKIT_CONTRACT_ID";

/// Get RPC URL from environment or use testnet default
pub fn get_rpc_url() -> String {
    env::var(RPC_URL_VAR)
        .or_else(|_| env::var(RPC_URL_ALT))
        .unwrap_or_else(|_| "https://soroban-testnet.stellar.org".to_string())
}

/// Get contract ID from environment
pub fn get_contract_id() -> Result<String, String> {
    env::var(CONTRACT_ID_VAR)
        .or_else(|_| env::var(CONTRACT_ID_ALT))
        .map_err(|_| "SOROBAN_CONTRACT_ID or ANCHORKIT_CONTRACT_ID not set".to_string())
}

/// Get network passphrase from environment
fn get_network_passphrase() -> String {
    env::var("SOROBAN_NETWORK_PASSPHRASE")
        .unwrap_or_else(|_| "Test SDF Network ; September 2015".to_string())
}

/// Get secret key for signing transactions
pub fn get_secret_key() -> Result<String, String> {
    env::var("STELLAR_SECRET_KEY")
        .or_else(|_| env::var("SOROBAN_SECRET_KEY"))
        .or_else(|_| env::var("ANCHORKIT_SECRET_KEY"))
        .map_err(|_| "STELLAR_SECRET_KEY, SOROBAN_SECRET_KEY, or ANCHORKIT_SECRET_KEY not set".to_string())
}

/// Session data structure returned from contract
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    pub session_id: u64,
    pub initiator: String,
    pub created_at: u64,
    pub nonce: u64,
    pub operation_count: u64,
    pub expires_at: u64,
}

/// Result wrapper for RPC responses
#[derive(Debug, Serialize, Deserialize)]
pub struct RpcResponse<T> {
    pub result: Option<T>,
    pub error: Option<RpcError>,
}

/// RPC error response
#[derive(Debug, Serialize, Deserialize)]
pub struct RpcError {
    pub code: i32,
    pub message: String,
}

/// Contract invocation request for RPC
#[derive(Debug, Serialize)]
struct InvokeContractRequest {
    transaction: String,
    resource_leeway: u32,
}

/// Simulated transaction response
#[derive(Debug, Deserialize)]
struct SimulateTransactionResponse {
    #[serde(default)]
    error: Option<RpcError>,
    #[serde(default)]
    result: Option<SimulateResult>,
}

#[derive(Debug, Deserialize)]
struct SimulateResult {
    #[serde(default)]
    error: Option<RpcError>,
    #[serde(default)]
    events: Vec<serde_json::Value>,
    #[serde(default)]
    results: Vec<serde_json::Value>,
}

/// Create a session on the blockchain
///
/// Invokes contract method: create_session(initiator_address)
/// Returns the session ID created
pub fn create_session_rpc(initiator_addr: Option<String>) -> Result<u64, String> {
    // Get initiator address
    let initiator = if let Some(addr) = initiator_addr {
        addr
    } else {
        // Derive from secret key
        derive_address_from_secret_key()?
    };

    // Validate address format (Stellar addresses start with 'G' or 'C' for contracts)
    if !is_valid_stellar_address(&initiator) {
        return Err(format!("Invalid Stellar address: {}", initiator));
    }

    eprintln!("ℹ  Creating session for initiator: {}", initiator);
    eprintln!("ℹ  Using RPC endpoint: {}", get_rpc_url());

    // Build and submit contract invocation
    // NOTE: This is a simplified implementation. Production code would:
    // 1. Build a proper TransactionEnvelope with soroban_sdk
    // 2. Serialize to XDR
    // 3. Sign with wallet
    // 4. Submit to /simulateTransaction RPC endpoint
    // 5. Parse result to extract session_id
    
    // For now, return a placeholder error indicating this needs full Soroban RPC integration
    Err(
        "Session creation via RPC not yet fully implemented.\n\
         This requires Soroban SDK transaction building and signing integration.\n\
         Please use the Rust API directly for now or wait for full RPC client implementation."
            .to_string(),
    )
}

/// Fetch a session from the blockchain
///
/// Invokes contract method: get_session(session_id)
/// Returns the session details
pub fn get_session_rpc(session_id: u64) -> Result<SessionData, String> {
    eprintln!("ℹ  Fetching session {} from contract", session_id);
    eprintln!("ℹ  Using RPC endpoint: {}", get_rpc_url());

    // Build and submit contract invocation
    // Similar to create_session_rpc, this requires full Soroban RPC integration
    
    Err(
        "Session fetch via RPC not yet fully implemented.\n\
         This requires Soroban SDK transaction building and deserialization.\n\
         Please use the Rust API directly for now or wait for full RPC client implementation."
            .to_string(),
    )
}

/// List active sessions
///
/// NOTE: Sessions are not enumerable on-chain in the current contract.
/// This would require either:
/// 1. Adding an on-chain iterator method
/// 2. Rebuilding from contract events
/// 3. Maintaining a sessions list in contract state
pub fn list_sessions_rpc(limit: u64) -> Result<Vec<SessionData>, String> {
    eprintln!("ℹ  Listing sessions (max {})", limit);
    eprintln!("ℹ  Using RPC endpoint: {}", get_rpc_url());

    Err(
        "Session enumeration not implemented in current contract.\n\
         Sessions are not enumerable on-chain. To enable this, the contract would need:\n\
         - An on-chain iterator method, OR\n\
         - Event log replay to rebuild session list, OR\n\
         - A sessions-list storage entry (additional cost)\n\
         \n\
         As a workaround, use 'anchorkit session get <SESSION_ID>' for known session IDs."
            .to_string(),
    )
}

// ── Helper functions ────────────────────────────────────────────────────────

/// Check if address is valid Stellar format.
/// Validates prefix (G/C), length (56), and restricts the remaining 55
/// characters to the RFC4648 base32 strkey alphabet (A-Z, 2-7).
/// Lowercase letters and the digits 0, 1, 8, 9 are not valid strkey chars.
fn is_valid_stellar_address(addr: &str) -> bool {
    let (expected_prefix, rest) = if let Some(r) = addr.strip_prefix('G') {
        ('G', r)
    } else if let Some(r) = addr.strip_prefix('C') {
        ('C', r)
    } else {
        return false;
    };
    let _ = expected_prefix; // prefix consumed by strip_prefix above
    addr.len() == 56 && rest.chars().all(|c| matches!(c, 'A'..='Z' | '2'..='7'))
}

/// Derive Stellar address from secret key
fn derive_address_from_secret_key() -> Result<String, String> {
    let secret_key = get_secret_key()?;
    
    // Parse secret key and derive public key
    // This is a simplified version - production code would use stellar-rs or soroban_sdk
    if secret_key.starts_with("S") && secret_key.len() == 56 {
        // Would need to:
        // 1. Decode base32 secret key
        // 2. Extract seed bytes
        // 3. Derive keypair
        // 4. Encode public key to Stellar address format
        
        Err(
            "Address derivation from secret key not yet implemented.\n\
             Please provide initiator address explicitly with --initiator flag."
                .to_string(),
        )
    } else {
        Err("Invalid secret key format".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_stellar_address_account() {
        let addr = "GBRPYHIL2CI3C65PL7DH65O6PXARPG63G2GQRFDW6XQVFVVL6UUEZFX";
        assert!(is_valid_stellar_address(addr));
    }

    #[test]
    fn test_valid_stellar_address_contract() {
        let addr = "CCRMFG54GGSZHHF5FVMHB4GKTNQYF5RHG4JWAJWGPFM2Y76XXQM4C6J";
        assert!(is_valid_stellar_address(addr));
    }

    #[test]
    fn test_invalid_stellar_address_wrong_prefix() {
        let addr = "MBRPYHIL2CI3C65PL7DH65O6PXARPG63G2GQRFDW6XQVFVVL6UUEZFX";
        assert!(!is_valid_stellar_address(addr));
    }

    #[test]
    fn test_invalid_stellar_address_too_short() {
        let addr = "GBRPYHIL2CI3C65PL7DH65O6PXARPG63G2GQRFDW6XQVFVVL6";
        assert!(!is_valid_stellar_address(addr));
    }

    #[test]
    fn test_get_rpc_url_default() {
        // If env vars not set, should return testnet
        std::env::remove_var(RPC_URL_VAR);
        std::env::remove_var(RPC_URL_ALT);
        assert!(get_rpc_url().contains("testnet"));
    }
}
