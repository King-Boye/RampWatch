# Implementation Plan: Tracing Span Validation

## Overview

This implementation plan creates comprehensive test coverage for the existing tracing span and request ID functionality. The tests will validate that request IDs are generated correctly, tracing spans contain accurate metadata, and the tracing system integrates properly with contract operations without affecting their behavior.

## Tasks

- [ ] 1. Set up test infrastructure and helpers
  - Create test module structure for validation tests
  - Implement test data generators for random addresses, timestamps, and payloads
  - Create assertion helpers for span validation
  - _Requirements: All requirements (foundation for testing)_

- [ ]* 1.1 Write property tests for request ID generation (Properties 1-4)
  - **Property 1: Request ID Length Invariant**
  - **Property 2: Request ID Uniqueness**
  - **Property 3: Timestamp Positivity**
  - **Property 4: Timestamp Accuracy**
  - **Validates: Requirements 1.1, 1.2, 1.3, 1.4**

- [ ]* 1.2 Write unit tests for request ID edge cases
  - Test request ID generation at ledger boundaries
  - Test request ID generation with maximum timestamp values
  - _Requirements: 1.1, 1.2, 1.3, 1.4_

- [ ] 2. Implement span metadata validation tests
  - [ ] 2.1 Create test helpers for attestation and quote operations
    - Implement helper to submit test attestations with request IDs
    - Implement helper to submit test quotes with request IDs
    - _Requirements: 2.1, 2.2, 2.3, 3.1, 3.2_

  - [ ]* 2.2 Write property tests for span metadata completeness (Properties 5-11)
    - **Property 5: Request ID Preservation**
    - **Property 6: Operation Name Presence**
    - **Property 7: Actor Attribution**
    - **Property 8: Success Status Consistency**
    - **Property 9: Timing Invariant**
    - **Property 10: Failure Status Recording**
    - **Property 11: Failure Metadata Completeness**
    - **Validates: Requirements 2.1, 2.2, 2.3, 2.4, 2.5, 2.6, 2.7, 3.1, 3.2, 7.1, 7.2**

  - [ ]* 2.3 Write unit tests for metadata edge cases
    - Test span creation with empty operation names (should not occur)
    - Test span creation with zero timestamps
    - Test span metadata for concurrent operations
    - _Requirements: 2.1, 2.2, 2.3, 2.4, 2.5_


- [ ] 3. Implement storage and retrieval validation tests
  - [ ] 3.1 Create test helpers for span storage operations
    - Implement helper to store spans with various request IDs
    - Implement helper to generate non-existent request IDs
    - _Requirements: 3.4, 6.1, 6.2, 6.4_

  - [ ]* 3.2 Write property tests for storage and retrieval (Properties 12-15)
    - **Property 12: Span Storage Round-Trip**
    - **Property 13: Non-Existent ID Handling**
    - **Property 14: Independent Span Retrieval**
    - **Property 15: Span Overwrite Behavior**
    - **Validates: Requirements 3.3, 3.4, 6.1, 6.2, 6.4, 9.1**

  - [ ]* 3.3 Write unit tests for storage edge cases
    - Test retrieval of spans immediately after storage
    - Test retrieval with maximum number of stored spans
    - Test span overwrite with same request ID
    - _Requirements: 3.3, 3.4, 6.1, 6.2, 6.4, 9.1_

- [ ] 4. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 5. Implement operation type coverage tests
  - [ ] 5.1 Create test scenarios for different operation types
    - Set up test scenarios for attestation operations
    - Set up test scenarios for quote operations
    - Set up test scenarios for failed operations
    - _Requirements: 5.1, 5.2, 5.3, 5.4_

  - [ ]* 5.2 Write property tests for operation coverage (Properties 16-19)
    - **Property 16: Attestation Span Creation**
    - **Property 17: Quote Span Creation**
    - **Property 18: Operation Name Accuracy**
    - **Property 19: Failure Span Creation**
    - **Validates: Requirements 5.1, 5.2, 5.3, 5.4**

  - [ ]* 5.3 Write unit tests for operation-specific scenarios
    - Test attestation with invalid timestamp (should fail but create span)
    - Test quote with expired validity (should fail but create span)
    - Test operation with unregistered attestor (should fail but create span)
    - _Requirements: 5.1, 5.2, 5.3, 5.4_


- [ ] 6. Implement actor and status validation tests
  - [ ] 6.1 Create test helpers for actor verification
    - Implement helper to verify actor addresses in spans
    - Implement helper to trigger unauthorized operations
    - _Requirements: 7.1, 7.2, 7.4, 8.3, 8.4_

  - [ ]* 6.2 Write property tests for actor and status (Properties 20-22)
    - **Property 20: Unauthorized Actor Recording**
    - **Property 21: Status Value Validity**
    - **Property 22: Status Field Population**
    - **Validates: Requirements 7.4, 8.3, 8.4**

  - [ ]* 6.3 Write unit tests for actor and status edge cases
    - Test span with unauthorized attestor attempt
    - Test span with unauthorized anchor attempt
    - Test status field values for various failure types
    - _Requirements: 7.4, 8.3, 8.4_

- [ ] 7. Implement timing validation tests
  - [ ] 7.1 Create test helpers for timing verification
    - Implement helper to capture ledger timestamps during operations
    - Implement helper to verify timing constraints
    - _Requirements: 4.1, 4.2_

  - [ ]* 7.2 Write property tests for timing accuracy (Properties 23-24)
    - **Property 23: Start Time Accuracy**
    - **Property 24: Completion Time Accuracy**
    - **Validates: Requirements 4.1, 4.2**

  - [ ]* 7.3 Write unit tests for timing edge cases
    - Test timing with rapid successive operations
    - Test timing with ledger timestamp changes
    - _Requirements: 4.1, 4.2_

- [ ] 8. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.


- [ ] 9. Implement integration validation tests
  - [ ] 9.1 Create test scenarios comparing traced and untraced operations
    - Implement helper to execute operations without tracing
    - Implement helper to execute same operations with tracing
    - Implement comparison logic for operation results
    - _Requirements: 10.1, 10.2_

  - [ ]* 9.2 Write property test for tracing non-interference (Property 25)
    - **Property 25: Tracing Non-Interference with Success**
    - **Validates: Requirements 10.1, 10.2**

  - [ ]* 9.3 Write unit tests for integration scenarios
    - Test attestation submission with and without tracing
    - Test quote submission with and without tracing
    - Verify return values match between traced and untraced operations
    - _Requirements: 10.1, 10.2_

- [ ] 10. Final validation and documentation
  - [ ] 10.1 Run complete test suite
    - Execute all property tests with minimum 100 iterations
    - Execute all unit tests
    - Verify all tests pass
    - _Requirements: All requirements_

  - [ ]* 10.2 Document test coverage
    - Create test coverage report mapping tests to requirements
    - Document any limitations or edge cases not covered
    - _Requirements: All requirements_

- [ ] 11. Final checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

## Notes

- Tasks marked with `*` are optional and can be skipped for faster MVP
- Each property test must run minimum 100 iterations
- Each property test must include a comment tag: `// Feature: tracing-span-validation, Property {number}: {property_text}`
- Tests should not modify existing implementation code
- Tests will use existing test patterns from `src/request_id_tests.rs` as reference
- All test modules will be new files to avoid modifying existing tests
