# Property-Based Testing Architecture

## Overview

This document provides a visual overview of the property-based testing architecture for SwiftRemit.

## Test Structure

```
┌─────────────────────────────────────────────────────────────┐
│                    Property Test Suite                       │
│                   (src/test_property.rs)                     │
└─────────────────────────────────────────────────────────────┘
                              │
                              │
        ┌─────────────────────┼─────────────────────┐
        │                     │                     │
        ▼                     ▼                     ▼
┌──────────────┐      ┌──────────────┐      ┌──────────────┐
│  Invariant 1 │      │  Invariant 2 │      │  Invariant 3 │
│   Balance    │      │   Negative   │      │ Determinism  │
│ Conservation │      │  Prevention  │      │              │
└──────────────┘      └──────────────┘      └──────────────┘
   │  │  │               │  │                    │
   ▼  ▼  ▼               ▼  ▼                    ▼
  3 tests              2 tests                 1 test

        │                     │                     │
        ▼                     ▼                     ▼
┌──────────────┐      ┌──────────────┐      ┌──────────────┐
│  Invariant 4 │      │  Invariant 5 │      │  Invariant 6 │
│     Fee      │      │    State     │      │ Idempotency  │
│ Calculation  │      │  Transitions │      │              │
└──────────────┘      └──────────────┘      └──────────────┘
   │  │                   │  │                    │
   ▼  ▼                   ▼  ▼                    ▼
  2 tests                2 tests                 1 test

                              │
                              ▼
                      ┌──────────────┐
                      │  Invariant 7 │
                      │     Net      │
                      │  Settlement  │
                      └──────────────┘
                              │
                              ▼
                           1 test
```

## Test Flow

```
┌─────────────┐
│   Start     │
└──────┬──────┘
       │
       ▼
┌─────────────────────────────┐
│  Generate Random Inputs     │
│  - amount: 1..1,000,000     │
│  - fee_bps: 0..1000         │
│  - batch_size: 1..20        │
└──────┬──────────────────────┘
       │
       ▼
┌─────────────────────────────┐
│  Setup Test Environment     │
│  - Create token contract    │
│  - Initialize SwiftRemit    │
│  - Register agents          │
│  - Mint tokens              │
└──────┬──────────────────────┘
       │
       ▼
┌─────────────────────────────┐
│  Execute Operation          │
│  - Create remittance        │
│  - Settle remittance        │
│  - Cancel remittance        │
│  - Compute net settlement   │
└──────┬──────────────────────┘
       │
       ▼
┌─────────────────────────────┐
│  Validate Invariant         │
│  - Check property holds     │
│  - Assert conditions        │
│  - Verify balances          │
└──────┬──────────────────────┘
       │
       ├─── Pass ──────────────┐
       │                       │
       └─── Fail ──────────┐   │
                           │   │
                           ▼   │
                    ┌──────────┴────┐
                    │   Shrinking   │
                    │  Find minimal │
                    │ failing input │
                    └──────┬────────┘
                           │
                           ▼
                    ┌─────────────┐
                    │   Report    │
                    │   Failure   │
                    └─────────────┘
```

## Invariant Hierarchy

```
                    ┌─────────────────────┐
                    │  Contract Integrity │
                    └──────────┬──────────┘
                               │
                ┌──────────────┼──────────────┐
                │              │              │
                ▼              ▼              ▼
        ┌──────────────┐ ┌──────────┐ ┌──────────────┐
        │   Financial  │ │  State   │ │ Algorithmic  │
        │  Invariants  │ │ Machine  │ │  Invariants  │
        └──────┬───────┘ └────┬─────┘ └──────┬───────┘
               │              │              │
       ┌───────┼───────┐      │      ┌───────┼───────┐
       │       │       │      │      │       │       │
       ▼       ▼       ▼      ▼      ▼       ▼       ▼
    Balance  Fees   Negative State  Order  Duplicate Net
    Conserv. Calc. Prevent. Trans. Indep. Prevent. Conserv.
```

