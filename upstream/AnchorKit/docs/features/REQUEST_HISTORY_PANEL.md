# Request History Panel

The Request History Panel provides comprehensive tracking and monitoring of all API calls made to the AnchorKit contract. This feature enables developers to view recent API calls, their status, timestamps, and detailed information for debugging and monitoring purposes.

## Overview

The Request History Panel automatically tracks:
- All API calls with timestamps
- Success/failure status
- Duration of each operation
- Error codes for failed operations
- Detailed information about each call
- Aggregate statistics (total calls, success rate, etc.)

## Features

### 1. Automatic Call Tracking

API calls are automatically tracked when using the `*_tracked` variants of contract methods:

```rust
// Tracked methods automatically record API call history
client.submit_attestation_tracked(&issuer, &subject, &timestamp, &payload_hash, &signature)?;
client.submit_quote_tracked(&anchor, &base_asset, &quote_asset, &rate, ...)?;
client.register_attestor_tracked(&attestor)?;
```

### 2. Request History Panel Data

Retrieve the request history panel with recent API calls:

```rust
// Get up to 10 most recent API calls
let history = client.get_request_history(&10);

println!("Total Calls: {}", history.total_calls);
println!("Successful: {}", history.success_count);
println!("Failed: {}", history.failed_count);
println!("Last Updated: {}", history.last_updated);

// Iterate through recent calls
for call in history.recent_calls.iter() {
    println!("Operation: {}", call.operation);
    println!("Status: {:?}", call.status);
    println!("Duration: {}ms", call.duration_ms);
}
```

### 3. Detailed Call Information

Get detailed information about a specific API call:

```rust
let call_id = 42;
let details = client.get_api_call_details(&call_id);

if let Some(details) = details {
    println!("Call ID: {}", details.record.call_id);
    println!("Operation: {}", details.record.operation);
    println!("Timestamp: {}", details.record.timestamp);
    
    if let Some(target) = details.target_address {
        println!("Target: {:?}", target);
    }
    
    if let Some(result) = details.result_data {
        println!("Result: {}", result);
    }
}
```

### 4. Individual Call Retrieval

Retrieve a specific API call record:

```rust
let call = client.get_api_call(&call_id);

if let Some(call) = call {
    println!("Request ID: {:?}", call.request_id);
    println!("Caller: {:?}", call.caller);
    println!("Status: {:?}", call.status);
    
    if let Some(error_code) = call.error_code {
        println!("Error Code: {}", error_code);
    }
}
```

## Data Structures

### ApiCallRecord

Represents a single API call in the request history:

```rust
pub struct ApiCallRecord {
    pub call_id: u64,              // Unique call identifier
    pub request_id: BytesN<16>,    // Request UUID
    pub operation: String,          // Operation name (e.g., "submit_attestation")
    pub caller: Address,            // Address that made the call
    pub timestamp: u64,             // When the call was made
    pub status: ApiCallStatus,      // Success, Failed, or Pending
    pub duration_ms: u64,           // How long the operation took
    pub error_code: Option<u32>,    // Error code if failed
}
```

### ApiCallStatus

Status of an API call:

```rust
pub enum ApiCallStatus {
    Success = 1,  // Operation completed successfully
    Failed = 2,   // Operation failed with error
    Pending = 3,  // Operation is still in progress
}
```

### ApiCallDetails

Detailed information about an API call:

```rust
pub struct ApiCallDetails {
    pub record: ApiCallRecord,           // Basic call record
    pub target_address: Option<Address>, // Target of the operation
    pub amount: Option<u64>,             // Amount involved (if applicable)
    pub result_data: Option<String>,     // Result data (e.g., attestation ID)
}
```

### RequestHistoryPanel

Complete request history panel data:

```rust
pub struct RequestHistoryPanel {
    pub recent_calls: Vec<ApiCallRecord>, // Recent API calls
    pub total_calls: u64,                 // Total number of calls
    pub success_count: u64,               // Number of successful calls
    pub failed_count: u64,                // Number of failed calls
    pub last_updated: u64,                // Last update timestamp
}
```

## Error Codes

When an API call fails, the error code is automatically recorded:

| Error Code | Error Type | Description |
|------------|------------|-------------|
| 100 | AlreadyInitialized | Contract already initialized |
| 101 | NotInitialized | Contract not initialized |
| 102 | UnauthorizedAttestor | Attestor not authorized |
| 103 | AttestorAlreadyRegistered | Attestor already registered |
| 104 | AttestorNotRegistered | Attestor not found |
| 105 | InvalidTimestamp | Invalid timestamp provided |
| 106 | ReplayAttack | Replay attack detected |
| 107 | AttestationNotFound | Attestation not found |
| 108 | InvalidEndpointFormat | Invalid endpoint format |
| 109 | EndpointNotFound | Endpoint not found |
| 110 | InvalidServiceType | Invalid service type |
| 111 | ServicesNotConfigured | Services not configured |
| 112 | QuoteNotFound | Quote not found |
| 113 | InvalidQuote | Invalid quote data |
| 114 | StaleQuote | Quote has expired |
| 115 | NoQuotesAvailable | No quotes available |
| 116 | InvalidTransactionIntent | Invalid transaction intent |
| 117 | ComplianceNotMet | Compliance requirements not met |
| 118 | SessionNotFound | Session not found |
| 119 | InvalidSessionNonce | Invalid session nonce |
| 120 | CredentialNotFound | Credential not found |
| 121 | CredentialExpired | Credential has expired |
| 122 | InsecureCredentialStorage | Insecure credential storage |
| 123 | InvalidCredentialFormat | Invalid credential format |
| 124 | AnchorMetadataNotFound | Anchor metadata not found |
| 125 | InvalidAnchorMetadata | Invalid anchor metadata |
| 126 | NoAnchorsAvailable | No anchors available |
| 127 | RateLimitExceeded | Rate limit exceeded |
| 128 | InvalidConfig | Invalid configuration |

