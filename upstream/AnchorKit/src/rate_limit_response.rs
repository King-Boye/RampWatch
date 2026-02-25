use soroban_sdk::{contracttype, Env};

/// Information extracted from rate limit HTTP headers
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RateLimitInfo {
    /// Suggested delay in milliseconds from Retry-After header
    pub retry_after_ms: u64,
    /// Remaining requests allowed (from X-RateLimit-Remaining)
    pub remaining: Option<u32>,
    /// Reset timestamp in seconds (from X-RateLimit-Reset)
    pub reset_at: Option<u64>,
    /// Rate limit window in seconds (from X-RateLimit-Limit)
    pub limit: Option<u32>,
    /// Rate limit window in seconds (from X-RateLimit-Window)
    pub window_seconds: Option<u32>,
    /// Whether rate limiting was detected
    pub is_rate_limited: bool,
}

impl RateLimitInfo {
    /// Create a new RateLimitInfo with default values
    pub fn new() -> Self {
        Self {
            retry_after_ms: 0,
            remaining: None,
            reset_at: None,
            limit: None,
            window_seconds: None,
            is_rate_limited: false,
        }
    }

    /// Create rate limit info indicating rate limiting was detected
    pub fn rate_limited(retry_after_ms: u64) -> Self {
        Self {
            retry_after_ms,
            remaining: None,
            reset_at: None,
            limit: None,
            window_seconds: None,
            is_rate_limited: true,
        }
    }

    /// Check if Retry-After header value indicates rate limiting
    /// This is a placeholder for HTTP header parsing
    /// In a real implementation, this would parse actual HTTP headers
    #[allow(dead_code)]
    pub fn from_headers(
        _env: &Env,
        retry_after: Option<u64>,
        rate_limit_remaining: Option<u32>,
        rate_limit_reset: Option<u64>,
        rate_limit_limit: Option<u32>,
        rate_limit_window: Option<u32>,
    ) -> Self {
        let retry_after_ms = retry_after.unwrap_or(0) * 1000; // Convert seconds to ms
        let is_rate_limited = retry_after.is_some() || rate_limit_remaining == Some(0);

        Self {
            retry_after_ms,
            remaining: rate_limit_remaining,
            reset_at: rate_limit_reset,
            limit: rate_limit_limit,
            window_seconds: rate_limit_window,
            is_rate_limited,
        }
    }

    /// Parse Retry-After header value (supports seconds or HTTP-date format)
    /// In Soroban, we simplify to just seconds for no_std compatibility
    #[allow(dead_code)]
    pub fn parse_retry_after(retry_after_value: &str) -> Option<u64> {
        // Try to parse as simple seconds first
        if let Ok(seconds) = retry_after_value.parse::<u64>() {
            return Some(seconds);
        }

        // In a full implementation, we would parse HTTP-date format here
        // For now, return None if parsing fails
        None
    }

    /// Calculate the actual delay to use, respecting minimum and maximum bounds
    #[allow(dead_code)]
    pub fn calculate_delay(&self, min_delay_ms: u64, max_delay_ms: u64) -> u64 {
        if self.retry_after_ms > 0 {
            // Use Retry-After value, but cap at max_delay_ms
            self.retry_after_ms.min(max_delay_ms).max(min_delay_ms)
        } else {
            // No Retry-After, use provided default
            0
        }
    }

    /// Check if there's still capacity (remaining > 0)
    #[allow(dead_code)]
    pub fn has_capacity(&self) -> bool {
        match self.remaining {
            Some(remaining) => remaining > 0,
            None => true, // If unknown, assume we have capacity
        }
    }

    /// Get the time until reset if reset_at is known
    #[allow(dead_code)]
    pub fn time_until_reset(&self, current_time: u64) -> u64 {
        match self.reset_at {
            Some(reset_at) if reset_at > current_time => (reset_at - current_time) * 1000, // Convert to ms
            _ => 0,
        }
    }
}

impl Default for RateLimitInfo {
    fn default() -> Self {
        Self::new()
    }
}

/// Rate limit source types for logging and monitoring
#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum RateLimitSource {
    /// Rate limit from anchor API
    Anchor = 1,
    /// Rate limit from Stellar RPC
    StellarRpc = 2,
    /// Rate limit from Horizon API
    Horizon = 3,
    /// Rate limit from third-party API
    ThirdParty = 4,
    /// Unknown source
    Unknown = 0,
}

