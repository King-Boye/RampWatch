# SDK Configuration

## Overview

The SDK Configuration module provides a type-safe way to configure AnchorKit SDK client connections with network settings, anchor domains, timeouts, and custom HTTP headers.

## Data Structures

### SdkConfig

Main configuration structure for SDK clients.

```rust
pub struct SdkConfig {
    pub network: NetworkType,
    pub anchor_domain: String,
    pub timeout_seconds: u64,
    pub custom_headers: Vec<HttpHeader>,
}
```

### NetworkType

Enum for Stellar network selection.

```rust
pub enum NetworkType {
    Testnet = 1,
    Mainnet = 2,
}
```

### HttpHeader

Custom HTTP header for API requests.

```rust
pub struct HttpHeader {
    pub key: String,
    pub value: String,
}
```

## Validation Rules

The `SdkConfig::validate()` method enforces the following constraints:

### Anchor Domain
- **Minimum length**: 3 characters
- **Maximum length**: 253 characters
- **Format**: Valid domain name

### Timeout
- **Minimum**: 1 second
- **Maximum**: 300 seconds (5 minutes)
- **Default**: 30 seconds (recommended)

### Custom Headers
- **Maximum count**: 20 headers
- **Header key length**: 1-64 characters
- **Header value length**: 0-1024 characters

## Usage Example

### Rust (Contract)

```rust
use soroban_sdk::{Env, String, Vec};

let env = Env::default();

// Create headers
let mut headers = Vec::new(&env);
headers.push_back(HttpHeader {
    key: String::from_str(&env, "Authorization"),
    value: String::from_str(&env, "Bearer token123"),
});

// Create config
let config = SdkConfig {
    network: NetworkType::Testnet,
    anchor_domain: String::from_str(&env, "anchor.example.com"),
    timeout_seconds: 30,
    custom_headers: headers,
};

// Validate
if config.validate() {
    // Use config
}
```

### JavaScript (Client SDK)

```javascript
const config = {
    network: 'Testnet',
    anchor_domain: 'anchor.example.com',
    timeout_seconds: 30,
    custom_headers: [
        {
            key: 'Authorization',
            value: 'Bearer token123'
        },
        {
            key: 'X-Custom-Header',
            value: 'custom-value'
        }
    ]
};
```

## Configuration Form

An HTML form is provided in `sdk_config_form.html` for easy configuration generation. The form includes:

- Network selection (Testnet/Mainnet)
- Anchor domain input with validation
- Timeout configuration
- Dynamic custom header management
- JSON output generation

### Using the Form

1. Open `sdk_config_form.html` in a web browser
2. Select your network (Testnet or Mainnet)
3. Enter the anchor domain
4. Set the timeout (default: 30 seconds)
5. Add custom headers as needed
6. Click "Generate Configuration" to get JSON output

## Security Considerations

### Header Security
- Never include sensitive credentials directly in headers
- Use secure credential management (see `SECURE_CREDENTIALS.md`)
- Rotate tokens regularly
- Use HTTPS for all anchor communications

### Domain Validation
- Validate anchor domains against a whitelist
- Use DNS verification for production
- Implement certificate pinning for critical operations

### Timeout Configuration
- Set appropriate timeouts based on network conditions
- Consider retry logic for transient failures
- Monitor timeout rates for performance tuning

## Best Practices

### Network Selection
- Use **Testnet** for development and testing
- Use **Mainnet** only for production deployments
- Never mix testnet and mainnet configurations

### Timeout Settings
- **Development**: 60-120 seconds (for debugging)
- **Production**: 30 seconds (recommended)
- **High-latency networks**: 60-90 seconds
- **Low-latency networks**: 15-30 seconds

### Custom Headers
- Use headers for:
  - Authentication tokens
  - API versioning
  - Request tracing
  - Custom metadata
- Avoid headers for:
  - Large payloads (use request body)
  - Sensitive data without encryption
  - Unnecessary metadata

## Integration with AnchorKit

The SDK configuration integrates with:

- **Session Management**: Timeout settings affect session duration
- **Credential Management**: Headers can include auth tokens
- **Health Monitoring**: Timeout affects health check intervals
- **Rate Comparison**: Network selection determines available anchors

## Testing

Run the SDK configuration tests:

```bash
cargo test sdk_config_tests --lib
```

Test coverage includes:
- Valid configuration validation
- Domain length constraints
- Timeout boundary conditions
- Header count limits
- Header size constraints
- Network type enum values

## Error Handling

Configuration validation returns a boolean. For detailed error handling, check specific constraints:

```rust
if !config.validate() {
    // Check individual constraints
    if config.anchor_domain.len() < 3 {
        // Handle domain too short
    }
    if config.timeout_seconds < 1 || config.timeout_seconds > 300 {
        // Handle invalid timeout
    }
    if config.custom_headers.len() > 20 {
        // Handle too many headers
    }
}
```

## Future Enhancements

Potential improvements:
- Add retry configuration
- Support for connection pooling settings
- Circuit breaker configuration
- Rate limiting settings
- Custom DNS resolver configuration
- Proxy support

## Related Documentation

- [SECURE_CREDENTIALS.md](./SECURE_CREDENTIALS.md) - Credential management
- [HEALTH_MONITORING.md](./HEALTH_MONITORING.md) - Health check configuration
- [API_SPEC.md](./API_SPEC.md) - API specifications
- [QUICK_START.md](./QUICK_START.md) - Getting started guide
