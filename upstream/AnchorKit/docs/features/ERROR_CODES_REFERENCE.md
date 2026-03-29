# AnchorKit Error Codes Reference

Quick lookup table for all error codes and their properties.

## Error Code Ranges

| Range | Category | Count |
|-------|----------|-------|
| 1001-1099 | Initialization & State | 2 |
| 1101-1199 | Attestor Management | 3 |
| 1201-1299 | Security | 2 |
| 1301-1399 | Attestation | 1 |
| 1401-1499 | Endpoint Management | 2 |
| 1501-1599 | Service Configuration | 2 |
| 1601-1699 | Session Management | 2 |
| 1701-1799 | Quote Management | 3 |
| 1801-1899 | Transaction Intent | 2 |
| 1901-1999 | Configuration | 2 |
| 2001-2099 | Credentials | 3 |
| 2101-2199 | Anchor Metadata | 2 |
| 2201-2299 | Transport Layer | 3 |
| 2301-2399 | Protocol Layer | 3 |
| 2401-2499 | Cache | 2 |
| 2501-2599 | Rate Limiting | 1 |
| 2601-2699 | Asset Validation | 2 |

## Complete Error Code Table

### Initialization & State (1000-1099)

| Code | Name | Category | Severity | Retryable |
|------|------|----------|----------|-----------|
| 1001 | AlreadyInitialized | Application | Medium | No |
| 1002 | NotInitialized | Application | Medium | No |

### Attestor Management (1100-1199)

| Code | Name | Category | Severity | Retryable |
|------|------|----------|----------|-----------|
| 1101 | UnauthorizedAttestor | Application | High | No |
| 1102 | AttestorAlreadyRegistered | Application | Medium | No |
| 1103 | AttestorNotRegistered | Application | Medium | No |

### Security (1200-1299)

| Code | Name | Category | Severity | Retryable |
|------|------|----------|----------|-----------|
| 1201 | ReplayAttack | Application | Critical | No |
| 1202 | InvalidTimestamp | Application | Medium | No |

### Attestation (1300-1399)

| Code | Name | Category | Severity | Retryable |
|------|------|----------|----------|-----------|
| 1301 | AttestationNotFound | Application | Medium | Yes |

### Endpoint Management (1400-1499)

| Code | Name | Category | Severity | Retryable |
|------|------|----------|----------|-----------|
| 1401 | InvalidEndpointFormat | Application | Medium | No |
| 1402 | EndpointNotFound | Application | Medium | Yes |

### Service Configuration (1500-1599)

| Code | Name | Category | Severity | Retryable |
|------|------|----------|----------|-----------|
| 1501 | ServicesNotConfigured | Application | Medium | Yes |
| 1502 | InvalidServiceType | Application | Medium | No |

### Session Management (1600-1699)

| Code | Name | Category | Severity | Retryable |
|------|------|----------|----------|-----------|
| 1601 | SessionNotFound | Application | Medium | Yes |
| 1602 | InvalidSessionId | Application | Medium | No |

### Quote Management (1700-1799)

| Code | Name | Category | Severity | Retryable |
|------|------|----------|----------|-----------|
| 1701 | InvalidQuote | Application | Medium | No |
| 1702 | StaleQuote | Application | Low | Yes |
| 1703 | NoQuotesAvailable | Application | Low | Yes |

### Transaction Intent (1800-1899)

| Code | Name | Category | Severity | Retryable |
|------|------|----------|----------|-----------|
| 1801 | InvalidTransactionIntent | Application | Medium | No |
| 1802 | ComplianceNotMet | Application | Critical | No |

### Configuration (1900-1999)

| Code | Name | Category | Severity | Retryable |
|------|------|----------|----------|-----------|
| 1901 | InvalidConfig | Application | Medium | No |
| 1902 | DuplicateAttestor | Application | Medium | No |

### Credentials (2000-2099)

| Code | Name | Category | Severity | Retryable |
|------|------|----------|----------|-----------|
| 2001 | InvalidCredentialFormat | Application | Medium | No |
| 2002 | CredentialNotFound | Application | Medium | No |
| 2003 | CredentialExpired | Application | Medium | No |

### Anchor Metadata (2100-2199)

