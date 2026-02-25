# Migration Guide: SwiftRemit Contract

This guide outlines the key changes and migration steps for upgrading to the latest version of the SwiftRemit contract, focusing on deterministic settlement hashes and batch settlement features.

## New Features

### 1. Deterministic Settlement Hashing
The contract now utilizes a deterministic hashing mechanism for all settlements. This allows off-chain systems to pre-calculate settlement IDs and verify on-chain state with cryptographic certainty.

- **Internal Logic**: Uses SHA-256 over canonicalized fields (sender, agent, amount, fee, expiry).
- **Public API**: `compute_settlement_hash(env, remittance_id)` allows external callers to retrieve the expected hash for any pending remittance.

### 2. Batch Settlement with Netting
To optimize gas costs and reduce token transfer overhead, the contract now supports batch settlement of multiple remittances with netting logic.

- **Net Settlement**: Offsets opposing flows between the same two parties within a single batch.
- **Max Batch Size**: 50 remittances per transaction.
- **Function**: `batch_settle_with_netting(env, entries)`.

## Breaking Changes

### Function Signatures
The `create_remittance` method has been simplified. The `default_currency` and `default_country` arguments have been removed in favor of a simpler 4-argument signature (plus `Env`).

**Old Signature:**
```rust
pub fn create_remittance(
    env: Env,
    sender: Address,
    agent: Address,
    amount: i128,
    currency: String,
    country: String,
    expiry: Option<u64>,
) -> Result<u64, ContractError>
```

**New Signature:**
```rust
pub fn create_remittance(
    env: Env,
    sender: Address,
    agent: Address,
    amount: i128,
    expiry: Option<u64>,
) -> Result<u64, ContractError>
```

### Authorization Model
The `authorize_remittance` function has been removed. Payout confirmation is now handled directly via `confirm_payout`, which requires `require_auth` from the agent and the `Settler` role.

## Migration Steps

1. **Update Clients**: Update all off-chain clients to use the new `create_remittance` signature.
2. **Assign Roles**: Ensure all authorized agents are assigned the `Role::Settler` using the `assign_role` function.
3. **Verify Hashes**: Use `compute_settlement_hash` to reconcile existing pending transactions if necessary.
# Configuration Migration Guide

This guide helps existing developers migrate to the new environment-based configuration system.

## What Changed?

The SwiftRemit codebase has been refactored to eliminate hardcoded configuration values and use environment variables instead. This improves:

- **Maintainability**: Configuration is centralized in one place
- **Flexibility**: Easy to configure for different environments
- **Security**: Secrets are no longer in code
- **Deployment**: Simplified deployment to multiple environments

## Migration Steps

### Step 1: Create Your Environment File

Copy the example environment file to create your local configuration:

```bash
cp .env.example .env
```

### Step 2: Fill In Required Values

Edit the `.env` file and provide values for required variables:

```bash
# Required for client operations
SWIFTREMIT_CONTRACT_ID=your_contract_id_here
USDC_TOKEN_ID=your_usdc_token_id_here
```

If you were previously using hardcoded values in `examples/client-example.js`, copy those values to your `.env` file.

### Step 3: Customize Optional Settings (If Needed)

Most optional settings have sensible defaults, but you can customize them:

```bash
# Network configuration
NETWORK=testnet
RPC_URL=https://soroban-testnet.stellar.org:443

# Fee configuration
DEFAULT_FEE_BPS=250

# Transaction configuration
TRANSACTION_FEE=100000
TRANSACTION_TIMEOUT=30
POLL_INTERVAL_MS=1000

# Token configuration
USDC_DECIMALS=7

# Deployment configuration
DEPLOYER_IDENTITY=deployer
INITIAL_FEE_BPS=250

# Feature flags
ENABLE_DEBUG_LOG=true
```

### Step 4: Verify Configuration

Test that your configuration loads correctly:

```bash
cd examples
node config.js
```

If there are no errors, your configuration is valid.

### Step 5: Update Your Workflow

#### Running Client Code

No changes needed! The client code now automatically loads configuration from `.env`:

