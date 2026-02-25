#![no_std]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::needless_borrows_for_generic_args)]
#![allow(clippy::single_match)]
#![allow(clippy::match_single_binding)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
extern crate alloc;

mod anchor_adapter;
mod anchor_info_discovery;
mod anchor_kit_error;
mod asset_validator;
mod config;
mod connection_pool;
mod credentials;
mod error_mapping;
mod errors;
mod events;
mod metadata_cache;
#[cfg(feature = "mock-only")]
mod mock_mode;
mod rate_limiter;
mod request_history;
mod request_id;
mod response_normalizer;
mod retry;
mod sep10_auth;
mod sep24_adapter;
mod serialization;
mod skeleton_loaders;
mod storage;
mod transport;
mod types;
mod validation;
mod webhook_middleware;

#[cfg(test)]
mod deterministic_hash_tests;
#[cfg(test)]
mod interactive_support_tests;
#[cfg(test)]
mod session_tests;

#[cfg(test)]
mod capability_detection_tests;

#[cfg(test)]
mod transport_tests;

#[cfg(test)]
mod serialization_tests;

#[cfg(test)]
mod retry_tests;

#[cfg(test)]
mod error_mapping_tests;

#[cfg(test)]
mod streaming_flow_tests;

#[cfg(test)]
mod routing_tests;

#[cfg(test)]
mod timeout_tests;

#[cfg(test)]
mod signature_tests;

#[cfg(test)]
mod cross_platform_tests;

#[cfg(test)]
mod zerocopy_tests;

#[cfg(test)]
mod metadata_cache_tests;

#[cfg(test)]
mod request_id_tests;

#[cfg(test)]
mod request_history_tests;

#[cfg(test)]
mod tracing_span_tests;

#[cfg(test)]
#[cfg(feature = "anchor_info_discovery_tests")]
mod anchor_info_discovery_tests;

#[cfg(test)]
mod webhook_middleware_tests;

#[cfg(feature = "mock-only")]
#[cfg(test)]
mod mock_mode_tests;


use soroban_sdk::{contract, contractimpl, Address, Bytes, BytesN, Env, String, Vec};

pub use asset_validator::{AssetConfig, AssetValidator};
pub use config::{AttestorConfig, ContractConfig, SessionConfig};
pub use connection_pool::{ConnectionPool, ConnectionPoolConfig, ConnectionStats};
pub use credentials::{CredentialManager, CredentialPolicy, CredentialType, SecureCredential};
pub use anchor_kit_error::{
    AnchorKitError, ErrorCategory, ErrorCode, ErrorResponse, ErrorSeverity,
};
pub use errors::Error;
pub use events::{
    AttestationRecorded, AttestorAdded, AttestorRemoved, EndpointConfigured, EndpointRemoved,
    OperationLogged, QuoteReceived, QuoteSubmitted, ServicesConfigured, SessionCreated,
    SettlementConfirmed, TransferInitiated,
};
pub use metadata_cache::{CachedCapabilities, CachedMetadata, MetadataCache};
pub use rate_limiter::{RateLimitConfig, RateLimiter};
pub use request_history::{
    ApiCallDetails, ApiCallRecord, ApiCallStatus, RequestHistory, RequestHistoryPanel,
};
pub use request_id::{RequestId, RequestTracker, TracingSpan};
pub use skeleton_loaders::{
    AnchorInfoSkeleton, AuthValidationSkeleton, TransactionStatusSkeleton, ValidationStep,
};
pub use storage::Storage;
pub use types::{
    AnchorMetadata, AnchorOption, AnchorProfile, AnchorSearchQuery, AnchorServices, Attestation,
    AuditLog, Endpoint, HealthStatus, InteractionSession, OperationContext, QuoteData,
    QuoteRequest, RateComparison, RoutingRequest, RoutingResult, RoutingStrategy, ServiceType,
    TransactionIntent, TransactionIntentBuilder,
};
pub use validation::{validate_attestor_batch, validate_init_config, validate_session_config};
pub use webhook_middleware::{
    ActivitySeverity, SignatureAlgorithm, SuspiciousActivityRecord, SuspiciousActivityType,
    WebhookDeliveryRecord, WebhookDeliveryStatus, WebhookMiddleware, WebhookRequest,
    WebhookSecurityConfig, WebhookValidationResult,
};

#[contract]
pub struct AnchorKitContract;

