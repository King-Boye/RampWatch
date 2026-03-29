use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Bytes, Env, String, Symbol, Vec,
};

use crate::errors::ErrorCode;
use crate::sep10_jwt;

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

#[contracttype]
#[derive(Clone)]
pub struct Session {
    pub session_id: u64,
    pub initiator: Address,
    pub created_at: u64,
    pub nonce: u64,
    pub operation_count: u64,
}

#[contracttype]
#[derive(Clone)]
pub struct Quote {
    pub quote_id: u64,
    pub anchor: Address,
    pub base_asset: String,
    pub quote_asset: String,
    pub rate: u64,
    pub fee_percentage: u32,
    pub minimum_amount: u64,
    pub maximum_amount: u64,
    pub valid_until: u64,
}

#[contracttype]
#[derive(Clone)]
pub struct OperationContext {
    pub session_id: u64,
    pub operation_index: u64,
    pub operation_type: String,
    pub timestamp: u64,
    pub status: String,
    pub result_data: u64,
}

#[contracttype]
#[derive(Clone)]
pub struct AuditLog {
    pub log_id: u64,
    pub session_id: u64,
    pub actor: Address,
    pub operation: OperationContext,
}

#[contracttype]
#[derive(Clone)]
pub struct RequestId {
    pub id: Bytes,
    pub created_at: u64,
}

#[contracttype]
#[derive(Clone)]
pub struct Attestation {
    pub id: u64,
    pub issuer: Address,
    pub subject: Address,
    pub timestamp: u64,
    pub payload_hash: Bytes,
    pub signature: Bytes,
}

#[contracttype]
#[derive(Clone)]
pub struct TracingSpan {
    pub request_id: RequestId,
    pub operation: String,
    pub actor: Address,
    pub started_at: u64,
    pub completed_at: u64,
    pub status: String,
}

#[contracttype]
#[derive(Clone)]
pub struct AnchorServices {
    pub anchor: Address,
    pub services: Vec<u32>,
}

pub const SERVICE_DEPOSITS: u32 = 1;
pub const SERVICE_WITHDRAWALS: u32 = 2;
pub const SERVICE_QUOTES: u32 = 3;
pub const SERVICE_KYC: u32 = 4;

/// Typed representation of a service capability an anchor can support.
///
/// Each variant maps to a stable `u32` discriminant stored on-chain.
/// Use [`ServiceType::as_u32`] to convert before passing to contract functions.
#[derive(Clone, PartialEq)]
pub enum ServiceType {
    Deposits,
    Withdrawals,
    Quotes,
    KYC,
}

impl ServiceType {
    pub fn as_u32(&self) -> u32 {
        match self {
            ServiceType::Deposits => SERVICE_DEPOSITS,
            ServiceType::Withdrawals => SERVICE_WITHDRAWALS,
            ServiceType::Quotes => SERVICE_QUOTES,
            ServiceType::KYC => SERVICE_KYC,
        }
    }
}

// ---------------------------------------------------------------------------
// Routing types
// ---------------------------------------------------------------------------

#[contracttype]
#[derive(Clone)]
pub struct RoutingAnchorMeta {
    pub anchor: Address,
    pub reputation_score: u32,
    pub average_settlement_time: u64,
    pub liquidity_score: u32,
    pub uptime_percentage: u32,
    pub total_volume: u64,
    pub is_active: bool,
}

#[contracttype]
#[derive(Clone)]
pub struct RoutingRequest {
    pub base_asset: String,
    pub quote_asset: String,
    pub amount: u64,
    pub operation_type: u32,
}

#[contracttype]
#[derive(Clone)]
pub struct RoutingOptions {
    pub request: RoutingRequest,
    pub strategy: Vec<Symbol>,
    pub min_reputation: u32,
    pub max_anchors: u32,
    pub require_kyc: bool,
}

// ---------------------------------------------------------------------------
// Metadata cache types
// ---------------------------------------------------------------------------

#[contracttype]
#[derive(Clone)]
pub struct AnchorMetadata {
    pub anchor: Address,
    pub reputation_score: u32,
    pub liquidity_score: u32,
    pub uptime_percentage: u32,
    pub total_volume: u64,
    pub average_settlement_time: u64,
    pub is_active: bool,
}

#[contracttype]
#[derive(Clone)]
pub struct MetadataCache {
    pub metadata: AnchorMetadata,
    pub cached_at: u64,
    pub ttl_seconds: u64,
}

#[contracttype]
#[derive(Clone)]
pub struct CapabilitiesCache {
    pub toml_url: String,
    pub capabilities: String,
    pub cached_at: u64,
    pub ttl_seconds: u64,
}

// ---------------------------------------------------------------------------
// Anchor Info Discovery types
// ---------------------------------------------------------------------------

#[contracttype]
#[derive(Clone)]
pub struct AssetInfo {
    pub code: String,
    pub issuer: String,
    pub deposit_enabled: bool,
    pub withdrawal_enabled: bool,
    pub deposit_fee_fixed: u64,
    pub deposit_fee_percent: u32,
    pub withdrawal_fee_fixed: u64,
    pub withdrawal_fee_percent: u32,
    pub deposit_min_amount: u64,
    pub deposit_max_amount: u64,
    pub withdrawal_min_amount: u64,
    pub withdrawal_max_amount: u64,
}

#[contracttype]
#[derive(Clone)]
pub struct StellarToml {
    pub version: String,
    pub network_passphrase: String,
    pub accounts: Vec<String>,
    pub signing_key: String,
    pub currencies: Vec<AssetInfo>,
    pub transfer_server: String,
    pub transfer_server_sep0024: String,
    pub kyc_server: String,
    pub web_auth_endpoint: String,
}

#[contracttype]
#[derive(Clone)]
pub struct CachedToml {
    pub toml: StellarToml,
    pub cached_at: u64,
    pub ttl_seconds: u64,
}