| Code | Name | Category | Severity | Retryable |
|------|------|----------|----------|-----------|
| 2101 | InvalidAnchorMetadata | Application | Medium | No |
| 2102 | AnchorMetadataNotFound | Application | Medium | Yes |

### Transport Layer (2200-2299)

| Code | Name | Category | Severity | Retryable |
|------|------|----------|----------|-----------|
| 2201 | TransportError | Transport | Medium | Yes |
| 2202 | TransportTimeout | Transport | Low | Yes |
| 2203 | TransportUnauthorized | Transport | High | No |

### Protocol Layer (2300-2399)

| Code | Name | Category | Severity | Retryable |
|------|------|----------|----------|-----------|
| 2301 | ProtocolError | Protocol | Medium | No |
| 2302 | ProtocolInvalidPayload | Protocol | Medium | No |
| 2303 | ProtocolRateLimitExceeded | Protocol | Low | Yes |

### Cache (2400-2499)

| Code | Name | Category | Severity | Retryable |
|------|------|----------|----------|-----------|
| 2401 | CacheExpired | Application | Medium | Yes |
| 2402 | CacheNotFound | Application | Medium | Yes |

### Rate Limiting (2500-2599)

| Code | Name | Category | Severity | Retryable |
|------|------|----------|----------|-----------|
| 2501 | RateLimitExceeded | Application | Medium | No |

### Asset Validation (2600-2699)

| Code | Name | Category | Severity | Retryable |
|------|------|----------|----------|-----------|
| 2601 | AssetNotConfigured | Application | Medium | No |
| 2602 | UnsupportedAsset | Application | Medium | No |

## Error Code Lookup by Name

| Name | Code | Category | Severity | Retryable |
|------|------|----------|----------|-----------|
| AlreadyInitialized | 1001 | Application | Medium | No |
| NotInitialized | 1002 | Application | Medium | No |
| UnauthorizedAttestor | 1101 | Application | High | No |
| AttestorAlreadyRegistered | 1102 | Application | Medium | No |
| AttestorNotRegistered | 1103 | Application | Medium | No |
| ReplayAttack | 1201 | Application | Critical | No |
| InvalidTimestamp | 1202 | Application | Medium | No |
| AttestationNotFound | 1301 | Application | Medium | Yes |
| InvalidEndpointFormat | 1401 | Application | Medium | No |
| EndpointNotFound | 1402 | Application | Medium | Yes |
| ServicesNotConfigured | 1501 | Application | Medium | Yes |
| InvalidServiceType | 1502 | Application | Medium | No |
| SessionNotFound | 1601 | Application | Medium | Yes |
| InvalidSessionId | 1602 | Application | Medium | No |
| InvalidQuote | 1701 | Application | Medium | No |
| StaleQuote | 1702 | Application | Low | Yes |
| NoQuotesAvailable | 1703 | Application | Low | Yes |
| InvalidTransactionIntent | 1801 | Application | Medium | No |
| ComplianceNotMet | 1802 | Application | Critical | No |
| InvalidConfig | 1901 | Application | Medium | No |
| DuplicateAttestor | 1902 | Application | Medium | No |
| InvalidCredentialFormat | 2001 | Application | Medium | No |
| CredentialNotFound | 2002 | Application | Medium | No |
| CredentialExpired | 2003 | Application | Medium | No |
| InvalidAnchorMetadata | 2101 | Application | Medium | No |
| AnchorMetadataNotFound | 2102 | Application | Medium | Yes |
| TransportError | 2201 | Transport | Medium | Yes |
| TransportTimeout | 2202 | Transport | Low | Yes |
| TransportUnauthorized | 2203 | Transport | High | No |
| ProtocolError | 2301 | Protocol | Medium | No |
| ProtocolInvalidPayload | 2302 | Protocol | Medium | No |
| ProtocolRateLimitExceeded | 2303 | Protocol | Low | Yes |
| CacheExpired | 2401 | Application | Medium | Yes |
| CacheNotFound | 2402 | Application | Medium | Yes |
| RateLimitExceeded | 2501 | Application | Medium | No |
| AssetNotConfigured | 2601 | Application | Medium | No |
| UnsupportedAsset | 2602 | Application | Medium | No |

## Retryable Errors Summary

