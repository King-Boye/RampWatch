# Configuration Validation - Quick Start Guide

## What Changed?

The `anchorkit validate` command now provides **detailed field-level error messages** instead of just "valid" or "invalid".

## Quick Examples

### Before
```bash
$ anchorkit validate configs/bad-config.json
✔ configs/bad-config.json: valid JSON
```
❌ No indication of what's wrong with the configuration

### After
```bash
$ anchorkit validate configs/bad-config.json
✖ configs/bad-config.json: invalid configuration
  • field 'contract.network' is missing
  • field 'attestors.registry[0].address' has invalid Stellar address format (must start with 'G' and be 56 characters)
  • field 'attestors.registry[0].endpoint' has invalid URL 'http://example.com' — URL must use HTTPS (http:// and other schemes are not allowed)
```
✅ Clear, actionable error messages with exact field paths

## Usage

```bash
# Validate a single file
anchorkit validate configs/testnet-example.json

# Validate all configs in directory
anchorkit validate configs/

# Validate with default directory
anchorkit validate
```

## What Gets Validated?

### ✅ Syntax
- Valid JSON/TOML format
- Proper structure

### ✅ Required Fields
- `contract`: name, version, network
- `attestors.registry`: name, address, endpoint, role, enabled
- `sessions`: enable_session_tracking, session_timeout_seconds, operations_per_session, audit_log_retention_days

### ✅ Field Formats
- Contract name: lowercase, numbers, hyphens only
- Version: semantic versioning (1.0.0)
- Network: testnet/mainnet/futurenet
- Stellar address: starts with 'G', 56 characters
- Endpoint URL: HTTPS only, valid domain, no localhost

### ✅ Business Rules
- At least 1 attestor in registry
- At least 1 enabled attestor
- No duplicate attestor names or addresses
- Session timeout ≥ 60 seconds
- Operations per session: 1-10000
- Audit retention: 1-3650 days

### ⚠️ Warnings (Non-blocking)
- Session timeout > 24 hours
- Operations per session > 5000

## Common Errors and Fixes

### Error: `field 'contract.network' is missing`
**Fix:** Add the network field:
```json
"contract": {
  "name": "my-anchor",
  "version": "1.0.0",
  "network": "stellar-testnet"
}
```

### Error: `field 'contract.version' must follow semantic versioning`
**Fix:** Use format X.Y.Z:
```json
"version": "1.0.0"  // ✅ Correct
"version": "1.0"    // ❌ Wrong
```

### Error: `field 'attestors.registry[0].address' has invalid Stellar address format`
**Fix:** Use valid Stellar public key (starts with 'G', 56 chars):
```json
"address": "GBAA5XKQC3KVDPD5OS3CHJJ24SB3BX7GI7XBXKNNCKQVPQVX6S3VT5O"
```

### Error: `URL must use HTTPS`
**Fix:** Change http:// to https://:
```json
"endpoint": "https://api.example.com/verify"  // ✅ Correct
"endpoint": "http://api.example.com/verify"   // ❌ Wrong
```

### Error: `at least one attestor must be enabled`
**Fix:** Set at least one attestor to enabled:
```json
{
  "name": "kyc-provider",
  "enabled": true  // ✅ At least one must be true
}
```

## Exit Codes

- **0**: All validations passed
- **1**: One or more validations failed

## Testing Your Changes

1. **Build the CLI:**
   ```bash
   cargo build --bin anchorkit
   ```

2. **Test a valid config:**
   ```bash
   ./target/debug/anchorkit validate configs/testnet-example.json
   ```
   Expected: ✔ message, exit code 0

3. **Test an invalid config:**
   ```bash
   ./target/debug/anchorkit validate configs/test-invalid.json
   ```
   Expected: ✖ message with errors, exit code 1

4. **Test all configs:**
   ```bash
   ./target/debug/anchorkit validate configs/
   ```
   Expected: Status for each file

## Integration with CI/CD

Add to your pipeline:
```yaml
- name: Validate Configurations
  run: |
    cargo build --bin anchorkit
    ./target/debug/anchorkit validate configs/
```

The command will fail (exit code 1) if any config is invalid, stopping the deployment.

## Need More Details?

- **VALIDATION_ENHANCEMENT.md** - Full implementation details
- **VALIDATION_TEST_EXAMPLES.md** - Complete test cases
- **ISSUE_RESOLUTION_VALIDATION.md** - Issue resolution summary
- **config_schema.json** - Schema definition

## Questions?

The validation logic matches `validate_config_strict.py` - if you're familiar with that, the behavior is identical.