/// Detailed rate limit incident for logging/monitoring
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RateLimitIncident {
    /// Source of the rate limit
    pub source: RateLimitSource,
    /// Endpoint that was rate limited
    pub endpoint: Option<soroban_sdk::String>,
    /// Retry-After value in seconds
    pub retry_after_seconds: u64,
    /// Remaining requests at time of detection
    pub remaining: Option<u32>,
    /// Reset timestamp
    pub reset_at: Option<u64>,
    /// Timestamp when incident was recorded
    pub recorded_at: u64,
    /// Number of consecutive rate limit hits
    pub consecutive_hits: u32,
}

impl RateLimitIncident {
    /// Create a new rate limit incident
    #[allow(dead_code)]
    pub fn new(
        env: &Env,
        source: RateLimitSource,
        endpoint: Option<soroban_sdk::String>,
        retry_after_seconds: u64,
        remaining: Option<u32>,
        reset_at: Option<u64>,
        consecutive_hits: u32,
    ) -> Self {
        Self {
            source,
            endpoint,
            retry_after_seconds,
            remaining,
            reset_at,
            recorded_at: env.ledger().timestamp(),
            consecutive_hits,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limit_info_default() {
        let info = RateLimitInfo::new();
        assert_eq!(info.retry_after_ms, 0);
        assert!(!info.is_rate_limited);
        assert!(info.has_capacity());
    }

    #[test]
    fn test_rate_limit_info_rate_limited() {
        let info = RateLimitInfo::rate_limited(30);
        assert!(info.is_rate_limited);
        assert_eq!(info.retry_after_ms, 30000); // 30 seconds in ms
    }

    #[test]
    fn test_rate_limit_info_from_headers() {
        // Test with Retry-After header
        let info = RateLimitInfo::from_headers(
            &Env::default(),
            Some(60),              // retry_after: 60 seconds
            Some(0),               // remaining: 0
            Some(1234567890),     // reset_at: timestamp
            Some(100),             // limit: 100
            Some(60),              // window: 60 seconds
        );

        assert!(info.is_rate_limited);
        assert_eq!(info.retry_after_ms, 60000); // 60 seconds * 1000
        assert_eq!(info.remaining, Some(0));
        assert_eq!(info.reset_at, Some(1234567890));
        assert_eq!(info.limit, Some(100));
    }

    #[test]
    fn test_calculate_delay() {
        let info = RateLimitInfo::rate_limited(100); // 100 seconds = 100000ms

        // Test with bounds
        assert_eq!(info.calculate_delay(1000, 5000), 5000); // Capped at max
        assert_eq!(info.calculate_delay(1000, 200000), 100000); // Within bounds
    }

    #[test]
    fn test_has_capacity() {
        let info_with_remaining = RateLimitInfo::from_headers(
            &Env::default(),
            None,
            Some(5),  // remaining: 5
            None,
            None,
            None,
        );
        assert!(info_with_remaining.has_capacity());

        let info_no_remaining = RateLimitInfo::new();
        assert!(info_no_remaining.has_capacity()); // Unknown = has capacity

        let info_zero_remaining = RateLimitInfo::from_headers(
            &Env::default(),
            None,
            Some(0),  // remaining: 0
            None,
            None,
            None,
        );
        assert!(!info_zero_remaining.has_capacity());
    }

    #[test]
    fn test_parse_retry_after() {
        // Test valid seconds
        assert_eq!(RateLimitInfo::parse_retry_after("30"), Some(30));
        assert_eq!(RateLimitInfo::parse_retry_after("0"), Some(0));
        assert_eq!(RateLimitInfo::parse_retry_after("3600"), Some(3600));

        // Test invalid values
        assert_eq!(RateLimitInfo::parse_retry_after("invalid"), None);
        assert_eq!(RateLimitInfo::parse_retry_after("-1"), None);
    }

    #[test]
    fn test_time_until_reset() {
        let info = RateLimitInfo {
            retry_after_ms: 0,
            remaining: None,
            reset_at: Some(100),  // reset at timestamp 100
            limit: None,
            window_seconds: None,
            is_rate_limited: false,
        };

        // Current time is 50, so 50 seconds until reset
        assert_eq!(info.time_until_reset(50), 50000); // 50 * 1000

        // Current time is 100, so 0 seconds until reset
        assert_eq!(info.time_until_reset(100), 0);

        // Current time is 150, already past reset
        assert_eq!(info.time_until_reset(150), 0);
    }
}
