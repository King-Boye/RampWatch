#![cfg(test)]

extern crate alloc;
use alloc::vec::Vec;

use crate::{
    retry::{is_retryable_error, RetryConfig, RetryEngine, RetryResult},
    Error,
};

/// Test Goal 1: Exponential retry timing

#[test]
fn test_exponential_backoff_timing() {
    let config = RetryConfig::new(5, 100, 10000, 2);

    // Verify exponential growth: 0, 100, 200, 400, 800
    assert_eq!(config.calculate_delay(0), 0);
    assert_eq!(config.calculate_delay(1), 100);
    assert_eq!(config.calculate_delay(2), 200);
    assert_eq!(config.calculate_delay(3), 400);
    assert_eq!(config.calculate_delay(4), 800);
}

#[test]
fn test_exponential_backoff_with_multiplier_3() {
    let config = RetryConfig::new(5, 50, 10000, 3);

    // Verify exponential growth with multiplier 3: 0, 50, 150, 450, 1350
    assert_eq!(config.calculate_delay(0), 0);
    assert_eq!(config.calculate_delay(1), 50);
    assert_eq!(config.calculate_delay(2), 150); // 50 * 3^1
    assert_eq!(config.calculate_delay(3), 450); // 50 * 3^2
    assert_eq!(config.calculate_delay(4), 1350); // 50 * 3^3
}

#[test]
fn test_exponential_backoff_respects_max_delay() {
    let config = RetryConfig::new(10, 1000, 5000, 2);

    // Should cap at max_delay_ms (5000)
    assert_eq!(config.calculate_delay(0), 0);
    assert_eq!(config.calculate_delay(1), 1000);
    assert_eq!(config.calculate_delay(2), 2000);
    assert_eq!(config.calculate_delay(3), 4000);
    assert_eq!(config.calculate_delay(4), 5000); // Capped
    assert_eq!(config.calculate_delay(5), 5000); // Still capped
    assert_eq!(config.calculate_delay(10), 5000); // Still capped
}

#[test]
fn test_total_delay_accumulation() {
    let config = RetryConfig::new(4, 100, 10000, 2);
    let engine = RetryEngine::new(config);

    let mut attempt_count = 0;
    let result = engine.execute(|_attempt| {
        attempt_count += 1;
        if attempt_count < 4 {
            Err(Error::EndpointNotFound) // Retryable
        } else {
            Ok(42)
        }
    });

    assert!(result.is_success());
    assert_eq!(result.attempts, 4);
    // Total delay: 0 + 100 + 200 + 400 = 700ms
    assert_eq!(result.total_delay_ms, 700);
}

/// Test Goal 2: Stops after max attempts

#[test]
fn test_stops_after_max_attempts() {
    let config = RetryConfig::new(3, 100, 5000, 2);
    let engine = RetryEngine::new(config);

    let mut attempt_count = 0;
    let result: RetryResult<i32> = engine.execute(|_attempt| {
        attempt_count += 1;
        Err(Error::EndpointNotFound) // Always fail with retryable error
    });

    assert!(result.is_failure());
    assert_eq!(result.attempts, 3); // Exactly max_attempts
    assert_eq!(attempt_count, 3);
    assert_eq!(result.error, Some(Error::EndpointNotFound));
}

#[test]
fn test_stops_after_max_attempts_with_different_configs() {
    // Test with max_attempts = 1
    let config1 = RetryConfig::new(1, 100, 5000, 2);
    let engine1 = RetryEngine::new(config1);

    let mut count1 = 0;
    let result1: RetryResult<i32> = engine1.execute(|_| {
        count1 += 1;
        Err(Error::QuoteNotFound)
    });

    assert_eq!(result1.attempts, 1);
    assert_eq!(count1, 1);

    // Test with max_attempts = 5
    let config2 = RetryConfig::new(5, 100, 5000, 2);
    let engine2 = RetryEngine::new(config2);

    let mut count2 = 0;
    let result2: RetryResult<i32> = engine2.execute(|_| {
        count2 += 1;
        Err(Error::StaleQuote)
    });

    assert_eq!(result2.attempts, 5);
    assert_eq!(count2, 5);

    // Test with max_attempts = 10
    let config3 = RetryConfig::new(10, 50, 5000, 2);
    let engine3 = RetryEngine::new(config3);

    let mut count3 = 0;
    let result3: RetryResult<i32> = engine3.execute(|_| {
        count3 += 1;
        Err(Error::NoQuotesAvailable)
    });

    assert_eq!(result3.attempts, 10);
    assert_eq!(count3, 10);
}

#[test]
fn test_succeeds_before_max_attempts() {
    let config = RetryConfig::new(5, 100, 5000, 2);
    let engine = RetryEngine::new(config);

    let mut attempt_count = 0;
    let result = engine.execute(|_attempt| {
        attempt_count += 1;
        if attempt_count < 3 {
            Err(Error::EndpointNotFound)
        } else {
            Ok("success")
        }
    });

    assert!(result.is_success());
    assert_eq!(result.attempts, 3); // Succeeded on 3rd attempt
    assert_eq!(attempt_count, 3);
    assert_eq!(result.value, Some("success"));
}

