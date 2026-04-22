use soroban_sdk::{
    contract, contractimpl, panic_with_error, symbol_short, Address, Bytes, BytesN,
    Env, String, Symbol, Vec,
};

use crate::deterministic_hash::{compute_payload_hash, verify_payload_hash};
use crate::errors::ErrorCode;
use crate::sep10_jwt;
use crate::types::{
    AnchorMetadata, AnchorServices, Attestation, AuditLog, CapabilitiesCache, CachedToml,
    HealthStatus, MetadataCache, OperationContext, Quote, RequestId, RoutingAnchorMeta,
    RoutingOptions, Session, StellarToml, TracingSpan, AssetInfo,
    SERVICE_QUOTES,
};
use crate::events::{
    AnchorDeactivated, AttestEvent, AuditLogEvent, EndpointUpdated, QuoteReceivedEvent,
    QuoteSubmitEvent, SessionCreatedEvent,
};

// ---------------------------------------------------------------------------
// TTLs (in ledgers)
// ---------------------------------------------------------------------------
const PERSISTENT_TTL: u32 = 1_555_200;
const SPAN_TTL: u32 = 17_280;
const INSTANCE_TTL: u32 = 518_400;
const MIN_TEMP_TTL: u32 = 15;

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
        if admin == env.current_contract_address() {
            panic_with_error!(&env, ErrorCode::ValidationError);
        }
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

    pub fn generate_request_id(env: Env) -> RequestId {
        let ts = env.ledger().timestamp();
        let seq = env.ledger().sequence() as u32;

        let mut input = Bytes::new(&env);
        for b in ts.to_be_bytes().iter() {
            input.push_back(*b);
        }
        for b in seq.to_be_bytes().iter() {
            input.push_back(*b);
        }

        let hash = env.crypto().sha256(&input);
        let hash_bytes = Bytes::from_array(&env, &hash.into());
        let mut id = Bytes::new(&env);
        for i in 0..16u32 {
            id.push_back(hash_bytes.get(i).unwrap());
        }

        RequestId { id, created_at: ts }
    }

    // -----------------------------------------------------------------------
    // Attestor management
    // -----------------------------------------------------------------------

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
    // Attestor endpoint management
    // -----------------------------------------------------------------------

    pub fn set_endpoint(env: Env, attestor: Address, endpoint: String) {
        attestor.require_auth();
        Self::check_attestor(&env, &attestor);

        let len = endpoint.len() as usize;
        let mut rust_buf = [0u8; 128];
        if len > 128 {
            panic_with_error!(&env, ErrorCode::InvalidEndpointFormat);
        }
        endpoint.copy_into_slice(&mut rust_buf[..len]);
        let endpoint_str = core::str::from_utf8(&rust_buf[..len]).unwrap_or("");

        if crate::validate_anchor_domain(endpoint_str).is_err() {
            panic_with_error!(&env, ErrorCode::InvalidEndpointFormat);
        }

        let key = (symbol_short!("ENDPOINT"), attestor.clone());
        env.storage().persistent().set(&key, &endpoint);
        env.storage().persistent().extend_ttl(&key, PERSISTENT_TTL, PERSISTENT_TTL);
        env.events().publish(
            (symbol_short!("endpoint"), symbol_short!("updated")),
            EndpointUpdated { attestor, endpoint },
        );
    }

    pub fn get_endpoint(env: Env, attestor: Address) -> String {
        if !Self::is_attestor(env.clone(), attestor.clone()) {
            panic_with_error!(&env, ErrorCode::AttestorNotRegistered);
        }
        env.storage().persistent()
            .get::<_, String>(&(symbol_short!("ENDPOINT"), attestor))
            .unwrap_or_else(|| panic_with_error!(&env, ErrorCode::AttestorNotRegistered))
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
        Self::store_attestation(&env, id, issuer.clone(), subject.clone(), timestamp, payload_hash.clone(), signature);

        env.storage().persistent().set(&used_key, &true);
        env.storage().persistent().extend_ttl(&used_key, PERSISTENT_TTL, PERSISTENT_TTL);

        env.events().publish(
            (symbol_short!("attest"), symbol_short!("recorded"), id, subject),
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
        Self::store_attestation(&env, id, issuer.clone(), subject.clone(), timestamp, payload_hash.clone(), signature);

        env.storage().persistent().set(&used_key, &true);
        env.storage().persistent().extend_ttl(&used_key, PERSISTENT_TTL, PERSISTENT_TTL);

        let now = env.ledger().timestamp();
        Self::store_span(&env, &request_id, String::from_str(&env, "submit_attestation"), issuer.clone(), now, String::from_str(&env, "success"));

        env.events().publish(
            (symbol_short!("attest"), symbol_short!("recorded"), id, subject),
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
        Self::store_span(&env, &request_id, String::from_str(&env, "submit_quote"), anchor, now, String::from_str(&env, "success"));
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

    pub fn list_attestations(env: Env, subject: Address, offset: u64, limit: u32) -> Vec<Attestation> {
        let actual_limit = if limit > 50 { 50 } else { limit };
        let mut results = Vec::new(&env);

        let count_key = (symbol_short!("SUBCNT"), subject.clone());
        let total_count: u64 = env.storage().persistent().get(&count_key).unwrap_or(0);

        if offset >= total_count || actual_limit == 0 {
            return results;
        }

        let end = if offset + (actual_limit as u64) > total_count {
            total_count
        } else {
            offset + (actual_limit as u64)
        };

        for i in offset..end {
            let index_key = (symbol_short!("SUBATT"), subject.clone(), i);
            if let Some(attestation_id) = env.storage().persistent().get::<_, u64>(&index_key) {
                let main_key = (symbol_short!("ATTEST"), attestation_id);
                if let Some(attestation) = env.storage().persistent().get::<_, Attestation>(&main_key) {
                    results.push_back(attestation);
                }
            }
        }

        results
    }

    // -----------------------------------------------------------------------
    // Deterministic hash utilities
    // -----------------------------------------------------------------------

    pub fn compute_payload_hash(env: Env, subject: Address, timestamp: u64, data: Bytes) -> BytesN<32> {
        compute_payload_hash(&env, &subject, timestamp, &data)
    }

    pub fn verify_payload_hash(env: Env, attestation_id: u64, expected_hash: BytesN<32>) -> bool {
        let attestation = env
            .storage()
            .persistent()
            .get::<_, Attestation>(&(symbol_short!("ATTEST"), attestation_id))
            .unwrap_or_else(|| panic_with_error!(&env, ErrorCode::AttestationNotFound));

        let stored: BytesN<32> = attestation.payload_hash.try_into().unwrap_or_else(|_| {
            panic_with_error!(&env, ErrorCode::StorageCorrupted)
        });
        verify_payload_hash(&stored, &expected_hash)
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
            QuoteSubmitEvent { quote_id: next, anchor, base_asset, quote_asset, rate, valid_until },
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
            QuoteReceivedEvent { quote_id, receiver, timestamp: env.ledger().timestamp() },
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
        Self::store_attestation(&env, id, issuer.clone(), subject.clone(), timestamp, payload_hash.clone(), signature);

        env.storage().persistent().set(&used_key, &true);
        env.storage().persistent().extend_ttl(&used_key, PERSISTENT_TTL, PERSISTENT_TTL);

        let sopcnt_key = (symbol_short!("SOPCNT"), session_id);
        let op_index: u64 = env.storage().persistent().get(&sopcnt_key).unwrap_or(0u64);
        env.storage().persistent().set(&sopcnt_key, &(op_index + 1));
        env.storage().persistent().extend_ttl(&sopcnt_key, PERSISTENT_TTL, PERSISTENT_TTL);

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
            (symbol_short!("attest"), symbol_short!("recorded"), id, subject),
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

        env.events().publish((symbol_short!("attestor"), symbol_short!("added"), attestor), ());
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

        env.events().publish((symbol_short!("attestor"), symbol_short!("removed"), attestor), ());
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
    // Health monitoring
    // -----------------------------------------------------------------------

    pub fn set_health_failure_threshold(env: Env, threshold: u32) {
        Self::require_admin(&env);
        env.storage().instance().set(&symbol_short!("HTHRESH"), &threshold);
        env.storage().instance().extend_ttl(INSTANCE_TTL, INSTANCE_TTL);
    }

    pub fn update_health_status(
        env: Env,
        anchor: Address,
        latency_ms: u64,
        failure_count: u32,
        availability_percent: u32,
    ) {
        let status = HealthStatus { anchor: anchor.clone(), latency_ms, failure_count, availability_percent };
        let key = (symbol_short!("HEALTH"), anchor.clone());
        env.storage().persistent().set(&key, &status);
        env.storage().persistent().extend_ttl(&key, PERSISTENT_TTL, PERSISTENT_TTL);

        let threshold: u32 = env.storage().instance().get(&symbol_short!("HTHRESH")).unwrap_or(0u32);

        if threshold > 0 && failure_count >= threshold {
            let meta_key = (symbol_short!("ANCHMETA"), anchor.clone());
            if let Some(mut meta) = env.storage().persistent().get::<_, RoutingAnchorMeta>(&meta_key) {
                if meta.is_active {
                    meta.is_active = false;
                    env.storage().persistent().set(&meta_key, &meta);
                    env.storage().persistent().extend_ttl(&meta_key, PERSISTENT_TTL, PERSISTENT_TTL);
                    env.events().publish(
                        (symbol_short!("anchor"), symbol_short!("deactivate")),
                        AnchorDeactivated { anchor, failure_count, threshold },
                    );
                }
            }
        }
    }

    pub fn get_health_status(env: Env, anchor: Address) -> Option<HealthStatus> {
        env.storage()
            .persistent()
            .get::<_, HealthStatus>(&(symbol_short!("HEALTH"), anchor))
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

    pub fn get_routing_anchors(env: Env) -> Vec<Address> {
        let list_key = soroban_sdk::vec![&env, symbol_short!("ANCHLIST")];
        env.storage().persistent()
            .get::<_, Vec<Address>>(&list_key)
            .unwrap_or_else(|| Vec::new(&env))
    }

    pub fn route_transaction(env: Env, options: RoutingOptions) -> Quote {
        let now = env.ledger().timestamp();
        let list_key = soroban_sdk::vec![&env, symbol_short!("ANCHLIST")];
        let anchors: Vec<Address> = env.storage().persistent()
            .get::<_, Vec<Address>>(&list_key)
            .unwrap_or_else(|| Vec::new(&env));

        let mut candidates: Vec<Quote> = Vec::new(&env);
        for anchor in anchors.iter() {
            let meta_key = (symbol_short!("ANCHMETA"), anchor.clone());
            let meta: RoutingAnchorMeta = match env.storage().persistent().get(&meta_key) {
                Some(m) => m,
                None => continue,
            };
            if !meta.is_active { continue; }
            if meta.reputation_score < options.min_reputation { continue; }

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

            if quote.valid_until <= now { continue; }
            if options.request.amount < quote.minimum_amount || options.request.amount > quote.maximum_amount {
                continue;
            }

            candidates.push_back(quote);
        }

        if candidates.is_empty() {
            panic_with_error!(&env, ErrorCode::NoQuotesAvailable);
        }

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

    pub fn fetch_anchor_info(env: Env, anchor: Address, toml_data: StellarToml, ttl_seconds: u64) {
        anchor.require_auth();
        let now = env.ledger().timestamp();
        let cached = CachedToml { toml: toml_data, cached_at: now, ttl_seconds };
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

    pub fn get_anchor_asset_info(env: Env, anchor: Address, asset_code: String) -> AssetInfo {
        let toml = Self::get_anchor_toml(env.clone(), anchor);
        for asset in toml.currencies.iter() {
            if asset.code == asset_code {
                return asset;
            }
        }
        panic_with_error!(&env, ErrorCode::ValidationError);
    }

    pub fn get_anchor_deposit_limits(env: Env, anchor: Address, asset_code: String) -> (u64, u64) {
        let asset = Self::get_anchor_asset_info(env, anchor, asset_code);
        (asset.deposit_min_amount, asset.deposit_max_amount)
    }

    pub fn get_anchor_withdrawal_limits(env: Env, anchor: Address, asset_code: String) -> (u64, u64) {
        let asset = Self::get_anchor_asset_info(env, anchor, asset_code);
        (asset.withdrawal_min_amount, asset.withdrawal_max_amount)
    }

    pub fn get_anchor_deposit_fees(env: Env, anchor: Address, asset_code: String) -> (u64, u32) {
        let asset = Self::get_anchor_asset_info(env, anchor, asset_code);
        (asset.deposit_fee_fixed, asset.deposit_fee_percent)
    }

    pub fn get_anchor_withdrawal_fees(env: Env, anchor: Address, asset_code: String) -> (u64, u32) {
        let asset = Self::get_anchor_asset_info(env, anchor, asset_code);
        (asset.withdrawal_fee_fixed, asset.withdrawal_fee_percent)
    }

    pub fn anchor_supports_deposits(env: Env, anchor: Address, asset_code: String) -> bool {
        match Self::get_anchor_asset_info(env, anchor, asset_code) {
            asset => asset.deposit_enabled,
        }
    }

    pub fn anchor_supports_withdrawals(env: Env, anchor: Address, asset_code: String) -> bool {
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
        if !env.storage().persistent().has(&(symbol_short!("ATTESTOR"), attestor.clone())) {
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
        let attestation = Attestation { id, issuer, subject: subject.clone(), timestamp, payload_hash, signature };
        let key = (symbol_short!("ATTEST"), id);
        env.storage().persistent().set(&key, &attestation);
        env.storage().persistent().extend_ttl(&key, PERSISTENT_TTL, PERSISTENT_TTL);

        let count_key = (symbol_short!("SUBCNT"), subject.clone());
        let count: u64 = env.storage().persistent().get(&count_key).unwrap_or(0);

        let subj_att_key = (symbol_short!("SUBATT"), subject.clone(), count);
        env.storage().persistent().set(&subj_att_key, &id);
        env.storage().persistent().extend_ttl(&subj_att_key, PERSISTENT_TTL, PERSISTENT_TTL);

        env.storage().persistent().set(&count_key, &(count + 1));
        env.storage().persistent().extend_ttl(&count_key, PERSISTENT_TTL, PERSISTENT_TTL);
    }

    fn store_span(env: &Env, request_id: &RequestId, operation: String, actor: Address, now: u64, status: String) {
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
        env.storage().temporary().extend_ttl(&key, SPAN_TTL, SPAN_TTL);
    }
}

pub fn get_endpoint(env: Env, attestor: Address) -> String {
    AnchorKitContract::get_endpoint(env, attestor)
}

pub fn set_endpoint(env: Env, attestor: Address, endpoint: String) {
    AnchorKitContract::set_endpoint(env, attestor, endpoint)
}