## Test Execution Model

```
┌────────────────────────────────────────────────────────────┐
│                     Test Execution                          │
├────────────────────────────────────────────────────────────┤
│                                                             │
│  For each property test:                                   │
│                                                             │
│    ┌─────────────────────────────────────────┐            │
│    │  Repeat N times (default: 50)           │            │
│    │                                          │            │
│    │  1. Generate random inputs               │            │
│    │  2. Setup test environment               │            │
│    │  3. Execute operation                    │            │
│    │  4. Validate invariant                   │            │
│    │  5. Clean up                             │            │
│    │                                          │            │
│    │  If failure:                             │            │
│    │    - Shrink to minimal case              │            │
│    │    - Report failure with seed            │            │
│    │    - Stop execution                      │            │
│    └─────────────────────────────────────────┘            │
│                                                             │
└────────────────────────────────────────────────────────────┘
```

## Input Generation Strategy

```
┌─────────────────────────────────────────────────────────────┐
│                   Input Strategies                           │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  amount_strategy()                                          │
│  ┌────────────────────────────────────────┐                │
│  │  Range: 1 to 1,000,000                 │                │
│  │  Distribution: Uniform                  │                │
│  │  Examples: 1, 500, 1000, 999999        │                │
│  └────────────────────────────────────────┘                │
│                                                              │
│  fee_bps_strategy()                                         │
│  ┌────────────────────────────────────────┐                │
│  │  Range: 0 to 1000 (0% to 10%)          │                │
│  │  Distribution: Uniform                  │                │
│  │  Examples: 0, 250, 500, 1000           │                │
│  └────────────────────────────────────────┘                │
│                                                              │
│  batch_size_strategy()                                      │
│  ┌────────────────────────────────────────┐                │
│  │  Range: 1 to 20                        │                │
│  │  Distribution: Uniform                  │                │
│  │  Examples: 1, 5, 10, 20                │                │
│  └────────────────────────────────────────┘                │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

## Validation Pipeline

```
┌─────────────────────────────────────────────────────────────┐
│                    Validation Pipeline                       │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
                    ┌──────────────────┐
                    │  Pre-Conditions  │
                    │  - Valid inputs  │
                    │  - Setup correct │
                    └────────┬─────────┘
                             │
                             ▼
                    ┌──────────────────┐
                    │    Operation     │
                    │   Execution      │
                    └────────┬─────────┘
                             │
                             ▼
                    ┌──────────────────┐
                    │ Post-Conditions  │
                    │ - Invariant holds│
                    │ - State valid    │
                    └────────┬─────────┘
                             │
                             ▼
                    ┌──────────────────┐
                    │   Assertions     │
                    │  - Balance check │
                    │  - Fee check     │
                    │  - State check   │
                    └────────┬─────────┘
                             │
                             ▼
                    ┌──────────────────┐
                    │     Result       │
                    │   Pass / Fail    │
                    └──────────────────┘