/// Test Goal 3: Does not retry non-recoverable errors

#[test]
fn test_does_not_retry_invalid_config() {
    let config = RetryConfig::new(5, 100, 5000, 2);
    let engine = RetryEngine::new(config);

    let mut attempt_count = 0;
    let result: RetryResult<i32> = engine.execute(|_attempt| {
        attempt_count += 1;
        Err(Error::InvalidConfig) // Non-retryable
    });

    assert!(result.is_failure());
    assert_eq!(result.attempts, 1); // Only 1 attempt, no retries
    assert_eq!(attempt_count, 1);
    assert_eq!(result.error, Some(Error::InvalidConfig));
}

#[test]
fn test_does_not_retry_unauthorized_attestor() {
    let config = RetryConfig::new(5, 100, 5000, 2);
    let engine = RetryEngine::new(config);

    let mut attempt_count = 0;
    let result: RetryResult<i32> = engine.execute(|_attempt| {
        attempt_count += 1;
        Err(Error::UnauthorizedAttestor) // Non-retryable
    });

    assert!(result.is_failure());
    assert_eq!(result.attempts, 1);
    assert_eq!(attempt_count, 1);
}

#[test]
fn test_does_not_retry_replay_attack() {
    let config = RetryConfig::new(5, 100, 5000, 2);
    let engine = RetryEngine::new(config);

    let mut attempt_count = 0;
    let result: RetryResult<i32> = engine.execute(|_attempt| {
        attempt_count += 1;
        Err(Error::ReplayAttack) // Non-retryable
    });

    assert!(result.is_failure());
    assert_eq!(result.attempts, 1);
    assert_eq!(attempt_count, 1);
}

#[test]
fn test_does_not_retry_compliance_not_met() {
    let config = RetryConfig::new(5, 100, 5000, 2);
    let engine = RetryEngine::new(config);

    let mut attempt_count = 0;
    let result: RetryResult<i32> = engine.execute(|_attempt| {
        attempt_count += 1;
        Err(Error::ComplianceNotMet) // Non-retryable
    });

    assert!(result.is_failure());
    assert_eq!(result.attempts, 1);
    assert_eq!(attempt_count, 1);
}

#[test]
fn test_retryable_vs_non_retryable_classification() {
    // Retryable errors (transient/temporary)
    assert!(is_retryable_error(&Error::EndpointNotFound));
    assert!(is_retryable_error(&Error::ServicesNotConfigured));
    assert!(is_retryable_error(&Error::AttestationNotFound));
    assert!(is_retryable_error(&Error::QuoteNotFound));
    assert!(is_retryable_error(&Error::SessionNotFound));
    assert!(is_retryable_error(&Error::StaleQuote));
    assert!(is_retryable_error(&Error::NoQuotesAvailable));
    assert!(is_retryable_error(&Error::NoAnchorsAvailable));

    // Non-retryable errors (permanent/logic errors)
    assert!(!is_retryable_error(&Error::InvalidConfig));
    assert!(!is_retryable_error(&Error::InvalidEndpointFormat));
    assert!(!is_retryable_error(&Error::UnauthorizedAttestor));
    assert!(!is_retryable_error(&Error::AttestorNotRegistered));
    assert!(!is_retryable_error(&Error::AttestorAlreadyRegistered));
    assert!(!is_retryable_error(&Error::ReplayAttack));
    assert!(!is_retryable_error(&Error::SessionReplayAttack));
    assert!(!is_retryable_error(&Error::InvalidQuote));
    assert!(!is_retryable_error(&Error::InvalidTimestamp));
    assert!(!is_retryable_error(&Error::ComplianceNotMet));
    assert!(!is_retryable_error(&Error::CredentialExpired));
    assert!(!is_retryable_error(&Error::AlreadyInitialized));
}

/// Test: Mixed retryable and non-retryable errors

#[test]
fn test_stops_on_non_retryable_after_retries() {
    let config = RetryConfig::new(5, 100, 5000, 2);
    let engine = RetryEngine::new(config);

    let mut attempt_count = 0;
    let result: RetryResult<i32> = engine.execute(|_attempt| {
        attempt_count += 1;
        if attempt_count < 3 {
            Err(Error::EndpointNotFound) // Retryable
        } else {
            Err(Error::InvalidConfig) // Non-retryable
        }
    });

    assert!(result.is_failure());
    assert_eq!(result.attempts, 3); // Stopped when hit non-retryable
    assert_eq!(attempt_count, 3);
    assert_eq!(result.error, Some(Error::InvalidConfig));
}

/// Test: First attempt success (no retries needed)

#[test]
fn test_first_attempt_success() {
    let config = RetryConfig::new(5, 100, 5000, 2);
    let engine = RetryEngine::new(config);

    let mut attempt_count = 0;
    let result = engine.execute(|_attempt| {
        attempt_count += 1;
        Ok(100)
    });

    assert!(result.is_success());
    assert_eq!(result.attempts, 1);
    assert_eq!(attempt_count, 1);
    assert_eq!(result.value, Some(100));
    assert_eq!(result.total_delay_ms, 0); // No delay on first attempt
}

