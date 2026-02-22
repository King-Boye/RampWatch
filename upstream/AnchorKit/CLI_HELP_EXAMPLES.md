# AnchorKit CLI Help Examples

This document shows example help output for the new CLI commands.

## Main Help

```
$ anchorkit --help

AnchorKit - Soroban toolkit for anchoring off-chain attestations to Stellar

AnchorKit enables smart contracts to verify real-world events such as KYC 
approvals, payment confirmations, and signed claims in a trust-minimized way.

Usage: anchorkit <COMMAND>

Commands:
  build     Build the AnchorKit smart contract
  deploy    Deploy compiled contract to configured network
  init      Initialize deployed contract with admin account
  register  Register a new attestor/anchor
  attest    Submit an attestation for verification
  query     Query attestation by ID
  health    Check health status of registered attestors
  test      Run contract tests
  validate  Validate configuration files
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Deploy Command Help

```
$ anchorkit deploy --help

Deploy compiled contract to configured network

Deploys your AnchorKit contract to the specified Stellar network.
Requires a funded admin account and network configuration.
Use --dry-run to validate deployment without executing.

Examples:
  anchorkit deploy
  anchorkit deploy --network devnet
  anchorkit deploy --dry-run

Usage: anchorkit deploy [OPTIONS]

Options:
  -n, --network <NETWORK>  Target network (testnet, devnet, mainnet)
                           [default: testnet]
      --dry-run            Validate deployment without executing
  -h, --help               Print help
```

## Register Command Help

```
$ anchorkit register --help

Register a new attestor/anchor

Adds an attestor to the contract, allowing them to submit attestations.
Only the contract admin can register attestors.
Optionally configure supported services during registration.

Examples:
  anchorkit register --address GANCHOR123...
  anchorkit register --address GANCHOR123... --services deposits,withdrawals,kyc
  anchorkit register --address GANCHOR123... --endpoint https://anchor.example.com

Usage: anchorkit register [OPTIONS] --address <ADDRESS>

Options:
  -a, --address <ADDRESS>      Attestor account address
  -s, --services <SERVICES>    Supported services (deposits, withdrawals, quotes, kyc)
  -e, --endpoint <ENDPOINT>    Attestor endpoint URL
  -n, --network <NETWORK>      Target network [default: testnet]
  -h, --help                   Print help
```

## Attest Command Help

```
$ anchorkit attest --help

Submit an attestation for verification

Creates an attestation linking an off-chain event to on-chain verification.
Requires the submitter to be a registered attestor.
Includes replay protection and timestamp validation.

Examples:
  anchorkit attest --subject GUSER123... --payload-hash abc123...
  anchorkit attest --subject GUSER123... --payload-hash abc123... --session session-001

Usage: anchorkit attest [OPTIONS] --subject <SUBJECT> --payload-hash <PAYLOAD_HASH>

Options:
  -s, --subject <SUBJECT>            Subject account address
  -p, --payload-hash <PAYLOAD_HASH>  SHA-256 hash of attestation payload
      --session <SESSION>            Optional session ID for traceability
  -n, --network <NETWORK>            Target network [default: testnet]
  -h, --help                         Print help
```

## Health Command Help

```
$ anchorkit health --help

Check health status of registered attestors

Monitors attestor availability, latency, and failure rates.
Helps identify performance issues and service degradation.
Use --watch for continuous monitoring.

Examples:
  anchorkit health
  anchorkit health --attestor GANCHOR123...
  anchorkit health --watch --interval 30

Usage: anchorkit health [OPTIONS]

Options:
  -a, --attestor <ATTESTOR>  Specific attestor to check (optional)
  -w, --watch                Continuous monitoring mode
  -i, --interval <INTERVAL>  Check interval in seconds (for watch mode) [default: 60]
  -n, --network <NETWORK>    Target network [default: testnet]
  -h, --help                 Print help
```

## Build Command Help

```
$ anchorkit build --help

Build the AnchorKit smart contract

Compiles the contract to WASM format optimized for Soroban deployment.
Use this before deploying to ensure your contract is ready.

Examples:
  anchorkit build
  anchorkit build --release

Usage: anchorkit build [OPTIONS]

Options:
  -r, --release  Build with release optimizations
  -h, --help     Print help
```

## Key Features

All commands now include:

1. **Clear descriptions** - Explains when and why to use each command
2. **Real examples** - At least one practical usage example per command
3. **Readable formatting** - Optimized for 80-100 character terminals
4. **Contextual help** - Describes the purpose, not just the syntax
5. **Network support** - Most commands support network selection
6. **Optional flags** - Clearly marked with descriptions

## Benefits for Developers

- **Self-documenting** - No need to open external docs for basic usage
- **Faster onboarding** - New developers can learn by exploring help
- **Reduced errors** - Clear examples prevent common mistakes
- **Better UX** - Professional CLI experience matching industry standards