```

## Integration Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Development Workflow                      │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  Developer                                                   │
│     │                                                        │
│     ├─── Code Changes ──────────────────────┐               │
│     │                                        │               │
│     ▼                                        ▼               │
│  ┌──────────────┐                    ┌──────────────┐      │
│  │  Unit Tests  │                    │   Property   │      │
│  │  (Specific)  │                    │    Tests     │      │
│  │              │                    │  (General)   │      │
│  └──────┬───────┘                    └──────┬───────┘      │
│         │                                    │               │
│         └────────────┬───────────────────────┘               │
│                      │                                       │
│                      ▼                                       │
│              ┌──────────────┐                               │
│              │   All Pass   │                               │
│              └──────┬───────┘                               │
│                     │                                       │
│                     ▼                                       │
│              ┌──────────────┐                               │
│              │    Commit    │                               │
│              └──────┬───────┘                               │
│                     │                                       │
│                     ▼                                       │
│              ┌──────────────┐                               │
│              │    CI/CD     │                               │
│              │  (200 cases) │                               │
│              └──────┬───────┘                               │
│                     │                                       │
│                     ▼                                       │
│              ┌──────────────┐                               │
│              │   Deploy     │                               │
│              └──────────────┘                               │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

## Test Coverage Map

```
┌─────────────────────────────────────────────────────────────┐
│                    Contract Operations                       │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  initialize()          ✓ Tested in setup                    │
│  register_agent()      ✓ Tested in setup                    │
│  assign_role()         ✓ Tested in setup                    │
│  create_remittance()   ✓ 15 property tests                  │
│  confirm_payout()      ✓ 8 property tests                   │
│  cancel_remittance()   ✓ 3 property tests                   │
│  compute_net_settlements() ✓ 3 property tests               │
│                                                              │
├─────────────────────────────────────────────────────────────┤
│                    Invariants Covered                        │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  Balance Conservation      ✓✓✓ (3 tests)                    │
│  Non-Negative Amounts      ✓✓ (2 tests)                     │
│  Deterministic Results     ✓ (1 test)                       │
│  Fee Calculation           ✓✓ (2 tests)                     │
│  State Transitions         ✓✓ (2 tests)                     │
│  Duplicate Prevention      ✓ (1 test)                       │
│  Net Settlement            ✓ (1 test)                       │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

## Performance Profile

```
Test Cases vs Duration

1000 │                                              ●
     │
 800 │
     │
 600 │                                    ●
     │
 400 │                          ●
     │
 200 │              ●
     │
  50 │    ●
     │
  10 │●
     └────────────────────────────────────────────────
      5s   15s   30s   1m    2m    3m    4m    5m
                        Duration

Legend:
● = Test execution point
Recommended ranges:
  Development: 10 cases (~5s)
  Standard: 50 cases (~15s)
  CI/CD: 200 cases (~1m)
  Release: 1000 cases (~5m)
```

## Documentation Structure

```
┌─────────────────────────────────────────────────────────────┐
│                    Documentation Layers                      │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  Level 1: Quick Start                                       │
│  ┌────────────────────────────────────────────┐            │
│  │  PROPERTY_TESTS_README.md                  │            │
│  │  - What is property testing?               │            │
│  │  - Quick start commands                    │            │
│  │  - Basic examples                          │            │
│  └────────────────────────────────────────────┘            │
│                                                              │
│  Level 2: Reference                                         │
│  ┌────────────────────────────────────────────┐            │
│  │  PROPERTY_TESTING_QUICKREF.md              │            │
│  │  - Command reference                       │            │
│  │  - Environment variables                   │            │
│  │  - Troubleshooting                         │            │
│  └────────────────────────────────────────────┘            │
│                                                              │
│  Level 3: Deep Dive                                         │
│  ┌────────────────────────────────────────────┐            │
│  │  PROPERTY_BASED_TESTING.md                 │            │
│  │  - Detailed invariant explanations         │            │
│  │  - Test strategies                         │            │
│  │  - Best practices                          │            │
│  └────────────────────────────────────────────┘            │
│                                                              │
│  Level 4: Implementation                                    │
│  ┌────────────────────────────────────────────┐            │
│  │  src/test_property.rs                      │            │
│  │  - Actual test code                        │            │
│  │  - Implementation details                  │            │
│  │  - Code comments                           │            │
│  └────────────────────────────────────────────┘            │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

## Summary

The property-based testing architecture provides:

- **Comprehensive Coverage**: 15 tests validating 7 invariants
- **Scalable Design**: Configurable test case counts
- **Clear Structure**: Well-organized test hierarchy
- **Excellent Documentation**: Multiple documentation levels
- **Easy Integration**: Simple CI/CD integration
- **Developer-Friendly**: Clear workflows and debugging

This architecture ensures robust validation of contract invariants while maintaining developer productivity and test execution efficiency.