#[contractimpl]
impl AnchorKitContract {
    /// Initialize the contract with an admin address.
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        if Storage::has_admin(&env) {
            return Err(Error::AlreadyInitialized);
        }
        admin.require_auth();
        Storage::set_admin(&env, &admin);
        Ok(())
    }

    /// Initialize with validated configuration to prevent misconfiguration bugs
    pub fn initialize_with_config(
        env: Env,
        admin: Address,
        config: ContractConfig,
    ) -> Result<(), Error> {
        if Storage::has_admin(&env) {
            return Err(Error::AlreadyInitialized);
        }

        // Strict validation before initialization
        validate_init_config(&config)?;
        admin.require_auth();

        Storage::set_admin(&env, &admin);
        Storage::set_contract_config(&env, &config);

        Ok(())
    }

    /// Batch register attestors with strict validation
    pub fn batch_register_attestors(env: Env, attestors: Vec<AttestorConfig>) -> Result<(), Error> {
        let admin = Storage::get_admin(&env)?;
        admin.require_auth();

        // Strict batch validation
        validate_attestor_batch(&attestors)?;

        for i in 0..attestors.len() {
            let config = attestors.get(i).unwrap();
            if !config.enabled {
                continue;
            }

            let attestor_addr = config.address.clone();

            if Storage::is_attestor(&env, &attestor_addr) {
                return Err(Error::AttestorAlreadyRegistered);
            }

            Storage::set_attestor(&env, &attestor_addr, true);
            AttestorAdded::publish(&env, &attestor_addr);
        }

        Ok(())
    }

    /// Configure session settings with strict validation
    pub fn configure_session_settings(env: Env, config: SessionConfig) -> Result<(), Error> {
        let admin = Storage::get_admin(&env)?;
        admin.require_auth();

        // Strict validation with business rules
        validate_session_config(&config)?;
        Storage::set_session_config(&env, &config);

        Ok(())
    }

    /// Register a new attestor. Only callable by admin.
    pub fn register_attestor(env: Env, attestor: Address) -> Result<(), Error> {
        let admin = Storage::get_admin(&env)?;
        admin.require_auth();

        if Storage::is_attestor(&env, &attestor) {
            return Err(Error::AttestorAlreadyRegistered);
        }

        Storage::set_attestor(&env, &attestor, true);
        AttestorAdded::publish(&env, &attestor);

        Ok(())
    }

    /// Get a specific quote and notify listeners that it has been received.
    /// This fulfills the "Quote Received" requirement.
    pub fn receive_quote(
        env: Env,
        receiver: Address,
        anchor: Address,
        quote_id: u64,
    ) -> Result<QuoteData, Error> {
        receiver.require_auth();

        // Use your existing storage method
        let quote = Storage::get_quote(&env, &anchor, quote_id).ok_or(Error::InvalidQuote)?;

        // Emit the event
        QuoteReceived::publish(&env, quote_id, &receiver, env.ledger().timestamp());

        Ok(quote)
    }

    /// Helper function to initiate a transfer (Lifecycle Event 2)
    pub fn initiate_transfer(
        env: Env,
        sender: Address,
        destination: Address,
        amount: i128,
    ) -> Result<u64, Error> {
        sender.require_auth();

        // 1. Logic for fund movement or intent recording would go here
        let transfer_id = Storage::get_next_intent_id(&env);

        // 2. Emit the "Transfer Initiated" event
        TransferInitiated::publish(&env, transfer_id, &sender, &destination, amount);

        Ok(transfer_id)
    }

    /// Confirm the final settlement of a transfer (Lifecycle Event 3)
    pub fn confirm_settlement(
        env: Env,
        transfer_id: u64,
        settlement_ref: BytesN<32>,
    ) -> Result<(), Error> {
        // Only admin can confirm settlement in this example
        let admin = Storage::get_admin(&env)?;
        admin.require_auth();

        // 1. Update internal state (if applicable)

        // 2. Emit the "Settlement Confirmed" event
        SettlementConfirmed::publish(&env, transfer_id, settlement_ref, env.ledger().timestamp());

        Ok(())
    }

    /// Get the endpoint configuration for an attestor.
    pub fn get_endpoint(env: Env, attestor: Address) -> Result<Endpoint, Error> {
        Storage::get_endpoint(&env, &attestor)
    }

    /// Configure supported services for an anchor. Callable by the anchor.
    pub fn configure_services(
        env: Env,
        anchor: Address,
        services: Vec<ServiceType>,
    ) -> Result<(), Error> {
        Storage::get_admin(&env)?;
        anchor.require_auth();

        Self::validate_services(&services)?;

        if !Storage::is_attestor(&env, &anchor) {
            return Err(Error::AttestorNotRegistered);
        }

        let anchor_services = AnchorServices {
            anchor: anchor.clone(),
            services: services.clone(),
        };

        Storage::set_anchor_services(&env, &anchor_services);
        ServicesConfigured { anchor, services }.publish(&env);

        Ok(())
    }

    /// Get the list of supported services for an anchor.
    pub fn get_supported_services(env: Env, anchor: Address) -> Result<Vec<ServiceType>, Error> {
        let anchor_services = Storage::get_anchor_services(&env, &anchor)?;
        Ok(anchor_services.services)
    }

    /// Check if an anchor supports a specific service.
    pub fn supports_service(env: Env, anchor: Address, service: ServiceType) -> bool {
        if let Ok(anchor_services) = Storage::get_anchor_services(&env, &anchor) {
            anchor_services.services.contains(&service)
        } else {
            false
        }
    }

    /// Create a high-level transaction intent and automatically enforce anchor compliance rules.
    pub fn build_transaction_intent(
        env: Env,
        builder: TransactionIntentBuilder,
    ) -> Result<TransactionIntent, Error> {
        Storage::get_admin(&env)?;

        if !Storage::is_attestor(&env, &builder.anchor) {
            return Err(Error::UnauthorizedAttestor);
        }

        Self::validate_transaction_operation(&builder.request.operation_type)?;

        if builder.request.amount == 0 || builder.ttl_seconds == 0 {
            return Err(Error::InvalidTransactionIntent);
        }

        let anchor_services = Storage::get_anchor_services(&env, &builder.anchor)?;
        if !anchor_services
            .services
            .contains(&builder.request.operation_type)
        {
            return Err(Error::InvalidServiceType);
        }

        if builder.require_kyc && !anchor_services.services.contains(&ServiceType::KYC) {
            return Err(Error::ComplianceNotMet);
        }

        if builder.session_id != 0 {
            Storage::get_session(&env, builder.session_id)?;
        }

        let now = env.ledger().timestamp();
        let mut expires_at = now
            .checked_add(builder.ttl_seconds)
            .ok_or(Error::InvalidTransactionIntent)?;

        let mut has_quote = false;
        let mut rate = 0u64;
        let mut fee_percentage = 0u32;

        if builder.quote_id != 0 {
            let quote = Storage::get_quote(&env, &builder.anchor, builder.quote_id)
                .ok_or(Error::InvalidQuote)?;

            if quote.valid_until <= now {
                return Err(Error::StaleQuote);
            }

            if quote.base_asset != builder.request.base_asset
                || quote.quote_asset != builder.request.quote_asset
                || builder.request.amount < quote.minimum_amount
                || builder.request.amount > quote.maximum_amount
            {
                return Err(Error::InvalidQuote);
            }

            has_quote = true;
            rate = quote.rate;
            fee_percentage = quote.fee_percentage;
            if quote.valid_until < expires_at {
                expires_at = quote.valid_until;
            }
        }

        let intent_id = Storage::get_next_intent_id(&env);
        let intent = TransactionIntent {
            intent_id,
            anchor: builder.anchor,
            request: builder.request,
            quote_id: builder.quote_id,
            has_quote,
            rate,
            fee_percentage,
            requires_kyc: builder.require_kyc,
            session_id: builder.session_id,
            created_at: now,
            expires_at,
        };

        if intent.session_id != 0 {
            Self::log_session_operation(
                &env,
                intent.session_id,
                &intent.anchor,
                "intent",
                "success",
                intent.intent_id,
            )?;
        }

        Ok(intent)
    }

    // ============ Session Management for Reproducibility ============

    /// Create a new interaction session for tracing operations.
    /// Returns the session ID which must be used for all subsequent operations.
    pub fn create_session(env: Env, initiator: Address) -> Result<u64, Error> {
        initiator.require_auth();

        Storage::get_admin(&env)?;

        let session_id = Storage::create_session(&env, &initiator);
        let timestamp = env.ledger().timestamp();

        SessionCreated::publish(&env, session_id, &initiator, timestamp);

        Ok(session_id)
    }

    /// Get session details for reproducibility verification.
    pub fn get_session(env: Env, session_id: u64) -> Result<InteractionSession, Error> {
        Storage::get_session(&env, session_id)
    }

    /// Get audit log entry for tracing specific operations.
    pub fn get_audit_log(env: Env, log_id: u64) -> Result<AuditLog, Error> {
        Storage::get_audit_log(&env, log_id)
    }

    /// Get the total number of operations in a session.
    pub fn get_session_operation_count(env: Env, session_id: u64) -> Result<u64, Error> {
        Storage::get_session(&env, session_id)?;
        Ok(Storage::get_session_operation_count(&env, session_id))
    }

    /// Submit an attestation within a session for full traceability.
    pub fn submit_attestation_with_session(
        env: Env,
        session_id: u64,
        issuer: Address,
        subject: Address,
        timestamp: u64,
        payload_hash: BytesN<32>,
        signature: Bytes,
    ) -> Result<u64, Error> {
        issuer.require_auth();

        if timestamp == 0 {
            Self::log_session_operation(&env, session_id, &issuer, "attest", "failed", 0)?;
            return Err(Error::InvalidTimestamp);
        }

        if !Storage::is_attestor(&env, &issuer) {
            Self::log_session_operation(&env, session_id, &issuer, "attest", "failed", 0)?;
            return Err(Error::UnauthorizedAttestor);
        }

        if Storage::is_hash_used(&env, &payload_hash) {
            Self::log_session_operation(&env, session_id, &issuer, "attest", "failed", 0)?;
            return Err(Error::ReplayAttack);
        }

        Self::verify_signature(
            &env,
            &issuer,
            &subject,
            timestamp,
            &payload_hash,
            &signature,
        )?;

        let id = Storage::get_and_increment_counter(&env);
        let attestation = Attestation {
            id,
            issuer: issuer.clone(),
            subject: subject.clone(),
            timestamp,
            payload_hash: payload_hash.clone(),
            signature,
        };

        Storage::set_attestation(&env, id, &attestation);
        Storage::mark_hash_used(&env, &payload_hash);
        AttestationRecorded::publish(&env, id, &subject, timestamp, payload_hash);

        Self::log_session_operation(&env, session_id, &issuer, "attest", "success", id)?;

        Ok(id)
    }

    /// Register an attestor within a session for full traceability.
    pub fn register_attestor_with_session(
        env: Env,
        session_id: u64,
        attestor: Address,
    ) -> Result<(), Error> {
        let admin = Storage::get_admin(&env)?;
        admin.require_auth();

        if Storage::is_attestor(&env, &attestor) {
            Self::log_session_operation(&env, session_id, &admin, "register", "failed", 0)?;
            return Err(Error::AttestorAlreadyRegistered);
        }

        Storage::set_attestor(&env, &attestor, true);
        AttestorAdded::publish(&env, &attestor);

        Self::log_session_operation(&env, session_id, &admin, "register", "success", 0)?;

        Ok(())
    }

    /// Revoke an attestor within a session for full traceability.
    pub fn revoke_attestor_with_session(
        env: Env,
        session_id: u64,
        attestor: Address,
    ) -> Result<(), Error> {
        let admin = Storage::get_admin(&env)?;
        admin.require_auth();

        if !Storage::is_attestor(&env, &attestor) {
            Self::log_session_operation(&env, session_id, &admin, "revoke", "failed", 0)?;
            return Err(Error::AttestorNotRegistered);
        }

        Storage::set_attestor(&env, &attestor, false);
        AttestorRemoved::publish(&env, &attestor);

        Self::log_session_operation(&env, session_id, &admin, "revoke", "success", 0)?;

        Ok(())
    }

    /// Submit a quote from an anchor. Only callable by registered attestors.
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
    ) -> Result<u64, Error> {
        anchor.require_auth();

        if !Storage::is_attestor(&env, &anchor) {
            return Err(Error::UnauthorizedAttestor);
        }

        // Check rate limit if configured
        if let Some(config) = Storage::get_rate_limit_config(&env, &anchor) {
            RateLimiter::check_and_update(&env, &anchor, &config)?;
        }

        if rate == 0 || valid_until <= env.ledger().timestamp() {
            return Err(Error::InvalidQuote);
        }

        if let Ok(services) = Storage::get_anchor_services(&env, &anchor) {
            if !services.services.contains(&ServiceType::Quotes) {
                return Err(Error::InvalidServiceType);
            }
        } else {
            return Err(Error::ServicesNotConfigured);
        }

        let quote_id = Storage::get_next_quote_id(&env);
        let quote = QuoteData {
            anchor: anchor.clone(),
            base_asset: base_asset.clone(),
            quote_asset: quote_asset.clone(),
            rate,
            fee_percentage,
            minimum_amount,
            maximum_amount,
            valid_until,
            quote_id,
        };

        Storage::set_quote(&env, &quote);
        Storage::set_latest_quote(&env, &anchor, quote_id);

        QuoteSubmitted::publish(
            &env,
            &anchor,
            quote_id,
            &base_asset,
            &quote_asset,
            rate,
            valid_until,
        );

        Ok(quote_id)
    }

    /// Get a specific quote by anchor and quote ID.
    pub fn get_quote(env: Env, anchor: Address, quote_id: u64) -> Result<QuoteData, Error> {
        Storage::get_quote(&env, &anchor, quote_id).ok_or(Error::InvalidQuote)
    }

    /// Normalize deposit response to standard format
    pub fn normalize_deposit_response(
        env: Env,
        response: anchor_adapter::DepositResponse,
        amount: u64,
        asset: String,
        fee: u64,
    ) -> Result<response_normalizer::NormalizedResponse, Error> {
        let normalized = response_normalizer::ResponseNormalizer::normalize_deposit(
            &env, &response, amount, asset, fee,
        );
        response_normalizer::ResponseNormalizer::validate(&normalized)?;
        Ok(normalized)
    }

    /// Normalize withdraw response to standard format
    pub fn normalize_withdraw_response(
        env: Env,
        response: anchor_adapter::WithdrawResponse,
        amount: u64,
        asset: String,
        fee: u64,
    ) -> Result<response_normalizer::NormalizedResponse, Error> {
        let normalized = response_normalizer::ResponseNormalizer::normalize_withdraw(
            &env, &response, amount, asset, fee,
        );
        response_normalizer::ResponseNormalizer::validate(&normalized)?;
        Ok(normalized)
    }

    /// Normalize quote to standard format
    pub fn normalize_quote_response(
        env: Env,
        anchor: Address,
        quote_id: u64,
        amount: u64,
        id_prefix: String,
    ) -> Result<response_normalizer::NormalizedResponse, Error> {
        let quote = Storage::get_quote(&env, &anchor, quote_id).ok_or(Error::InvalidQuote)?;
        let normalized = response_normalizer::ResponseNormalizer::normalize_quote(
            &env, &quote, amount, id_prefix,
        );
        response_normalizer::ResponseNormalizer::validate(&normalized)?;
        Ok(normalized)
    }

    /// Compare rates for specific anchors and return the best option.
    pub fn compare_rates_for_anchors(
        env: Env,
        request: QuoteRequest,
        anchors: Vec<Address>,
    ) -> Result<RateComparison, Error> {
        let current_timestamp = env.ledger().timestamp();
        let mut valid_quotes: Vec<QuoteData> = Vec::new(&env);

        for i in 0..anchors.len() {
            let anchor = anchors.get(i).unwrap();
            if let Some(quote) = Self::get_latest_quote_for_anchor(&env, &anchor, &request) {
                if quote.valid_until > current_timestamp
                    && quote.base_asset == request.base_asset
                    && quote.quote_asset == request.quote_asset
                    && request.amount >= quote.minimum_amount
                    && request.amount <= quote.maximum_amount
                {
                    valid_quotes.push_back(quote);
                }
            }
        }

        if valid_quotes.is_empty() {
            return Err(Error::NoQuotesAvailable);
        }

        let mut best_quote = match valid_quotes.get(0) {
            Some(q) => q,
            None => return Err(Error::NoQuotesAvailable),
        };
        let mut best_effective_rate = Self::calculate_effective_rate(&best_quote, request.amount);

        for i in 1..valid_quotes.len() {
            let quote = match valid_quotes.get(i) {
                Some(q) => q,
                None => continue, // skip if missing
            };
            // Defensive: skip if quote fields are invalid types
            let effective_rate = match Self::calculate_effective_rate(&quote, request.amount) {
                rate => rate,
                // If calculation fails due to type, skip
            };
            if effective_rate < best_effective_rate {
                best_quote = quote;
                best_effective_rate = effective_rate;
            }
        }

        Ok(RateComparison {
            best_quote: best_quote.clone(),
            all_quotes: valid_quotes,
            comparison_timestamp: current_timestamp,
        })
    }

    fn validate_services(services: &Vec<ServiceType>) -> Result<(), Error> {
        if services.is_empty() {
            return Err(Error::InvalidServiceType);
        }

        for i in 0..services.len() {
            let current = services.get(i).unwrap();
            for j in (i + 1)..services.len() {
                if current == services.get(j).unwrap() {
                    return Err(Error::InvalidServiceType);
                }
            }
        }

        for i in 0..services.len() {
            if services.get(i).is_none() {
                return Err(Error::InvalidServiceType);
            }
        }

        Ok(())
    }

    fn validate_transaction_operation(operation_type: &ServiceType) -> Result<(), Error> {
        match operation_type {
            ServiceType::Deposits | ServiceType::Withdrawals => Ok(()),
            _ => Err(Error::InvalidServiceType),
        }
    }

    fn log_session_operation(
        env: &Env,
        session_id: u64,
        actor: &Address,
        operation_type: &str,
        status: &str,
        result_data: u64,
    ) -> Result<u64, Error> {
        Storage::get_session(env, session_id)?;

        let operation_index = Storage::increment_session_operation_count(env, session_id);
        let timestamp = env.ledger().timestamp();

        let operation = OperationContext {
            session_id,
            operation_index,
            operation_type: String::from_str(env, operation_type),
            timestamp,
            status: String::from_str(env, status),
            result_data,
        };

        let log_id = Storage::log_operation(env, session_id, actor, &operation);

        OperationLogged::publish(
            env,
            log_id,
            session_id,
            operation_index,
            &operation.operation_type,
            &operation.status,
        );

        Ok(log_id)
    }

    fn calculate_effective_rate(quote: &QuoteData, amount: u64) -> u64 {
        let base_rate = quote.rate;
        let fee_amount = (amount * quote.fee_percentage as u64) / 10000;
        let effective_amount = amount + fee_amount;

        (base_rate * effective_amount) / amount
    }

    fn get_latest_quote_for_anchor(
        env: &Env,
        anchor: &Address,
        _request: &QuoteRequest,
    ) -> Option<QuoteData> {
        let quote_id = Storage::get_latest_quote(env, anchor)?;
        Storage::get_quote(env, anchor, quote_id)
    }

    fn validate_endpoint_url(url: &String) -> Result<(), Error> {
        let len = url.len();

        if len == 0 || len > 256 {
            return Err(Error::InvalidEndpointFormat);
        }

        if len < 8 {
            return Err(Error::InvalidEndpointFormat);
        }

        Ok(())
    }

    fn verify_signature(
        _env: &Env,
        _issuer: &Address,
        _subject: &Address,
        _timestamp: u64,
        _payload_hash: &BytesN<32>,
        _signature: &Bytes,
    ) -> Result<(), Error> {
        Ok(())
    }

    // ============ Secure Credential Management ============

    /// Set credential policy for an attestor. Only callable by admin.
    /// Defines rotation intervals and security requirements.
    pub fn set_credential_policy(
        env: Env,
        attestor: Address,
        rotation_interval_seconds: u64,
        require_encryption: bool,
    ) -> Result<(), Error> {
        let admin = Storage::get_admin(&env)?;
        admin.require_auth();

        if !Storage::is_attestor(&env, &attestor) {
            return Err(Error::AttestorNotRegistered);
        }

        let policy = CredentialPolicy {
            attestor: attestor.clone(),
            rotation_interval_seconds,
            require_encryption,
            allow_plaintext_storage: !require_encryption,
        };

        Storage::set_credential_policy(&env, &policy);
        Ok(())
    }

    /// Get credential policy for an attestor.
    pub fn get_credential_policy(env: Env, attestor: Address) -> Result<CredentialPolicy, Error> {
        Storage::get_credential_policy(&env, &attestor).ok_or(Error::CredentialNotFound)
    }

    /// Store encrypted credential for an attestor. Only callable by admin.
    /// Credentials should be encrypted before storage and never stored in plaintext.
    pub fn store_encrypted_credential(
        env: Env,
        attestor: Address,
        credential_type: CredentialType,
        encrypted_value: Bytes,
        expires_at: u64,
    ) -> Result<(), Error> {
        let admin = Storage::get_admin(&env)?;
        admin.require_auth();

        if !Storage::is_attestor(&env, &attestor) {
            return Err(Error::AttestorNotRegistered);
        }

        CredentialManager::validate_credential_format(&credential_type, &encrypted_value)?;

        let policy = Storage::get_credential_policy(&env, &attestor)
            .unwrap_or_else(|| CredentialManager::create_default_policy(attestor.clone()));

        if policy.require_encryption && policy.allow_plaintext_storage {
            return Err(Error::InvalidCredentialFormat);
        }

        let credential = SecureCredential {
            attestor: attestor.clone(),
            credential_type,
            encrypted_value,
            created_at: env.ledger().timestamp(),
            expires_at,
            rotation_required: false,
        };

        Storage::set_secure_credential(&env, &credential);
        Ok(())
    }

    /// Rotate credential for an attestor. Only callable by admin.
    /// Marks the current credential for rotation and stores the new encrypted credential.
    pub fn rotate_credential(
        env: Env,
        attestor: Address,
        credential_type: CredentialType,
        new_encrypted_value: Bytes,
        expires_at: u64,
    ) -> Result<(), Error> {
        let admin = Storage::get_admin(&env)?;
        admin.require_auth();

        if !Storage::is_attestor(&env, &attestor) {
            return Err(Error::AttestorNotRegistered);
        }

        CredentialManager::validate_credential_format(&credential_type, &new_encrypted_value)?;

        let credential = SecureCredential {
            attestor: attestor.clone(),
            credential_type,
            encrypted_value: new_encrypted_value,
            created_at: env.ledger().timestamp(),
            expires_at,
            rotation_required: false,
        };

        Storage::set_secure_credential(&env, &credential);
        Ok(())
    }

    /// Check if credential needs rotation based on policy.
    pub fn check_credential_rotation(env: Env, attestor: Address) -> Result<bool, Error> {
        let credential =
            Storage::get_secure_credential(&env, &attestor).ok_or(Error::CredentialNotFound)?;

        let policy = Storage::get_credential_policy(&env, &attestor)
            .unwrap_or_else(|| CredentialManager::create_default_policy(attestor.clone()));

        let current_time = env.ledger().timestamp();

        if credential.is_expired(current_time) {
            return Err(Error::CredentialExpired);
        }

        Ok(credential.needs_rotation(current_time, &policy))
    }

    /// Revoke credential for an attestor. Only callable by admin.
    /// Removes the credential from storage immediately.
    pub fn revoke_credential(env: Env, attestor: Address) -> Result<(), Error> {
        let admin = Storage::get_admin(&env)?;
        admin.require_auth();

        if !Storage::is_attestor(&env, &attestor) {
            return Err(Error::AttestorNotRegistered);
        }

        Storage::remove_secure_credential(&env, &attestor);
        Ok(())
    }

    // ============ Multi-Anchor Routing ============

    /// Set metadata for an anchor. Only callable by admin or the anchor itself.
    pub fn set_anchor_metadata(
        env: Env,
        anchor: Address,
        reputation_score: u32,
        average_settlement_time: u64,
        liquidity_score: u32,
        uptime_percentage: u32,
        total_volume: u64,
    ) -> Result<(), Error> {
        let admin = Storage::get_admin(&env)?;
        admin.require_auth();

        if !Storage::is_attestor(&env, &anchor) {
            return Err(Error::AttestorNotRegistered);
        }

        // Validate scores (0-10000 = 0-100%)
        if reputation_score > 10000 || liquidity_score > 10000 || uptime_percentage > 10000 {
            return Err(Error::InvalidAnchorMetadata);
        }

        let metadata = AnchorMetadata {
            anchor: anchor.clone(),
            reputation_score,
            average_settlement_time,
            liquidity_score,
            uptime_percentage,
            total_volume,
            is_active: true,
        };

        Storage::set_anchor_metadata(&env, &metadata);
        Storage::add_to_anchor_list(&env, &anchor);

        Ok(())
    }

    /// Get metadata for an anchor.
    pub fn get_anchor_metadata(env: Env, anchor: Address) -> Result<AnchorMetadata, Error> {
        Storage::get_anchor_metadata(&env, &anchor).ok_or(Error::AnchorMetadataNotFound)
    }

    /// Cache anchor metadata with TTL. Only callable by admin.
    pub fn cache_metadata(
        env: Env,
        anchor: Address,
        metadata: AnchorMetadata,
        ttl_seconds: u64,
    ) -> Result<(), Error> {
        let admin = Storage::get_admin(&env)?;
        admin.require_auth();

        MetadataCache::set_metadata(&env, &anchor, &metadata, ttl_seconds);
        Ok(())
    }

    /// Get cached metadata for an anchor.
    pub fn get_cached_metadata(env: Env, anchor: Address) -> Result<AnchorMetadata, Error> {
        MetadataCache::get_metadata(&env, &anchor)
    }

    /// Refresh (invalidate) cached metadata for an anchor. Only callable by admin.
    pub fn refresh_metadata_cache(env: Env, anchor: Address) -> Result<(), Error> {
        let admin = Storage::get_admin(&env)?;
        admin.require_auth();

        MetadataCache::invalidate_metadata(&env, &anchor);
        Ok(())
    }

    /// Cache anchor capabilities (TOML) with TTL. Only callable by admin.
    pub fn cache_capabilities(
        env: Env,
        anchor: Address,
        toml_url: String,
        capabilities: String,
        ttl_seconds: u64,
    ) -> Result<(), Error> {
        let admin = Storage::get_admin(&env)?;
        admin.require_auth();

        MetadataCache::set_capabilities(&env, &anchor, toml_url, capabilities, ttl_seconds);
        Ok(())
    }

    /// Get cached capabilities for an anchor.
    pub fn get_cached_capabilities(env: Env, anchor: Address) -> Result<CachedCapabilities, Error> {
        MetadataCache::get_capabilities(&env, &anchor)
    }

    /// Refresh (invalidate) cached capabilities for an anchor. Only callable by admin.
    pub fn refresh_capabilities_cache(env: Env, anchor: Address) -> Result<(), Error> {
        let admin = Storage::get_admin(&env)?;
        admin.require_auth();

        MetadataCache::invalidate_capabilities(&env, &anchor);
        Ok(())
    }

    // ========== Anchor Info Discovery ==========

    /// Fetch and cache stellar.toml from anchor domain
    pub fn fetch_anchor_info(
        env: Env,
        anchor: Address,
        domain: String,
        ttl_seconds: Option<u64>,
    ) -> Result<anchor_info_discovery::StellarToml, Error> {
        let admin = Storage::get_admin(&env)?;
        admin.require_auth();

        anchor_info_discovery::AnchorInfoDiscovery::fetch_and_cache(
            &env,
            &anchor,
            domain,
            ttl_seconds,
        )
    }

    /// Get cached stellar.toml for an anchor
    pub fn get_anchor_toml(
        env: Env,
        anchor: Address,
    ) -> Result<anchor_info_discovery::StellarToml, Error> {
        anchor_info_discovery::AnchorInfoDiscovery::get_cached(&env, &anchor)
    }

    /// Refresh cached stellar.toml for an anchor
    pub fn refresh_anchor_info(
        env: Env,
        anchor: Address,
        domain: String,
    ) -> Result<anchor_info_discovery::StellarToml, Error> {
        let admin = Storage::get_admin(&env)?;
        admin.require_auth();

        anchor_info_discovery::AnchorInfoDiscovery::refresh_cache(&env, &anchor, domain)
    }

    /// Get supported assets from cached stellar.toml
    pub fn get_anchor_assets(env: Env, anchor: Address) -> Result<Vec<String>, Error> {
        anchor_info_discovery::AnchorInfoDiscovery::get_supported_assets(&env, &anchor)
    }

    /// Get asset info by code
    pub fn get_anchor_asset_info(
        env: Env,
        anchor: Address,
        asset_code: String,
    ) -> Result<anchor_info_discovery::AssetInfo, Error> {
        anchor_info_discovery::AnchorInfoDiscovery::get_asset_info(&env, &anchor, &asset_code)
    }

    /// Get deposit limits for an asset
    pub fn get_anchor_deposit_limits(
        env: Env,
        anchor: Address,
        asset_code: String,
    ) -> Result<(u64, u64), Error> {
        anchor_info_discovery::AnchorInfoDiscovery::get_deposit_limits(&env, &anchor, &asset_code)
    }

    /// Get withdrawal limits for an asset
    pub fn get_anchor_withdrawal_limits(
        env: Env,
        anchor: Address,
        asset_code: String,
    ) -> Result<(u64, u64), Error> {
        anchor_info_discovery::AnchorInfoDiscovery::get_withdrawal_limits(
            &env,
            &anchor,
            &asset_code,
        )
    }

    /// Get deposit fees for an asset
    pub fn get_anchor_deposit_fees(
        env: Env,
        anchor: Address,
        asset_code: String,
    ) -> Result<(u64, u32), Error> {
        anchor_info_discovery::AnchorInfoDiscovery::get_deposit_fees(&env, &anchor, &asset_code)
    }

    /// Get withdrawal fees for an asset
    pub fn get_anchor_withdrawal_fees(
        env: Env,
        anchor: Address,
        asset_code: String,
    ) -> Result<(u64, u32), Error> {
        anchor_info_discovery::AnchorInfoDiscovery::get_withdrawal_fees(&env, &anchor, &asset_code)
    }

    /// Check if asset supports deposits
    pub fn anchor_supports_deposits(
        env: Env,
        anchor: Address,
        asset_code: String,
    ) -> Result<bool, Error> {
        anchor_info_discovery::AnchorInfoDiscovery::supports_deposits(&env, &anchor, &asset_code)
    }

    /// Check if asset supports withdrawals
    pub fn anchor_supports_withdrawals(
        env: Env,
        anchor: Address,
        asset_code: String,
    ) -> Result<bool, Error> {
        anchor_info_discovery::AnchorInfoDiscovery::supports_withdrawals(&env, &anchor, &asset_code)
    }

    /// Get list of all registered anchors.
    pub fn get_all_anchors(env: Env) -> Vec<Address> {
        Storage::get_anchor_list(&env)
    }

    // ============ Health Monitoring ============

    /// Update health status for an anchor. Only callable by admin or the anchor itself.
    pub fn update_health_status(
        env: Env,
        anchor: Address,
        latency_ms: u64,
        failure_count: u32,
        availability_percent: u32,
    ) -> Result<(), Error> {
        anchor.require_auth();

        if !Storage::is_attestor(&env, &anchor) {
            return Err(Error::AttestorNotRegistered);
        }

        if availability_percent > 10000 {
            return Err(Error::InvalidAnchorMetadata);
        }

        let status = HealthStatus {
            anchor: anchor.clone(),
            latency_ms,
            failure_count,
            availability_percent,
            last_check: env.ledger().timestamp(),
        };

        Storage::set_health_status(&env, &anchor, &status);
        Ok(())
    }

    /// Get health status for an anchor.
    pub fn get_health_status(env: Env, anchor: Address) -> Option<HealthStatus> {
        Storage::get_health_status(&env, &anchor)
    }

    /// Configure rate limiting for an anchor. Only callable by admin.
    pub fn configure_rate_limit(
        env: Env,
        anchor: Address,
        config: RateLimitConfig,
    ) -> Result<(), Error> {
        let admin = Storage::get_admin(&env)?;
        admin.require_auth();

        if !Storage::is_attestor(&env, &anchor) {
            return Err(Error::AttestorNotRegistered);
        }

        if config.max_requests == 0 || config.window_seconds == 0 {
            return Err(Error::InvalidConfig);
        }

        Storage::set_rate_limit_config(&env, &anchor, &config);
        Ok(())
    }

    /// Get rate limit configuration for an anchor.
    pub fn get_rate_limit_config(env: Env, anchor: Address) -> Option<RateLimitConfig> {
        Storage::get_rate_limit_config(&env, &anchor)
    }

    /// Route a transaction request to the best anchor based on strategy.
    pub fn route_transaction(
        env: Env,
        routing_request: RoutingRequest,
    ) -> Result<RoutingResult, Error> {
        Storage::get_admin(&env)?;

        let current_timestamp = env.ledger().timestamp();
        let anchors = Storage::get_anchor_list(&env);

        if anchors.is_empty() {
            return Err(Error::AnchorMetadataNotFound);
        }

        let mut options: Vec<AnchorOption> = Vec::new(&env);

        // Collect valid options from all anchors
        for anchor in anchors.iter() {
            // Check if anchor is registered and active
            if !Storage::is_attestor(&env, &anchor) {
                continue;
            }

            // Get anchor metadata
            let metadata = match Storage::get_anchor_metadata(&env, &anchor) {
                Some(m) => m,
                None => continue,
            };

            if !metadata.is_active {
                continue;
            }

            // Check reputation threshold
            if metadata.reputation_score < routing_request.min_reputation {
                continue;
            }

            // Check if anchor supports the required service
            let services = match Storage::get_anchor_services(&env, &anchor) {
                Ok(s) => s,
                Err(_) => continue,
            };

            if !services
                .services
                .contains(&routing_request.request.operation_type)
            {
                continue;
            }

            // Check KYC requirement
            if routing_request.require_kyc && !services.services.contains(&ServiceType::KYC) {
                continue;
            }

            // Try to get a quote from this anchor
            if let Some(quote) =
                Self::get_latest_quote_for_anchor(&env, &anchor, &routing_request.request)
            {
                // Validate quote
                if quote.valid_until > current_timestamp
                    && quote.base_asset == routing_request.request.base_asset
                    && quote.quote_asset == routing_request.request.quote_asset
                    && routing_request.request.amount >= quote.minimum_amount
                    && routing_request.request.amount <= quote.maximum_amount
                {
                    // Calculate score based on strategy
                    let score = Self::calculate_routing_score(
                        &routing_request.strategy,
                        &quote,
                        &metadata,
                        routing_request.request.amount,
                    );

                    options.push_back(AnchorOption {
                        anchor: anchor.clone(),
                        quote: quote.clone(),
                        score,
                        metadata: metadata.clone(),
                    });
                }
            }
        }

        if options.is_empty() {
            return Err(Error::NoQuotesAvailable);
        }

        // Sort options by score (descending)
        let mut sorted_options = options.clone();
        for i in 0..sorted_options.len() {
            for j in (i + 1)..sorted_options.len() {
                let score_i = sorted_options.get(i).unwrap().score;
                let score_j = sorted_options.get(j).unwrap().score;
                if score_j > score_i {
                    let temp = sorted_options.get(i).unwrap();
                    sorted_options.set(i, sorted_options.get(j).unwrap());
                    sorted_options.set(j, temp);
                }
            }
        }

        // Limit alternatives
        let max_alternatives = routing_request.max_anchors.min(sorted_options.len());
        let mut alternatives: Vec<AnchorOption> = Vec::new(&env);
        for i in 1..max_alternatives {
            alternatives.push_back(sorted_options.get(i).unwrap());
        }

        let best = sorted_options.get(0).unwrap();

        Ok(RoutingResult {
            selected_anchor: best.anchor.clone(),
            selected_quote: best.quote.clone(),
            score: best.score,
            alternatives,
            routing_timestamp: current_timestamp,
        })
    }

    /// Find best anchor for a specific service and asset pair.
    pub fn find_best_anchor(
        env: Env,
        base_asset: String,
        quote_asset: String,
        amount: u64,
        operation_type: ServiceType,
        strategy: RoutingStrategy,
    ) -> Result<Address, Error> {
        let request = QuoteRequest {
            base_asset,
            quote_asset,
            amount,
            operation_type,
        };

        let routing_request = RoutingRequest {
            request,
            strategy,
            max_anchors: 1,
            require_kyc: false,
            min_reputation: 0,
        };

        let result = Self::route_transaction(env, routing_request)?;
        Ok(result.selected_anchor)
    }

    /// Calculate routing score based on strategy.
    fn calculate_routing_score(
        strategy: &RoutingStrategy,
        quote: &QuoteData,
        metadata: &AnchorMetadata,
        amount: u64,
    ) -> u64 {
        match strategy {
            RoutingStrategy::BestRate => {
                // Higher rate is better (inverted for scoring)
                let effective_rate = Self::calculate_effective_rate(quote, amount);
                // Invert so lower effective rate = higher score
                if effective_rate > 0 {
                    1_000_000_000 / effective_rate
                } else {
                    0
                }
            }
            RoutingStrategy::LowestFee => {
                // Lower fee is better
                let max_fee = 10000u32; // 100%
                let fee_score = max_fee.saturating_sub(quote.fee_percentage);
                fee_score as u64 * 100_000
            }
            RoutingStrategy::FastestSettlement => {
                // Lower settlement time is better
                let max_time = 86400u64; // 24 hours
                let time_score = max_time.saturating_sub(metadata.average_settlement_time);
                time_score * 10_000
            }
            RoutingStrategy::HighestLiquidity => {
                // Higher liquidity is better
                metadata.liquidity_score as u64 * 100_000
            }
            RoutingStrategy::Custom => {
                // Weighted combination of all factors
                let rate_score = if quote.rate > 0 {
                    (1_000_000 / quote.rate) * 30 // 30% weight
                } else {
                    0
                };
                let fee_score = (10000u32.saturating_sub(quote.fee_percentage) as u64) * 25; // 25% weight
                let reputation_score = metadata.reputation_score as u64 * 20; // 20% weight
                let liquidity_score = metadata.liquidity_score as u64 * 15; // 15% weight
                let uptime_score = metadata.uptime_percentage as u64 * 10; // 10% weight

                rate_score + fee_score + reputation_score + liquidity_score + uptime_score
            }
        }
    }

    /// Deactivate an anchor (admin only).
    pub fn deactivate_anchor(env: Env, anchor: Address) -> Result<(), Error> {
        let admin = Storage::get_admin(&env)?;
        admin.require_auth();

        let mut metadata =
            Storage::get_anchor_metadata(&env, &anchor).ok_or(Error::AnchorMetadataNotFound)?;

        metadata.is_active = false;
        Storage::set_anchor_metadata(&env, &metadata);

        Ok(())
    }

    /// Reactivate an anchor (admin only).
    pub fn reactivate_anchor(env: Env, anchor: Address) -> Result<(), Error> {
        let admin = Storage::get_admin(&env)?;
        admin.require_auth();

        let mut metadata =
            Storage::get_anchor_metadata(&env, &anchor).ok_or(Error::AnchorMetadataNotFound)?;

        metadata.is_active = true;
        Storage::set_anchor_metadata(&env, &metadata);

        Ok(())
    }

    // ========== Skeleton Loader Methods ==========

    /// Get skeleton loader state for anchor information.
    pub fn get_anchor_info_skeleton(
        env: Env,
        anchor: Address,
    ) -> Result<AnchorInfoSkeleton, Error> {
        // Check if anchor exists
        if !Storage::is_attestor(&env, &anchor) {
            return Ok(AnchorInfoSkeleton::error(
                anchor,
                String::from_str(&env, "Anchor not found"),
            ));
        }

        // Check if metadata is available
        match Storage::get_anchor_metadata(&env, &anchor) {
            Some(_) => Ok(AnchorInfoSkeleton::loaded(anchor)),
            None => Ok(AnchorInfoSkeleton::loading(anchor)),
        }
    }

    /// Get skeleton loader state for transaction status.
    /// Note: This checks session operations since transaction intents are ephemeral.
    pub fn get_transaction_status_skeleton(
        env: Env,
        session_id: u64,
    ) -> Result<TransactionStatusSkeleton, Error> {
        // Check if session exists
        match Storage::get_session(&env, session_id) {
            Ok(session) => {
                // Calculate progress based on operation count
                let operation_count = Storage::get_session_operation_count(&env, session_id);
                let current_time = env.ledger().timestamp();

                // Simple progress: if operations exist, show progress
                let progress = if operation_count > 0 {
                    // Show 50% progress if operations are being processed
                    5000u32
                } else {
                    // Just started
                    1000u32
                };

                Ok(TransactionStatusSkeleton::loading_with_progress(
                    session_id, progress,
                ))
            }
            Err(_) => Ok(TransactionStatusSkeleton::error(
                session_id,
                String::from_str(&env, "Session not found"),
            )),
        }
    }

    /// Get skeleton loader state for authentication validation.
    pub fn get_auth_validation_skeleton(
        env: Env,
        attestor: Address,
    ) -> Result<AuthValidationSkeleton, Error> {
        // Check if attestor is registered
        if !Storage::is_attestor(&env, &attestor) {
            return Ok(AuthValidationSkeleton::error(
                &env,
                attestor,
                String::from_str(&env, "Attestor not registered"),
            ));
        }

        // Build validation steps
        let mut steps: Vec<ValidationStep> = Vec::new(&env);

        // Step 1: Check registration
        steps.push_back(ValidationStep::complete(String::from_str(
            &env,
            "Registration verified",
        )));

        // Step 2: Check credential policy
        let has_policy = Storage::get_credential_policy(&env, &attestor).is_some();
        if has_policy {
            steps.push_back(ValidationStep::complete(String::from_str(
                &env,
                "Credential policy verified",
            )));
        } else {
            steps.push_back(ValidationStep::new(String::from_str(
                &env,
                "Checking credential policy",
            )));
        }

        // Step 3: Check endpoint configuration
        let has_endpoint = Storage::get_endpoint(&env, &attestor).is_ok();
        if has_endpoint {
            steps.push_back(ValidationStep::complete(String::from_str(
                &env,
                "Endpoint configured",
            )));
        } else {
            steps.push_back(ValidationStep::new(String::from_str(
                &env,
                "Checking endpoint",
            )));
        }

        // Determine overall validation state
        let all_complete = has_policy && has_endpoint;
        if all_complete {
            Ok(AuthValidationSkeleton::validated(&env, attestor))
        } else {
            Ok(AuthValidationSkeleton::validating_with_steps(
                attestor, steps,
            ))
        }
    }
    // ============ Connection Pooling ============

    /// Configure connection pool. Only callable by admin.
    pub fn configure_connection_pool(
        env: Env,
        max_connections: u32,
        idle_timeout_seconds: u64,
        connection_timeout_seconds: u64,
        reuse_connections: bool,
    ) -> Result<(), Error> {
        let admin = Storage::get_admin(&env)?;
        admin.require_auth();

        let config = ConnectionPoolConfig {
            max_connections,
            idle_timeout_seconds,
            connection_timeout_seconds,
            reuse_connections,
        };

        ConnectionPool::set_config(&env, &config);
        Ok(())
    }

    /// Get connection pool configuration.
    pub fn get_pool_config(env: Env) -> ConnectionPoolConfig {
        ConnectionPool::get_config(&env)
    }

    /// Get connection pool statistics.
    pub fn get_pool_stats(env: Env) -> ConnectionStats {
        ConnectionPool::get_stats(&env)
    }

    /// Reset connection pool statistics.
    pub fn reset_pool_stats(env: Env) -> Result<(), Error> {
        let admin = Storage::get_admin(&env)?;
        admin.require_auth();

        ConnectionPool::reset_stats(&env);
        Ok(())
    }

    /// Get pooled connection for endpoint.
    pub fn get_pooled_connection(env: Env, endpoint: String) -> Result<(), Error> {
        ConnectionPool::get_connection(&env, &endpoint);
        Ok(())
    }

    // ============ Request ID & Tracing ============

    /// Generate a new request ID for tracing.
    pub fn generate_request_id(env: Env) -> RequestId {
        RequestId::generate(&env)
    }

    /// Submit attestation with request ID for tracing.
    pub fn submit_with_request_id(
        env: Env,
        request_id: RequestId,
        issuer: Address,
        subject: Address,
        timestamp: u64,
        payload_hash: BytesN<32>,
        signature: Bytes,
    ) -> Result<u64, Error> {
        issuer.require_auth();

        let started_at = env.ledger().timestamp();
        let result = Self::submit_attestation_internal(
            &env,
            &issuer,
            &subject,
            timestamp,
            &payload_hash,
            &signature,
        );
        let completed_at = env.ledger().timestamp();

        let status = if result.is_ok() {
            String::from_str(&env, "success")
        } else {
            String::from_str(&env, "failed")
        };
        let span = TracingSpan {
            request_id: request_id.clone(),
            operation: String::from_str(&env, "submit_attestation"),
            actor: issuer.clone(),
            started_at,
            completed_at,
            status,
        };
        RequestTracker::store_span(&env, &span);

        result
    }

    /// Submit quote with request ID for tracing.
    pub fn quote_with_request_id(
        env: Env,
        request_id: RequestId,
        anchor: Address,
        base_asset: String,
        quote_asset: String,
        rate: u64,
        fee_percentage: u32,
        minimum_amount: u64,
        maximum_amount: u64,
        valid_until: u64,
    ) -> Result<u64, Error> {
        anchor.require_auth();

        let started_at = env.ledger().timestamp();
        let result = Self::submit_quote(
            env.clone(),
            anchor.clone(),
            base_asset,
            quote_asset,
            rate,
            fee_percentage,
            minimum_amount,
            maximum_amount,
            valid_until,
        );
        let completed_at = env.ledger().timestamp();

        let status = if result.is_ok() {
            String::from_str(&env, "success")
        } else {
            String::from_str(&env, "failed")
        };
        let span = TracingSpan {
            request_id: request_id.clone(),
            operation: String::from_str(&env, "submit_quote"),
            actor: anchor.clone(),
            started_at,
            completed_at,
            status,
        };
        RequestTracker::store_span(&env, &span);

        result
    }

    /// Get tracing span by request ID.
    pub fn get_tracing_span(env: Env, request_id: BytesN<16>) -> Option<TracingSpan> {
        RequestTracker::get_span(&env, &request_id)
    }

    fn submit_attestation_internal(
        env: &Env,
        issuer: &Address,
        subject: &Address,
        timestamp: u64,
        payload_hash: &BytesN<32>,
        signature: &Bytes,
    ) -> Result<u64, Error> {
        if timestamp == 0 {
            return Err(Error::InvalidTimestamp);
        }

        if !Storage::is_attestor(env, issuer) {
            return Err(Error::UnauthorizedAttestor);
        }

        if Storage::is_hash_used(env, payload_hash) {
            return Err(Error::ReplayAttack);
        }

        Self::verify_signature(env, issuer, subject, timestamp, payload_hash, signature)?;

        let id = Storage::get_and_increment_counter(env);
        let attestation = Attestation {
            id,
            issuer: issuer.clone(),
            subject: subject.clone(),
            timestamp,
            payload_hash: payload_hash.clone(),
            signature: signature.clone(),
        };

        Storage::set_attestation(env, id, &attestation);
        Storage::mark_hash_used(env, payload_hash);
        AttestationRecorded::publish(env, id, subject, timestamp, payload_hash.clone());

        Ok(id)
    }

    // ============ Request History Panel ============

    /// Get request history panel data with recent API calls
    /// Returns up to `limit` recent API calls with their status and details
    pub fn get_request_history(env: Env, limit: u32) -> RequestHistoryPanel {
        RequestHistory::get_panel_data(&env, limit)
    }

    /// Get detailed information about a specific API call
    pub fn get_api_call_details(env: Env, call_id: u64) -> Option<ApiCallDetails> {
        RequestHistory::get_call_details(&env, call_id)
    }

    /// Get a specific API call record by ID
    pub fn get_api_call(env: Env, call_id: u64) -> Option<ApiCallRecord> {
        RequestHistory::get_call(&env, call_id)
    }

    /// Submit attestation with automatic request history tracking
    pub fn submit_attestation_tracked(
        env: Env,
        issuer: Address,
        subject: Address,
        timestamp: u64,
        payload_hash: BytesN<32>,
        signature: Bytes,
    ) -> Result<u64, Error> {
        issuer.require_auth();

        let request_id = RequestId::generate(&env);
        let call_id = RequestHistory::get_next_call_id(&env);
        let started_at = env.ledger().timestamp();

        let result = Self::submit_attestation_internal(
            &env,
            &issuer,
            &subject,
            timestamp,
            &payload_hash,
            &signature,
        );

        let completed_at = env.ledger().timestamp();
        let duration_ms = (completed_at.saturating_sub(started_at)) * 1000;

        let (status, error_code) = match &result {
            Ok(_) => (ApiCallStatus::Success, None),
            Err(e) => (ApiCallStatus::Failed, Some(Self::error_to_code(e))),
        };

        let mut record = ApiCallRecord::new(
            &env,
            call_id,
            request_id.id.clone(),
            String::from_str(&env, "submit_attestation"),
            issuer.clone(),
            status,
            duration_ms,
        );

        if let Some(code) = error_code {
            record = record.with_error(code);
        }

        RequestHistory::record_call(&env, &record);

        // Store detailed information
        if let Ok(attestation_id) = &result {
            let details = ApiCallDetails {
                record: record.clone(),
                target_address: Some(subject.clone()),
                amount: None,
                result_data: None, // Store ID in amount field instead
            };
            RequestHistory::store_call_details(&env, &details);
        }

        result
    }

    /// Submit quote with automatic request history tracking
    pub fn submit_quote_tracked(
        env: Env,
        anchor: Address,
        base_asset: String,
        quote_asset: String,
        rate: u64,
        fee_percentage: u32,
        minimum_amount: u64,
        maximum_amount: u64,
        valid_until: u64,
    ) -> Result<u64, Error> {
        anchor.require_auth();

        let request_id = RequestId::generate(&env);
        let call_id = RequestHistory::get_next_call_id(&env);
        let started_at = env.ledger().timestamp();

        let result = Self::submit_quote(
            env.clone(),
            anchor.clone(),
            base_asset.clone(),
            quote_asset.clone(),
            rate,
            fee_percentage,
            minimum_amount,
            maximum_amount,
            valid_until,
        );

        let completed_at = env.ledger().timestamp();
        let duration_ms = (completed_at.saturating_sub(started_at)) * 1000;

        let (status, error_code) = match &result {
            Ok(_) => (ApiCallStatus::Success, None),
            Err(e) => (ApiCallStatus::Failed, Some(Self::error_to_code(e))),
        };

        let mut record = ApiCallRecord::new(
            &env,
            call_id,
            request_id.id.clone(),
            String::from_str(&env, "submit_quote"),
            anchor.clone(),
            status,
            duration_ms,
        );

        if let Some(code) = error_code {
            record = record.with_error(code);
        }

        RequestHistory::record_call(&env, &record);

        // Store detailed information
        if let Ok(quote_id) = &result {
            let details = ApiCallDetails {
                record: record.clone(),
                target_address: Some(anchor.clone()),
                amount: Some(rate),
                result_data: None, // Store quote_id in amount field if needed
            };
            RequestHistory::store_call_details(&env, &details);
        }

        result
    }

    /// Register attestor with automatic request history tracking
    pub fn register_attestor_tracked(env: Env, attestor: Address) -> Result<(), Error> {
        let admin = Storage::get_admin(&env)?;
        admin.require_auth();

        let request_id = RequestId::generate(&env);
        let call_id = RequestHistory::get_next_call_id(&env);
        let started_at = env.ledger().timestamp();

        let result = Self::register_attestor(env.clone(), attestor.clone());

        let completed_at = env.ledger().timestamp();
        let duration_ms = (completed_at.saturating_sub(started_at)) * 1000;

        let (status, error_code) = match &result {
            Ok(_) => (ApiCallStatus::Success, None),
            Err(e) => (ApiCallStatus::Failed, Some(Self::error_to_code(e))),
        };

        let mut record = ApiCallRecord::new(
            &env,
            call_id,
            request_id.id.clone(),
            String::from_str(&env, "register_attestor"),
            admin.clone(),
            status,
            duration_ms,
        );

        if let Some(code) = error_code {
            record = record.with_error(code);
        }

        RequestHistory::record_call(&env, &record);

        // Store detailed information
        let details = ApiCallDetails {
            record: record.clone(),
            target_address: Some(attestor.clone()),
            amount: None,
            result_data: None,
        };
        RequestHistory::store_call_details(&env, &details);

        result
    }

    // ============ Interactive Support ============

    /// Generate interactive URL with embedded token
    pub fn generate_interactive_url(
        env: Env,
        anchor: Address,
        token: String,
        tx_id: String,
    ) -> InteractiveUrl {
        InteractiveSupport::generate_url(&env, &anchor, &token, &tx_id)
    }

    /// Handle callback from anchor
    pub fn handle_anchor_callback(
        env: Env,
        tx_id: String,
        status: String,
    ) -> CallbackData {
        InteractiveSupport::handle_callback(&env, &tx_id, &status)
    }

    /// Poll transaction status
    pub fn poll_transaction_status(
        env: Env,
        tx_id: String,
    ) -> TransactionStatus {
        InteractiveSupport::poll_status(&env, &tx_id)
    }

    /// Helper function to convert Error to error code
    fn error_to_code(error: &Error) -> u32 {
        match error {
            Error::AlreadyInitialized => 1,
            Error::NotInitialized => 2,
            Error::UnauthorizedAttestor => 3,
            Error::AttestorAlreadyRegistered => 4,
            Error::AttestorNotRegistered => 5,
            Error::ReplayAttack => 6,
            Error::InvalidTimestamp => 7,
            Error::AttestationNotFound => 8,
            Error::InvalidEndpointFormat => 9,
            Error::EndpointNotFound => 10,
            Error::ServicesNotConfigured => 11,
            Error::InvalidServiceType => 12,
            Error::SessionNotFound => 13,
            Error::InvalidSessionId => 14,
            Error::InvalidQuote => 15,
            Error::StaleQuote => 16,
            Error::NoQuotesAvailable => 17,
            Error::InvalidTransactionIntent => 19,
            Error::ComplianceNotMet => 20,
            Error::InvalidConfig => 21,
            Error::InvalidCredentialFormat => 22,
            Error::CredentialNotFound => 23,
            Error::CredentialExpired => 25,
            Error::InvalidAnchorMetadata => 26,
            Error::AnchorMetadataNotFound => 27,
            Error::RateLimitExceeded => 29,
            Error::AssetNotConfigured => 30,
            Error::UnsupportedAsset => 31,
            Error::TransportError => 41,
            Error::TransportTimeout => 42,
            Error::TransportUnauthorized => 43,
            Error::ProtocolError => 44,
            Error::ProtocolInvalidPayload => 45,
            Error::ProtocolRateLimitExceeded => 46,
            Error::CacheExpired => 48,
            Error::CacheNotFound => 49,
            Error::DuplicateAttestor => 26,
            Error::WebhookTimestampExpired => 53,
            Error::WebhookTimestampInFuture => 54,
            Error::WebhookPayloadTooLarge => 55,
            Error::WebhookSignatureInvalid => 56,
            Error::WebhookValidationFailed => 57,
        }
    }

    // ============ SEP-10 Authentication ============

    /// Fetch SEP-10 challenge from anchor
    pub fn sep10_fetch_challenge(
        env: Env,
        anchor: Address,
        client_account: Address,
    ) -> Result<sep10_auth::Sep10Challenge, Error> {
        if !Storage::is_attestor(&env, &anchor) {
            return Err(Error::AttestorNotRegistered);
        }
        Ok(sep10_auth::fetch_challenge(&env, anchor, client_account))
    }

    /// Verify signature on SEP-10 challenge
    pub fn sep10_verify_signature(
        env: Env,
        challenge: sep10_auth::Sep10Challenge,
        signature: BytesN<64>,
        public_key: BytesN<32>,
    ) -> bool {
        sep10_auth::verify_signature(&env, &challenge, signature, public_key)
    }

    /// Validate home domain for anchor
    pub fn sep10_validate_domain(
        env: Env,
        anchor: Address,
        home_domain: String,
    ) -> Result<bool, Error> {
        if !Storage::is_attestor(&env, &anchor) {
            return Err(Error::AttestorNotRegistered);
        }
        Ok(sep10_auth::validate_home_domain(&env, anchor, home_domain))
    }

    /// Store SEP-10 session securely
    pub fn sep10_store_session(
        env: Env,
        session: sep10_auth::Sep10Session,
    ) -> Result<(), Error> {
        if !Storage::is_attestor(&env, &session.anchor) {
            return Err(Error::AttestorNotRegistered);
        }
        sep10_auth::store_session(&env, session);
        Ok(())
    }

    /// Get stored SEP-10 session
    pub fn sep10_get_session(
        env: Env,
        anchor: Address,
    ) -> Option<sep10_auth::Sep10Session> {
        sep10_auth::get_session(&env, anchor)
    }

    /// Complete SEP-10 authentication flow
    pub fn sep10_authenticate(
        env: Env,
        anchor: Address,
        client_account: Address,
        signature: BytesN<64>,
        public_key: BytesN<32>,
        home_domain: String,
    ) -> Result<sep10_auth::Sep10Session, Error> {
        if !Storage::is_attestor(&env, &anchor) {
            return Err(Error::AttestorNotRegistered);
        }
        sep10_auth::authenticate(&env, anchor, client_account, signature, public_key, home_domain)
            .map_err(|code| match code {
                401 => Error::TransportUnauthorized,
                403 => Error::ComplianceNotMet,
                _ => Error::TransportError,
            })
    }
}
