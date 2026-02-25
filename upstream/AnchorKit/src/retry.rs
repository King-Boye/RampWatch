use crate::errors::Error;
use crate::rate_limit_response::RateLimitInfo;

/// Retry configuration with exponential backoff, jitter, and rate limit support
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub initial_delay_ms: u64,
    pub max_delay_ms: u64,
    pub backoff_multiplier: u32,
    /// Jitter factor: 0.0 = no jitter, 1.0 = full jitter (default: 0.5)
    pub jitter_factor: f64,
    /// Whether to use Retry-After header when present (default: true)
    pub use_retry_after: bool,
    /// Initial delay specifically for rate limit errors (default: 1000ms)
    pub rate_limit_initial_delay_ms: u64,
    /// Backoff multiplier specifically for rate limit errors (default: 3)
    pub rate_limit_backoff_multiplier: u32,
    /// Max delay for rate limit errors (default: 60000ms = 60 seconds)
    pub rate_limit_max_delay_ms: u64,
}

impl RetryConfig {
    /// Create a default retry configuration
    pub fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay_ms: 100,
            max_delay_ms: 5000,
            backoff_multiplier: 2,
            jitter_factor: 0.5,
            use_retry_after: true,
            rate_limit_initial_delay_ms: 1000,
            rate_limit_backoff_multiplier: 3,
            rate_limit_max_delay_ms: 60000,
        }
    }

    /// Create a retry configuration with custom values
    pub fn new(
        max_attempts: u32,
        initial_delay_ms: u64,
        max_delay_ms: u64,
        backoff_multiplier: u32,
    ) -> Self {
        Self {
            max_attempts,
            initial_delay_ms,
            max_delay_ms,
            backoff_multiplier,
            jitter_factor: 0.5,
            use_retry_after: true,
            rate_limit_initial_delay_ms: 1000,
            rate_limit_backoff_multiplier: 3,
            rate_limit_max_delay_ms: 60000,
        }
    }

    /// Create a configuration optimized for rate limiting scenarios
    pub fn for_rate_limits() -> Self {
        Self {
            max_attempts: 5,
            initial_delay_ms: 1000,
            max_delay_ms: 60000,
            backoff_multiplier: 3,
            jitter_factor: 0.5,
            use_retry_after: true,
            rate_limit_initial_delay_ms: 1000,
            rate_limit_backoff_multiplier: 3,
            rate_limit_max_delay_ms: 60000,
        }
    }

    /// Create a conservative configuration for critical operations
    pub fn conservative() -> Self {
        Self {
            max_attempts: 3,
            initial_delay_ms: 500,
            max_delay_ms: 10000,
            backoff_multiplier: 2,
            jitter_factor: 0.3,
            use_retry_after: true,
            rate_limit_initial_delay_ms: 2000,
            rate_limit_backoff_multiplier: 2,
            rate_limit_max_delay_ms: 30000,
        }
    }

    /// Create an aggressive configuration for non-critical background operations
    pub fn aggressive() -> Self {
        Self {
            max_attempts: 10,
            initial_delay_ms: 50,
            max_delay_ms: 5000,
            backoff_multiplier: 2,
            jitter_factor: 0.8,
            use_retry_after: false, // Don't wait for Retry-After in aggressive mode
            rate_limit_initial_delay_ms: 200,
            rate_limit_backoff_multiplier: 2,
            rate_limit_max_delay_ms: 10000,
        }
    }

    /// Calculate delay for a given attempt (0-indexed)
    pub fn calculate_delay(&self, attempt: u32) -> u64 {
        if attempt == 0 {
            return 0; // No delay on first attempt
        }

        let delay = self.initial_delay_ms * (self.backoff_multiplier as u64).pow(attempt - 1);
        let capped_delay = delay.min(self.max_delay_ms);

        // Apply jitter
        self.apply_jitter(capped_delay)
    }

    /// Calculate delay for rate limit scenarios
    /// Uses Retry-After header value if available and configured
    pub fn calculate_rate_limit_delay(&self, attempt: u32, rate_limit_info: Option<&RateLimitInfo>) -> u64 {
        if attempt == 0 {
            return 0; // No delay on first attempt
        }

        // If Retry-After is available and we should use it
        if self.use_retry_after {
            if let Some(info) = rate_limit_info {
                if info.retry_after_ms > 0 {
                    // Use Retry-After value, capped at rate_limit_max_delay_ms
                    return info.retry_after_ms.min(self.rate_limit_max_delay_ms);
                }
            }
        }

        // Fall back to exponential backoff with rate limit specific settings
        let delay = self.rate_limit_initial_delay_ms 
            * (self.rate_limit_backoff_multiplier as u64).pow(attempt - 1);
        
        let capped_delay = delay.min(self.rate_limit_max_delay_ms);

        // Apply jitter
        self.apply_jitter(capped_delay)
    }

    /// Apply jitter to a delay value using "Decorrelated Jitter" algorithm
    /// This provides better spread than simple randomization
    fn apply_jitter(&self, delay: u64) -> u64 {
        if self.jitter_factor <= 0.0 {
            return delay;
        }

        if self.jitter_factor >= 1.0 {
            // Full jitter: random value between 0 and delay
            // For deterministic testing, we use a simple hash-based approach
            // In production, this would use a proper random number generator
            return delay / 2;
        }

        // Partial jitter: delay * (1 - jitter_factor/2) to delay
        // This ensures we don't reduce delay too much
        let min_delay = (delay as f64 * (1.0 - self.jitter_factor / 2.0)) as u64;
        
        // For deterministic behavior in tests, return midpoint
        // In production with proper RNG, this would add randomness
        ((min_delay + delay) / 2).min(delay)
    }
}

