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

## Reputation Decay

Cached verification results degrade over time so that stale data does not
permanently lock an asset into a high-reputation state.  The decay model is
**linear**: a result keeps its full score when fresh and reaches zero after
2 × `DECAY_HALF_LIFE` (currently 48 hours).

```
decayed_score = score × max(0, 1 − age / (2 × half_life))
```

The stored score is never mutated — decay is applied on read so the original
verification result remains available for audit purposes.  When the decayed
score crosses a status boundary the displayed status updates accordingly:

| Decayed score | Status      |
| ------------- | ----------- |
| ≥ 70 (+ ≥ 3 verified sources) | `verified`   |
| < 30 or suspicious indicators  | `suspicious` |
| otherwise                      | `unverified` |

The `verified_at` ISO-8601 timestamp on every `VerificationResult` is the
anchor used to compute age.

## Weighted Scoring

Each verification source is assigned a **reliability weight** that reflects
its quality as a signal.  The final `reputation_score` is a weighted average
of the scores from sources that individually passed their verification check.

| Source               | Weight | Rationale                                         |
| -------------------- | ------ | ------------------------------------------------- |
| Stellar Expert       | 0.40   | Curated community rating; highest signal quality  |
| Stellar TOML         | 0.30   | Issuer self-attestation with on-chain anchor      |
| Trustline Analysis   | 0.20   | On-chain adoption; gameable at low cost           |
| Transaction History  | 0.10   | Activity proxy; least discriminating signal       |

Weights are normalised internally.  A source that fails its check contributes
`score = 0` but is **excluded** from the weighted average so it does not drag
down a result where other sources could not be reached.

The `reliability_weight` field is exposed on each `VerificationSource` object
in the API response so consumers can see how the final score was computed.

## Future Enhancements

- Multi-network support (mainnet, testnet) — implemented: verification now
  resolves and reports per network as described above.
- Reputation decay over time — implemented: linear decay with 24-hour half-life
  applied on cache read (see Reputation Decay section above). Closes #1543.
- Weighted scoring based on source reliability — implemented: weighted-average
  score using per-source reliability weights (see Weighted Scoring section
  above). Closes #1544.
- Additional networks (e.g. local/dev) can be added by extending the network
  configuration and registry without changing the verification flow.
