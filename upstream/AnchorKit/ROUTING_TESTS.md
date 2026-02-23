# Multi-Anchor Routing Simulation Tests

## Overview
Tests for simulating routing decisions between multiple anchors based on rules like best quote selection and handling unavailable anchors.

## Test Implementation

### File: `src/routing_tests.rs`

### Test Cases

#### 1. `test_select_best_quote_from_multiple_anchors`
**Purpose**: Select the best quote from multiple available anchors.

**Setup**:
- 3 anchors with different rates:
  - Anchor1: 10100 (worst)
  - Anchor2: 10000 (best)
  - Anchor3: 10050 (medium)

**Assertions**:
- ✅ Anchor2 has the lowest rate
- ✅ All quotes are retrievable
- ✅ Rate comparison works correctly

#### 2. `test_select_lowest_fee_anchor`
**Purpose**: Identify anchor with lowest fees when rates are equal.

**Setup**:
- 2 anchors with same rate (10000) but different fees:
  - Anchor1: 50 basis points
  - Anchor2: 20 basis points

**Assertions**:
- ✅ Anchor2 has lower fee
- ✅ Fee comparison is accurate

#### 3. `test_handle_unavailable_anchors`
**Purpose**: Handle scenarios where some anchors are unavailable.

**Setup**:
- 2 registered anchors with quotes
- 1 unregistered (unavailable) anchor

**Assertions**:
- ✅ Unavailable anchor doesn't support services
- ✅ Only available anchors are considered

#### 4. `test_expired_quotes_filtered`
**Purpose**: Verify quotes with different expiration times.

**Setup**:
- Quote expiring soon (100 seconds)
- Quote valid for longer (3600 seconds)

**Assertions**:
- ✅ Expiration times are correctly set
- ✅ Valid quote has later expiration

#### 5. `test_no_anchors_available`
**Purpose**: Handle case when no anchors are registered.

**Setup**:
- No anchors registered
- Attempt to get quote from unregistered anchor

**Assertions**:
- ✅ Returns error for unavailable anchor

#### 6. `test_amount_outside_quote_limits`
**Purpose**: Verify quote amount limits are enforced.

**Setup**:
- Quote with min=100, max=100000

**Assertions**:
- ✅ Amounts below minimum are detected
- ✅ Amounts above maximum are detected

## Key Features

✅ **Best Quote Selection**: Identifies optimal rates  
✅ **Fee Comparison**: Selects lowest fees  
✅ **Availability Handling**: Gracefully handles unavailable anchors  
✅ **Expiration Validation**: Checks quote validity periods  
✅ **Limit Enforcement**: Validates amount boundaries  
✅ **Error Handling**: Proper error responses

## Running Tests

```bash
cargo test routing_tests --lib
```

## Test Results
```
running 6 tests
test routing_tests::test_amount_outside_quote_limits ... ok
test routing_tests::test_expired_quotes_filtered ... ok
test routing_tests::test_handle_unavailable_anchors ... ok
test routing_tests::test_no_anchors_available ... ok
test routing_tests::test_select_best_quote_from_multiple_anchors ... ok
test routing_tests::test_select_lowest_fee_anchor ... ok

test result: ok. 6 passed; 0 failed
```
