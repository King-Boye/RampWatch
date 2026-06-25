# AnchorKit Structured Logging

This document describes the structured logging implementation in AnchorKit, including debug mode toggle and request/response logging with sensitive data redaction.

## Features

### ✅ Structured Logs
- **Multiple log levels**: ERROR, WARN, INFO, DEBUG, TRACE
- **Structured data**: JSON-formatted metadata and context
- **Request correlation**: Request IDs for distributed tracing
- **Timestamp tracking**: Automatic timestamping of all log entries
- **Actor tracking**: Log which address performed each operation

### ✅ Debug Mode Toggle
- **CLI flags**: `--debug` and `--verbose` for enabling debug output
- **Runtime configuration**: Configure logging via contract methods
- **Level filtering**: Debug/trace logs filtered out in production mode
- **Performance optimized**: No overhead when debug mode is disabled

### ✅ Request/Response Logging
- **HTTP request logging**: Method, endpoint, payload, timing
- **HTTP response logging**: Status, duration, response payload
- **Timing information**: Automatic duration calculation
- **Payload size tracking**: Monitor request/response sizes
- **Correlation**: Link requests and responses via request IDs

### ✅ Sensitive Data Redaction
- **Pattern-based redaction**: Automatically detect sensitive fields
- **Configurable**: Enable/disable redaction per environment
- **Security-first**: Redaction enabled by default
- **Truncation**: Limit log size to prevent memory issues

## Usage

### CLI Integration

```bash
# Enable debug mode
anchorkit --debug build
anchorkit --debug --verbose deploy --network testnet

# Disable request logging (reduce log volume)
anchorkit --debug --no-request-logging health

# Disable redaction (development only)
anchorkit --debug --no-redaction attest --subject GUSER123 --payload-hash abc123
```

### Contract Integration

```rust
use anchorkit::{Logger, LoggingConfig, RequestId};

// Configure logging
let config = LoggingConfig {
    debug_mode: true,
    log_requests: true,
    log_responses: true,
    redact_sensitive: true,
    max_log_size: 2048,
};
AnchorKitContract::configure_logging(env, config)?;

// Log operations
let request_id = RequestId::generate(&env);
Logger::info(&env, String::from_str(&env, "Operation started"), Some(request_id));
Logger::error(&env, String::from_str(&env, "Operation failed"), Some(request_id), Some(error));
```

### Operation Logging

```rust
// Automatic operation tracking
Logger::operation_start(&env, operation_name, actor, request_id, params);
// ... perform operation ...
Logger::operation_complete(&env, operation_name, actor, request_id, duration_ms, success);
```

### Request/Response Logging

```rust
// Log HTTP requests
Logger::log_request(&env, request_id, method, endpoint, payload);

// Log HTTP responses  
Logger::log_response(&env, request_id, status, duration_ms, response_payload);
```

## Configuration

### LoggingConfig Structure

```rust
pub struct LoggingConfig {
    pub debug_mode: bool,        // Enable debug/trace logs
    pub log_requests: bool,      // Log HTTP requests
    pub log_responses: bool,     // Log HTTP responses
    pub redact_sensitive: bool,  // Redact sensitive data
    pub max_log_size: u32,      // Maximum log entry size
}
```

### Default Configuration

```rust
LoggingConfig {
    debug_mode: false,           // Production-safe default
    log_requests: true,          // Monitor network activity
    log_responses: true,         // Monitor network activity
    redact_sensitive: true,      // Security-first approach
    max_log_size: 1024,         // Reasonable size limit
}
```

## Log Levels

| Level | Description | When to Use |
|-------|-------------|-------------|
| **ERROR** | System errors, failures | Always logged, triggers alerts |
| **WARN** | Warnings, degraded performance | Always logged, monitoring |
| **INFO** | Normal operations, state changes | Always logged, audit trail |
| **DEBUG** | Detailed debugging information | Only in debug mode |
| **TRACE** | Very detailed execution flow | Only in debug mode |

## Sensitive Data Patterns

The following patterns are automatically redacted when `redact_sensitive: true`:

- `password`
- `secret`
- `key`
- `token`
- `auth`
- `credential`
- `private`
- `seed`
- `mnemonic`

