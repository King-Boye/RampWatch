# Asset Verification

The asset verification system validates that assets referenced by the application
exist, are well-formed, and match their expected metadata before they are used.

## Overview

Asset verification runs against a configured network. Each network has its own
set of asset identifiers, contract addresses, and metadata sources, so
verification must be performed per network rather than against a single global
asset list.

## Supported Networks

| Network  | Description                                             |
| -------- | ------------------------------------------------------- |
| mainnet  | Production network with real assets and live metadata.  |
| testnet  | Test network with sandbox assets and staging metadata.  |

## Configuration

The active network is selected through configuration. Verification resolves the
network first, then loads the asset registry and metadata source associated with
that network.

```
ASSET_VERIFICATION_NETWORK=mainnet   # or: testnet
```

When no network is configured, verification defaults to `mainnet` to preserve
existing behavior.

## Verification Flow

1. Resolve the active network (`mainnet` or `testnet`).
2. Load the asset registry for that network.
3. Verify each asset against the network-specific metadata source.
4. Report results, tagging every result with the network it was verified on.

## Network-Specific Behavior

- **Asset registry:** each network maintains its own registry of known assets.
- **Metadata source:** mainnet uses production metadata; testnet uses staging
  metadata.
- **Contract addresses:** addresses are resolved per network and are never
  shared across networks.
- **Result reporting:** verification results include the network so that
  mainnet and testnet results are never conflated.

## Future Enhancements

- Multi-network support (mainnet, testnet) — implemented: verification now
  resolves and reports per network as described above.
- Additional networks (e.g. local/dev) can be added by extending the network
  configuration and registry without changing the verification flow.
- Advanced analytics dashboard — (#1541) implemented: verification metrics,
  reputation score distributions, trustline counts, and verification status
  breakdowns are exposed via the `/api/verification` endpoints and surfaced in
  the Grafana monitoring dashboards under `monitoring/dashboards/`.
- Automated dispute resolution — (#1542) implemented: the `resolve_dispute`
  contract function handles dispute resolution with configurable outcomes
  (in-favour-of-sender refunds escrow; in-favour-of-agent completes the
  remittance). The dispute window is configurable via `set_dispute_window`.
  See `src/lib.rs` for full contract details.