const MIN_TEMP_TTL: u32 = 15; // min_temp_entry_ttl - 1

// ---------------------------------------------------------------------------
// Event structs
// ---------------------------------------------------------------------------

#[contracttype]
#[derive(Clone)]
struct SessionCreatedEvent {
    session_id: u64,
    initiator: Address,
    timestamp: u64,
}

#[contracttype]
#[derive(Clone)]
struct QuoteSubmitEvent {
    quote_id: u64,
    anchor: Address,
    base_asset: String,
    quote_asset: String,
    rate: u64,
    valid_until: u64,
}

#[contracttype]
#[derive(Clone)]
struct QuoteReceivedEvent {
    quote_id: u64,
    receiver: Address,
    timestamp: u64,
}

#[contracttype]
#[derive(Clone)]
struct AuditLogEvent {
    log_id: u64,
    session_id: u64,
    operation_index: u64,
    operation_type: String,
    status: String,
}

#[contracttype]
#[derive(Clone)]
struct AttestEvent {
    payload_hash: Bytes,
    timestamp: u64,
}

// ---------------------------------------------------------------------------
// TTLs (in ledgers)
// ---------------------------------------------------------------------------
const PERSISTENT_TTL: u32 = 1_555_200;
const SPAN_TTL: u32 = 17_280;
const INSTANCE_TTL: u32 = 518_400;

// ---------------------------------------------------------------------------
// Storage key helpers
// ---------------------------------------------------------------------------

fn admin_key(env: &Env) -> soroban_sdk::Vec<soroban_sdk::Symbol> {
    soroban_sdk::vec![env, symbol_short!("ADMIN")]
}

// ---------------------------------------------------------------------------
// Contract
// ---------------------------------------------------------------------------

#[contract]
pub struct AnchorKitContract;

#[contractimpl]
impl AnchorKitContract {
    // -----------------------------------------------------------------------
    // Initialization
    // -----------------------------------------------------------------------

    pub fn initialize(env: Env, admin: Address) {
        admin.require_auth();
        let inst = env.storage().instance();
        if inst.has(&admin_key(&env)) {
            panic_with_error!(&env, ErrorCode::AlreadyInitialized);
        }
        inst.set(&admin_key(&env), &admin);
        inst.extend_ttl(INSTANCE_TTL, INSTANCE_TTL);
    }

    pub fn get_admin(env: Env) -> Address {
        env.storage()
            .instance()
            .get::<_, Address>(&admin_key(&env))
            .unwrap_or_else(|| panic_with_error!(&env, ErrorCode::NotInitialized))
    }

    // -----------------------------------------------------------------------
    // Request ID generation
    // -----------------------------------------------------------------------

    /// Generate a deterministic request ID: sha256(timestamp_u64_be || sequence_number_u32_be)[:16]
    pub fn generate_request_id(env: Env) -> RequestId {
        let ts = env.ledger().timestamp();
        let seq = env.ledger().sequence() as u32;

        // Build input: 8-byte timestamp || 4-byte sequence number (big-endian)
        let mut input = Bytes::new(&env);
        for b in ts.to_be_bytes().iter() {
            input.push_back(*b);
        }
        for b in seq.to_be_bytes().iter() {
            input.push_back(*b);
        }

        let hash = env.crypto().sha256(&input);
        let mut id = Bytes::new(&env);
        for i in 0..16u32 {
            id.push_back(hash.get(i).unwrap());
        }

        RequestId { id, created_at: ts }
    }

    // -----------------------------------------------------------------------
    // Attestor management
    // -----------------------------------------------------------------------

    /// Stores the 32-byte Ed25519 public key used to verify SEP-10 JWTs for `issuer`
    /// (the anchor identity whose signing key appears in stellar.toml / SEP-10 flow).
    pub fn set_sep10_jwt_verifying_key(env: Env, issuer: Address, public_key: Bytes) {
        Self::require_admin(&env);
        if public_key.len() != 32 {
            panic_with_error!(&env, ErrorCode::ValidationError);
        }
        let storage_key = (symbol_short!("SEP10KEY"), issuer.clone());
        env.storage().persistent().set(&storage_key, &public_key);
        env.storage()
            .persistent()
            .extend_ttl(&storage_key, PERSISTENT_TTL, PERSISTENT_TTL);
    }

    /// Verifies a SEP-10 JWT (JWS compact, EdDSA) using the stored key for `issuer`: signature, `exp`, and `sub`.
    pub fn verify_sep10_token(env: Env, token: String, issuer: Address) {
        let pk: Bytes = env
            .storage()
            .persistent()
            .get(&(symbol_short!("SEP10KEY"), issuer.clone()))
            .unwrap_or_else(|| panic_with_error!(&env, ErrorCode::InvalidSep10Token));
        if sep10_jwt::verify_sep10_jwt(&env, &token, &pk, None).is_err() {
            panic_with_error!(&env, ErrorCode::InvalidSep10Token);
        }
    }

    fn verify_sep10_token_matches_attestor(
        env: &Env,
        token: &String,
        issuer: &Address,
        attestor: &Address,
    ) {
        let pk: Bytes = env
            .storage()
            .persistent()
            .get(&(symbol_short!("SEP10KEY"), issuer.clone()))
            .unwrap_or_else(|| panic_with_error!(env, ErrorCode::InvalidSep10Token));
        let expected = attestor.to_string();
        if sep10_jwt::verify_sep10_jwt(env, token, &pk, Some(&expected)).is_err() {
            panic_with_error!(env, ErrorCode::InvalidSep10Token);
        }
    }

