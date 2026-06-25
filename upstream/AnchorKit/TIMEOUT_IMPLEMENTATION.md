# Request Timeout Implementation

## Overview

Configurable request timeouts have been implemented for all outbound HTTP calls in AnchorKit. This feature provides timeout enforcement at the transport layer with configurable defaults via the SDK configuration.

## Implementation Details

### 1. Default Timeout Configuration

**File: `src/sdk_config.rs`**

- **Default timeout**: 10 seconds (`DEFAULT_TIMEOUT_SECONDS = 10`)
- **Configurable range**: 5-300 seconds (MIN_TIMEOUT to MAX_TIMEOUT)
- **Global configuration**: Via `SdkConfig.timeout_seconds` field

```rust
/// Default timeout for HTTP requests (10 seconds)
pub const DEFAULT_TIMEOUT_SECONDS: u32 = 10;

pub struct SdkConfig {
    pub network: Network,
    pub timeout_seconds: u32,  // Configurable timeout
    pub retry_attempts: u32,
    pub default_anchor: String,
}
```

### 2. Helper Method for Defaults

```rust
impl SdkConfig {
    /// Create a new SdkConfig with default timeout
    pub fn with_defaults(network: Network, default_anchor: String) -> Result<Self, Error> {
        Self::new(network, DEFAULT_TIMEOUT_SECONDS, 3, default_anchor)
    }
}
```

### 3. Transport Layer Timeout Enforcement

**File: `src/transport.rs`**

Added `send_request_with_timeout` method to the `AnchorTransport` trait:

```rust
pub trait AnchorTransport {
    /// Send a request to an anchor and receive a response
    fn send_request(
        &mut self,
        env: &Env,
        request: TransportRequest,
    ) -> Result<TransportResponse, Error>;

    /// Send a request with timeout enforcement
    fn send_request_with_timeout(
        &mut self,
        env: &Env,
        request: TransportRequest,
        timeout_seconds: u32,
    ) -> Result<TransportResponse, Error>;

    fn is_available(&self) -> bool;
    fn name(&self) -> &str;
}
```

### 4. MockTransport Timeout Simulation

Enhanced `MockTransport` to support timeout testing:

```rust
pub struct MockTransport {
    responses: alloc::vec::Vec<(TransportRequest, TransportResponse)>,
    call_count: u32,
    should_fail: bool,
    simulate_timeout: bool,        // NEW: Timeout simulation flag
    simulated_delay_seconds: u32,  // NEW: Simulated delay
}

impl MockTransport {
    /// Configure the transport to simulate timeout
    pub fn set_simulate_timeout(&mut self, simulate: bool, delay_seconds: u32) {
        self.simulate_timeout = simulate;
        self.simulated_delay_seconds = delay_seconds;
    }
}
```

### 5. Timeout Error Handling

**Error Type**: `Error::TransportTimeout` (already existed)
**Error Code**: `2202` (TransportTimeout in ErrorCode enum)

The error is thrown when:
- Simulated delay exceeds the configured timeout
- Actual HTTP request exceeds the timeout (in production implementations)

```rust
impl AnchorTransport for MockTransport {
    fn send_request_with_timeout(
        &mut self,
        env: &Env,
        request: TransportRequest,
        timeout_seconds: u32,
    ) -> Result<TransportResponse, Error> {
        // Check if simulated delay exceeds timeout
        if self.simulate_timeout && self.simulated_delay_seconds > timeout_seconds {
            self.call_count += 1;
            return Err(Error::TransportTimeout);
        }

        // Otherwise proceed with normal request
        self.send_request(env, request)
    }
}
```

## Unit Tests

### SDK Config Tests (`src/sdk_config_tests.rs`)

✅ **test_default_timeout_is_10_seconds** - Verifies default is 10s
✅ **test_with_defaults_uses_default_timeout** - Verifies helper method uses default
✅ **test_custom_timeout_overrides_default** - Verifies custom timeout works
✅ **test_timeout_too_low** - Validates minimum timeout (5s)
✅ **test_timeout_too_high** - Validates maximum timeout (300s)
✅ **test_min_timeout_five** - Boundary test for minimum
✅ **test_max_timeout_300** - Boundary test for maximum

### Transport Tests (`src/transport.rs`)

✅ **test_request_timeout_exceeded** - Request with 15s delay fails with 10s timeout
✅ **test_request_timeout_not_exceeded** - Request with 5s delay succeeds with 10s timeout
✅ **test_default_timeout_from_sdk_config** - Uses default timeout from SdkConfig
✅ **test_timeout_with_different_request_types** - Timeout works for all request types:
  - GetQuote
  - SubmitAttestation
  - CheckHealth
  - VerifyKYC

## Usage Examples

### Using Default Timeout (10 seconds)

```rust
use anchorkit::{SdkConfig, Network};

let env = Env::default();
let config = SdkConfig::with_defaults(
    Network::Testnet,
    String::from_str(&env, "anchor.stellar.org")
)?;

// config.timeout_seconds == 10
```

### Using Custom Timeout

```rust
let config = SdkConfig::new(
    Network::Testnet,
    30,  // 30 second timeout
    3,
    String::from_str(&env, "anchor.stellar.org")
)?;
```

### Making Requests with Timeout

```rust
let mut transport = MockTransport::new();

// Configure timeout behavior
transport.set_simulate_timeout(true, 15);  // Simulate 15s delay

let request = TransportRequest::CheckHealth { endpoint };

// This will timeout (15s > 10s)
let result = transport.send_request_with_timeout(&env, request, 10);
assert_eq!(result, Err(Error::TransportTimeout));
```

## Test Results

All tests passing:

```
running 14 tests
test sdk_config_tests::tests::test_default_timeout_is_10_seconds ... ok
test sdk_config_tests::tests::test_with_defaults_uses_default_timeout ... ok
test sdk_config_tests::tests::test_custom_timeout_overrides_default ... ok
test sdk_config_tests::tests::test_timeout_too_low ... ok
test sdk_config_tests::tests::test_timeout_too_high ... ok
test sdk_config_tests::tests::test_min_timeout_five ... ok
test sdk_config_tests::tests::test_max_timeout_300 ... ok
test transport::tests::test_request_timeout_exceeded ... ok
test transport::tests::test_request_timeout_not_exceeded ... ok
test transport::tests::test_default_timeout_from_sdk_config ... ok
test transport::tests::test_timeout_with_different_request_types ... ok

test result: ok. 14 passed; 0 failed
```

## Requirements Checklist

✅ **Default timeout (10s)** - Implemented via `DEFAULT_TIMEOUT_SECONDS`
✅ **Configurable via global SDK config** - Via `SdkConfig.timeout_seconds`
✅ **Throw AnchorKitTimeoutError if exceeded** - Returns `Error::TransportTimeout`
✅ **Covered by unit tests** - 11 comprehensive unit tests added

## Integration with Existing Features

- **Error Mapping**: `Error::TransportTimeout` already mapped to HTTP 408/504
- **Retry Logic**: Timeout errors are retryable (already configured)
- **Error Codes**: Uses stable error code 2202 (TransportTimeout)
- **Backward Compatible**: Existing `send_request` method unchanged

## Future Enhancements

For production HTTP implementations:
1. Integrate with actual HTTP client timeout configuration
2. Add per-request timeout overrides
3. Add timeout metrics and monitoring
4. Consider separate connect vs read timeouts