/// Test: Delay calculation edge cases

#[test]
fn test_delay_calculation_with_zero_initial_delay() {
    let config = RetryConfig::new(5, 0, 5000, 2);

    // All delays should be 0
    assert_eq!(config.calculate_delay(0), 0);
    assert_eq!(config.calculate_delay(1), 0);
    assert_eq!(config.calculate_delay(2), 0);
}

#[test]
fn test_delay_calculation_with_multiplier_1() {
    let config = RetryConfig::new(5, 100, 5000, 1);

    // Delays should stay constant (no exponential growth)
    assert_eq!(config.calculate_delay(0), 0);
    assert_eq!(config.calculate_delay(1), 100);
    assert_eq!(config.calculate_delay(2), 100);
    assert_eq!(config.calculate_delay(3), 100);
}

#[test]
fn test_delay_calculation_large_multiplier() {
    let config = RetryConfig::new(5, 10, 10000, 10);

    assert_eq!(config.calculate_delay(0), 0);
    assert_eq!(config.calculate_delay(1), 10);
    assert_eq!(config.calculate_delay(2), 100); // 10 * 10^1
    assert_eq!(config.calculate_delay(3), 1000); // 10 * 10^2
    assert_eq!(config.calculate_delay(4), 10000); // 10 * 10^3, capped at max
}

/// Test: Retry engine with default config

#[test]
fn test_retry_engine_default_config() {
    let engine = RetryEngine::with_default_config();
    let config = engine.get_config();

    assert_eq!(config.max_attempts, 3);
    assert_eq!(config.initial_delay_ms, 100);
    assert_eq!(config.max_delay_ms, 5000);
    assert_eq!(config.backoff_multiplier, 2);
}

/// Test: Attempt parameter passed correctly

#[test]
fn test_attempt_parameter_increments() {
    let config = RetryConfig::new(5, 100, 5000, 2);
    let engine = RetryEngine::new(config);

    let mut attempts_seen = Vec::new();
    let _result: RetryResult<i32> = engine.execute(|attempt| {
        attempts_seen.push(attempt);
        Err(Error::EndpointNotFound)
    });

    // Should see attempts 0, 1, 2, 3, 4
    let expected = alloc::vec![0, 1, 2, 3, 4];
    assert_eq!(attempts_seen, expected);
}

/// Test: Complex retry scenario

#[test]
fn test_complex_retry_scenario() {
    let config = RetryConfig::new(6, 50, 5000, 2);
    let engine = RetryEngine::new(config);

    let mut attempt_count = 0;
    let result = engine.execute(|attempt| {
        attempt_count += 1;

        match attempt {
            0 => Err(Error::EndpointNotFound),  // Retry
            1 => Err(Error::QuoteNotFound),     // Retry
            2 => Err(Error::StaleQuote),        // Retry
            3 => Err(Error::NoQuotesAvailable), // Retry
            4 => Ok("finally succeeded"),       // Success
            _ => Err(Error::InvalidConfig),     // Should not reach
        }
    });

    assert!(result.is_success());
    assert_eq!(result.attempts, 5);
    assert_eq!(attempt_count, 5);
    assert_eq!(result.value, Some("finally succeeded"));
    // Total delay: 0 + 50 + 100 + 200 + 400 = 750ms
    assert_eq!(result.total_delay_ms, 750);
}

/// Test: All attempts fail with retryable error

#[test]
fn test_all_attempts_fail_retryable() {
    let config = RetryConfig::new(3, 100, 5000, 2);
    let engine = RetryEngine::new(config);

    let errors = alloc::vec![
        Error::EndpointNotFound,
        Error::QuoteNotFound,
        Error::StaleQuote,
    ];

    let mut attempt_count = 0;
    let result: RetryResult<i32> = engine.execute(|attempt| {
        let error = errors[attempt as usize % errors.len()].clone();
        attempt_count += 1;
        Err(error)
    });

    assert!(result.is_failure());
    assert_eq!(result.attempts, 3);
    assert_eq!(attempt_count, 3);
    // Last error should be returned
    assert_eq!(result.error, Some(Error::StaleQuote));
}

/// Test: Verify delay accumulation is correct

#[test]
fn test_delay_accumulation_correctness() {
    let config = RetryConfig::new(4, 100, 10000, 2);
    let engine = RetryEngine::new(config);

    let result: RetryResult<i32> = engine.execute(|_| Err(Error::EndpointNotFound));

    // Delays: 0 (attempt 0) + 100 (attempt 1) + 200 (attempt 2) + 400 (attempt 3)
    assert_eq!(result.total_delay_ms, 700);
    assert_eq!(result.attempts, 4);
}

/// Test: Zero max attempts edge case

#[test]
fn test_zero_max_attempts() {
    let config = RetryConfig::new(0, 100, 5000, 2);
    let engine = RetryEngine::new(config);

    let mut attempt_count = 0;
    let result: RetryResult<i32> = engine.execute(|_| {
        attempt_count += 1;
        Err(Error::EndpointNotFound)
    });

    // Should not execute at all
    assert_eq!(attempt_count, 0);
    assert!(result.is_failure());
}
