# Implementation Plan: Idempotency Protection for Transfer Requests

## Overview

This implementation adds idempotency protection to the `create_remittance` function in the SwiftRemit contract. The approach follows a minimal modification strategy: add new storage functions, extend error types, and modify only the transfer request handling logic. All changes are isolated to transfer-related code with no impact on other contract functions.

## Tasks

- [ ] 1. Add idempotency data structures and error types
  - Add `IdempotencyRecord` struct to `types.rs` with fields: key, request_hash, remittance_id, expires_at
  - Add `IdempotencyConflict` error variant to `errors.rs` with error code 23
  - Add `IdempotencyRecord(String)` and `IdempotencyTTL` keys to `DataKey` enum in `storage.rs`
  - _Requirements: 3.2, 5.2_

- [ ] 2. Implement idempotency storage functions
  - [ ] 2.1 Implement storage helper functions
    - Write `get_idempotency_record(env, key)` to retrieve records from persistent storage
    - Write `set_idempotency_record(env, key, record)` to store records in persistent storage
    - Write `get_idempotency_ttl(env)` to retrieve TTL from instance storage (default: 86400)
    - Write `set_idempotency_ttl(env, ttl_seconds)` to configure TTL in instance storage
    - _Requirements: 3.1, 3.3, 6.1, 6.6_
  
  - [ ]* 2.2 Write property test for storage functions
    - **Property 11: Successful Request Storage**
    - **Validates: Requirements 3.1, 3.2**

- [ ] 3. Implement request hash computation
  - [ ] 3.1 Create hash computation function
    - Write `compute_request_hash(env, sender, agent, amount, expiry)` in `hashing.rs`
    - Use SHA-256 to hash the concatenation of all parameters
    - Return `BytesN<32>` hash value
    - _Requirements: 2.1, 2.2, 2.5_
  
  - [ ]* 3.2 Write property tests for hash computation
    - **Property 1: Hash Determinism**
    - **Validates: Requirements 2.1, 2.3**
  
  - [ ]* 3.3 Write property test for hash sensitivity
    - **Property 2: Hash Sensitivity**
    - **Validates: Requirements 2.4**
  
  - [ ]* 3.4 Write property test for hash input completeness
    - **Property 3: Hash Input Completeness**
    - **Validates: Requirements 2.2**

- [ ] 4. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 5. Modify create_remittance function for idempotency
  - [ ] 5.1 Add idempotency_key parameter
    - Add `idempotency_key: Option<String>` parameter to `create_remittance` function signature
    - Update function documentation to describe idempotency behavior
    - _Requirements: 1.1_
  
  - [ ] 5.2 Implement idempotency check logic
    - After existing validation, check if `idempotency_key` is `Some`
    - If present, compute request hash using `compute_request_hash`
    - Call `get_idempotency_record` to check for existing record
    - If record exists and not expired, compare hashes
    - If hashes match, return stored `remittance_id` immediately
    - If hashes differ, return `IdempotencyConflict` error
    - If no record or expired, continue to transfer execution
    - _Requirements: 4.1, 4.2, 5.1, 5.2, 6.3_
  
  - [ ] 5.3 Implement idempotency record storage
    - After successful transfer execution, check if `idempotency_key` is `Some`
    - If present, calculate `expires_at = current_time + get_idempotency_ttl()`
    - Create `IdempotencyRecord` with key, hash, remittance_id, expires_at
    - Call `set_idempotency_record` to store the record
    - _Requirements: 3.1, 6.2_
  
  - [ ]* 5.4 Write property test for idempotent retry
    - **Property 4: Idempotent Retry Returns Same Response**
    - **Validates: Requirements 4.2**
  
  - [ ]* 5.5 Write property test for no side effects
    - **Property 5: Idempotent Retry Has No Side Effects**
    - **Validates: Requirements 4.3, 4.4**
  
  - [ ]* 5.6 Write property test for conflict detection
    - **Property 6: Idempotency Conflict Detection**
    - **Validates: Requirements 5.2, 5.3**
  
  - [ ]* 5.7 Write unit tests for idempotency scenarios
    - Test retry with same parameters returns same remittance_id
    - Test retry with different amount returns IdempotencyConflict error
    - Test conflict error includes both expected and actual hashes
    - _Requirements: 4.2, 5.2, 5.4_

- [ ] 6. Implement expiration handling
  - [ ] 6.1 Add expiration check in idempotency lookup
    - In `get_idempotency_record`, check if `current_time > record.expires_at`
    - If expired, return `None` to treat as non-existent
    - _Requirements: 6.3_
  
  - [ ]* 6.2 Write property test for expired key behavior
    - **Property 7: Expired Keys Allow Re-execution**
    - **Validates: Requirements 6.3, 6.4, 6.5**
  
  - [ ]* 6.3 Write property test for expiration timestamp
    - **Property 8: Expiration Timestamp Calculation**
    - **Validates: Requirements 6.2**
  
  - [ ]* 6.4 Write unit tests for expiration scenarios
    - Test expired record allows new execution
    - Test expired record is overwritten with new data
    - Test default TTL is 24 hours (86400 seconds)
    - _Requirements: 6.4, 6.5, 6.6_

- [ ] 7. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 8. Implement backward compatibility and validation
  - [ ] 8.1 Add idempotency key validation
    - Validate key length (max 255 characters)
    - Validate key characters (alphanumeric, hyphens, underscores)
    - Return appropriate error if validation fails
    - _Requirements: 1.3, 1.4_
  
  - [ ]* 8.2 Write property test for key acceptance
    - **Property 10: Idempotency Key Acceptance**
    - **Validates: Requirements 1.1, 1.3, 1.4**
  
  - [ ]* 8.3 Write property test for backward compatibility
    - **Property 9: Backward Compatibility**
    - **Validates: Requirements 1.2, 7.3, 7.4**
  
  - [ ]* 8.4 Write property test for failed request no storage
    - **Property 12: Failed Request No Storage**
    - **Validates: Requirements 3.4**
  
  - [ ]* 8.5 Write unit tests for backward compatibility
    - Test request without idempotency key executes normally
    - Test request without key doesn't create idempotency record
    - Test existing validation still works (InvalidAmount, AgentNotRegistered)
    - _Requirements: 1.2, 7.1, 7.3, 7.4_

- [ ] 9. Add admin function for TTL configuration
  - [ ] 9.1 Implement set_idempotency_ttl admin function
    - Add public contract function `set_idempotency_ttl(env, caller, ttl_seconds)`
    - Require admin authorization using `require_admin`
    - Call storage function `set_idempotency_ttl` to persist value
    - _Requirements: 6.1_
  
  - [ ]* 9.2 Write unit tests for TTL configuration
    - Test admin can set TTL
    - Test non-admin cannot set TTL
    - Test configured TTL is used for new records
    - _Requirements: 6.1_

- [ ] 10. Final checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

## Notes

- Tasks marked with `*` are optional and can be skipped for faster MVP
- Each task references specific requirements for traceability
- Checkpoints ensure incremental validation
- Property tests validate universal correctness properties (minimum 100 iterations each)
- Unit tests validate specific examples and edge cases
- All modifications are isolated to transfer request handling and supporting storage
- No changes to `confirm_payout`, `cancel_remittance`, or other non-transfer functions
