# Request ID Propagation

## Overview

Generate unique request IDs for each flow and propagate across logs and tracing spans.

## Features

- ✅ UUID per flow (128-bit)
- ✅ Visible in tracing spans
- ✅ Automatic timing tracking
- ✅ Success/failure status recording

## Usage

### Generate Request ID

```rust
let request_id = client.generate_request_id();
```

### Submit with Request ID

```rust
let attestation_id = client.submit_with_request_id(
    &request_id,
    &issuer,
    &subject,
    &timestamp,
    &payload_hash,
    &signature,
);
```

### Retrieve Tracing Span

```rust
let span = client.get_tracing_span(&request_id.id);

println!("Operation: {}", span.operation);
println!("Actor: {}", span.actor);
println!("Status: {}", span.status);
println!("Duration: {} seconds", span.completed_at - span.started_at);
```

## API Methods

```rust
// Generate unique request ID
pub fn generate_request_id() -> RequestId

// Submit attestation with tracking
pub fn submit_with_request_id(
    request_id: RequestId,
    issuer: Address,
    subject: Address,
    timestamp: u64,
    payload_hash: BytesN<32>,
    signature: Bytes,
) -> Result<u64, Error>

// Submit quote with tracking
pub fn quote_with_request_id(
    request_id: RequestId,
    anchor: Address,
    base_asset: String,
    quote_asset: String,
    rate: u64,
    fee_percentage: u32,
    minimum_amount: u64,
    maximum_amount: u64,
    valid_until: u64,
) -> Result<u64, Error>

// Get tracing span
pub fn get_tracing_span(request_id: BytesN<16>) -> Option<TracingSpan>
```

## Tracing Span Structure

```rust
pub struct TracingSpan {
    pub request_id: RequestId,
    pub operation: String,      // Operation name
    pub actor: Address,          // Who performed it
    pub started_at: u64,         // Start timestamp
    pub completed_at: u64,       // End timestamp
    pub status: String,          // "success" or "failed"
}
```

## Use Cases

### Debugging
Track request flow through system for troubleshooting.

### Performance Monitoring
Measure operation duration via `completed_at - started_at`.

### Audit Trail
Complete record of who did what and when.

### Distributed Tracing
Correlate operations across multiple calls.

## Storage

Tracing spans stored in temporary storage with 1-day TTL.

## Best Practices

1. **Generate once per flow** - Reuse same request ID for related operations
2. **Check span status** - Verify success/failure in tracing data
3. **Monitor timing** - Track performance via span timestamps
4. **Log request IDs** - Include in external logs for correlation