    pub fn register_attestor(env: Env, attestor: Address, sep10_token: String, sep10_issuer: Address) {
        Self::require_admin(&env);
        Self::verify_sep10_token_matches_attestor(&env, &sep10_token, &sep10_issuer, &attestor);
        let key = (symbol_short!("ATTESTOR"), attestor.clone());
        if env.storage().persistent().has(&key) {
            panic_with_error!(&env, ErrorCode::AttestorAlreadyRegistered);
        }
        env.storage().persistent().set(&key, &true);
        env.storage()
            .persistent()
            .extend_ttl(&key, PERSISTENT_TTL, PERSISTENT_TTL);
        env.events().publish(
            (symbol_short!("attestor"), symbol_short!("added"), attestor),
            (),
        );
    }

    pub fn revoke_attestor(env: Env, attestor: Address) {
        Self::require_admin(&env);
        let key = (symbol_short!("ATTESTOR"), attestor.clone());
        if !env.storage().persistent().has(&key) {
            panic_with_error!(&env, ErrorCode::AttestorNotRegistered);
        }
        env.storage().persistent().remove(&key);
        env.events().publish(
            (symbol_short!("attestor"), symbol_short!("removed"), attestor),
            (),
        );
    }

    pub fn is_attestor(env: Env, attestor: Address) -> bool {
        env.storage()
            .persistent()
            .get::<_, bool>(&(symbol_short!("ATTESTOR"), attestor))
            .unwrap_or(false)
    }

    // -----------------------------------------------------------------------
    // Service configuration
    // -----------------------------------------------------------------------

    pub fn configure_services(env: Env, anchor: Address, services: Vec<u32>) {
        anchor.require_auth();
        if !env
            .storage()
            .persistent()
            .has(&(symbol_short!("ATTESTOR"), anchor.clone()))
        {
            panic_with_error!(&env, ErrorCode::AttestorNotRegistered);
        }
        if services.is_empty() {
            panic_with_error!(&env, ErrorCode::InvalidServiceType);
        }
        let mut seen = Vec::new(&env);
        for s in services.iter() {
            if seen.contains(&s) {
                panic_with_error!(&env, ErrorCode::InvalidServiceType);
            }
            seen.push_back(s);
        }
        let record = AnchorServices {
            anchor: anchor.clone(),
            services: services.clone(),
        };
        let key = (symbol_short!("SERVICES"), anchor.clone());
        env.storage().persistent().set(&key, &record);
        env.storage()
            .persistent()
            .extend_ttl(&key, PERSISTENT_TTL, PERSISTENT_TTL);
        env.events()
            .publish((symbol_short!("services"), symbol_short!("config")), record);
    }

    pub fn get_supported_services(env: Env, anchor: Address) -> AnchorServices {
        env.storage()
            .persistent()
            .get::<_, AnchorServices>(&(symbol_short!("SERVICES"), anchor))
            .unwrap_or_else(|| panic_with_error!(&env, ErrorCode::ServicesNotConfigured))
    }

    pub fn supports_service(env: Env, anchor: Address, service: u32) -> bool {
        let record = env
            .storage()
            .persistent()
            .get::<_, AnchorServices>(&(symbol_short!("SERVICES"), anchor))
            .unwrap_or_else(|| panic_with_error!(&env, ErrorCode::ServicesNotConfigured));
        record.services.contains(&service)
    }

    // -----------------------------------------------------------------------
    // Attestation submission (plain)
    // -----------------------------------------------------------------------

    pub fn submit_attestation(
        env: Env,
        issuer: Address,
        subject: Address,
        timestamp: u64,
        payload_hash: Bytes,
        signature: Bytes,
    ) -> u64 {
        issuer.require_auth();
        Self::check_attestor(&env, &issuer);
        Self::check_timestamp(&env, timestamp);

        let used_key = (symbol_short!("USED"), payload_hash.clone());
        if env.storage().persistent().has(&used_key) {
            panic_with_error!(&env, ErrorCode::ReplayAttack);
        }

        let id = Self::next_attestation_id(&env);
        Self::store_attestation(
            &env,
            id,
            issuer.clone(),
            subject.clone(),
            timestamp,
            payload_hash.clone(),
            signature,
        );

        env.storage().persistent().set(&used_key, &true);
        env.storage()
            .persistent()
            .extend_ttl(&used_key, PERSISTENT_TTL, PERSISTENT_TTL);

        env.events().publish(
            (
                symbol_short!("attest"),
                symbol_short!("recorded"),
                id,
                subject,
            ),
            AttestEvent { payload_hash, timestamp },
        );

        id
    }

    // -----------------------------------------------------------------------
    // Attestation submission with request ID + tracing span
    // -----------------------------------------------------------------------

    pub fn submit_with_request_id(
        env: Env,
        request_id: RequestId,
        issuer: Address,
        subject: Address,
        timestamp: u64,
        payload_hash: Bytes,
        signature: Bytes,
    ) -> u64 {
        issuer.require_auth();
        Self::check_attestor(&env, &issuer);
        Self::check_timestamp(&env, timestamp);

        let used_key = (symbol_short!("USED"), payload_hash.clone());
        if env.storage().persistent().has(&used_key) {
            panic_with_error!(&env, ErrorCode::ReplayAttack);
        }

        let id = Self::next_attestation_id(&env);
        Self::store_attestation(
            &env,
            id,
            issuer.clone(),
            subject.clone(),
            timestamp,
            payload_hash.clone(),
            signature,
        );

        env.storage().persistent().set(&used_key, &true);
        env.storage()
            .persistent()
            .extend_ttl(&used_key, PERSISTENT_TTL, PERSISTENT_TTL);

        let now = env.ledger().timestamp();
        Self::store_span(
            &env,
            &request_id,
            String::from_str(&env, "submit_attestation"),
            issuer.clone(),
            now,
            String::from_str(&env, "success"),
        );

        env.events().publish(
            (
                symbol_short!("attest"),
                symbol_short!("recorded"),
                id,
                subject,
            ),
            AttestEvent { payload_hash, timestamp },
        );

        id
    }

