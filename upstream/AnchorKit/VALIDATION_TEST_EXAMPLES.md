# Validation Test Examples

This document shows the expected output from the enhanced validation for various test configurations.

## Test Case 1: Valid Configuration

**File:** `configs/testnet-example.json`

**Expected Output:**
```
✔ configs/testnet-example.json: valid configuration
```

**Exit Code:** 0

---

## Test Case 2: Missing Required Fields

**File:** `configs/test-invalid.json`

**Content:**
```json
{
  "contract": {
    "name": "test-anchor",
    "version": "1.0.0"
  },
  "attestors": {
    "registry": []
  }
}
```

**Expected Output:**
```
✖ configs/test-invalid.json: invalid configuration
  • field 'contract.network' is missing
  • field 'sessions' is missing
  • field 'attestors.registry' must contain at least one attestor
```

**Exit Code:** 1

---

## Test Case 3: Multiple Validation Errors

**File:** `configs/test-validation-errors.json`

**Content:**
```json
{
  "contract": {
    "name": "Invalid_Name_With_Uppercase",
    "version": "1.0",
    "network": "invalid-network"
  },
  "attestors": {
    "registry": [
      {
        "name": "test-attestor",
        "address": "INVALID_ADDRESS",
        "endpoint": "http://localhost:8080/api",
        "role": "invalid-role",
        "enabled": false
      }
    ]
  },
  "sessions": {
    "enable_session_tracking": true,
    "session_timeout_seconds": 30,
    "operations_per_session": 15000,
    "audit_log_retention_days": 5000
  }
}
```

**Expected Output:**
```
✖ configs/test-validation-errors.json: invalid configuration
  • field 'contract.name' must contain only lowercase letters, numbers, and hyphens
  • field 'contract.version' must follow semantic versioning (e.g., 1.0.0)
  • field 'contract.network' must be one of: stellar-testnet, stellar-mainnet, stellar-futurenet
  • field 'attestors.registry[0].address' has invalid Stellar address format (must start with 'G' and be 56 characters)
  • field 'attestors.registry[0].address' length must be 54-56 characters, got 15
  • field 'attestors.registry[0].endpoint' has invalid URL 'http://localhost:8080/api' — URL must use HTTPS (http:// and other schemes are not allowed)
  • field 'attestors.registry[0].role' must be one of: kyc-issuer, transfer-verifier, compliance-approver, rate-provider, attestor
  • at least one attestor must be enabled (attestors.registry[].enabled = true)
  • field 'sessions.session_timeout_seconds' must be at least 60 seconds
  • field 'sessions.operations_per_session' exceeds maximum of 10000, got 15000
  • field 'sessions.audit_log_retention_days' exceeds maximum of 3650 days, got 5000
```

**Exit Code:** 1

---

## Test Case 4: Syntax Error (Invalid JSON)

**File:** `configs/syntax-error.json`

**Content:**
```json
{
  "contract": {
    "name": "test",
    "version": "1.0.0"
  }
  // Missing comma here
  "attestors": {}
}
```

**Expected Output:**
```
✖ configs/syntax-error.json: invalid JSON at line 6, column 3: expected `,` or `}`
```

**Exit Code:** 1

---

## Test Case 5: TOML Format

**File:** `configs/stablecoin-issuer.toml`

**Expected Output:**
```
✔ configs/stablecoin-issuer.toml: valid configuration
```

**Exit Code:** 0

---

## Test Case 6: Directory Validation

**Command:** `anchorkit validate configs/`

**Expected Output:**
```
✔ configs/fiat-on-off-ramp.json: valid configuration
✔ configs/fiat-on-off-ramp.toml: valid configuration
✔ configs/remittance-anchor.json: valid configuration
✔ configs/remittance-anchor.toml: valid configuration
✔ configs/stablecoin-issuer.json: valid configuration
✔ configs/stablecoin-issuer.toml: valid configuration
✔ configs/testnet-example.json: valid configuration
✖ configs/test-invalid.json: invalid configuration
  • field 'contract.network' is missing
  • field 'sessions' is missing
  • field 'attestors.registry' must contain at least one attestor
✖ configs/test-validation-errors.json: invalid configuration
  • field 'contract.name' must contain only lowercase letters, numbers, and hyphens
  • field 'contract.version' must follow semantic versioning (e.g., 1.0.0)
  • field 'contract.network' must be one of: stellar-testnet, stellar-mainnet, stellar-futurenet
  • field 'attestors.registry[0].address' has invalid Stellar address format (must start with 'G' and be 56 characters)
  • field 'attestors.registry[0].address' length must be 54-56 characters, got 15
  • field 'attestors.registry[0].endpoint' has invalid URL 'http://localhost:8080/api' — URL must use HTTPS (http:// and other schemes are not allowed)
  • field 'attestors.registry[0].role' must be one of: kyc-issuer, transfer-verifier, compliance-approver, rate-provider, attestor
  • at least one attestor must be enabled (attestors.registry[].enabled = true)
  • field 'sessions.session_timeout_seconds' must be at least 60 seconds
  • field 'sessions.operations_per_session' exceeds maximum of 10000, got 15000
  • field 'sessions.audit_log_retention_days' exceeds maximum of 3650 days, got 5000
```

**Exit Code:** 1 (because some files failed validation)

---

## Key Features Demonstrated

1. **Field-Level Errors**: Each error message specifies the exact field path (e.g., `contract.network`, `attestors.registry[0].address`)

2. **Multiple Errors**: All validation errors are reported at once, not just the first one

3. **Clear Messages**: Each error explains what's wrong and often suggests the fix

4. **Format Support**: Works with both JSON and TOML files

5. **Syntax vs Schema**: Distinguishes between syntax errors (invalid JSON/TOML) and schema errors (missing/invalid fields)

6. **Batch Validation**: Can validate entire directories and reports status for each file

7. **Exit Codes**: Returns 0 for success, 1 for any validation failures

## Usage

```bash
# Validate a single file
anchorkit validate configs/testnet-example.json

# Validate all configs in a directory
anchorkit validate configs/

# Validate with default directory
anchorkit validate
```