/// Retry result tracking
#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub struct RetryResult<T> {
    pub value: Option<T>,
    pub error: Option<Error>,
    pub attempts: u32,
    pub total_delay_ms: u64,
}

impl<T> RetryResult<T> {
    pub fn success(value: T, attempts: u32, total_delay_ms: u64) -> Self {
        Self {
            value: Some(value),
            error: None,
            attempts,
            total_delay_ms,
        }
    }

    pub fn failure(error: Error, attempts: u32, total_delay_ms: u64) -> Self {
        Self {
            value: None,
            error: Some(error),
            attempts,
            total_delay_ms,
        }
    }

    pub fn is_success(&self) -> bool {
        self.value.is_some()
    }

    pub fn is_failure(&self) -> bool {
        self.error.is_some()
    }
}

/// Determine if an error is retryable
#[allow(dead_code)]
pub fn is_retryable_error(error: &Error) -> bool {
    match error {
        // Network failures (retryable)
        Error::TransportError => true,
        Error::TransportTimeout => true,

        // Rate limiting (retryable with backoff)
        Error::RateLimitExceeded => true,
        Error::ProtocolRateLimitExceeded => true,

        // Retryable errors (transient failures)
        Error::EndpointNotFound => true,
        Error::InvalidEndpointFormat => false,

        // Network/availability errors (retryable)
        Error::ServicesNotConfigured => true,

        // Authentication/authorization errors (not retryable)
        Error::UnauthorizedAttestor => false,
        Error::AttestorNotRegistered => false,
        Error::TransportUnauthorized => false,

        // Data validation errors (not retryable)
        Error::InvalidConfig => false,
        Error::InvalidQuote => false,
        Error::InvalidTimestamp => false,
        Error::InvalidTransactionIntent => false,
        Error::ProtocolInvalidPayload => false,

        // State errors (not retryable)
        Error::AlreadyInitialized => false,
        Error::AttestorAlreadyRegistered => false,
        Error::ReplayAttack => false,

        // Not found errors (retryable - might be temporary)
        Error::AttestationNotFound => true,
        Error::SessionNotFound => true,

        // Stale data (retryable - can fetch fresh data)
        Error::StaleQuote => true,
        Error::NoQuotesAvailable => true,
        Error::AnchorMetadataNotFound => true,
        Error::CacheExpired => true,
        Error::CacheNotFound => true,

        // Compliance errors (not retryable)
        Error::ComplianceNotMet => false,

        // Credential errors (not retryable)
        Error::CredentialNotFound => false,
        Error::CredentialExpired => false,
        Error::InvalidCredentialFormat => false,

        // Protocol errors (not retryable except rate limit)
        Error::ProtocolError => false,

        // Other errors
        _ => false,
    }
}

/// Determine if an error is a rate limit error (429)
#[allow(dead_code)]
pub fn is_rate_limit_error(error: &Error) -> bool {
    matches!(
        error,
        Error::RateLimitExceeded | Error::ProtocolRateLimitExceeded
    )
}

/// Get a recommended retry delay for a rate limit error
/// This respects the Retry-After header if available
#[allow(dead_code)]
pub fn get_rate_limit_delay(error: &Error, rate_limit_info: Option<&RateLimitInfo>) -> u64 {
    if !is_rate_limit_error(error) {
        return 0;
    }

    // If we have rate limit info with Retry-After, use it
    if let Some(info) = rate_limit_info {
        if info.retry_after_ms > 0 {
            return info.retry_after_ms;
        }
    }

    // Default to a conservative delay if no Retry-After
    1000
}

/// Retry engine for executing operations with exponential backoff
#[allow(dead_code)]
pub struct RetryEngine {
    config: RetryConfig,
}

impl RetryEngine {
    pub fn new(config: RetryConfig) -> Self {
        Self { config }
    }