## Event Integration

All logs are published as Soroban events for external consumption:

```rust
// Log entries
env.events().publish(("log", "entry"), LogEntry { ... });

// HTTP requests
env.events().publish(("http", "request"), RequestLog { ... });

// HTTP responses
env.events().publish(("http", "response"), RequestLog { ... });
```

## Performance Considerations

### Debug Mode Filtering
- Debug/trace logs are filtered at the source when debug mode is disabled
- No performance overhead for disabled log levels
- String formatting only occurs when logs will be emitted

### Log Size Limits
- Configurable maximum log size prevents memory issues
- Large payloads are truncated with `[TRUNCATED]` indicator
- Payload size is always tracked regardless of truncation

### Request Correlation
- Request IDs enable distributed tracing
- Minimal overhead for ID generation
- Optional correlation - can be omitted for performance

## Security Best Practices

### Production Configuration
```rust
LoggingConfig {
    debug_mode: false,          // Disable verbose logging
    log_requests: true,         // Monitor for security
    log_responses: true,        // Monitor for security  
    redact_sensitive: true,     // Always redact in production
    max_log_size: 1024,        // Limit log size
}
```

### Development Configuration
```rust
LoggingConfig {
    debug_mode: true,           // Enable debugging
    log_requests: true,         // Full request logging
    log_responses: true,        // Full response logging
    redact_sensitive: false,    // Optional: disable for debugging
    max_log_size: 4096,        // Larger logs for debugging
}
```

## Integration Examples

### Monitoring System Integration

```bash
# Capture Soroban events for monitoring
soroban events --start-ledger 1000 --filter "log" | jq '.[] | select(.type == "contract")'
```

### Log Aggregation

```bash
# Stream logs to external system
soroban events --start-ledger 1000 --filter "log" | \
  while read event; do
    curl -X POST https://logs.example.com/ingest \
      -H "Content-Type: application/json" \
      -d "$event"
  done
```

### Alerting

```bash
# Alert on ERROR level logs
soroban events --start-ledger 1000 --filter "log" | \
  jq -r '.[] | select(.data.level == "Error") | .data.message' | \
  while read error; do
    echo "ALERT: $error" | mail -s "AnchorKit Error" admin@example.com
  done
```

## Testing

### Run Logging Tests
```bash
cargo test logging_tests
```

### Run Logging Example
```bash
cargo run --example logging_example
```

### Run Demo Script
```bash
./examples/logging_demo.sh
```

## Migration Guide

### Existing Code
If you have existing logging code, you can migrate gradually:

```rust
// Old approach
println!("Operation completed");

// New approach
Logger::info(&env, String::from_str(&env, "Operation completed"), Some(request_id));
```

### Event Listeners
Update your event listeners to capture the new log events:

```rust
// Listen for log entries
env.events().subscribe(("log", "entry"), |event| {
    // Process structured log entry
});

// Listen for HTTP logs
env.events().subscribe(("http", "request"), |event| {
    // Process HTTP request log
});
```

## Troubleshooting

### Common Issues

1. **Debug logs not appearing**
   - Ensure `debug_mode: true` in configuration
   - Check that you're using `--debug` CLI flag

2. **Sensitive data not redacted**
   - Verify `redact_sensitive: true` in configuration
   - Check that sensitive patterns are recognized

3. **Logs truncated**
   - Increase `max_log_size` in configuration
   - Consider if full payload logging is necessary

4. **Performance impact**
   - Disable debug mode in production
   - Reduce `max_log_size` if needed
   - Consider disabling request/response logging for high-volume endpoints

### Debug Commands

```bash
# Check current logging configuration
anchorkit --debug query --id 1  # Should show debug logs

# Test redaction
anchorkit --debug --no-redaction attest --subject GUSER123 --payload-hash abc123

# Test without request logging
anchorkit --debug --no-request-logging health
```

## Future Enhancements

- [ ] Log rotation and archival
- [ ] Custom redaction patterns
- [ ] Log sampling for high-volume scenarios
- [ ] Integration with OpenTelemetry
- [ ] Structured query interface
- [ ] Real-time log streaming
- [ ] Log-based metrics and dashboards