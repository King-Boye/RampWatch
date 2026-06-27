# Validation Enhancement: Before vs After

## Overview

This document shows the concrete improvements in the validation output.

---

## Scenario 1: Missing Required Field

### Config File
```json
{
  "contract": {
    "name": "my-anchor",
    "version": "1.0.0"
  },
  "attestors": {
    "registry": [
      {
        "name": "kyc-provider",
        "address": "GBAA5XKQC3KVDPD5OS3CHJJ24SB3BX7GI7XBXKNNCKQVPQVX6S3VT5O",
        "endpoint": "https://kyc.example.com",
        "role": "kyc-issuer",
        "enabled": true
      }
    ]
  }
}
```

### Before
```
✔ configs/example.json: valid JSON
```
**Problem:** File is syntactically valid JSON, but missing required `contract.network` and `sessions` sections. User has no idea what's wrong.

### After
```
✖ configs/example.json: invalid configuration
  • field 'contract.network' is missing
  • field 'sessions' is missing
```
**Solution:** User immediately knows exactly which fields to add.

---

## Scenario 2: Invalid Field Format

### Config File
```json
{
  "contract": {
    "name": "My-Anchor-Name",
    "version": "1.0",
    "network": "testnet"
  },
  ...
}
```

### Before
```
✔ configs/example.json: valid JSON
```
**Problem:** Name has uppercase letters (invalid), version is missing patch number, network value is wrong. No indication of issues.

### After
```
✖ configs/example.json: invalid configuration
  • field 'contract.name' must contain only lowercase letters, numbers, and hyphens
  • field 'contract.version' must follow semantic versioning (e.g., 1.0.0)
  • field 'contract.network' must be one of: stellar-testnet, stellar-mainnet, stellar-futurenet
```
**Solution:** User knows exactly what format each field requires.

---

## Scenario 3: Invalid Stellar Address

### Config File
```json
{
  "attestors": {
    "registry": [
      {
        "name": "kyc-provider",
        "address": "INVALID_ADDRESS_123",
        "endpoint": "https://kyc.example.com",
        "role": "kyc-issuer",
        "enabled": true
      }
    ]
  }
}
```

### Before
```
✔ configs/example.json: valid JSON
```
**Problem:** Invalid Stellar address format, but no indication.

### After
```
✖ configs/example.json: invalid configuration
  • field 'attestors.registry[0].address' has invalid Stellar address format (must start with 'G' and be 56 characters)
  • field 'attestors.registry[0].address' length must be 54-56 characters, got 19
```
**Solution:** User knows the address is wrong, what format is expected, and which attestor (index 0) has the issue.

---

## Scenario 4: Insecure Endpoint URL

### Config File
```json
{
  "attestors": {
    "registry": [
      {
        "name": "kyc-provider",
        "address": "GBAA5XKQC3KVDPD5OS3CHJJ24SB3BX7GI7XBXKNNCKQVPQVX6S3VT5O",
        "endpoint": "http://localhost:8080/api",
        "role": "kyc-issuer",
        "enabled": true
      }
    ]
  }
}
```

### Before
```
✔ configs/example.json: valid JSON
```
**Problem:** Using HTTP instead of HTTPS, and localhost is not allowed. No indication.

### After
```
✖ configs/example.json: invalid configuration
  • field 'attestors.registry[0].endpoint' has invalid URL 'http://localhost:8080/api' — URL must use HTTPS (http:// and other schemes are not allowed)
```
**Solution:** User knows HTTPS is required and sees the problematic URL.

---

## Scenario 5: Multiple Errors