    // -----------------------------------------------------------------------
    // Quote submission with request ID + tracing span
    // -----------------------------------------------------------------------

    #[allow(unused_variables)]
    pub fn quote_with_request_id(
        env: Env,
        request_id: RequestId,
        anchor: Address,
        from_asset: String,
        to_asset: String,
        amount: u64,
        fee_bps: u32,
        min_amount: u64,
        max_amount: u64,
        expires_at: u64,
    ) {
        anchor.require_auth();

        let services_record = env
            .storage()
            .persistent()
            .get::<_, AnchorServices>(&(symbol_short!("SERVICES"), anchor.clone()))
            .unwrap_or_else(|| panic_with_error!(&env, ErrorCode::ServicesNotConfigured));
        if !services_record.services.contains(&SERVICE_QUOTES) {
            panic_with_error!(&env, ErrorCode::ServicesNotConfigured);
        }

        let now = env.ledger().timestamp();
        Self::store_span(
            &env,
            &request_id,
            String::from_str(&env, "submit_quote"),
            anchor,
            now,
            String::from_str(&env, "success"),
        );
    }

    // -----------------------------------------------------------------------
    // Tracing span retrieval
    // -----------------------------------------------------------------------

    pub fn get_tracing_span(env: Env, request_id_bytes: Bytes) -> Option<TracingSpan> {
        env.storage()
            .temporary()
            .get::<_, TracingSpan>(&(symbol_short!("SPAN"), request_id_bytes))
    }

    // -----------------------------------------------------------------------
    // Attestation retrieval
    // -----------------------------------------------------------------------

    pub fn get_attestation(env: Env, id: u64) -> Attestation {
        env.storage()
            .persistent()
            .get::<_, Attestation>(&(symbol_short!("ATTEST"), id))
            .unwrap_or_else(|| panic_with_error!(&env, ErrorCode::AttestationNotFound))
    }

    // -----------------------------------------------------------------------
    // Session management
    // -----------------------------------------------------------------------

    pub fn create_session(env: Env, initiator: Address) -> u64 {
        initiator.require_auth();
        let inst = env.storage().instance();
        let scnt_key = soroban_sdk::vec![&env, symbol_short!("SCNT")];
        let session_id: u64 = inst.get(&scnt_key).unwrap_or(0u64);
        inst.set(&scnt_key, &(session_id + 1));
        inst.extend_ttl(INSTANCE_TTL, INSTANCE_TTL);

        let now = env.ledger().timestamp();
        let session = Session {
            session_id,
            initiator: initiator.clone(),
            created_at: now,
            nonce: 0,
            operation_count: 0,
        };
        let sess_key = (symbol_short!("SESS"), session_id);
        env.storage().persistent().set(&sess_key, &session);
        env.storage().persistent().extend_ttl(&sess_key, PERSISTENT_TTL, PERSISTENT_TTL);

        let snonce_key = (symbol_short!("SNONCE"), session_id);
        env.storage().persistent().set(&snonce_key, &0u64);
        env.storage().persistent().extend_ttl(&snonce_key, PERSISTENT_TTL, PERSISTENT_TTL);

        env.events().publish(
            (symbol_short!("session"), symbol_short!("created"), session_id),
            SessionCreatedEvent { session_id, initiator, timestamp: now },
        );

        session_id
    }

    // -----------------------------------------------------------------------
    // Quote management
    // -----------------------------------------------------------------------

    pub fn submit_quote(
        env: Env,
        anchor: Address,
        base_asset: String,
        quote_asset: String,
        rate: u64,
        fee_percentage: u32,
        minimum_amount: u64,
        maximum_amount: u64,
        valid_until: u64,
    ) -> u64 {
        anchor.require_auth();
        let inst = env.storage().instance();
        let qcnt_key = soroban_sdk::vec![&env, symbol_short!("QCNT")];
        let next: u64 = inst.get(&qcnt_key).unwrap_or(0u64) + 1;
        inst.set(&qcnt_key, &next);
        inst.extend_ttl(INSTANCE_TTL, INSTANCE_TTL);

        let quote = Quote {
            quote_id: next,
            anchor: anchor.clone(),
            base_asset: base_asset.clone(),
            quote_asset: quote_asset.clone(),
            rate,
            fee_percentage,
            minimum_amount,
            maximum_amount,
            valid_until,
        };
        let q_key = (symbol_short!("QUOTE"), anchor.clone(), next);
        env.storage().persistent().set(&q_key, &quote);
        env.storage().persistent().extend_ttl(&q_key, PERSISTENT_TTL, PERSISTENT_TTL);

        let lq_key = (symbol_short!("LATESTQ"), anchor.clone());
        env.storage().persistent().set(&lq_key, &next);
        env.storage().persistent().extend_ttl(&lq_key, PERSISTENT_TTL, PERSISTENT_TTL);

        env.events().publish(
            (symbol_short!("quote"), symbol_short!("submit"), next),
            QuoteSubmitEvent {
                quote_id: next,
                anchor,
                base_asset,
                quote_asset,
                rate,
                valid_until,
            },
        );

        next
    }

    pub fn receive_quote(env: Env, receiver: Address, anchor: Address, quote_id: u64) -> Quote {
        receiver.require_auth();
        let q_key = (symbol_short!("QUOTE"), anchor.clone(), quote_id);
        let quote: Quote = env.storage().persistent().get(&q_key)
            .unwrap_or_else(|| panic_with_error!(&env, ErrorCode::AttestationNotFound));

        env.events().publish(
            (symbol_short!("quote"), symbol_short!("received"), quote_id),
            QuoteReceivedEvent {
                quote_id,
                receiver,
                timestamp: env.ledger().timestamp(),
            },
        );

        quote
    }

    // -----------------------------------------------------------------------
    // Session-aware attestation
    // -----------------------------------------------------------------------