```bash
cd examples
node client-example.js
```

#### Deploying Contracts

Deployment scripts now read from environment variables. You can either:

**Option A: Use environment variables**
```bash
export NETWORK=testnet
export INITIAL_FEE_BPS=250
./deploy.sh
```

**Option B: Use CLI overrides**
```bash
./deploy.sh testnet
```

**Option C: Set in .env file**
```bash
# In .env
NETWORK=testnet
INITIAL_FEE_BPS=250

# Then run
./deploy.sh
```

## What Was Changed?

### JavaScript Client Code

**Before:**
```javascript
const CONFIG = {
  network: 'testnet',
  rpcUrl: 'https://soroban-testnet.stellar.org:443',
  contractId: 'CAAAA...',
  // ... hardcoded values
};
```

**After:**
```javascript
const config = require('./config');

// Use config.network, config.rpcUrl, config.contractId, etc.
```

### Deployment Scripts

**Before (deploy.sh):**
```bash
NETWORK="testnet"
DEPLOYER="deployer"
# ... hardcoded values
```

**After (deploy.sh):**
```bash
NETWORK=${NETWORK:-testnet}
DEPLOYER=${DEPLOYER_IDENTITY:-deployer}
INITIAL_FEE_BPS=${INITIAL_FEE_BPS:-250}
# ... reads from environment with defaults
```

### Rust Contract Code

The Rust contract code remains largely unchanged. Constants like `MAX_FEE_BPS` and `FEE_DIVISOR` are still hardcoded in the contract for on-chain consistency, but they are now documented with comments explaining their purpose.

## Breaking Changes

### None for Normal Usage

If you were using the system normally, there are no breaking changes. The refactoring maintains backward compatibility:

- All existing functionality works the same way
- Default values match previous hardcoded values
- Tests continue to pass

### If You Modified Hardcoded Values

If you previously modified hardcoded values in the code, you now need to set them via environment variables instead:

1. Identify the values you changed
2. Add them to your `.env` file
3. Remove your code modifications

## Troubleshooting

### "Missing required environment variable" Error

**Problem**: You're missing a required configuration value

**Solution**: Add the variable to your `.env` file. Check `.env.example` for the complete list of variables.

### "Configuration validation failed" Error

**Problem**: A configuration value is invalid (wrong type, out of range, etc.)

**Solution**: Check the error message for details. Common issues:
- Fee values must be 0-10000
- URLs must be HTTPS
- Network must be 'testnet' or 'mainnet'
- Numeric values must be valid numbers

### Client Code Not Finding Configuration

**Problem**: Client code can't load configuration

**Solution**: 
1. Ensure `.env` file exists in project root
2. Ensure you're running from the correct directory
3. Check that `dotenv` package is installed: `npm install`

### Deployment Script Not Using Environment Variables

**Problem**: Deployment script uses defaults instead of your values

**Solution**:
1. Export variables before running script: `export NETWORK=testnet`
2. Or set them in `.env` file
3. Or use CLI overrides: `./deploy.sh testnet`

## Getting Help

If you encounter issues during migration:

1. Check the [Configuration Guide](CONFIGURATION.md) for detailed documentation
2. Review error messages carefully - they indicate which variable is problematic
3. Verify your `.env` file against `.env.example`
4. Ensure all required variables are set
5. Check that values are within valid ranges

## Benefits of the New System

After migration, you'll benefit from:

1. **Easier Environment Management**: Switch between testnet and mainnet by changing one variable
2. **Better Security**: Secrets are in `.env` (gitignored) instead of code
3. **Simplified Deployment**: Deploy to multiple environments without code changes
4. **Centralized Configuration**: All settings in one place
5. **Validation**: Configuration errors caught at startup, not runtime
6. **Documentation**: Clear documentation of all configuration options

## Next Steps

After completing migration:

1. Delete any local modifications to hardcoded values
2. Commit your updated code (but not `.env`!)
3. Share `.env.example` with your team
4. Update your deployment documentation
5. Consider setting up environment-specific `.env` files (`.env.testnet`, `.env.mainnet`)