### Always Retryable
- 2202 TransportTimeout
- 2201 TransportError
- 2303 ProtocolRateLimitExceeded

### Conditionally Retryable (Data/State)
- 1301 AttestationNotFound
- 1402 EndpointNotFound
- 1501 ServicesNotConfigured
- 1601 SessionNotFound
- 1702 StaleQuote
- 1703 NoQuotesAvailable
- 2102 AnchorMetadataNotFound
- 2401 CacheExpired
- 2402 CacheNotFound

### Never Retryable
- 1001 AlreadyInitialized
- 1002 NotInitialized
- 1101 UnauthorizedAttestor
- 1102 AttestorAlreadyRegistered
- 1103 AttestorNotRegistered
- 1201 ReplayAttack
- 1202 InvalidTimestamp
- 1401 InvalidEndpointFormat
- 1502 InvalidServiceType
- 1602 InvalidSessionId
- 1701 InvalidQuote
- 1801 InvalidTransactionIntent
- 1802 ComplianceNotMet
- 1901 InvalidConfig
- 1902 DuplicateAttestor
- 2001 InvalidCredentialFormat
- 2002 CredentialNotFound
- 2003 CredentialExpired
- 2101 InvalidAnchorMetadata
- 2203 TransportUnauthorized
- 2301 ProtocolError
- 2302 ProtocolInvalidPayload
- 2501 RateLimitExceeded
- 2601 AssetNotConfigured
- 2602 UnsupportedAsset

## Critical Errors (Severity 4)

- 1201 ReplayAttack
- 1802 ComplianceNotMet

**Action**: Stop operation immediately, alert, investigate.

## High Severity Errors (Severity 3)

- 1101 UnauthorizedAttestor
- 2203 TransportUnauthorized

**Action**: Log, handle carefully, may require user intervention.

## Medium Severity Errors (Severity 2)

- 1001 AlreadyInitialized
- 1002 NotInitialized
- 1102 AttestorAlreadyRegistered
- 1103 AttestorNotRegistered
- 1202 InvalidTimestamp
- 1301 AttestationNotFound
- 1401 InvalidEndpointFormat
- 1402 EndpointNotFound
- 1501 ServicesNotConfigured
- 1502 InvalidServiceType
- 1601 SessionNotFound
- 1602 InvalidSessionId
- 1701 InvalidQuote
- 1801 InvalidTransactionIntent
- 1901 InvalidConfig
- 1902 DuplicateAttestor
- 2001 InvalidCredentialFormat
- 2002 CredentialNotFound
- 2003 CredentialExpired
- 2101 InvalidAnchorMetadata
- 2102 AnchorMetadataNotFound
- 2201 TransportError
- 2301 ProtocolError
- 2302 ProtocolInvalidPayload
- 2501 RateLimitExceeded
- 2601 AssetNotConfigured
- 2602 UnsupportedAsset

**Action**: Log and handle appropriately.

## Low Severity Errors (Severity 1)

- 1702 StaleQuote
- 1703 NoQuotesAvailable
- 2202 TransportTimeout
- 2303 ProtocolRateLimitExceeded
- 2401 CacheExpired
- 2402 CacheNotFound

**Action**: Retry or ignore, typically transient.

## Error Response Format

All errors return this standardized format:

```json
{
  "code": 2202,
  "name": "TransportTimeout",
  "category": "transport",
  "severity": "low",
  "retryable": true
}
```

## Usage Examples

### Check if error is retryable
```rust
let error = AnchorKitError::from(Error::TransportTimeout);
if error.is_retryable() {
    // Implement retry logic
}
```

### Get error code
```rust
let error = AnchorKitError::from(Error::InvalidConfig);
let code = error.code_u32();  // 1901
```

### Check error severity
```rust
let error = AnchorKitError::from(Error::ReplayAttack);
match error.severity() {
    ErrorSeverity::Critical => { /* alert */ }
    ErrorSeverity::High => { /* log */ }
    ErrorSeverity::Medium => { /* handle */ }
    ErrorSeverity::Low => { /* retry */ }
}
```

### Get standardized response
```rust
let error = AnchorKitError::from(Error::SessionNotFound);
let response = error.response;
println!("Error {}: {} ({})", response.code, response.name, response.category);
```
