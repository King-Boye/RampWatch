# Design Document: Tracing Span Validation

## Overview

This design specifies comprehensive test coverage for the existing tracing span and request ID functionality in the AnchorKit smart contract. The system already implements request ID generation (`RequestId::generate`), tracing span storage (`RequestTracker::store_span`), and retrieval (`RequestTracker::get_span`). This feature adds systematic validation to ensure these mechanisms work correctly across all operations, edge cases, and failure scenarios.

The validation framework will use property-based testing to verify universal correctness properties and unit tests for specific examples and edge cases. All tests will be implemented using the Soroban SDK's built-in testing utilities.

## Architecture

The validation system consists of three main components:

1. **Request ID Validation**: Tests for request ID generation, uniqueness, and timestamp accuracy
2. **Span Metadata Validation**: Tests for completeness and accuracy of tracing span fields
3. **Integration Validation**: Tests for request ID propagation through operations and span storage/retrieval

The testing approach follows a layered strategy:
- Unit tests validate individual components and specific scenarios
- Property-based tests validate universal properties across randomized inputs
- Integration tests validate end-to-end workflows

## Components and Interfaces

### Request ID Generator Tests

Tests the `RequestId::generate` function to ensure:
- Correct byte length (16 bytes)
- Uniqueness across multiple generations
- Accurate timestamp capture
- Deterministic behavior based on ledger state

### Tracing Span Validator Tests

Tests the `TracingSpan` structure and `RequestTracker` to ensure:
- Complete metadata capture (request_id, operation, actor, timestamps, status)
- Accurate timing information
- Correct status values
- Proper storage and retrieval

### Operation Integration Tests

Tests contract methods that use request IDs:
- `submit_with_request_id`: Attestation submission with tracing
- `quote_with_request_id`: Quote submission with tracing
- `get_tracing_span`: Span retrieval by request ID


## Data Models

### Test Data Structures

```rust
// Test helper for generating random valid attestations
struct TestAttestation {
    issuer: Address,
    subject: Address,
    timestamp: u64,
    payload_hash: BytesN<32>,
    signature: Bytes,
}

// Test helper for generating random valid quotes
struct TestQuote {
    anchor: Address,
    base_asset: String,
    quote_asset: String,
    rate: u64,
    fee_percentage: u32,
    minimum_amount: u64,
    maximum_amount: u64,
    valid_until: u64,
}

// Test helper for span validation
struct SpanAssertion {
    expected_request_id: BytesN<16>,
    expected_operation: String,
    expected_actor: Address,
    expected_status: String,
}
```

### Existing Data Models (from implementation)

```rust
pub struct RequestId {
    pub id: BytesN<16>,
    pub created_at: u64,
}

pub struct TracingSpan {
    pub request_id: RequestId,
    pub operation: String,
    pub actor: Address,
    pub started_at: u64,
    pub completed_at: u64,
    pub status: String,
}
```


## Correctness Properties

A property is a characteristic or behavior that should hold true across all valid executions of a system—essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.

### Request ID Generation Properties

Property 1: Request ID Length Invariant
*For any* generated request ID, the ID field must be exactly 16 bytes in length.
**Validates: Requirements 1.1**

Property 2: Request ID Uniqueness
*For any* sequence of request ID generations with changing ledger state, all generated IDs must be unique.
**Validates: Requirements 1.2**

Property 3: Timestamp Positivity
*For any* generated request ID, the created_at timestamp must be greater than zero.
**Validates: Requirements 1.3**

Property 4: Timestamp Accuracy
*For any* generated request ID, the created_at timestamp must equal the ledger timestamp at generation time.
**Validates: Requirements 1.4**

### Span Metadata Properties

Property 5: Request ID Preservation
*For any* operation with a request ID, the resulting tracing span must contain the same request ID.
**Validates: Requirements 2.1, 3.1, 3.2**

Property 6: Operation Name Presence
*For any* completed operation, the tracing span must contain a non-empty operation name.
**Validates: Requirements 2.2**

Property 7: Actor Attribution
*For any* operation performed by an actor, the tracing span must record that actor's address.
**Validates: Requirements 2.3, 7.1, 7.2**

Property 8: Success Status Consistency
*For any* successfully completed operation, the tracing span status must be "success".
**Validates: Requirements 2.4**

Property 9: Timing Invariant
*For any* tracing span, the completed_at timestamp must be greater than or equal to the started_at timestamp.
**Validates: Requirements 2.5, 4.3**

Property 10: Failure Status Recording
*For any* failed operation, the tracing span status must indicate failure (not "success").
**Validates: Requirements 2.6**

Property 11: Failure Metadata Completeness
*For any* failed operation, the tracing span must contain all required fields (request_id, operation, actor, timestamps, status).
**Validates: Requirements 2.7**


### Storage and Retrieval Properties

Property 12: Span Storage Round-Trip
*For any* tracing span stored with a request ID, retrieving by that request ID must return an equivalent span.
**Validates: Requirements 3.4, 6.1**

Property 13: Non-Existent ID Handling
*For any* request ID that has not been used to store a span, retrieval must return None.
**Validates: Requirements 6.2**

Property 14: Independent Span Retrieval
*For any* set of tracing spans with distinct request IDs, each span must be retrievable independently without affecting others.
**Validates: Requirements 6.4**