    pub fn submit_attestation_with_session(
        env: Env,
        session_id: u64,
        issuer: Address,
        subject: Address,
        timestamp: u64,
        payload_hash: Bytes,
        signature: Bytes,
    ) -> u64 {
        issuer.require_auth();
        Self::check_attestor(&env, &issuer);
        Self::check_timestamp(&env, timestamp);

        let used_key = (symbol_short!("USED"), payload_hash.clone());
        if env.storage().persistent().has(&used_key) {
            panic_with_error!(&env, ErrorCode::ReplayAttack);
        }

        let id = Self::next_attestation_id(&env);
        Self::store_attestation(
            &env,
            id,
            issuer.clone(),
            subject.clone(),
            timestamp,
            payload_hash.clone(),
            signature,
        );

        env.storage().persistent().set(&used_key, &true);
        env.storage().persistent().extend_ttl(&used_key, PERSISTENT_TTL, PERSISTENT_TTL);

        // Get and increment session operation count
        let sopcnt_key = (symbol_short!("SOPCNT"), session_id);
        let op_index: u64 = env.storage().persistent().get(&sopcnt_key).unwrap_or(0u64);
        env.storage().persistent().set(&sopcnt_key, &(op_index + 1));
        env.storage().persistent().extend_ttl(&sopcnt_key, PERSISTENT_TTL, PERSISTENT_TTL);

        // Write audit log
        let inst = env.storage().instance();
        let acnt_key = soroban_sdk::vec![&env, symbol_short!("ACNT")];
        let log_id: u64 = inst.get(&acnt_key).unwrap_or(0u64);
        inst.set(&acnt_key, &(log_id + 1));
        inst.extend_ttl(INSTANCE_TTL, INSTANCE_TTL);

        let now = env.ledger().timestamp();
        let audit = AuditLog {
            log_id,
            session_id,
            actor: issuer.clone(),
            operation: OperationContext {
                session_id,
                operation_index: op_index,
                operation_type: String::from_str(&env, "attest"),
                timestamp: now,
                status: String::from_str(&env, "success"),
                result_data: id,
            },
        };
        let audit_key = (symbol_short!("AUDIT"), log_id);
        env.storage().persistent().set(&audit_key, &audit);
        env.storage().persistent().extend_ttl(&audit_key, PERSISTENT_TTL, PERSISTENT_TTL);

        env.events().publish(
            (
                symbol_short!("attest"),
                symbol_short!("recorded"),
                id,
                subject,
            ),
            AttestEvent { payload_hash, timestamp },
        );

        env.events().publish(
            (symbol_short!("audit"), symbol_short!("logged"), log_id),
            AuditLogEvent {
                log_id,
                session_id,
                operation_index: op_index,
                operation_type: String::from_str(&env, "attest"),
                status: String::from_str(&env, "success"),
            },
        );

        id
    }

    pub fn register_attestor_with_session(env: Env, session_id: u64, attestor: Address) {
        Self::require_admin(&env);
        let key = (symbol_short!("ATTESTOR"), attestor.clone());
        if env.storage().persistent().has(&key) {
            panic_with_error!(&env, ErrorCode::AttestorAlreadyRegistered);
        }
        env.storage().persistent().set(&key, &true);
        env.storage().persistent().extend_ttl(&key, PERSISTENT_TTL, PERSISTENT_TTL);

        let sopcnt_key = (symbol_short!("SOPCNT"), session_id);
        let op_index: u64 = env.storage().persistent().get(&sopcnt_key).unwrap_or(0u64);
        env.storage().persistent().set(&sopcnt_key, &(op_index + 1));
        env.storage().persistent().extend_ttl(&sopcnt_key, PERSISTENT_TTL, PERSISTENT_TTL);

        let inst = env.storage().instance();
        let acnt_key = soroban_sdk::vec![&env, symbol_short!("ACNT")];
        let log_id: u64 = inst.get(&acnt_key).unwrap_or(0u64);
        inst.set(&acnt_key, &(log_id + 1));
        inst.extend_ttl(INSTANCE_TTL, INSTANCE_TTL);

        let admin: Address = inst
            .get::<_, Address>(&admin_key(&env))
            .unwrap_or_else(|| panic_with_error!(&env, ErrorCode::NotInitialized));
        let now = env.ledger().timestamp();
        let audit = AuditLog {
            log_id,
            session_id,
            actor: admin,
            operation: OperationContext {
                session_id,
                operation_index: op_index,
                operation_type: String::from_str(&env, "register"),
                timestamp: now,
                status: String::from_str(&env, "success"),
                result_data: 0,
            },
        };
        let audit_key = (symbol_short!("AUDIT"), log_id);
        env.storage().persistent().set(&audit_key, &audit);
        env.storage().persistent().extend_ttl(&audit_key, PERSISTENT_TTL, PERSISTENT_TTL);

        env.events().publish(
            (symbol_short!("attestor"), symbol_short!("added"), attestor),
            (),
        );
        env.events().publish(
            (symbol_short!("audit"), symbol_short!("logged"), log_id),
            AuditLogEvent {
                log_id,
                session_id,
                operation_index: op_index,
                operation_type: String::from_str(&env, "register"),
                status: String::from_str(&env, "success"),
            },
        );
    }

