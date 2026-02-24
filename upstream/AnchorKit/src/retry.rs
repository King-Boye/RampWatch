use crate::Error;

/// Retry configuration with exponential backoff
#[derive(Clone, Debug)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub initial_delay_ms: u64,
    pub max_delay_ms: u64,
    pub backoff_multiplier: u32,
}

impl RetryConfig {
    /// Create a default retry configuration
    pub fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay_ms: 100,
            max_delay_ms: 5000,
            backoff_multiplier: 2,
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
        }
    }

    /// Calculate delay for a given attempt (0-indexed)
    pub fn calculate_delay(&self, attempt: u32) -> u64 {
        if attempt == 0 {
            return 0; // No delay on first attempt
        }

        let delay = self.initial_delay_ms * (self.backoff_multiplier as u64).pow(attempt - 1);
        delay.min(self.max_delay_ms)
    }
}

/// Retry result tracking
#[derive(Clone, Debug, PartialEq)]
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
pub fn is_retryable_error(error: &Error) -> bool {
    match error {
        // Retryable errors (transient failures)
        Error::EndpointNotFound => true,
        Error::InvalidEndpointFormat => false, // Configuration error, not retryable
        Error::EndpointAlreadyExists => false, // Logic error, not retryable

        // Network/availability errors (retryable)
        Error::ServicesNotConfigured => true,

        // Authentication/authorization errors (not retryable)
        Error::UnauthorizedAttestor => false,
        Error::AttestorNotRegistered => false,

        // Data validation errors (not retryable)
        Error::InvalidConfig => false,
        Error::InvalidQuote => false,
        Error::InvalidTimestamp => false,
        Error::InvalidTransactionIntent => false,

        // State errors (not retryable)
        Error::AlreadyInitialized => false,
        Error::AttestorAlreadyRegistered => false,
        Error::ReplayAttack => false,
        Error::SessionReplayAttack => false,

        // Not found errors (retryable - might be temporary)
        Error::AttestationNotFound => true,
        Error::QuoteNotFound => true,
        Error::SessionNotFound => true,

        // Stale data (retryable - can fetch fresh data)
        Error::StaleQuote => true,
        Error::NoQuotesAvailable => true,
        Error::NoAnchorsAvailable => true,

        // Compliance errors (not retryable)
        Error::ComplianceNotMet => false,

        // Credential errors (not retryable)
        Error::CredentialNotFound => false,
        Error::CredentialExpired => false,
        Error::InsecureCredentialStorage => false,

        // Other errors
        _ => false, // Default to not retryable for safety
    }
}

/// Retry engine for executing operations with exponential backoff
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
        assert!(is_retryable_error(&Error::QuoteNotFound));
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