### Config File
```json
{
  "contract": {
    "name": "Invalid_Name",
    "version": "1.0",
    "network": "wrong-network"
  },
  "attestors": {
    "registry": [
      {
        "name": "attestor-1",
        "address": "INVALID",
        "endpoint": "http://example.com",
        "role": "wrong-role",
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

### Before
```
✔ configs/example.json: valid JSON
```
**Problem:** Multiple issues across all sections, but user sees nothing wrong.

### After
```
✖ configs/example.json: invalid configuration
  • field 'contract.name' must contain only lowercase letters, numbers, and hyphens
  • field 'contract.version' must follow semantic versioning (e.g., 1.0.0)
  • field 'contract.network' must be one of: stellar-testnet, stellar-mainnet, stellar-futurenet
  • field 'attestors.registry[0].address' has invalid Stellar address format (must start with 'G' and be 56 characters)
  • field 'attestors.registry[0].address' length must be 54-56 characters, got 7
  • field 'attestors.registry[0].endpoint' has invalid URL 'http://example.com' — URL must use HTTPS (http:// and other schemes are not allowed)
  • field 'attestors.registry[0].role' must be one of: kyc-issuer, transfer-verifier, compliance-approver, rate-provider, attestor
  • at least one attestor must be enabled (attestors.registry[].enabled = true)
  • field 'sessions.session_timeout_seconds' must be at least 60 seconds
  • field 'sessions.operations_per_session' exceeds maximum of 10000, got 15000
  • field 'sessions.audit_log_retention_days' exceeds maximum of 3650 days, got 5000
```
**Solution:** User sees ALL issues at once and can fix them in one pass instead of discovering them one at a time.

---

## Scenario 6: Warnings (Non-blocking)

### Config File
```json
{
  "sessions": {
    "enable_session_tracking": true,
    "session_timeout_seconds": 90000,
    "operations_per_session": 8000,
    "audit_log_retention_days": 365
  }
}
```

### Before
```
✔ configs/example.json: valid JSON
```
**Problem:** Config is valid but has suboptimal settings. No guidance provided.

### After
```
⚠  configs/example.json: field 'sessions.session_timeout_seconds' exceeds 24 hours — consider shorter timeouts for security
⚠  configs/example.json: field 'sessions.operations_per_session' is high (>5000) and may impact performance
✔ configs/example.json: valid configuration
```
**Solution:** User is warned about potential issues but validation still passes. They can make informed decisions.

---

## Scenario 7: Duplicate Attestors

### Config File
```json
{
  "attestors": {
    "registry": [
      {
        "name": "kyc-provider",
        "address": "GBAA5XKQC3KVDPD5OS3CHJJ24SB3BX7GI7XBXKNNCKQVPQVX6S3VT5O",
        "endpoint": "https://kyc1.example.com",
        "role": "kyc-issuer",
        "enabled": true
      },
      {
        "name": "kyc-provider",
        "address": "GBAA5XKQC3KVDPD5OS3CHJJ24SB3BX7GI7XBXKNNCKQVPQVX6S3VT5O",
        "endpoint": "https://kyc2.example.com",
        "role": "kyc-issuer",
        "enabled": true
      }
    ]
  }
}
```

### Before
```
✔ configs/example.json: valid JSON
```
**Problem:** Duplicate names and addresses, but no detection.

### After
```
✖ configs/example.json: invalid configuration
  • duplicate attestor name found: 'kyc-provider'
  • duplicate attestor address found: 'GBAA5XKQC3KVDPD5OS3CHJJ24SB3BX7GI7XBXKNNCKQVPQVX6S3VT5O'
```
**Solution:** User knows there are duplicates and which ones.

---

## Scenario 8: Directory Validation

### Before
```bash
$ anchorkit validate configs/
✔ configs/config1.json: valid JSON
✔ configs/config2.json: valid JSON
✔ configs/config3.json: valid JSON
```
**Problem:** All show as valid even if they have schema issues.

### After
```bash
$ anchorkit validate configs/
✔ configs/config1.json: valid configuration
✖ configs/config2.json: invalid configuration
  • field 'contract.network' is missing
  • field 'sessions' is missing
✔ configs/config3.json: valid configuration
```
**Solution:** Clear status for each file, with errors shown for invalid ones.

---

## Key Improvements Summary

| Aspect | Before | After |
|--------|--------|-------|
| **Error Detection** | Syntax only | Syntax + Schema + Business rules |
| **Error Messages** | Generic "invalid" | Specific field paths |
| **Multiple Errors** | Not shown | All errors at once |
| **Field Paths** | None | Dot notation (contract.network) |
| **Array Indices** | None | Bracket notation (registry[0]) |
| **Actionable Info** | No | Yes (what's wrong + how to fix) |
| **Warnings** | None | Non-blocking best practices |
| **URL Validation** | None | Comprehensive security checks |
| **Duplicates** | Not detected | Detected and reported |
| **Business Rules** | Not checked | Fully validated |

---

## Impact

### For Developers
- **Faster debugging**: See all issues immediately
- **Clear guidance**: Know exactly what to fix
- **Fewer iterations**: Fix all issues in one pass
- **Better configs**: Warnings guide best practices

### For Operations
- **Pre-deployment safety**: Catch errors before deployment
- **Reduced downtime**: Fewer config-related failures
- **Security**: HTTPS enforcement, localhost rejection
- **Compliance**: Proper validation of all fields

### For CI/CD
- **Reliable validation**: Fails fast on invalid configs
- **Clear logs**: Error messages in pipeline output
- **Automated checks**: No manual config review needed
- **Consistent enforcement**: Same rules everywhere
