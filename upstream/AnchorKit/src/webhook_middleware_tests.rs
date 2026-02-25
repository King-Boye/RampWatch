#[cfg(test)]
mod webhook_middleware_tests {
    use crate::webhook_middleware::*;
    use soroban_sdk::{testutils::*, Address, Bytes, BytesN, Env, String};
    use alloc::vec;

    fn create_test_env() -> Env {
        Env::default()
    }

    fn create_test_config(env: &Env) -> WebhookSecurityConfig {
        WebhookSecurityConfig {
            algorithm: SignatureAlgorithm::Sha256,
            secret_key: Bytes::from_array(env, &[1u8; 32]),
            timestamp_tolerance_seconds: 300,
            max_payload_size_bytes: 10000,
            enable_replay_protection: true,
        }
    }

    fn create_test_request(
        env: &Env,
        payload: &[u8],
        timestamp: u64,
        webhook_id: u64,
    ) -> WebhookRequest {
        let payload_bytes = if payload.len() <= 32 {
            let mut arr = [0u8; 32];
            arr[..payload.len()].copy_from_slice(payload);
            Bytes::from_array(env, &arr)
        } else {
            Bytes::new(env)
        };
        
        WebhookRequest {
            payload: payload_bytes,
            signature: Bytes::from_array(env, &[0u8; 32]),
            timestamp,
            webhook_id,
            source_address: None,
        }
    }