Property 15: Span Overwrite Behavior
*For any* request ID used for multiple operations, only the most recent operation's span must be retrievable.
**Validates: Requirements 3.3, 9.1**

### Operation Type Coverage Properties

Property 16: Attestation Span Creation
*For any* attestation submitted with a request ID, a tracing span must be created and retrievable.
**Validates: Requirements 5.1**

Property 17: Quote Span Creation
*For any* quote submitted with a request ID, a tracing span must be created and retrievable.
**Validates: Requirements 5.2**

Property 18: Operation Name Accuracy
*For any* operation type (attestation or quote), the tracing span operation name must correctly identify the operation type.
**Validates: Requirements 5.3**

Property 19: Failure Span Creation
*For any* operation that fails, a tracing span must still be created and retrievable.
**Validates: Requirements 5.4**

### Actor and Status Properties

Property 20: Unauthorized Actor Recording
*For any* operation that fails due to unauthorized access, the tracing span must record the attempted actor's address.
**Validates: Requirements 7.4**

Property 21: Status Value Validity
*For any* tracing span, the status field must contain either "success" or a failure indicator.
**Validates: Requirements 8.3**

Property 22: Status Field Population
*For any* completed operation, the tracing span status field must be non-empty.
**Validates: Requirements 8.4**


### Timing Properties

Property 23: Start Time Accuracy
*For any* operation, the tracing span started_at timestamp must be less than or equal to the ledger timestamp after operation completion.
**Validates: Requirements 4.1**

Property 24: Completion Time Accuracy
*For any* operation, the tracing span completed_at timestamp must reflect the ledger timestamp at or after operation completion.
**Validates: Requirements 4.2**

### Integration Properties

Property 25: Tracing Non-Interference with Success
*For any* operation that succeeds without tracing, the same operation with tracing must also succeed and return the same result.
**Validates: Requirements 10.1, 10.2**

## Error Handling

The validation tests will handle the following error scenarios:

1. **Invalid Request IDs**: Tests will verify behavior when querying non-existent request IDs
2. **Failed Operations**: Tests will verify spans are created even when operations fail
3. **Unauthorized Access**: Tests will verify actor recording when operations are rejected
4. **Concurrent Operations**: Tests will verify span storage handles multiple operations correctly

Error handling validation will use both positive tests (operations that should succeed) and negative tests (operations that should fail with specific errors).

## Testing Strategy

### Dual Testing Approach

The validation will use both unit tests and property-based tests:

- **Unit tests**: Verify specific examples, edge cases, and error conditions
- **Property tests**: Verify universal properties across randomized inputs

Both types of tests are complementary and necessary for comprehensive coverage. Unit tests catch concrete bugs in specific scenarios, while property tests verify general correctness across many inputs.

### Property-Based Testing Configuration

- **Framework**: Soroban SDK's built-in testing utilities with custom property test helpers
- **Iterations**: Minimum 100 iterations per property test (due to randomization)
- **Test Tagging**: Each property test must reference its design document property
- **Tag Format**: `// Feature: tracing-span-validation, Property {number}: {property_text}`


### Test Organization

Tests will be organized into modules:

1. **request_id_validation_tests.rs**: Properties 1-4 (request ID generation)
2. **span_metadata_validation_tests.rs**: Properties 5-11 (span metadata)
3. **span_storage_validation_tests.rs**: Properties 12-15 (storage/retrieval)
4. **operation_coverage_validation_tests.rs**: Properties 16-19 (operation types)
5. **actor_status_validation_tests.rs**: Properties 20-22 (actor and status)
6. **timing_validation_tests.rs**: Properties 23-24 (timing accuracy)
7. **integration_validation_tests.rs**: Property 25 (non-interference)

### Unit Test Strategy

Unit tests will focus on:
- Specific examples demonstrating correct behavior
- Edge cases (empty inputs, boundary values, concurrent operations)
- Error conditions (unauthorized access, invalid inputs)
- Integration points between tracing and existing operations

### Property Test Strategy

Property tests will focus on:
- Universal invariants (length, uniqueness, timing constraints)
- Round-trip properties (storage/retrieval, request ID preservation)
- Metamorphic properties (tracing doesn't change operation results)
- Error condition properties (failures still create spans)

### Test Data Generation

For property-based tests, we will generate:
- Random addresses for actors, attestors, and anchors
- Random timestamps within valid ranges
- Random payload hashes
- Random asset names and amounts
- Random ledger states (sequence numbers, timestamps)

### Validation Approach

Each property test will:
1. Generate random test inputs
2. Execute the operation with tracing enabled
3. Retrieve the tracing span
4. Assert the property holds
5. Repeat for minimum 100 iterations

Each unit test will:
1. Set up specific test scenario
2. Execute the operation
3. Verify expected behavior
4. Clean up test state

## Implementation Notes

- Tests will use `Env::default()` for test environment setup
- Tests will use `env.mock_all_auths()` to bypass authentication for test scenarios
- Tests will use `env.ledger().with_mut()` to manipulate ledger state for uniqueness testing
- Tests will use existing test utilities from `src/request_id_tests.rs` as reference
- All tests will be added to new test modules to avoid modifying existing test files
- Tests will verify behavior without modifying the implementation code
