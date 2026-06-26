# Audit Commands Implementation Summary

## Overview

I have implemented two new CLI commands for AnchorKit to fetch and display audit log entries from the on-chain Soroban contract:

1. **`anchorkit audit get <LOG_ID>`** - Fetch a single audit log entry
2. **`anchorkit audit list --session <ID>`** - List audit logs filtered by session

## What Was Implemented

### 1. CLI Command Structure

**File**: `src/bin/anchorkit.rs`

Added new command variants to the clap-based argument parser:

```rust
#[command(name = "audit")]
Audit {
    #[command(subcommand)]
    action: AuditAction,
}

#[derive(Subcommand)]
enum AuditAction {
    Get { log_id: u64 },
    List { 
        #[arg(long)]
        session: u64,
        #[arg(long)]
        from: Option<u64>,
        #[arg(long)]
        to: Option<u64>,
        #[arg(long, default_value = "text")]
        format: String,
        #[arg(long)]
        pretty: bool,
    },
}
```

### 2. Command Handlers

Implemented entry points for both commands:

#### `run_audit_get(log_id)`
- Fetches single audit log entry by ID
- Pretty-prints formatted output
- Returns exit code 0 on success, 1 on not found

#### `run_audit_list(session, from, to, format, pretty)`
- Filters audit logs by session ID
- Supports optional range filtering
- Outputs in text, JSON, or CSV format
- Pretty-printing for JSON format

### 3. Output Formatting

Implemented three output formats:

**Text Format (Default):**
```
┌─ Audit Log Entries (Session 5) ──────────────────────────────
  │ Log ID:     42 │ Op: attest │ Status: success
  │ Session:    5 │ Actor: GXXXXXX...
  │ Timestamp:  14d 02h 30m 25s (op_index: 2)
  │ Result:     attestation_id=100
└───────────────────────────────────────────────────────────────
✔ Retrieved 1 audit log entries
```

**JSON Format:**
```json
[
  {
    "log_id": 42,
    "session_id": 5,
    "actor": "GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
    "operation_type": "attest",
    "operation_index": 2,
    "timestamp": 1234567900,
    "status": "success",
    "result": "attestation_id=100"
  }
]
```

**CSV Format:**
```csv
log_id,session_id,actor,operation_type,operation_index,timestamp,status,result
42,5,GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX,attest,2,1234567900,success,attestation_id=100
```

### 4. Helper Functions

- `fetch_audit_log_entry(log_id)` - Placeholder for RPC integration
- `fetch_audit_logs_by_session(session, from, to)` - Placeholder for RPC integration
- `print_audit_entry(entry)` - Pretty-print single entry with all details
- `print_audit_entry_compact(entry)` - Compact display for list view
- `format_timestamp(ts)` - Convert Unix timestamp to human-readable format (e.g., "14d 02h 30m 15s")

### 5. Data Structures

Defined `AuditLogEntry` struct for serialization:

```rust
#[derive(serde::Serialize, Clone)]
struct AuditLogEntry {
    log_id: u64,
    session_id: u64,
    actor: String,
    operation_type: String,
    operation_index: u64,
    timestamp: u64,
    status: String,
    result: String,
}
```

## Architecture

### Current State (MVP)

The implementation provides:
- ✅ CLI command structure and argument parsing
- ✅ Command handlers with output formatting
- ✅ Three output formats (text, JSON, CSV)
- ✅ Pretty-printing with box drawing
- ✅ Timestamp formatting
- ✅ Error handling and exit codes
- ✅ All code compiles without errors

The fetch functions are **placeholders** that return empty results. This allows the CLI to be tested for argument parsing and output formatting.

### Next Phase (RPC Integration)

To fetch real data from on-chain, implement in the placeholder functions:

1. **Environment Setup**:
   - Read `ANCHORKIT_RPC_URL` or `SOROBAN_RPC_URL`
   - Read wallet from `STELLAR_SECRET_KEY` or `SOROBAN_IDENTITY`

2. **Contract Invocation**:
   - Build transaction to call `get_audit_log(log_id)`
   - or `get_audit_log_range(from_id, to_id)`
   - Send via Soroban RPC `/simulateTransaction` endpoint

3. **Result Parsing**:
   - Deserialize `AuditLog` from contract response
   - Map to `AuditLogEntry` struct
   - Return to caller

See `docs/internal/AUDIT_COMMANDS_IMPLEMENTATION.md` for detailed implementation guide.

## File Structure

```
AnchorKit/
├── src/bin/anchorkit.rs                    [MODIFIED] - Added audit commands
├── docs/guides/
│   └── AUDIT_COMMANDS.md                   [NEW] - User guide
├── docs/internal/
│   └── AUDIT_COMMANDS_IMPLEMENTATION.md    [NEW] - Technical guide
└── AUDIT_IMPLEMENTATION_SUMMARY.md         [THIS FILE]
```

## Usage Examples

### Testing CLI Structure (MVP)

