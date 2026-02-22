# Lifecycle Transitions - Deployment Checklist

## Pre-Deployment

### Code Review
- [ ] Review `src/transitions.rs` for validation logic
- [ ] Review `src/lib.rs` for new functions integration
- [ ] Review `src/events.rs` for event emission
- [ ] Review `src/types.rs` for state definitions
- [ ] Verify all imports and module declarations

### Testing
- [ ] Run all unit tests: `cargo test transitions::tests`
- [ ] Run all integration tests: `cargo test test_transitions`
- [ ] Run full test suite: `cargo test`
- [ ] Verify all tests pass
- [ ] Check test coverage for edge cases

### Documentation Review
- [ ] Read `LIFECYCLE_TRANSITIONS.md`
- [ ] Read `STATE_MACHINE.md`
- [ ] Read `TRANSITIONS_QUICKREF.md`
- [ ] Verify README.md updates
- [ ] Review API documentation

## Build & Compile

### Local Build
- [ ] Clean build: `cargo clean`
- [ ] Build contract: `cargo build --target wasm32-unknown-unknown --release`
- [ ] Optimize WASM: `soroban contract optimize --wasm target/wasm32-unknown-unknown/release/swiftremit.wasm`
- [ ] Verify no compilation errors
- [ ] Check WASM size is reasonable

### Contract Validation
- [ ] Inspect optimized WASM
- [ ] Verify contract exports
- [ ] Check for any warnings

## Testnet Deployment

### Setup
- [ ] Configure testnet network
- [ ] Fund deployer account
- [ ] Prepare USDC token address
- [ ] Prepare admin address
- [ ] Prepare test agent addresses

### Deploy
- [ ] Deploy contract to testnet
- [ ] Save contract ID
- [ ] Initialize contract
- [ ] Register test agents
- [ ] Verify initialization

### Testnet Testing

#### Basic Lifecycle Tests
- [ ] Create remittance (verify Pending state)
- [ ] Call `start_processing()` (verify Processing state)
- [ ] Call `confirm_payout()` (verify Completed state)
- [ ] Verify funds transferred correctly
- [ ] Verify fees accumulated

#### Cancellation Flow
- [ ] Create remittance (Pending)
- [ ] Call `cancel_remittance()` (verify Cancelled state)
- [ ] Verify full refund to sender

#### Failed Payout Flow
- [ ] Create remittance (Pending)
- [ ] Call `start_processing()` (Processing)
- [ ] Call `mark_failed()` (verify Failed state)
- [ ] Verify full refund to sender

#### Invalid Transition Tests
- [ ] Try Pending → Completed (should fail)
- [ ] Try Pending → Failed (should fail)
- [ ] Try Processing → Cancelled (should fail)
- [ ] Try Completed → Processing (should fail)
- [ ] Verify all return InvalidStatus error

#### Event Verification
- [ ] Monitor for `("status", "transit")` events
- [ ] Verify event data structure
- [ ] Verify actor addresses in events
- [ ] Verify timestamps and ledger sequences

#### Authorization Tests
- [ ] Try `start_processing()` as non-agent (should fail)
- [ ] Try `confirm_payout()` as non-agent (should fail)
- [ ] Try `mark_failed()` as non-agent (should fail)
- [ ] Try `cancel_remittance()` as non-sender (should fail)

#### Edge Cases
- [ ] Multiple remittances in parallel
- [ ] Expired remittances
- [ ] Large amounts
- [ ] Minimum amounts
- [ ] Multiple agents

## Off-Chain System Updates

### Backend Services
- [ ] Update API to handle new states
- [ ] Add Processing state handling
- [ ] Add Failed state handling
- [ ] Update status enums
- [ ] Add transition event listeners

### Agent Dashboard
- [ ] Add "Start Processing" button
- [ ] Add "Mark Failed" button
- [ ] Update status displays
- [ ] Add transition history view
- [ ] Update workflow instructions

### Sender Dashboard
- [ ] Update status displays
- [ ] Show Processing state
- [ ] Show Failed state with refund info
- [ ] Update cancellation logic (Pending only)
- [ ] Add transition timeline view

### Monitoring & Alerts
- [ ] Add transition event monitoring
- [ ] Alert on stuck Processing states
- [ ] Alert on high failure rates
- [ ] Dashboard for state distribution
- [ ] Metrics for transition times

