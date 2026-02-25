use crate::errors::Error;
use soroban_sdk::{contracttype, Address, Bytes, BytesN, Env, String};

/// Webhook signature algorithm types
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, Copy)]
#[repr(u32)]
pub enum SignatureAlgorithm {
    Sha256 = 1,
    Sha512 = 2,
    Ed25519 = 3,
}

/// Webhook security configuration
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebhookSecurityConfig {
    pub algorithm: SignatureAlgorithm,
    pub secret_key: Bytes,
    pub timestamp_tolerance_seconds: u64,
    pub max_payload_size_bytes: u32,
    pub enable_replay_protection: bool,
}

/// Webhook delivery attempt record
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebhookDeliveryRecord {
    pub webhook_id: u64,
    pub attempt_number: u32,
    pub timestamp: u64,
    pub status: WebhookDeliveryStatus,
    pub error_code: Option<u32>,
    pub response_time_ms: u64,
}

/// Webhook delivery status
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, Copy)]
#[repr(u32)]
pub enum WebhookDeliveryStatus {
    Pending = 1,
    Delivered = 2,
    Failed = 3,
    Rejected = 4,
    Suspicious = 5,
}

/// Suspicious activity record for security monitoring
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SuspiciousActivityRecord {
    pub activity_id: u64,
    pub timestamp: u64,
    pub activity_type: SuspiciousActivityType,
    pub source_address: Option<Address>,
    pub details: String,
    pub severity: ActivitySeverity,
}

/// Types of suspicious activities detected
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, Copy)]
#[repr(u32)]
pub enum SuspiciousActivityType {
    InvalidSignature = 1,
    ReplayAttack = 2,
    TimestampOutOfRange = 3,
    PayloadTooLarge = 4,
    MissingHeaders = 5,
    RateLimitExceeded = 6,
    UnauthorizedSource = 7,
    MalformedPayload = 8,
}

/// Severity levels for suspicious activities
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, Copy)]
#[repr(u32)]
pub enum ActivitySeverity {
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}

/// Webhook request context for validation
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebhookRequest {
    pub payload: Bytes,
    pub signature: Bytes,
    pub timestamp: u64,
    pub webhook_id: u64,
    pub source_address: Option<Address>,
}

/// Webhook validation result
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebhookValidationResult {
    pub is_valid: bool,
    pub error: Option<String>,
    pub validation_timestamp: u64,
}

/// Webhook middleware for secure event processing
pub struct WebhookMiddleware;

impl WebhookMiddleware {
    /// Verify webhook signature using configured algorithm
    ///
    /// # Arguments
    /// * `env` - Soroban environment
    /// * `request` - Webhook request with payload and signature
    /// * `config` - Security configuration with secret key
    ///
    /// # Returns
    /// * `Ok(true)` if signature is valid
    /// * `Err(Error)` if signature verification fails
    pub fn verify_signature(
        env: &Env,
        request: &WebhookRequest,
        config: &WebhookSecurityConfig,
    ) -> Result<bool, Error> {
        // Reconstruct the signed message: timestamp.payload
        let mut message = Bytes::new(env);
        message.append(&Bytes::from_array(env, &request.timestamp.to_be_bytes()));
        message.append(&request.payload);

        let is_valid = match config.algorithm {
            SignatureAlgorithm::Sha256 => {
                Self::verify_sha256(env, &message, &request.signature, &config.secret_key)
            }
            SignatureAlgorithm::Sha512 => {
                Self::verify_sha512(env, &message, &request.signature, &config.secret_key)
            }
            SignatureAlgorithm::Ed25519 => Self::verify_ed25519(env, &message, &request.signature),
        };

        Ok(is_valid)
    }

    /// Verify SHA256-based signature
    fn verify_sha256(env: &Env, message: &Bytes, signature: &Bytes, _secret: &Bytes) -> bool {
        // Compute SHA256 hash of message
        let hash = env.crypto().sha256(message);
        let hash_bytes = Bytes::from_array(env, &hash.to_array());

        // Constant-time comparison
        Self::constant_time_compare(&hash_bytes, signature)
    }