## Storage and TTL

Request history data is stored in temporary storage with a 1-day TTL:
- Individual call records: 1 day
- Call details: 1 day
- Recent calls list: 1 day
- Statistics: 1 day

The maximum history size is 100 calls. Older calls are automatically removed when the limit is reached.

## Usage Examples

### Example 1: Basic Request History

```rust
use anchorkit::{AnchorKitContract, AnchorKitContractClient};

let client = AnchorKitContractClient::new(&env, &contract_id);

// Perform some operations
client.register_attestor_tracked(&attestor1)?;
client.register_attestor_tracked(&attestor2)?;

// Get request history
let history = client.get_request_history(&10);

println!("Recent API Calls:");
for call in history.recent_calls.iter() {
    println!("  {} - {} ({:?})", 
        call.timestamp, 
        call.operation, 
        call.status
    );
}
```

### Example 2: Monitoring Failed Operations

```rust
// Get request history
let history = client.get_request_history(&50);

// Filter failed calls
for call in history.recent_calls.iter() {
    if call.status == ApiCallStatus::Failed {
        println!("Failed operation: {}", call.operation);
        
        if let Some(error_code) = call.error_code {
            println!("  Error code: {}", error_code);
        }
        
        // Get detailed information
        if let Some(details) = client.get_api_call_details(&call.call_id) {
            println!("  Caller: {:?}", details.record.caller);
            println!("  Timestamp: {}", details.record.timestamp);
        }
    }
}
```

### Example 3: Performance Monitoring

```rust
// Get request history
let history = client.get_request_history(&100);

// Calculate average duration
let mut total_duration = 0u64;
let mut count = 0u32;

for call in history.recent_calls.iter() {
    if call.status == ApiCallStatus::Success {
        total_duration += call.duration_ms;
        count += 1;
    }
}

if count > 0 {
    let avg_duration = total_duration / count as u64;
    println!("Average operation duration: {}ms", avg_duration);
}

// Calculate success rate
let success_rate = (history.success_count as f64 / history.total_calls as f64) * 100.0;
println!("Success rate: {:.2}%", success_rate);
```

### Example 4: Debugging Specific Operations

```rust
// Submit an attestation with tracking
let result = client.submit_attestation_tracked(
    &attestor,
    &subject,
    &timestamp,
    &payload_hash,
    &signature,
);

// Get the most recent call
let history = client.get_request_history(&1);
let last_call = history.recent_calls.get(0).unwrap();

// Check if it succeeded
if last_call.status == ApiCallStatus::Failed {
    println!("Operation failed!");
    
    if let Some(error_code) = last_call.error_code {
        println!("Error code: {}", error_code);
        
        // Get detailed information for debugging
        if let Some(details) = client.get_api_call_details(&last_call.call_id) {
            println!("Details: {:?}", details);
        }
    }
}
```

## CLI Integration

The Request History Panel can be integrated into the CLI:

```bash
# View recent API calls
anchorkit history --limit 10

# View detailed information about a specific call
anchorkit history --call-id 42

# Monitor API calls in real-time
anchorkit history --watch --interval 5

# Filter by status
anchorkit history --status failed

# Export history to JSON
anchorkit history --export history.json
```

## Best Practices

1. **Use Tracked Methods**: Always use `*_tracked` variants for operations you want to monitor
2. **Set Appropriate Limits**: Request only the number of records you need to avoid unnecessary data transfer
3. **Monitor Failed Operations**: Regularly check for failed operations and investigate error codes
4. **Performance Monitoring**: Track operation durations to identify performance bottlenecks
5. **Error Handling**: Use error codes to implement proper error handling and user feedback

## Integration with Other Features

The Request History Panel integrates seamlessly with:

- **Request ID Propagation**: Each call has a unique request ID for tracing
- **Session Management**: Calls can be associated with sessions for grouped tracking
- **Audit Logs**: Request history complements audit logs for compliance
- **Health Monitoring**: Track operation success rates for health metrics

## Limitations

- Maximum 100 calls stored in history
- 1-day TTL for all request history data
- Temporary storage (data is not permanent)
- No filtering or search capabilities (retrieve and filter client-side)

## Future Enhancements

Potential future improvements:
- Persistent storage option for long-term history
- Advanced filtering and search capabilities
- Aggregated metrics and analytics
- Real-time notifications for failed operations
- Integration with external monitoring systems

## See Also

- [REQUEST_ID_PROPAGATION.md](./REQUEST_ID_PROPAGATION.md) - Request ID tracking
- [SESSION_TRACEABILITY.md](./SESSION_TRACEABILITY.md) - Session management
- [API_SPEC.md](./API_SPEC.md) - Complete API specification
- [HEALTH_MONITORING.md](./HEALTH_MONITORING.md) - Health monitoring features
