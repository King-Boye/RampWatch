# API Versioning & Deprecation Policy

## Versioning Scheme

SwiftRemit uses URL-prefix versioning. All endpoints are reachable under
`/v1/`. Example: `GET /v1/remittances`.

Unversioned paths (`GET /remittances`) are aliased to the current version
with a `Deprecation: true` header. Clients should migrate to versioned
paths.

An `Accept-Version` header is also supported as a fallback.

## Support Window

Each API version is supported for **12 months** after its successor is
released. During this window:

- The version continues to receive security fixes.
- No breaking changes are made.
- Deprecation and Sunset headers are emitted.

## Deprecation Headers

Deprecated endpoints include:

- `Deprecation: date="YYYY-MM-DD"` — when the deprecation was announced.
- `Sunset: YYYY-MM-DD` — when the endpoint will be removed.
- `Link: </v2/new-path>; rel="successor-version"` — where to migrate.

## Breaking Changes

A breaking change is any of:

- Removing a field from a response.
- Changing a field's type.
- Removing an endpoint.
- Changing error codes.

Breaking changes require a new API version. Additive changes (new fields,
new endpoints) are made within the current version.

## Off-Chain Proof Validation Errors

Settlement flows that require off-chain proof validation may return the
following contract errors.

| Error | Code | Meaning |
| --- | ---: | --- |
| `InvalidProof` | `51` | The submitted proof does not match the commitment stored for the remittance. |
| `MissingProof` | `52` | The settlement requires proof validation, but no proof was supplied to `confirm_payout`. |
| `InvalidOracleAddress` | `53` | Proof validation requires a valid oracle address, but the settlement configuration does not provide one or the configured oracle address is invalid. |

### `InvalidProof` — code 51

Returned when a proof is supplied but does not match the expected payout
commitment for the remittance.

Clients must not retry with the same invalid proof. A new proof matching the
commitment for the specific remittance must be obtained before attempting
settlement again.

### `MissingProof` — code 52

Returned by `confirm_payout` when the remittance has
`require_proof = true` but the caller does not provide a proof.

Clients should obtain the required proof before retrying the settlement.

### `InvalidOracleAddress` — code 53

Returned when proof validation is configured without a valid oracle address.
In particular, creating a remittance with `require_proof = true` and no
`oracle_address` is rejected.

Clients should provide a valid oracle address when proof validation is enabled.

## Contract Tests

The v1 response shapes are pinned by contract tests. Any change to a v1
response shape fails CI, ensuring backwards compatibility.