## Documentation Updates

### Developer Docs
- [ ] Update API documentation
- [ ] Add transition examples
- [ ] Update error code documentation
- [ ] Add migration guide
- [ ] Update integration guide

### User Docs
- [ ] Update user guide
- [ ] Add new state explanations
- [ ] Update FAQ
- [ ] Add troubleshooting section

### Operations Docs
- [ ] Update runbook
- [ ] Add monitoring procedures
- [ ] Add incident response procedures
- [ ] Update deployment procedures

## Mainnet Preparation

### Security Review
- [ ] Code audit completed
- [ ] Security review of transition logic
- [ ] Review authorization checks
- [ ] Review event emission
- [ ] Review terminal state enforcement

### Performance Testing
- [ ] Load testing on testnet
- [ ] Gas cost analysis
- [ ] Transaction throughput testing
- [ ] Concurrent remittance testing

### Rollback Plan
- [ ] Document rollback procedure
- [ ] Prepare old contract version
- [ ] Test rollback on testnet
- [ ] Define rollback triggers

### Communication
- [ ] Notify agents of changes
- [ ] Notify users of new features
- [ ] Update API documentation
- [ ] Send migration guide to integrators
- [ ] Schedule maintenance window if needed

## Mainnet Deployment

### Pre-Deploy
- [ ] Final code review
- [ ] Final test run on testnet
- [ ] Backup current contract state
- [ ] Notify stakeholders
- [ ] Schedule deployment time

### Deploy
- [ ] Deploy to mainnet
- [ ] Initialize contract
- [ ] Register agents
- [ ] Verify initialization
- [ ] Test basic operations

### Post-Deploy
- [ ] Monitor first transactions
- [ ] Verify events are emitted
- [ ] Check state transitions
- [ ] Monitor error rates
- [ ] Verify fee accumulation

### Validation
- [ ] Run smoke tests
- [ ] Create test remittance
- [ ] Complete full lifecycle
- [ ] Verify all states work
- [ ] Check event logs

## Monitoring (First 24 Hours)

### Metrics to Watch
- [ ] Transaction success rate
- [ ] State transition distribution
- [ ] Error rate by type
- [ ] Average time in each state
- [ ] Failed payout rate
- [ ] Cancellation rate

### Alerts to Configure
- [ ] High error rate
- [ ] Stuck in Processing state
- [ ] High failure rate
- [ ] Unusual state transitions
- [ ] Authorization failures

### Manual Checks
- [ ] Review transaction logs
- [ ] Check event emission
- [ ] Verify state transitions
- [ ] Check agent feedback
- [ ] Monitor user reports

## Post-Deployment

### Week 1
- [ ] Daily monitoring
- [ ] Collect agent feedback
- [ ] Collect user feedback
- [ ] Review error logs
- [ ] Analyze state transition patterns

### Week 2-4
- [ ] Weekly review
- [ ] Performance analysis
- [ ] Identify optimization opportunities
- [ ] Update documentation based on feedback
- [ ] Plan improvements

### Ongoing
- [ ] Monthly state transition analysis
- [ ] Quarterly security review
- [ ] Regular performance optimization
- [ ] Continuous documentation updates

## Rollback Triggers

Initiate rollback if:
- [ ] Critical security vulnerability discovered
- [ ] High rate of failed transactions (>10%)
- [ ] State transition logic errors
- [ ] Data corruption detected
- [ ] Unrecoverable contract state

## Success Criteria

Deployment is successful when:
- [ ] All tests pass
- [ ] No critical errors in first 24 hours
- [ ] State transitions work as expected
- [ ] Events are emitted correctly
- [ ] Agents can complete workflows
- [ ] Users can cancel when appropriate
- [ ] Failed payouts refund correctly
- [ ] Performance is acceptable
- [ ] No security issues detected

## Sign-Off

- [ ] Technical Lead approval
- [ ] Security Team approval
- [ ] Product Team approval
- [ ] Operations Team ready
- [ ] Support Team trained
- [ ] Documentation complete

---

**Deployment Date**: _______________

**Deployed By**: _______________

**Contract ID**: _______________

**Notes**: _______________
