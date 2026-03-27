# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive CONTRIBUTING.md with contributor guidelines
- MIT LICENSE file at repository root
- Unit tests for domain_validator edge cases (Unicode/IDN, IP addresses, trailing slashes, length boundaries)
- License field to Cargo.toml

### Changed
- Updated README.md license section to reference LICENSE file

## [0.1.0] - 2024-01-01

### Added
- **Attestation Management**: Register attestors, submit and retrieve attestations with replay attack protection
- **Endpoint Configuration**: Manage attestor endpoints for off-chain integration
- **Unified Anchor Adapter**: Consistent API for multiple anchor integrations
- **Session Management**: Group operations into logical sessions for traceability
- **Audit Trail**: Complete immutable record of all operations
- **Reproducibility**: Deterministic operation replay for verification
- **Replay Protection**: Multi-level protection against unauthorized replays
- **Secure Credential Management**: Runtime credential injection with automatic rotation
- **Domain Validation**: HTTPS-only domain validation with comprehensive edge case handling
- **Transaction State Tracker**: Track and manage transaction states
- **Response Validator**: Validate anchor responses with error handling
- **Service Capability Discovery**: Discover anchor services (deposits, withdrawals, quotes, KYC)
- **Anchor Info Discovery**: Fetch and parse stellar.toml, cache assets/fees/limits
- **Health Monitoring**: Monitor anchor latency, failures, and availability
- **Metadata Caching**: TTL-based caching with manual refresh
- **Request ID Propagation**: UUID per flow with tracing
- **Event Emission**: Emit events for all state changes
- **Comprehensive Error Handling**: Stable error codes (100-120) for API compatibility
- **CLI Tool**: Command-line interface for contract interaction
- **Cross-Platform Support**: Linux, macOS, and Windows compatibility
- **Configuration Validation**: Validate configuration files with JSON schema
- **Environment Diagnostics**: Doctor command for troubleshooting setup issues

### Security
- Stable error codes for API compatibility
- Replay protection at multiple levels
- Immutable audit logs
- Authorization checks on all operations
- Complete operation context for verification

### Performance
- Efficient storage with TTL management
- Minimal event data
- Sequential IDs (no hash lookups)
- Optimized for Soroban constraints

## [0.0.1] - 2023-12-01

### Added
- Initial project structure
- Basic contract skeleton
- Core type definitions
- Error handling framework
