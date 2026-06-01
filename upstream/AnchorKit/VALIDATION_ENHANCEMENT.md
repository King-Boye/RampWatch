# Configuration Validation Enhancement

## Problem
The `run_validate` command only reported whether a config file was valid or invalid, without indicating which field failed or why. Users had to manually inspect configs to find problems.

## Solution
Enhanced the validation logic in `src/bin/anchorkit.rs` to provide detailed field-level error messages, similar to the Python `validate_config_strict.py` script.

## Changes Made

### 1. Dependencies Added (Cargo.toml)
- Added `regex = { version = "1.10", optional = true }` for pattern validation
- Updated `std` feature to include `regex`

### 2. Enhanced Validation Functions (src/bin/anchorkit.rs)

#### New Validation Architecture
```
run_validate()
  └─> validate_file()
      ├─> Parse JSON/TOML (with syntax error reporting)
      └─> validate_config_schema()
          ├─> validate_contract_section()
          ├─> validate_attestors_section()
          └─> validate_sessions_section()
```

#### Field-Level Validation

**Contract Section:**
- ✅ Required fields: `name`, `version`, `network`
- ✅ Name format: lowercase letters, numbers, hyphens only
- ✅ Version format: semantic versioning (e.g., 1.0.0)
- ✅ Network: must be one of `stellar-testnet`, `stellar-mainnet`, `stellar-futurenet`
- ✅ Description: optional, max 256 characters

**Attestors Section:**
- ✅ Required field: `registry` (array)
- ✅ Registry must contain at least 1 attestor
- ✅ Maximum 100 attestors
- ✅ Each attestor requires: `name`, `address`, `endpoint`, `role`, `enabled`
- ✅ Name format validation (lowercase, numbers, hyphens)
- ✅ Stellar address validation (starts with 'G', 56 characters)
- ✅ Endpoint URL validation (HTTPS required, domain validation, no localhost)
- ✅ Role validation (must be valid role type)
- ✅ Duplicate name/address detection
- ✅ At least one enabled attestor required

**Sessions Section:**
- ✅ Required fields: `enable_session_tracking`, `session_timeout_seconds`, `operations_per_session`, `audit_log_retention_days`
- ✅ Timeout: minimum 60 seconds, warning if > 24 hours
- ✅ Operations: 1-10000, warning if > 5000
- ✅ Retention: 1-3650 days

#### URL Validation
Comprehensive endpoint URL validation matching Python implementation:
- HTTPS required (no HTTP or other schemes)
- No localhost/loopback addresses
- Proper domain structure (TLD required)
- No raw IP addresses
- No Punycode (homograph attack prevention)
- Port validation (1-65535)
- Character validation (no control characters)

## Example Output

### Before (Old Validation)
```
✔ configs/testnet-example.json: valid JSON
```

### After (New Validation)

**Valid Config:**
```
✔ configs/testnet-example.json: valid configuration
```

**Invalid Config with Missing Fields:**
```
✖ configs/test-invalid.json: invalid configuration
  • field 'contract.network' is missing
  • field 'sessions' is missing
  • field 'attestors.registry' must contain at least one attestor
```

**Invalid Config with Format Errors:**
```
✖ configs/bad-config.json: invalid configuration
  • field 'contract.version' must follow semantic versioning (e.g., 1.0.0)
  • field 'attestors.registry[0].address' has invalid Stellar address format (must start with 'G' and be 56 characters)
  • field 'attestors.registry[0].endpoint' has invalid URL 'http://example.com' — URL must use HTTPS (http:// and other schemes are not allowed)
  • at least one attestor must be enabled (attestors.registry[].enabled = true)
```

**Config with Warnings:**
```
⚠  configs/high-limits.json: field 'sessions.session_timeout_seconds' exceeds 24 hours — consider shorter timeouts for security
⚠  configs/high-limits.json: field 'sessions.operations_per_session' is high (>5000) and may impact performance
✔ configs/high-limits.json: valid configuration
```

## Testing

### Test Valid Config
```bash
./target/debug/anchorkit validate configs/testnet-example.json
```

### Test Invalid Config
```bash
./target/debug/anchorkit validate configs/test-invalid.json
```

### Test Directory
```bash
./target/debug/anchorkit validate configs/
```

## Benefits

1. **Immediate Problem Identification**: Users see exactly which field is missing or invalid
2. **Clear Error Messages**: Each error explains what's wrong and how to fix it
3. **Multiple Errors Reported**: All validation errors shown at once, not just the first one
4. **Warnings for Best Practices**: Non-blocking warnings for configuration issues
5. **Consistent with Python Validator**: Matches the behavior of `validate_config_strict.py`
6. **Pre-deployment Safety**: Catches configuration errors before deployment

## Migration Notes

- The validation is backward compatible - valid configs remain valid
- Error messages are more detailed but the exit codes remain the same (0 = success, 1 = failure)
- Both JSON and TOML formats are fully supported
- The validation runs automatically when using `anchorkit validate`
