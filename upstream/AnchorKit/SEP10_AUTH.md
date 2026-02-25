# SEP-10 Authentication Module

## Overview
Minimal SEP-10 (Stellar Web Authentication) implementation for AnchorKit following the Stellar SEP-10 specification.

## Features
- ✅ Challenge fetching from anchor
- ✅ Signature verification (Ed25519)
- ✅ Home domain validation with caching
- ✅ Secure JWT session storage with TTL
- ✅ Complete authentication flow

## Module Structure

### Types (`src/sep10_auth.rs`)

#### `Sep10Challenge`
```rust
pub struct Sep10Challenge {
    pub transaction: String,
    pub network_passphrase: String,
}
```

#### `Sep10Session`
```rust
pub struct Sep10Session {
    pub jwt: String,
    pub anchor: Address,
    pub expires_at: u64,
    pub home_domain: String,
}
```

### Functions

#### `fetch_challenge(env, anchor, client_account) -> Sep10Challenge`
Fetches SEP-10 challenge transaction from anchor.

#### `verify_signature(env, challenge, signature, public_key) -> bool`
Verifies Ed25519 signature on challenge transaction.

#### `validate_home_domain(env, anchor, home_domain) -> bool`
Validates and caches home domain for anchor. Returns false if domain doesn't match cached value.

#### `store_session(env, session)`
Stores JWT session securely with 1-day TTL in persistent storage.

#### `get_session(env, anchor) -> Option<Sep10Session>`
Retrieves stored session for anchor.

#### `authenticate(env, anchor, client_account, signature, public_key, home_domain) -> Result<Sep10Session, u32>`
Complete authentication flow:
1. Fetches challenge
2. Verifies signature
3. Validates home domain
4. Creates and stores session

Returns `Err(401)` for invalid signature, `Err(403)` for invalid domain.

## Contract Methods

The following methods are available on `AnchorKitContract`:

```rust
// Fetch challenge
sep10_fetch_challenge(env, anchor, client_account) -> Result<Sep10Challenge, Error>

// Verify signature
sep10_verify_signature(env, challenge, signature, public_key) -> bool

// Validate domain
sep10_validate_domain(env, anchor, home_domain) -> Result<bool, Error>

// Store session
sep10_store_session(env, session) -> Result<(), Error>

// Get session
sep10_get_session(env, anchor) -> Option<Sep10Session>

// Complete flow
sep10_authenticate(env, anchor, client_account, signature, public_key, home_domain) 
    -> Result<Sep10Session, Error>
```

## Usage Example

```rust
use anchorkit::{Sep10Challenge, Sep10Session};

// 1. Fetch challenge from anchor
let challenge = contract.sep10_fetch_challenge(&anchor, &client)?;

// 2. Client signs challenge (off-chain)
let signature = sign_challenge(&challenge);

// 3. Complete authentication
let session = contract.sep10_authenticate(
    &anchor,
    &client,
    signature,
    public_key,
    home_domain
)?;

// 4. Use JWT for subsequent requests
let jwt = session.jwt;
```

## Testing

Run SEP-10 tests:
```bash
cargo test --lib sep10
```

Current test coverage:
- ✅ Challenge fetching
- ✅ Signature verification

## Compliance

Follows [SEP-10: Stellar Web Authentication](https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0010.md) specification:
- Challenge transaction format
- Ed25519 signature verification
- Home domain validation
- JWT session management

## Security

- Signatures verified using Soroban SDK's Ed25519 verification
- Home domains cached and validated on each auth
- Sessions stored with 1-day TTL
- Persistent storage for session data

## Integration

The module is fully integrated into AnchorKit:
- Exported types: `Sep10Challenge`, `Sep10Session`
- Contract methods prefixed with `sep10_`
- Error handling via `Error` enum
- Requires anchor to be registered attestor
