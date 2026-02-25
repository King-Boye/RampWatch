#![cfg(test)]

use crate::{
    retry::{is_retryable_error, RetryConfig, RetryEngine},
    Error,
};

#[test]
fn test_timeout_enforced_on_slow_anchor() {
    let config = RetryConfig::new(3, 100, 5000, 2);
    let engine = RetryEngine::new(config);

    let mut attempt_count = 0;
    let result: crate::retry::RetryResult<i32> = engine.execute(|_| {
        attempt_count += 1;
        Err(Error::EndpointNotFound) // Simulate timeout
    });

    assert!(result.is_failure());
    assert_eq!(result.attempts, 3);
    assert_eq!(attempt_count, 3);
}

#[test]
fn test_retries_triggered_correctly() {
    let config = RetryConfig::new(3, 100, 5000, 2);
    let engine = RetryEngine::new(config);

    let mut attempts = 0;
    let result = engine.execute(|attempt| {
        attempts += 1;
        if attempt < 2 {
            Err(Error::TransportTimeout) // Retryable
        } else {
            Ok(42) // Success on 3rd attempt
        }
    });

    assert!(result.is_success());
    assert_eq!(result.attempts, 3);
    assert_eq!(attempts, 3);
    assert_eq!(result.value, Some(42));
}

#[test]
fn test_timeout_with_exponential_backoff() {
    let config = RetryConfig::new(4, 100, 5000, 2);
    let engine = RetryEngine::new(config);

    let result: crate::retry::RetryResult<i32> =
        engine.execute(|_| Err(Error::ServicesNotConfigured));

    assert!(result.is_failure());
    assert_eq!(result.attempts, 4);
    // 0 + 100 + 200 + 400 = 700ms total delay
    assert_eq!(result.total_delay_ms, 700);
}

#[test]
fn test_non_retryable_error_stops_immediately() {
    let config = RetryConfig::new(5, 100, 5000, 2);
    let engine = RetryEngine::new(config);

    let mut attempts = 0;
    let result: crate::retry::RetryResult<i32> = engine.execute(|_| {
        attempts += 1;
        Err(Error::InvalidConfig) // Non-retryable
    });

    assert!(result.is_failure());
    assert_eq!(result.attempts, 1); // Only 1 attempt
    assert_eq!(attempts, 1);
}

#[test]
fn test_success_on_first_attempt_no_retry() {
    let config = RetryConfig::new(3, 100, 5000, 2);
    let engine = RetryEngine::new(config);

    let result = engine.execute(|_| Ok(100));

    assert!(result.is_success());
    assert_eq!(result.attempts, 1);
    assert_eq!(result.total_delay_ms, 0);
}

#[test]
fn test_max_delay_cap_enforced() {
    let config = RetryConfig::new(10, 1000, 3000, 2);

    assert_eq!(config.calculate_delay(0), 0);
    assert_eq!(config.calculate_delay(1), 1000);
    assert_eq!(config.calculate_delay(2), 2000);
    assert_eq!(config.calculate_delay(3), 3000); // Capped
    assert_eq!(config.calculate_delay(10), 3000); // Still capped
}

#[test]
fn test_retryable_errors_classification() {
    // Retryable (transient failures)
    assert!(is_retryable_error(&Error::EndpointNotFound));
    assert!(is_retryable_error(&Error::ServicesNotConfigured));
    assert!(is_retryable_error(&Error::StaleQuote));
    assert!(is_retryable_error(&Error::NoQuotesAvailable));
    assert!(is_retryable_error(&Error::TransportError));
    assert!(is_retryable_error(&Error::TransportTimeout));

    // Non-retryable (permanent failures)
    assert!(!is_retryable_error(&Error::InvalidConfig));
    assert!(!is_retryable_error(&Error::UnauthorizedAttestor));
    assert!(!is_retryable_error(&Error::ReplayAttack));
    assert!(!is_retryable_error(&Error::ComplianceNotMet));
    assert!(!is_retryable_error(&Error::InvalidQuote));
}

#[test]
fn test_retry_with_alternating_errors() {
    let config = RetryConfig::new(5, 50, 1000, 2);
    let engine = RetryEngine::new(config);

    let mut attempt = 0;
    let result = engine.execute(|_| {
        attempt += 1;
        match attempt {
            1 => Err(Error::StaleQuote),       // Retryable
            2 => Err(Error::EndpointNotFound), // Retryable
            3 => Ok(999),                      // Success
            _ => Err(Error::InvalidConfig),
        }
    });

    assert!(result.is_success());
    assert_eq!(result.attempts, 3);
    assert_eq!(result.value, Some(999));
}

#[test]
fn test_all_retries_exhausted() {
    let config = RetryConfig::new(2, 100, 5000, 2);
    let engine = RetryEngine::new(config);

    let result: crate::retry::RetryResult<i32> =
        engine.execute(|_| Err(Error::AnchorMetadataNotFound));

    assert!(result.is_failure());
    assert_eq!(result.attempts, 2);
    assert_eq!(result.error, Some(Error::AnchorMetadataNotFound));
}

#[test]
fn test_timeout_delay_calculation() {
    let config = RetryConfig::new(5, 200, 10000, 3);

    assert_eq!(config.calculate_delay(0), 0); // No delay
    assert_eq!(config.calculate_delay(1), 200); // 200 * 3^0
    assert_eq!(config.calculate_delay(2), 600); // 200 * 3^1
    assert_eq!(config.calculate_delay(3), 1800); // 200 * 3^2
    assert_eq!(config.calculate_delay(4), 5400); // 200 * 3^3
}

#[test]
fn test_custom_retry_config() {
    let config = RetryConfig::new(4, 50, 2000, 2);
    let engine = RetryEngine::new(config);

    assert_eq!(engine.get_config().max_attempts, 4);
    assert_eq!(engine.get_config().initial_delay_ms, 50);
    assert_eq!(engine.get_config().max_delay_ms, 2000);
}
