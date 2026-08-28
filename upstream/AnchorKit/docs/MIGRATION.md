# AnchorKit Migration Guide

This document describes breaking changes, new features, and error code additions across
AnchorKit releases. Use it when upgrading between major versions.

---

## New Error Codes

The following error codes were introduced in recent releases. All discriminant values are
stable — they will not change between patch or minor releases.

| Code                     | Value | Meaning                                              |
|--------------------------|-------|------------------------------------------------------|
| `NoQuotesAvailable`      | 13    | No valid quotes found for the requested asset pair   |
| `ServicesNotConfigured`  | 14    | Anchor has not configured any supported services     |
| `ValidationError`        | 15    | Input failed schema or business-rule validation      |
| `RateLimitExceeded`      | 16    | Request rate limit exceeded; retry after back-off    |
| `InvalidSep10Token`      | 18    | Provided SEP-10 challenge token is invalid           |
| `StorageCorrupted`       | 19    | On-chain storage entry is unreadable                 |
| `CacheExpired`           | 48    | Cached value has passed its TTL and is no longer valid |
| `CacheNotFound`          | 49    | Requested key is not present in the cache            |
| `AuditLogMaxSizeInvalid` | 51    | Configured audit-log maximum size is out of range    |

> **Note:** Value `50` is intentionally unassigned. Do not hard-code numeric comparisons
> against error values; always use the named constant.

---

## Upgrading

### From v1.x to v2.x

- `StorageCorrupted` (value `19`) replaces the former generic `StorageError`. Update any
  client-side error-handling branches that matched on the old code.
- The gap at value `50` is deliberate and reserved; no variant maps to `50`.

### General advice

- Always match on named error variants, not raw integer values.
- Consult `docs/features/ERROR_CODES_REFERENCE.md` for the authoritative full listing.