    pub fn revoke_attestor_with_session(env: Env, session_id: u64, attestor: Address) {
        Self::require_admin(&env);
        let key = (symbol_short!("ATTESTOR"), attestor.clone());
        if !env.storage().persistent().has(&key) {
            panic_with_error!(&env, ErrorCode::AttestorNotRegistered);
        }
        env.storage().persistent().remove(&key);

        let sopcnt_key = (symbol_short!("SOPCNT"), session_id);
        let op_index: u64 = env.storage().persistent().get(&sopcnt_key).unwrap_or(0u64);
        env.storage().persistent().set(&sopcnt_key, &(op_index + 1));
        env.storage().persistent().extend_ttl(&sopcnt_key, PERSISTENT_TTL, PERSISTENT_TTL);

        let inst = env.storage().instance();
        let acnt_key = soroban_sdk::vec![&env, symbol_short!("ACNT")];
        let log_id: u64 = inst.get(&acnt_key).unwrap_or(0u64);
        inst.set(&acnt_key, &(log_id + 1));
        inst.extend_ttl(INSTANCE_TTL, INSTANCE_TTL);

        let admin: Address = inst
            .get::<_, Address>(&admin_key(&env))
            .unwrap_or_else(|| panic_with_error!(&env, ErrorCode::NotInitialized));
        let now = env.ledger().timestamp();
        let audit = AuditLog {
            log_id,
            session_id,
            actor: admin,
            operation: OperationContext {
                session_id,
                operation_index: op_index,
                operation_type: String::from_str(&env, "revoke"),
                timestamp: now,
                status: String::from_str(&env, "success"),
                result_data: 0,
            },
        };
        let audit_key = (symbol_short!("AUDIT"), log_id);
        env.storage().persistent().set(&audit_key, &audit);
        env.storage().persistent().extend_ttl(&audit_key, PERSISTENT_TTL, PERSISTENT_TTL);

        env.events().publish(
            (symbol_short!("attestor"), symbol_short!("removed"), attestor),
            (),
        );
        env.events().publish(
            (symbol_short!("audit"), symbol_short!("logged"), log_id),
            AuditLogEvent {
                log_id,
                session_id,
                operation_index: op_index,
                operation_type: String::from_str(&env, "revoke"),
                status: String::from_str(&env, "success"),
            },
        );
    }

    pub fn get_session(env: Env, session_id: u64) -> Session {
        env.storage()
            .persistent()
            .get::<_, Session>(&(symbol_short!("SESS"), session_id))
            .unwrap_or_else(|| panic_with_error!(&env, ErrorCode::AttestationNotFound))
    }

    pub fn get_audit_log(env: Env, log_id: u64) -> AuditLog {
        env.storage()
            .persistent()
            .get::<_, AuditLog>(&(symbol_short!("AUDIT"), log_id))
            .unwrap_or_else(|| panic_with_error!(&env, ErrorCode::AttestationNotFound))
    }

    pub fn get_session_operation_count(env: Env, session_id: u64) -> u64 {
        env.storage()
            .persistent()
            .get::<_, u64>(&(symbol_short!("SOPCNT"), session_id))
            .unwrap_or(0)
    }

    // -----------------------------------------------------------------------
    // Metadata cache
    // -----------------------------------------------------------------------

    pub fn cache_metadata(env: Env, anchor: Address, metadata: AnchorMetadata, ttl_seconds: u64) {
        Self::require_admin(&env);
        let now = env.ledger().timestamp();
        let entry = MetadataCache { metadata, cached_at: now, ttl_seconds };
        let key = (symbol_short!("METACACHE"), anchor);
        let ledger_ttl = if ttl_seconds as u32 > MIN_TEMP_TTL { ttl_seconds as u32 } else { MIN_TEMP_TTL };
        env.storage().temporary().set(&key, &entry);
        env.storage().temporary().extend_ttl(&key, ledger_ttl, ledger_ttl);
    }

    pub fn get_cached_metadata(env: Env, anchor: Address) -> AnchorMetadata {
        let key = (symbol_short!("METACACHE"), anchor);
        let entry: MetadataCache = env.storage().temporary().get(&key)
            .unwrap_or_else(|| panic_with_error!(&env, ErrorCode::CacheNotFound));
        let now = env.ledger().timestamp();
        if entry.cached_at + entry.ttl_seconds <= now {
            panic_with_error!(&env, ErrorCode::CacheExpired);
        }
        entry.metadata
    }

    pub fn refresh_metadata_cache(env: Env, anchor: Address) {
        Self::require_admin(&env);
        let key = (symbol_short!("METACACHE"), anchor);
        env.storage().temporary().remove(&key);
    }

    // -----------------------------------------------------------------------
    // Capabilities cache
    // -----------------------------------------------------------------------

    pub fn cache_capabilities(env: Env, anchor: Address, toml_url: String, capabilities: String, ttl_seconds: u64) {
        Self::require_admin(&env);
        let now = env.ledger().timestamp();
        let entry = CapabilitiesCache { toml_url, capabilities, cached_at: now, ttl_seconds };
        let key = (symbol_short!("CAPCACHE"), anchor);
        let ledger_ttl = if ttl_seconds as u32 > MIN_TEMP_TTL { ttl_seconds as u32 } else { MIN_TEMP_TTL };
        env.storage().temporary().set(&key, &entry);
        env.storage().temporary().extend_ttl(&key, ledger_ttl, ledger_ttl);
    }

    pub fn get_cached_capabilities(env: Env, anchor: Address) -> CapabilitiesCache {
        let key = (symbol_short!("CAPCACHE"), anchor);
        let entry: CapabilitiesCache = env.storage().temporary().get(&key)
            .unwrap_or_else(|| panic_with_error!(&env, ErrorCode::CacheNotFound));
        let now = env.ledger().timestamp();
        if entry.cached_at + entry.ttl_seconds <= now {
            panic_with_error!(&env, ErrorCode::CacheExpired);
        }
        entry
    }

    pub fn refresh_capabilities_cache(env: Env, anchor: Address) {
        Self::require_admin(&env);
        let key = (symbol_short!("CAPCACHE"), anchor);
        env.storage().temporary().remove(&key);
    }

    // -----------------------------------------------------------------------
    // Routing
    // -----------------------------------------------------------------------

    pub fn get_quote(env: Env, anchor: Address, quote_id: u64) -> Quote {
        let key = (symbol_short!("QUOTE"), anchor.clone(), quote_id);
        env.storage().persistent().get::<_, Quote>(&key)
            .unwrap_or_else(|| panic_with_error!(&env, ErrorCode::NoQuotesAvailable))
    }