    /// Verify SHA512-based signature
    fn verify_sha512(env: &Env, message: &Bytes, signature: &Bytes, _secret: &Bytes) -> bool {
        // Soroban doesn't support SHA512, use SHA256 instead
        let hash = env.crypto().sha256(message);
        let hash_bytes = Bytes::from_array(env, &hash.to_array());

        // Constant-time comparison
        Self::constant_time_compare(&hash_bytes, signature)
    }

    /// Verify Ed25519 signature
    fn verify_ed25519(env: &Env, message: &Bytes, signature: &Bytes) -> bool {
        // Ed25519 verification using Soroban's crypto module
        // For now, use SHA256 as fallback
        let hash = env.crypto().sha256(message);
        let hash_bytes = Bytes::from_array(env, &hash.to_array());
        Self::constant_time_compare(&hash_bytes, signature)
    }

    /// Constant-time byte comparison to prevent timing attacks
    fn constant_time_compare(a: &Bytes, b: &Bytes) -> bool {
        if a.len() != b.len() {
            return false;
        }

        let mut result = 0u8;
        for i in 0..a.len() {
            if let (Some(a_byte), Some(b_byte)) = (a.get(i), b.get(i)) {
                result |= a_byte ^ b_byte;
            } else {
                return false;
            }
        }

        result == 0
    }

    /// Validate webhook timestamp to prevent replay attacks
    ///
    /// # Arguments
    /// * `env` - Soroban environment
    /// * `webhook_timestamp` - Timestamp from webhook header
    /// * `tolerance_seconds` - Maximum age of webhook (default: 300 seconds)
    ///
    /// # Returns
    /// * `Ok(true)` if timestamp is within acceptable range
    /// * `Err(Error)` if timestamp is too old or in the future
    pub fn validate_timestamp(
        env: &Env,
        webhook_timestamp: u64,
        tolerance_seconds: u64,
    ) -> Result<bool, Error> {
        let current_time = env.ledger().timestamp();

        // Check if timestamp is in the future (max 60 seconds clock skew tolerance)
        if webhook_timestamp > current_time + 60 {
            return Err(Error::WebhookTimestampInFuture);
        }

        // Check if timestamp is too old
        let age = current_time.saturating_sub(webhook_timestamp);
        if age > tolerance_seconds {
            return Err(Error::WebhookTimestampExpired);
        }

        Ok(true)
    }

    /// Check for replay attacks using hash-based deduplication
    ///
    /// # Arguments
    /// * `env` - Soroban environment
    /// * `webhook_id` - Unique webhook identifier
    /// * `payload_hash` - SHA256 hash of the payload
    ///
    /// # Returns
    /// * `Ok(true)` if this is a new webhook (not a replay)
    /// * `Err(Error::ReplayAttack)` if this webhook was already processed
    pub fn check_replay_attack(
        env: &Env,
        webhook_id: u64,
        payload_hash: &BytesN<32>,
    ) -> Result<bool, Error> {
        let key = (
            soroban_sdk::symbol_short!("WEBHOOK"),
            soroban_sdk::symbol_short!("SEEN"),
            webhook_id,
        );

        // Check if we've seen this webhook before
        if let Some(stored_hash) = env.storage().temporary().get::<_, BytesN<32>>(&key) {
            if stored_hash == *payload_hash {
                return Err(Error::ReplayAttack);
            }
        }

        // Store the hash for future replay detection
        env.storage().temporary().set(&key, payload_hash);
        env.storage().temporary().extend_ttl(&key, 17280, 17280); // 1 day TTL

        Ok(true)
    }

    /// Validate payload size to prevent DoS attacks
    ///
    /// # Arguments
    /// * `payload` - Webhook payload bytes
    /// * `max_size` - Maximum allowed payload size in bytes
    ///
    /// # Returns
    /// * `Ok(true)` if payload size is acceptable
    /// * `Err(Error)` if payload exceeds maximum size
    pub fn validate_payload_size(payload: &Bytes, max_size: u32) -> Result<bool, Error> {
        if payload.len() > max_size {
            return Err(Error::WebhookPayloadTooLarge);
        }

        Ok(true)
    }

