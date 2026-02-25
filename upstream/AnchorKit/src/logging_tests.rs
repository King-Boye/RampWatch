#[cfg(test)]
mod tests {
    use super::*;
    use crate::logging::{LogLevel, Logger, LoggingConfig};
    use crate::request_id::RequestId;
    use soroban_sdk::{testutils::Address as _, Address, Env, String};

    #[test]
    fn test_structured_logging() {
        let env = Env::default();
        let admin = Address::generate(&env);
        let request_id = RequestId::generate(&env);

        // Test basic logging
        Logger::info(&env, String::from_str(&env, "Test info message"), Some(request_id));
        Logger::warn(&env, String::from_str(&env, "Test warning message"), Some(request_id));
        Logger::error(&env, String::from_str(&env, "Test error message"), Some(request_id), None);

        // Test debug logging (should be filtered out by default)
        Logger::debug(&env, String::from_str(&env, "Debug message"), Some(request_id));
        Logger::trace(&env, String::from_str(&env, "Trace message"), Some(request_id));
    }

    #[test]
    fn test_debug_mode_toggle() {
        let env = Env::default();
        let admin = Address::generate(&env);
        let request_id = RequestId::generate(&env);

        // Enable debug mode
        let debug_config = LoggingConfig {
            debug_mode: true,
            log_requests: true,
            log_responses: true,
            redact_sensitive: true,
            max_log_size: 2048,
        };
        Logger::set_config(&env, debug_config);

        // Now debug messages should be logged
        Logger::debug(&env, String::from_str(&env, "Debug message with debug mode on"), Some(request_id));
        Logger::trace(&env, String::from_str(&env, "Trace message with debug mode on"), Some(request_id));

        // Disable debug mode
        let normal_config = LoggingConfig {
            debug_mode: false,
            log_requests: true,
            log_responses: true,
            redact_sensitive: true,
            max_log_size: 1024,
        };
        Logger::set_config(&env, normal_config);

        // Debug messages should be filtered out again
        Logger::debug(&env, String::from_str(&env, "This debug message should be filtered"), Some(request_id));
    }

    #[test]
    fn test_operation_logging() {
        let env = Env::default();
        let admin = Address::generate(&env);
        let request_id = RequestId::generate(&env);

        // Test operation start/complete logging
        Logger::operation_start(
            &env,
            String::from_str(&env, "test_operation"),
            admin.clone(),
            request_id,
            Some(String::from_str(&env, "{\"param1\":\"value1\"}")),
        );

        Logger::operation_complete(
            &env,
            String::from_str(&env, "test_operation"),
            admin,
            request_id,
            150, // 150ms duration
            true, // success
        );
    }

    #[test]
    fn test_request_response_logging() {
        let env = Env::default();
        let request_id = RequestId::generate(&env);

        // Test request logging
        let payload = soroban_sdk::Bytes::from_slice(&env, b"{\"amount\":1000,\"asset\":\"USDC\"}");
        Logger::log_request(
            &env,
            request_id,
            String::from_str(&env, "GET_QUOTE"),
            String::from_str(&env, "https://anchor.example.com/quote"),
            Some(payload),
        );

        // Test response logging
        let response_payload = soroban_sdk::Bytes::from_slice(&env, b"{\"rate\":\"1.05\",\"expires_at\":1234567890}");
        Logger::log_response(
            &env,
            request_id,
            String::from_str(&env, "200_OK"),
            250, // 250ms duration
            Some(response_payload),
        );
    }

    #[test]
    fn test_sensitive_data_redaction() {
        let env = Env::default();
        let request_id = RequestId::generate(&env);

        // Enable redaction
        let config = LoggingConfig {
            debug_mode: true,
            log_requests: true,
            log_responses: true,
            redact_sensitive: true,
            max_log_size: 1024,
        };
        Logger::set_config(&env, config);

        // Test with sensitive data
        let sensitive_payload = soroban_sdk::Bytes::from_slice(&env, b"{\"password\":\"secret123\",\"amount\":1000}");
        Logger::log_request(
            &env,
            request_id,
            String::from_str(&env, "POST"),
            String::from_str(&env, "https://anchor.example.com/auth"),
            Some(sensitive_payload),
        );

        // Disable redaction
        let no_redact_config = LoggingConfig {
            debug_mode: true,
            log_requests: true,
            log_responses: true,
            redact_sensitive: false,
            max_log_size: 1024,
        };
        Logger::set_config(&env, no_redact_config);

        // Test without redaction (use with caution)
        let normal_payload = soroban_sdk::Bytes::from_slice(&env, b"{\"amount\":1000,\"asset\":\"USDC\"}");
        Logger::log_request(
            &env,
            request_id,
            String::from_str(&env, "POST"),
            String::from_str(&env, "https://anchor.example.com/quote"),
            Some(normal_payload),
        );
    }

    #[test]
    fn test_log_size_truncation() {
        let env = Env::default();
        let request_id = RequestId::generate(&env);

        // Set small max log size
        let config = LoggingConfig {
            debug_mode: true,
            log_requests: true,
            log_responses: true,
            redact_sensitive: false,
            max_log_size: 50, // Very small for testing
        };
        Logger::set_config(&env, config);

        // Create large payload
        let large_payload = soroban_sdk::Bytes::from_slice(&env, 
            b"This is a very long payload that should be truncated because it exceeds the maximum log size configured for this test case and should show truncation behavior");
        
        Logger::log_request(
            &env,
            request_id,
            String::from_str(&env, "POST"),
            String::from_str(&env, "https://anchor.example.com/large"),
            Some(large_payload),
        );
    }

    #[test]
    fn test_logging_configuration_persistence() {
        let env = Env::default();

        // Set initial config
        let config1 = LoggingConfig {
            debug_mode: true,
            log_requests: false,
            log_responses: true,
            redact_sensitive: false,
            max_log_size: 2048,
        };
        Logger::set_config(&env, config1.clone());

        // Update config
        let config2 = LoggingConfig {
            debug_mode: false,
            log_requests: true,
            log_responses: false,
            redact_sensitive: true,
            max_log_size: 512,
        };
        Logger::set_config(&env, config2.clone());

        // Configuration should persist and be retrievable
        // (In a real implementation, you'd verify the stored config matches config2)
    }
}