    pub fn set_anchor_metadata(
        env: Env,
        anchor: Address,
        reputation_score: u32,
        average_settlement_time: u64,
        liquidity_score: u32,
        uptime_percentage: u32,
        total_volume: u64,
    ) {
        Self::require_admin(&env);
        let meta = RoutingAnchorMeta {
            anchor: anchor.clone(),
            reputation_score,
            average_settlement_time,
            liquidity_score,
            uptime_percentage,
            total_volume,
            is_active: true,
        };
        let meta_key = (symbol_short!("ANCHMETA"), anchor.clone());
        env.storage().persistent().set(&meta_key, &meta);
        env.storage().persistent().extend_ttl(&meta_key, PERSISTENT_TTL, PERSISTENT_TTL);

        // Maintain ANCHLIST
        let list_key = soroban_sdk::vec![&env, symbol_short!("ANCHLIST")];
        let mut list: Vec<Address> = env.storage().persistent()
            .get::<_, Vec<Address>>(&list_key)
            .unwrap_or_else(|| Vec::new(&env));
        if !list.contains(&anchor) {
            list.push_back(anchor);
            env.storage().persistent().set(&list_key, &list);
            env.storage().persistent().extend_ttl(&list_key, PERSISTENT_TTL, PERSISTENT_TTL);
        }
    }

    pub fn route_transaction(env: Env, options: RoutingOptions) -> Quote {
        let now = env.ledger().timestamp();
        let list_key = soroban_sdk::vec![&env, symbol_short!("ANCHLIST")];
        let anchors: Vec<Address> = env.storage().persistent()
            .get::<_, Vec<Address>>(&list_key)
            .unwrap_or_else(|| Vec::new(&env));

        // Collect valid quotes from active anchors
        let mut candidates: Vec<Quote> = Vec::new(&env);
        for anchor in anchors.iter() {
            // Check reputation filter
            let meta_key = (symbol_short!("ANCHMETA"), anchor.clone());
            let meta: RoutingAnchorMeta = match env.storage().persistent().get(&meta_key) {
                Some(m) => m,
                None => continue,
            };
            if !meta.is_active { continue; }
            if meta.reputation_score < options.min_reputation { continue; }

            // Get latest quote for this anchor
            let lq_key = (symbol_short!("LATESTQ"), anchor.clone());
            let quote_id: u64 = match env.storage().persistent().get(&lq_key) {
                Some(id) => id,
                None => continue,
            };
            let q_key = (symbol_short!("QUOTE"), anchor.clone(), quote_id);
            let quote: Quote = match env.storage().persistent().get(&q_key) {
                Some(q) => q,
                None => continue,
            };

            // Filter expired quotes
            if quote.valid_until <= now { continue; }

            // Filter by amount limits
            if options.request.amount < quote.minimum_amount
                || options.request.amount > quote.maximum_amount
            {
                continue;
            }

            candidates.push_back(quote);
        }

        if candidates.is_empty() {
            panic_with_error!(&env, ErrorCode::NoQuotesAvailable);
        }

        // Apply strategy: pick best candidate
        let strategy_sym = options.strategy.get(0)
            .unwrap_or_else(|| panic_with_error!(&env, ErrorCode::NoQuotesAvailable));

        let lowest_fee_sym = Symbol::new(&env, "LowestFee");
        let fastest_sym = Symbol::new(&env, "FastestSettlement");
        let reputation_sym = Symbol::new(&env, "HighestReputation");

        let mut best: Quote = candidates.get(0).unwrap();

        if strategy_sym == lowest_fee_sym {
            for q in candidates.iter() {
                if q.fee_percentage < best.fee_percentage {
                    best = q;
                }
            }
        } else if strategy_sym == fastest_sym {
            // Need settlement time from metadata
            let meta_key = (symbol_short!("ANCHMETA"), best.anchor.clone());
            let mut best_time: u64 = env.storage().persistent()
                .get::<_, RoutingAnchorMeta>(&meta_key)
                .map(|m| m.average_settlement_time)
                .unwrap_or(u64::MAX);
            for q in candidates.iter() {
                let mk = (symbol_short!("ANCHMETA"), q.anchor.clone());
                let t = env.storage().persistent()
                    .get::<_, RoutingAnchorMeta>(&mk)
                    .map(|m| m.average_settlement_time)
                    .unwrap_or(u64::MAX);
                if t < best_time {
                    best_time = t;
                    best = q;
                }
            }
        } else if strategy_sym == reputation_sym {
            let meta_key = (symbol_short!("ANCHMETA"), best.anchor.clone());
            let mut best_rep: u32 = env.storage().persistent()
                .get::<_, RoutingAnchorMeta>(&meta_key)
                .map(|m| m.reputation_score)
                .unwrap_or(0);
            for q in candidates.iter() {
                let mk = (symbol_short!("ANCHMETA"), q.anchor.clone());
                let rep = env.storage().persistent()
                    .get::<_, RoutingAnchorMeta>(&mk)
                    .map(|m| m.reputation_score)
                    .unwrap_or(0);
                if rep > best_rep {
                    best_rep = rep;
                    best = q;
                }
            }
        }

        best
    }

    // -----------------------------------------------------------------------
    // Anchor Info Discovery
    // -----------------------------------------------------------------------

    pub fn fetch_anchor_info(
        env: Env,
        anchor: Address,
        toml_data: StellarToml,
        ttl_seconds: u64,
    ) {
        anchor.require_auth();
        let now = env.ledger().timestamp();
        let cached = CachedToml {
            toml: toml_data,
            cached_at: now,
            ttl_seconds,
        };
        let key = (symbol_short!("TOMLCACHE"), anchor);
        let ledger_ttl = if ttl_seconds as u32 > MIN_TEMP_TTL { ttl_seconds as u32 } else { MIN_TEMP_TTL };
        env.storage().temporary().set(&key, &cached);
        env.storage().temporary().extend_ttl(&key, ledger_ttl, ledger_ttl);
    }