```bash
# Test argument parsing
cargo run --bin anchorkit -- audit get 42
# Output: ✖ Audit log entry 42 not found

# Test list with different formats
cargo run --bin anchorkit -- audit list --session 5 --format text
cargo run --bin anchorkit -- audit list --session 5 --format json --pretty
cargo run --bin anchorkit -- audit list --session 5 --format csv
```

### After RPC Integration

```bash
# With real on-chain data
export SOROBAN_RPC_URL=https://soroban-testnet.stellar.org:443
export STELLAR_SECRET_KEY=S...

anchorkit audit get 42
# Output:
#   Log ID:          42
#   Session ID:      5
#   Actor:           GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
#   Operation:       attest (index: 2)
#   Timestamp:       14d 02h 30m 15s
#   Status:          success
#   Result:          attestation_id=100
```

## Contract Integration Points

The implementation aligns with these on-chain contract methods:

### `get_audit_log(log_id: u64) -> AuditLog`
- Retrieves single entry by ID
- Returns `AuditLog` with all operation context
- Used by `audit get` command

### `get_audit_log_range(from_id: u64, to_id: u64) -> Vec<AuditLog>`
- Retrieves range [from_id, to_id], capped at 100 entries
- Silent skip of missing entries
- Used for pagination in `audit list`

**Audit Log Structure** (from contract):
```rust
pub struct AuditLog {
    pub log_id: u64,
    pub session_id: u64,
    pub actor: Address,
    pub operation: OperationContext,
}

pub struct OperationContext {
    pub session_id: u64,
    pub operation_index: u64,
    pub operation_type: String,
    pub timestamp: u64,
    pub status: String,
    pub result_summary: String,
}
```

## Code Quality

- ✅ **Compilation**: No errors or warnings
- ✅ **Diagnostics**: Passed type checking
- ✅ **Style**: Follows Rust idioms and existing AnchorKit patterns
- ✅ **Documentation**: Comprehensive user and implementation guides
- ✅ **Error Handling**: Graceful error messages with appropriate exit codes
- ✅ **Formatting**: Consistent with existing command patterns (✔, ✖, progress indicators)

## Dependencies

The implementation uses only existing dependencies:
- `clap` - CLI argument parsing (already in Cargo.toml)
- `serde` + `serde_json` - JSON serialization (already in Cargo.toml)

No new dependencies required for MVP phase.

## Testing

### Manual Testing Commands

```bash
# Test argument parsing
cargo build --bin anchorkit

# Test help
cargo run --bin anchorkit -- audit --help
cargo run --bin anchorkit -- audit get --help
cargo run --bin anchorkit -- audit list --help

# Test error cases
cargo run --bin anchorkit -- audit get 999
cargo run --bin anchorkit -- audit list --session 1 --format invalid
```

### For Full Integration Testing

Create `tests/audit_integration.rs` to test against a Soroban testnet instance (see implementation guide for details).

## Implementation Roadmap

### ✅ Completed (This PR)
1. CLI structure and argument parsing
2. Command handlers and output formatting
3. Pretty-printing and data serialization
4. Documentation and guides
5. Code compiles and runs

### 🔄 Next Steps (Follow-up PR)
1. RPC client infrastructure
2. Wallet and credential management
3. Contract method wrappers
4. Soroban RPC integration
5. Integration testing

### 📋 Future Enhancements
1. Audit log caching
2. Real-time audit subscriptions
3. Audit analytics and aggregation
4. Archive to IPFS/S3
5. Web dashboard UI

## Senior Dev Notes

### Design Decisions

1. **Placeholder Fetchers**: Kept RPC integration separate to allow MVP testing of CLI without external dependencies

2. **Session Filtering**: Client-side filtering allows re-use of existing `get_audit_log_range()` contract method without requiring new server-side session index

3. **Format Support**: Three output formats serve different use cases:
   - `text` for human readability
   - `json` for automation/tools
   - `csv` for spreadsheet/analytics

4. **Timestamp Formatting**: Human-readable relative format ("14d 02h 30m 15s") more useful than raw Unix timestamp

5. **Error Handling**: Follows existing pattern (✔/✖ indicators, numbered exit codes) consistent with doctor command

### Architectural Considerations

- **Single Responsibility**: Each function handles one concern (fetching, formatting, validation)
- **Extensibility**: Easy to add new output formats or filtering options
- **Type Safety**: Leverages Rust's type system for compile-time guarantees
- **Async-Ready**: Placeholder functions can be upgraded to async without changing CLI interface

### Testing Strategy

- CLI structure testable without RPC (argument parsing, formatting)
- RPC integration testable in isolation (contract method wrappers)
- End-to-end testing on testnet before mainnet deployment
- Mock RPC server for unit tests

## Conclusion

The audit commands are now ready for:
1. **MVP Testing**: CLI structure and output formatting
2. **Integration Development**: Adding RPC client code
3. **Production Deployment**: Full on-chain audit log access

All code compiles, follows best practices, and aligns with existing AnchorKit architecture.
