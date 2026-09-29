# Asset Verification

## Overview

The asset verification system validates ownership and authenticity of assets submitted to the platform. It combines automated checks with manual review to ensure that listed assets are legitimate and accurately represented.

## Verification Flow

1. **Submission** — A user submits an asset along with supporting documentation (proof of ownership, registration records, etc.).
2. **Automated Checks** — The system runs a series of automated validations against the submitted data.
3. **Manual Review** — Flagged or high-value submissions are routed to a human reviewer.
4. **Decision** — The asset is approved, rejected, or returned to the user for additional information.

## Automated Checks

- Document format and completeness validation
- Duplicate detection against existing verified assets
- Cross-referencing of identifiers with external registries
- Consistency checks between declared and documented attributes

## Anchor Registries

The system cross-references asset identifiers against external anchor registries. Each registry is integrated through a common adapter interface so that additional registries can be added without changing the verification flow.

### Supported Registries

| Registry | Identifier Type | Notes |
| --- | --- | --- |
| Default anchor registry | Anchor ID | Built-in registry used when no other registry is specified |

### Adding a Registry

To integrate an additional anchor registry:

1. Implement the registry adapter interface, mapping the registry's identifier format to the platform's anchor identifier.
2. Register the adapter with the verification system so it is consulted during the automated cross-referencing step.
3. Document the registry in the table above, including the identifier type it resolves.

Registries are consulted in order, and a match from any registered registry satisfies the cross-referencing check. Failures from a single registry do not block verification; they are recorded and surfaced to the reviewer.

## Manual Review

Reviewers inspect the submitted documentation and confirm that it matches the declared asset attributes. Reviewers can approve, reject, or request more information.

## Statuses

| Status | Description |
| --- | --- |
| `pending` | Awaiting automated or manual review |
| `verified` | Successfully verified |
| `rejected` | Failed verification |
| `needs_info` | Additional information requested from the user |

## Future Enhancements

- Machine learning for fraud detection
- Integration with additional external registries
- Automated document OCR and field extraction
- Risk scoring based on historical submission patterns
- Bulk verification for institutional users