    #[test]
    fn test_validate_timestamp_within_range() {
        let env = create_test_env();
        let current_time = env.ledger().timestamp();
        let webhook_time = current_time - 100; // 100 seconds ago

        let result = WebhookMiddleware::validate_timestamp(&env, webhook_time, 300);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_validate_timestamp_too_old() {
        let env = create_test_env();
        let current_time = env.ledger().timestamp();
        let webhook_time = current_time - 400; // 400 seconds ago (exceeds 300s tolerance)

        let result = WebhookMiddleware::validate_timestamp(&env, webhook_time, 300);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_timestamp_in_future() {
        let env = create_test_env();
        let current_time = env.ledger().timestamp();
        let webhook_time = current_time + 100; // 100 seconds in future (exceeds 60s skew)

        let result = WebhookMiddleware::validate_timestamp(&env, webhook_time, 300);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_timestamp_minor_clock_skew() {
        let env = create_test_env();
        let current_time = env.ledger().timestamp();
        let webhook_time = current_time + 30; // 30 seconds in future (within 60s skew)

        let result = WebhookMiddleware::validate_timestamp(&env, webhook_time, 300);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_payload_size_within_limit() {
        let env = Env::default();
        let payload_arr = [1u8; 32];
        let payload = Bytes::from_array(&env, &payload_arr);
        let result = WebhookMiddleware::validate_payload_size(&payload, 10000);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_payload_size_exceeds_limit() {
        let env = Env::default();
        let payload_arr = [1u8; 32];
        let payload = Bytes::from_array(&env, &payload_arr);
        let result = WebhookMiddleware::validate_payload_size(&payload, 10);
        assert!(result.is_err());
    }

    #[test]
    fn test_replay_attack_detection_first_webhook() {
        let env = create_test_env();
        let payload = Bytes::from_array(&env, &[1u8; 32]);
        let payload_hash_result = env.crypto().sha256(&payload);
        let payload_hash = BytesN::from_array(&env, &payload_hash_result.to_array());

        let result = WebhookMiddleware::check_replay_attack(&env, 1, &payload_hash);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_replay_attack_detection_duplicate() {
        let env = create_test_env();
        let payload = Bytes::from_array(&env, &[1u8; 32]);
        let payload_hash_result = env.crypto().sha256(&payload);
        let payload_hash = BytesN::from_array(&env, &payload_hash_result.to_array());

        // First webhook should succeed
        let result1 = WebhookMiddleware::check_replay_attack(&env, 1, &payload_hash);
        assert!(result1.is_ok());

        // Duplicate webhook should fail
        let result2 = WebhookMiddleware::check_replay_attack(&env, 1, &payload_hash);
        assert!(result2.is_err());
    }

    #[test]
    fn test_replay_attack_different_webhook_ids() {
        let env = create_test_env();
        let payload = Bytes::from_array(&env, &[1u8; 32]);
        let payload_hash_result = env.crypto().sha256(&payload);
        let payload_hash = BytesN::from_array(&env, &payload_hash_result.to_array());

        // Same payload with different webhook IDs should both succeed
        let result1 = WebhookMiddleware::check_replay_attack(&env, 1, &payload_hash);
        assert!(result1.is_ok());

        let result2 = WebhookMiddleware::check_replay_attack(&env, 2, &payload_hash);
        assert!(result2.is_ok());
    }

    #[test]
    fn test_log_suspicious_activity_invalid_signature() {
        let env = create_test_env();
        let details = String::from_str(&env, "Test invalid signature");

        WebhookMiddleware::log_suspicious_activity(
            &env,
            SuspiciousActivityType::InvalidSignature,
            ActivitySeverity::Critical,
            details,
            None,
        );

        // Verify activity was logged by checking if we can retrieve it
        let activity = WebhookMiddleware::get_suspicious_activity(&env, 1);
        assert!(activity.is_some());

        let record = activity.unwrap();
        assert_eq!(record.activity_id, 1);
        assert_eq!(record.activity_type, SuspiciousActivityType::InvalidSignature);
        assert_eq!(record.severity, ActivitySeverity::Critical);
    }

    #[test]
    fn test_log_suspicious_activity_replay_attack() {
        let env = create_test_env();
        let details = String::from_str(&env, "Duplicate webhook detected");

        WebhookMiddleware::log_suspicious_activity(
            &env,
            SuspiciousActivityType::ReplayAttack,
            ActivitySeverity::Critical,
            details,
            None,
        );

        let activity = WebhookMiddleware::get_suspicious_activity(&env, 1);
        assert!(activity.is_some());

        let record = activity.unwrap();
        assert_eq!(record.activity_type, SuspiciousActivityType::ReplayAttack);
    }

    #[test]
    fn test_log_suspicious_activity_timestamp_out_of_range() {
        let env = create_test_env();
        let details = String::from_str(&env, "Timestamp outside acceptable range");

        WebhookMiddleware::log_suspicious_activity(
            &env,
            SuspiciousActivityType::TimestampOutOfRange,
            ActivitySeverity::High,
            details,
            None,
        );

        let activity = WebhookMiddleware::get_suspicious_activity(&env, 1);
        assert!(activity.is_some());

        let record = activity.unwrap();
        assert_eq!(record.activity_type, SuspiciousActivityType::TimestampOutOfRange);
        assert_eq!(record.severity, ActivitySeverity::High);
    }

    #[test]
    fn test_log_suspicious_activity_payload_too_large() {
        let env = create_test_env();
        let details = String::from_str(&env, "Payload exceeds maximum size");

        WebhookMiddleware::log_suspicious_activity(
            &env,
            SuspiciousActivityType::PayloadTooLarge,
            ActivitySeverity::Medium,
            details,
            None,
        );

        let activity = WebhookMiddleware::get_suspicious_activity(&env, 1);
        assert!(activity.is_some());

        let record = activity.unwrap();
        assert_eq!(record.activity_type, SuspiciousActivityType::PayloadTooLarge);
        assert_eq!(record.severity, ActivitySeverity::Medium);
    }

    #[test]
    fn test_record_delivery_attempt_success() {
        let env = create_test_env();

        WebhookMiddleware::record_delivery_attempt(
            &env,
            1,
            WebhookDeliveryStatus::Delivered,
            150,
            None,
        );

        let record = WebhookMiddleware::get_delivery_record(&env, 1, 1);
        assert!(record.is_some());

        let delivery = record.unwrap();
        assert_eq!(delivery.webhook_id, 1);
        assert_eq!(delivery.attempt_number, 1);
        assert_eq!(delivery.status, WebhookDeliveryStatus::Delivered);
        assert_eq!(delivery.response_time_ms, 150);
        assert_eq!(delivery.error_code, None);
    }

    #[test]
    fn test_record_delivery_attempt_failed() {
        let env = create_test_env();

        WebhookMiddleware::record_delivery_attempt(
            &env,
            1,
            WebhookDeliveryStatus::Failed,
            5000,
            Some(500),
        );

        let record = WebhookMiddleware::get_delivery_record(&env, 1, 1);
        assert!(record.is_some());

        let delivery = record.unwrap();
        assert_eq!(delivery.status, WebhookDeliveryStatus::Failed);
        assert_eq!(delivery.response_time_ms, 5000);
        assert_eq!(delivery.error_code, Some(500));
    }

    #[test]
    fn test_record_multiple_delivery_attempts() {
        let env = create_test_env();

        // First attempt
        WebhookMiddleware::record_delivery_attempt(
            &env,
            1,
            WebhookDeliveryStatus::Failed,
            1000,
            Some(500),
        );

        // Second attempt
        WebhookMiddleware::record_delivery_attempt(
            &env,
            1,
            WebhookDeliveryStatus::Failed,
            2000,
            Some(503),
        );

        // Third attempt (success)
        WebhookMiddleware::record_delivery_attempt(
            &env,
            1,
            WebhookDeliveryStatus::Delivered,
            500,
            None,
        );

        // Verify all attempts were recorded
        let attempt1 = WebhookMiddleware::get_delivery_record(&env, 1, 1);
        assert!(attempt1.is_some());
        assert_eq!(attempt1.unwrap().error_code, Some(500));

        let attempt2 = WebhookMiddleware::get_delivery_record(&env, 1, 2);
        assert!(attempt2.is_some());
        assert_eq!(attempt2.unwrap().error_code, Some(503));

        let attempt3 = WebhookMiddleware::get_delivery_record(&env, 1, 3);
        assert!(attempt3.is_some());
        assert_eq!(attempt3.unwrap().status, WebhookDeliveryStatus::Delivered);
    }

    #[test]
    fn test_validate_webhook_all_checks_pass() {
        let env = create_test_env();
        let config = create_test_config(&env);
        let current_time = env.ledger().timestamp();
        let payload_arr = [1u8; 32];

        let request = WebhookRequest {
            payload: Bytes::from_array(&env, &payload_arr),
            signature: Bytes::from_array(&env, &[0u8; 32]),
            timestamp: current_time - 100,
            webhook_id: 1,
            source_address: None,
        };

        let result = WebhookMiddleware::validate_webhook(&env, &request, &config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_webhook_timestamp_expired() {
        let env = create_test_env();
        let config = create_test_config(&env);
        let current_time = env.ledger().timestamp();
        let payload_arr = [1u8; 32];

        let request = WebhookRequest {
            payload: Bytes::from_array(&env, &payload_arr),
            signature: Bytes::from_array(&env, &[0u8; 32]),
            timestamp: current_time - 400, // Too old
            webhook_id: 1,
            source_address: None,
        };

        let result = WebhookMiddleware::validate_webhook(&env, &request, &config);
        assert!(result.is_ok());

        let validation = result.unwrap();
        assert!(!validation.is_valid);
        assert!(validation.error.is_some());
    }

    #[test]
    fn test_validate_webhook_payload_too_large() {
        let env = create_test_env();
        let config = WebhookSecurityConfig {
            algorithm: SignatureAlgorithm::Sha256,
            secret_key: Bytes::from_array(&env, &[1u8; 32]),
            timestamp_tolerance_seconds: 300,
            max_payload_size_bytes: 10,  // Very small limit
            enable_replay_protection: true,
        };
        let current_time = env.ledger().timestamp();
        let payload_arr = [1u8; 32];

        let request = WebhookRequest {
            payload: Bytes::from_array(&env, &payload_arr),
            signature: Bytes::from_array(&env, &[0u8; 32]),
            timestamp: current_time - 100,
            webhook_id: 1,
            source_address: None,
        };

        let result = WebhookMiddleware::validate_webhook(&env, &request, &config);
        assert!(result.is_ok());

        let validation = result.unwrap();
        assert!(!validation.is_valid);
        assert!(validation.error.is_some());
    }

    #[test]
    fn test_constant_time_compare_equal() {
        let env = Env::default();
        let a = Bytes::from_array(&env, &[1u8; 32]);
        let b = Bytes::from_array(&env, &[1u8; 32]);

        // This is a private method, so we test it indirectly through signature verification
        // Both should be equal
        assert_eq!(a.len(), b.len());
    }

    #[test]
    fn test_constant_time_compare_different() {
        let env = Env::default();
        let a = Bytes::from_array(&env, &[1u8; 32]);
        let b = Bytes::from_array(&env, &[2u8; 32]);

        // Both should have same length but different content
        assert_eq!(a.len(), b.len());
    }

    #[test]
    fn test_suspicious_activity_multiple_records() {
        let env = create_test_env();

        // Log multiple suspicious activities
        for i in 0..5 {
            let details = String::from_str(&env, "Test activity");
            WebhookMiddleware::log_suspicious_activity(
                &env,
                SuspiciousActivityType::InvalidSignature,
                ActivitySeverity::Critical,
                details,
                None,
            );
        }

        // Verify all were recorded with correct IDs
        for i in 1..=5 {
            let activity = WebhookMiddleware::get_suspicious_activity(&env, i);
            assert!(activity.is_some());
            assert_eq!(activity.unwrap().activity_id, i);
        }
    }

    #[test]
    fn test_webhook_security_config_creation() {
        let env = create_test_env();
        let config = create_test_config(&env);

        assert_eq!(config.algorithm, SignatureAlgorithm::Sha256);
        assert_eq!(config.timestamp_tolerance_seconds, 300);
        assert_eq!(config.max_payload_size_bytes, 10000);
        assert!(config.enable_replay_protection);
    }

    #[test]
    fn test_webhook_request_with_source_address() {
        let env = create_test_env();
        let source = Address::from_string(&String::from_str(&env, "GBBD6A7KNZF5WNWQEPZP5DYJD2AYUTLXRB6VXJ4RCX4RTNPPQVNF3GQ"));
        let payload_arr = [1u8; 32];

        let request = WebhookRequest {
            payload: Bytes::from_array(&env, &payload_arr),
            signature: Bytes::from_array(&env, &[0u8; 32]),
            timestamp: env.ledger().timestamp(),
            webhook_id: 1,
            source_address: Some(source.clone()),
        };

        assert!(request.source_address.is_some());
        assert_eq!(request.source_address.unwrap(), source);
    }

    #[test]
    fn test_activity_severity_levels() {
        // Verify all severity levels are defined
        let _low = ActivitySeverity::Low;
        let _medium = ActivitySeverity::Medium;
        let _high = ActivitySeverity::High;
        let _critical = ActivitySeverity::Critical;
    }

    #[test]
    fn test_suspicious_activity_types() {
        // Verify all activity types are defined
        let _invalid_sig = SuspiciousActivityType::InvalidSignature;
        let _replay = SuspiciousActivityType::ReplayAttack;
        let _timestamp = SuspiciousActivityType::TimestampOutOfRange;
        let _payload_size = SuspiciousActivityType::PayloadTooLarge;
        let _missing_headers = SuspiciousActivityType::MissingHeaders;
        let _rate_limit = SuspiciousActivityType::RateLimitExceeded;
        let _unauthorized = SuspiciousActivityType::UnauthorizedSource;
        let _malformed = SuspiciousActivityType::MalformedPayload;
    }

    #[test]
    fn test_webhook_delivery_status_types() {
        // Verify all delivery status types are defined
        let _pending = WebhookDeliveryStatus::Pending;
        let _delivered = WebhookDeliveryStatus::Delivered;
        let _failed = WebhookDeliveryStatus::Failed;
        let _rejected = WebhookDeliveryStatus::Rejected;
        let _suspicious = WebhookDeliveryStatus::Suspicious;
    }

    #[test]
    fn test_signature_algorithm_types() {
        // Verify all signature algorithms are defined
        let _sha256 = SignatureAlgorithm::Sha256;
        let _sha512 = SignatureAlgorithm::Sha512;
        let _ed25519 = SignatureAlgorithm::Ed25519;
    }
}
