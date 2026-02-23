# Streaming Flow Lifecycle Tests

## Overview
Tests for multi-step flows using async streams with state transitions: **pending → awaiting_user → completed**

## Test Implementation

### File: `src/streaming_flow_tests.rs`

### State Machine
```rust
enum FlowState {
    Pending,      // Initial state - flow created
    AwaitingUser, // Waiting for user action
    Completed,    // Flow finished
}
```

### Test Cases

#### 1. `test_streaming_flow_pending_to_awaiting_user_to_completed`
**Purpose**: Validates basic state transition flow with quote submission and receipt.

**Flow**:
- **PENDING**: Create session
- **AWAITING_USER**: Submit quote from anchor
- **COMPLETED**: User receives quote

**Assertions**:
- State transitions occur correctly
- Quote data is preserved
- Session is properly tracked

#### 2. `test_multi_step_async_stream_with_attestation`
**Purpose**: Tests async stream with attestation submission.

**Flow**:
- **PENDING**: Create session
- **AWAITING_USER**: Submit attestation with session tracking
- **COMPLETED**: Verify session and attestation recorded

**Assertions**:
- Attestation ID is valid
- Session operation count increments
- State transitions are deterministic

#### 3. `test_concurrent_streaming_flows`
**Purpose**: Validates multiple concurrent flows don't interfere.

**Flow**:
- Two parallel flows with different users
- Each flow: PENDING → AWAITING_USER → COMPLETED
- Different quote parameters per flow

**Assertions**:
- Both flows complete independently
- No state leakage between flows
- Concurrent execution is safe

## Key Features

✅ **State Isolation**: Each flow maintains independent state  
✅ **Async Simulation**: Multi-step operations simulate async behavior  
✅ **Deterministic**: Tests are reproducible and predictable  
✅ **Minimal**: Only essential code for state transitions  
✅ **Concurrent Safe**: Multiple flows can execute simultaneously

## Running Tests

```bash
cargo test streaming_flow_tests --lib
```

## Integration

Tests integrate with existing AnchorKit contract features:
- Session management
- Quote submission/receipt
- Attestation tracking
- Service configuration
