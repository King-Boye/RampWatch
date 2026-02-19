use soroban_sdk::contracterror;

/// Error codes for AnchorKit contract operations.
/// All error codes are in the range 100-120 for stable API compatibility.
/// See API_SPEC.md for detailed documentation.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// Contract has already been initialized
    AlreadyInitialized = 100,
    /// Contract has not been initialized yet
    NotInitialized = 101,
    /// Caller is not a registered attestor
    UnauthorizedAttestor = 102,
    /// Attestor is already registered
    AttestorAlreadyRegistered = 103,
    /// Attestor is not registered
    AttestorNotRegistered = 104,
    /// Attestation with this hash has already been submitted (replay attack)
    ReplayAttack = 105,
    /// Timestamp is invalid (zero or in the future)
    InvalidTimestamp = 106,
    /// Attestation with the given ID was not found
    AttestationNotFound = 107,
    /// Public key format is invalid
    InvalidPublicKey = 108,
}