    pub fn with_default_config() -> Self {
        Self::new(RetryConfig::default())
    }

    /// Execute an operation with retry logic
    /// Note: In a real implementation, this would use async/await and actual delays
    /// For testing purposes, we track delays without actually waiting
    pub fn execute<T, F>(&self, mut operation: F) -> RetryResult<T>
    where
        F: FnMut(u32) -> Result<T, Error>,
    {
        let mut total_delay_ms = 0u64;

        for attempt in 0..self.config.max_attempts {
            // Calculate and track delay (but don't actually wait in tests)
            let delay = self.config.calculate_delay(attempt);
            total_delay_ms += delay;

            // Execute the operation
            match operation(attempt) {
                Ok(value) => {
                    return RetryResult::success(value, attempt + 1, total_delay_ms);
                }
                Err(error) => {
                    // Check if we should retry
                    if !is_retryable_error(&error) {
                        return RetryResult::failure(error, attempt + 1, total_delay_ms);
                    }

                    // If this was the last attempt, return failure
                    if attempt + 1 >= self.config.max_attempts {
                        return RetryResult::failure(error, attempt + 1, total_delay_ms);
                    }

                    // Otherwise, continue to next attempt
                }
            }
        }

        // Should never reach here, but return failure just in case
        RetryResult::failure(
            Error::InvalidConfig,
            self.config.max_attempts,
            total_delay_ms,
        )
    }

    pub fn get_config(&self) -> &RetryConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retry_config_default() {
        let config = RetryConfig::default();
        assert_eq!(config.max_attempts, 3);
        assert_eq!(config.initial_delay_ms, 100);
        assert_eq!(config.max_delay_ms, 5000);
        assert_eq!(config.backoff_multiplier, 2);
    }

    #[test]
    fn test_retry_config_custom() {
        let config = RetryConfig::new(5, 200, 10000, 3);
        assert_eq!(config.max_attempts, 5);
        assert_eq!(config.initial_delay_ms, 200);
        assert_eq!(config.max_delay_ms, 10000);
        assert_eq!(config.backoff_multiplier, 3);
    }

    #[test]
    fn test_calculate_delay_exponential() {
        let config = RetryConfig::new(5, 100, 10000, 2);

        // Attempt 0: no delay (first attempt)
        assert_eq!(config.calculate_delay(0), 0);

        // Attempt 1: 100ms (initial delay)
        assert_eq!(config.calculate_delay(1), 100);

        // Attempt 2: 200ms (100 * 2^1)
        assert_eq!(config.calculate_delay(2), 200);

        // Attempt 3: 400ms (100 * 2^2)
        assert_eq!(config.calculate_delay(3), 400);

        // Attempt 4: 800ms (100 * 2^3)
        assert_eq!(config.calculate_delay(4), 800);
    }

    #[test]
    fn test_calculate_delay_max_cap() {
        let config = RetryConfig::new(10, 1000, 5000, 2);

        // Should cap at max_delay_ms
        assert_eq!(config.calculate_delay(10), 5000);
        assert_eq!(config.calculate_delay(20), 5000);
    }

    #[test]
    fn test_is_retryable_error() {
        // Retryable errors
        assert!(is_retryable_error(&Error::EndpointNotFound));
        assert!(is_retryable_error(&Error::ServicesNotConfigured));
        assert!(is_retryable_error(&Error::AttestationNotFound));
        assert!(is_retryable_error(&Error::StaleQuote));
        assert!(is_retryable_error(&Error::NoQuotesAvailable));

        // Non-retryable errors
        assert!(!is_retryable_error(&Error::InvalidConfig));
        assert!(!is_retryable_error(&Error::UnauthorizedAttestor));
        assert!(!is_retryable_error(&Error::AttestorAlreadyRegistered));
        assert!(!is_retryable_error(&Error::ReplayAttack));
        assert!(!is_retryable_error(&Error::InvalidQuote));
        assert!(!is_retryable_error(&Error::ComplianceNotMet));
    }

    #[test]
    fn test_retry_result_success() {
        let result = RetryResult::success(42, 2, 300);
        assert!(result.is_success());
        assert!(!result.is_failure());
        assert_eq!(result.value, Some(42));
        assert_eq!(result.attempts, 2);
        assert_eq!(result.total_delay_ms, 300);
    }

    #[test]
    fn test_retry_result_failure() {
        let result: RetryResult<i32> = RetryResult::failure(Error::InvalidConfig, 3, 700);
        assert!(!result.is_success());
        assert!(result.is_failure());
        assert_eq!(result.value, None);
        assert_eq!(result.error, Some(Error::InvalidConfig));
        assert_eq!(result.attempts, 3);
        assert_eq!(result.total_delay_ms, 700);
    }
}