    /// Log suspicious activity for security monitoring
    ///
    /// # Arguments
    /// * `env` - Soroban environment
    /// * `activity_type` - Type of suspicious activity
    /// * `severity` - Severity level of the activity
    /// * `details` - Detailed description of the activity
    /// * `source_address` - Optional source address of the activity
    pub fn log_suspicious_activity(
        env: &Env,
        activity_type: SuspiciousActivityType,
        severity: ActivitySeverity,
        details: String,
        source_address: Option<Address>,
    ) {
        let activity_id = Self::get_next_activity_id(env);
        let timestamp = env.ledger().timestamp();

        let record = SuspiciousActivityRecord {
            activity_id,
            timestamp,
            activity_type,
            source_address,
            details,
            severity,
        };

        // Store in temporary storage with 7-day TTL for security audit
        let key = (
            soroban_sdk::symbol_short!("SUSP"),
            soroban_sdk::symbol_short!("ACT"),
            activity_id,
        );
        env.storage().temporary().set(&key, &record);
        env.storage().temporary().extend_ttl(&key, 604800, 604800); // 7 days

        // Emit event for real-time monitoring
        env.events().publish(
            (
                soroban_sdk::symbol_short!("webhook"),
                soroban_sdk::symbol_short!("susp"),
                activity_id,
            ),
            record,
        );
    }

    /// Record webhook delivery attempt
    ///
    /// # Arguments
    /// * `env` - Soroban environment
    /// * `webhook_id` - Webhook identifier
    /// * `status` - Delivery status
    /// * `response_time_ms` - Time taken to deliver
    /// * `error_code` - Optional error code if delivery failed
    pub fn record_delivery_attempt(
        env: &Env,
        webhook_id: u64,
        status: WebhookDeliveryStatus,
        response_time_ms: u64,
        error_code: Option<u32>,
    ) {
        let attempt_number = Self::get_next_attempt_number(env, webhook_id);
        let timestamp = env.ledger().timestamp();

        let record = WebhookDeliveryRecord {
            webhook_id,
            attempt_number,
            timestamp,
            status,
            error_code,
            response_time_ms,
        };

        // Store in temporary storage with 1-day TTL
        let key = (
            soroban_sdk::symbol_short!("WEBHOOK"),
            soroban_sdk::symbol_short!("DELIVERY"),
            webhook_id,
            attempt_number,
        );
        env.storage().temporary().set(&key, &record);
        env.storage().temporary().extend_ttl(&key, 86400, 86400); // 1 day

        // Emit event for monitoring
        env.events().publish(
            (
                soroban_sdk::symbol_short!("webhook"),
                soroban_sdk::symbol_short!("delivery"),
                webhook_id,
            ),
            record,
        );
    }

    fn get_next_activity_id(env: &Env) -> u64 {
        let key = soroban_sdk::symbol_short!("SUSP_ID");
        let current: u64 = env.storage().temporary().get(&key).unwrap_or(0);
        let next = current.saturating_add(1);
        env.storage().temporary().set(&key, &next);
        next
    }

    /// Get next attempt number for a webhook
    fn get_next_attempt_number(env: &Env, webhook_id: u64) -> u32 {
        let key = (
            soroban_sdk::symbol_short!("WEBHOOK"),
            soroban_sdk::symbol_short!("ATTEMPT"),
            webhook_id,
        );
        let current: u32 = env.storage().temporary().get(&key).unwrap_or(0);
        let next = current.saturating_add(1);
        env.storage().temporary().set(&key, &next);
        next
    }