    pub fn get_anchor_toml(env: Env, anchor: Address) -> StellarToml {
        let key = (symbol_short!("TOMLCACHE"), anchor);
        let cached: CachedToml = env.storage().temporary().get(&key)
            .unwrap_or_else(|| panic_with_error!(&env, ErrorCode::CacheNotFound));
        let now = env.ledger().timestamp();
        if cached.cached_at + cached.ttl_seconds <= now {
            panic_with_error!(&env, ErrorCode::CacheExpired);
        }
        cached.toml
    }

    pub fn refresh_anchor_info(env: Env, anchor: Address) {
        anchor.require_auth();
        let key = (symbol_short!("TOMLCACHE"), anchor);
        env.storage().temporary().remove(&key);
    }

    pub fn get_anchor_assets(env: Env, anchor: Address) -> Vec<String> {
        let toml = Self::get_anchor_toml(env.clone(), anchor);
        let mut assets = Vec::new(&env);
        for asset in toml.currencies.iter() {
            assets.push_back(asset.code.clone());
        }
        assets
    }

    pub fn get_anchor_asset_info(
        env: Env,
        anchor: Address,
        asset_code: String,
    ) -> AssetInfo {
        let toml = Self::get_anchor_toml(env.clone(), anchor);
        for asset in toml.currencies.iter() {
            if asset.code == asset_code {
                return asset;
            }
        }
        panic_with_error!(&env, ErrorCode::ValidationError);
    }

    pub fn get_anchor_deposit_limits(
        env: Env,
        anchor: Address,
        asset_code: String,
    ) -> (u64, u64) {
        let asset = Self::get_anchor_asset_info(env, anchor, asset_code);
        (asset.deposit_min_amount, asset.deposit_max_amount)
    }

    pub fn get_anchor_withdrawal_limits(
        env: Env,
        anchor: Address,
        asset_code: String,
    ) -> (u64, u64) {
        let asset = Self::get_anchor_asset_info(env, anchor, asset_code);
        (asset.withdrawal_min_amount, asset.withdrawal_max_amount)
    }

    pub fn get_anchor_deposit_fees(
        env: Env,
        anchor: Address,
        asset_code: String,
    ) -> (u64, u32) {
        let asset = Self::get_anchor_asset_info(env, anchor, asset_code);
        (asset.deposit_fee_fixed, asset.deposit_fee_percent)
    }

    pub fn get_anchor_withdrawal_fees(
        env: Env,
        anchor: Address,
        asset_code: String,
    ) -> (u64, u32) {
        let asset = Self::get_anchor_asset_info(env, anchor, asset_code);
        (asset.withdrawal_fee_fixed, asset.withdrawal_fee_percent)
    }

    pub fn anchor_supports_deposits(
        env: Env,
        anchor: Address,
        asset_code: String,
    ) -> bool {
        match Self::get_anchor_asset_info(env, anchor, asset_code) {
            asset => asset.deposit_enabled,
        }
    }

    pub fn anchor_supports_withdrawals(
        env: Env,
        anchor: Address,
        asset_code: String,
    ) -> bool {
        match Self::get_anchor_asset_info(env, anchor, asset_code) {
            asset => asset.withdrawal_enabled,
        }
    }

    // -----------------------------------------------------------------------
    // Internal helpers
    // -----------------------------------------------------------------------

    fn require_admin(env: &Env) {
        let admin: Address = env
            .storage()
            .instance()
            .get::<_, Address>(&admin_key(env))
            .unwrap_or_else(|| panic_with_error!(env, ErrorCode::NotInitialized));
        admin.require_auth();
    }

    fn check_attestor(env: &Env, attestor: &Address) {
        if !env
            .storage()
            .persistent()
            .has(&(symbol_short!("ATTESTOR"), attestor.clone()))
        {
            panic_with_error!(env, ErrorCode::AttestorNotRegistered);
        }
    }

    fn check_timestamp(env: &Env, timestamp: u64) {
        if timestamp == 0 {
            panic_with_error!(env, ErrorCode::InvalidTimestamp);
        }
    }

    fn next_attestation_id(env: &Env) -> u64 {
        let inst = env.storage().instance();
        let ck = soroban_sdk::vec![env, symbol_short!("COUNTER")];
        let id: u64 = inst.get(&ck).unwrap_or(0u64);
        inst.set(&ck, &(id + 1));
        inst.extend_ttl(INSTANCE_TTL, INSTANCE_TTL);
        id
    }

    fn store_attestation(
        env: &Env,
        id: u64,
        issuer: Address,
        subject: Address,
        timestamp: u64,
        payload_hash: Bytes,
        signature: Bytes,
    ) {
        let attestation = Attestation {
            id,
            issuer,
            subject,
            timestamp,
            payload_hash,
            signature,
        };
        let key = (symbol_short!("ATTEST"), id);
        env.storage().persistent().set(&key, &attestation);
        env.storage()
            .persistent()
            .extend_ttl(&key, PERSISTENT_TTL, PERSISTENT_TTL);
    }

    fn store_span(
        env: &Env,
        request_id: &RequestId,
        operation: String,
        actor: Address,
        now: u64,
        status: String,
    ) {
        let span = TracingSpan {
            request_id: request_id.clone(),
            operation,
            actor,
            started_at: now,
            completed_at: now,
            status,
        };
        let key = (symbol_short!("SPAN"), request_id.id.clone());
        env.storage().temporary().set(&key, &span);
        env.storage()
            .temporary()
            .extend_ttl(&key, SPAN_TTL, SPAN_TTL);
    }
}