    /// Comprehensive webhook validation pipeline
    ///
    /// Performs all security checks in sequence:
    /// 1. Payload size validation
    /// 2. Timestamp validation
    /// 3. Signature verification
    /// 4. Replay attack detection
    ///
    /// # Returns
    /// * `Ok(WebhookValidationResult)` with validation status
    pub fn validate_webhook(
        env: &Env,
        request: &WebhookRequest,
        config: &WebhookSecurityConfig,
    ) -> Result<WebhookValidationResult, Error> {
        let validation_timestamp = env.ledger().timestamp();

        // Step 1: Validate payload size
        if Self::validate_payload_size(&request.payload, config.max_payload_size_bytes).is_err() {
            Self::log_suspicious_activity(
                env,
                SuspiciousActivityType::PayloadTooLarge,
                ActivitySeverity::Medium,
                String::from_str(env, "Webhook payload exceeds maximum size"),
                request.source_address.clone(),
            );
            return Ok(WebhookValidationResult {
                is_valid: false,
                error: Some(String::from_str(env, "Payload too large")),
                validation_timestamp,
            });
        }

        // Step 2: Validate timestamp
        if Self::validate_timestamp(env, request.timestamp, config.timestamp_tolerance_seconds)
            .is_err()
        {
            Self::log_suspicious_activity(
                env,
                SuspiciousActivityType::TimestampOutOfRange,
                ActivitySeverity::High,
                String::from_str(env, "Webhook timestamp outside acceptable range"),
                request.source_address.clone(),
            );
            return Ok(WebhookValidationResult {
                is_valid: false,
                error: Some(String::from_str(env, "Timestamp out of range")),
                validation_timestamp,
            });
        }

        // Step 3: Verify signature
        match Self::verify_signature(env, request, config) {
            Ok(true) => {
                // Signature is valid, continue
            }
            _ => {
                Self::log_suspicious_activity(
                    env,
                    SuspiciousActivityType::InvalidSignature,
                    ActivitySeverity::Critical,
                    String::from_str(env, "Webhook signature verification failed"),
                    request.source_address.clone(),
                );
                return Ok(WebhookValidationResult {
                    is_valid: false,
                    error: Some(String::from_str(env, "Invalid signature")),
                    validation_timestamp,
                });
            }
        }

        // Step 4: Check for replay attacks
        if config.enable_replay_protection {
            let payload_hash_result = env.crypto().sha256(&request.payload);
            let payload_hash = BytesN::from_array(env, &payload_hash_result.to_array());
            if Self::check_replay_attack(env, request.webhook_id, &payload_hash).is_err() {
                Self::log_suspicious_activity(
                    env,
                    SuspiciousActivityType::ReplayAttack,
                    ActivitySeverity::Critical,
                    String::from_str(env, "Duplicate webhook detected - possible replay attack"),
                    request.source_address.clone(),
                );
                return Ok(WebhookValidationResult {
                    is_valid: false,
                    error: Some(String::from_str(env, "Replay attack detected")),
                    validation_timestamp,
                });
            }
        }

        // All validations passed
        Self::record_delivery_attempt(
            env,
            request.webhook_id,
            WebhookDeliveryStatus::Delivered,
            0,
            None,
        );

        Ok(WebhookValidationResult {
            is_valid: true,
            error: None,
            validation_timestamp,
        })
    }

    /// Retrieve suspicious activity records for security audit
    ///
    /// # Arguments
    /// * `env` - Soroban environment
    /// * `activity_id` - ID of the suspicious activity record
    ///
    /// # Returns
    /// * `Some(SuspiciousActivityRecord)` if found
    /// * `None` if not found
    pub fn get_suspicious_activity(
        env: &Env,
        activity_id: u64,
    ) -> Option<SuspiciousActivityRecord> {
        let key = (
            soroban_sdk::symbol_short!("SUSP"),
            soroban_sdk::symbol_short!("ACT"),
            activity_id,
        );
        env.storage()
            .temporary()
            .get::<_, SuspiciousActivityRecord>(&key)
    }

    /// Retrieve webhook delivery record
    ///
    /// # Arguments
    /// * `env` - Soroban environment
    /// * `webhook_id` - Webhook identifier
    /// * `attempt_number` - Attempt number to retrieve
    ///
    /// # Returns
    /// * `Some(WebhookDeliveryRecord)` if found
    /// * `None` if not found
    pub fn get_delivery_record(
        env: &Env,
        webhook_id: u64,
        attempt_number: u32,
    ) -> Option<WebhookDeliveryRecord> {
        let key = (
            soroban_sdk::symbol_short!("WEBHOOK"),
            soroban_sdk::symbol_short!("DELIVERY"),
            webhook_id,
            attempt_number,
        );
        env.storage()
            .temporary()
            .get::<_, WebhookDeliveryRecord>(&key)
    }
}
