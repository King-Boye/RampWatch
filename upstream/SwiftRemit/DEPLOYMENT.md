# SwiftRemit Deployment Guide

Complete guide for building, testing, and deploying the SwiftRemit smart contract to Stellar testnet.

## Prerequisites

1. Install Rust and Cargo:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Add the WebAssembly target:
```bash
rustup target add wasm32-unknown-unknown
```

3. Install Soroban CLI:
```bash
cargo install --locked soroban-cli --features opt
```

4. Configure Soroban CLI for testnet:
```bash
soroban network add --global testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015"
```

## Automated Deployment (Recommended)

For convenience, use the provided deployment scripts which handle building, optimizing, deploying, and initializing the contract in one step.

**Linux/macOS:**
```bash
./deploy.sh [network]
# Example: ./deploy.sh testnet
```

**Windows (PowerShell):**
```powershell
.\deploy.ps1 -Network [network]
# Example: .\deploy.ps1 -Network testnet
```

These scripts will automatically:
1. Build and optimize the contract
2. Create/fund a deployer identity (if needed)
3. Deploy the contract and a mock USDC token
4. Initialize the contract
5. Save contract IDs to `.env.local`

## Build the Contract

1. Navigate to the project directory:
```bash
cd SwiftRemit
```

2. Build the contract:
```bash
cargo build --target wasm32-unknown-unknown --release
```

3. Optimize the WASM binary:
```bash
soroban contract optimize --wasm target/wasm32-unknown-unknown/release/swiftremit.wasm
```

This creates an optimized `swiftremit.optimized.wasm` file.

## Run Tests

Execute the comprehensive test suite:

```bash
cargo test
```

For verbose output:
```bash
cargo test -- --nocapture
```

Run specific tests:
```bash
cargo test test_create_remittance
cargo test test_confirm_payout
cargo test test_fee_calculation
```

## Deploy to Stellar Testnet

### Step 1: Create and Fund Identity

Create a new identity for deployment:
```bash
soroban keys generate --global deployer --network testnet
```

Get the public key:
```bash
soroban keys address deployer
```

Fund the account using the Stellar Friendbot:
```bash
soroban keys fund deployer --network testnet
```

Or visit: https://laboratory.stellar.org/#account-creator?network=test

### Step 2: Deploy the Contract

Deploy the optimized contract:
```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/swiftremit.optimized.wasm \
  --source deployer \
  --network testnet
```

Save the returned contract ID (e.g., `CXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX`).

### Step 3: Deploy or Get USDC Token Address

For testnet, you can use the native USDC token or deploy a test token.

To deploy a test token contract:
```bash
soroban contract asset deploy \
  --asset USDC:GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX \
  --source deployer \
  --network testnet
```

Or use an existing testnet USDC token address.

### Step 4: Initialize the Contract

Initialize with admin address, USDC token, and fee (250 = 2.5%):

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source deployer \
  --network testnet \
  -- \
  initialize \
  --admin <ADMIN_ADDRESS> \
  --usdc_token <USDC_TOKEN_ADDRESS> \
  --fee_bps 250
```

Example:
```bash
soroban contract invoke \
  --id CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAD2KM \
  --source deployer \
  --network testnet \
  -- \
  initialize \
  --admin GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX \
  --usdc_token CBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBD2KM \
  --fee_bps 250
```

## Contract Interaction Examples

### Register an Agent

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source deployer \
  --network testnet \
  -- \
  register_agent \
  --agent <AGENT_ADDRESS>
```

### Create a Remittance

First, ensure the sender has USDC tokens and has approved the contract.

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source sender \
  --network testnet \
  -- \
  create_remittance \
  --sender <SENDER_ADDRESS> \
  --agent <AGENT_ADDRESS> \
  --amount 1000000000
```

Amount is in stroops (7 decimals for USDC).

### Confirm Payout

Agent confirms they paid out fiat:

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source agent \
  --network testnet \
  -- \
  confirm_payout \
  --remittance_id 1
```

### Cancel Remittance

Sender cancels before payout:

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source sender \
  --network testnet \
  -- \
  cancel_remittance \
  --remittance_id 1
```

### Withdraw Platform Fees

Admin withdraws accumulated fees:

```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source deployer \
  --network testnet \
  -- \
  withdraw_fees \
  --to <RECIPIENT_ADDRESS>
```

### Query Contract State

Get remittance details:
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source deployer \
  --network testnet \
  -- \
  get_remittance \
  --remittance_id 1
```

Check if agent is registered:
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source deployer \
  --network testnet \
  -- \
  is_agent_registered \
  --agent <AGENT_ADDRESS>
```

Get accumulated fees:
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source deployer \
  --network testnet \
  -- \
  get_accumulated_fees
```

Get platform fee in basis points:
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source deployer \
  --network testnet \
  -- \
  get_platform_fee_bps
```

## Mainnet Deployment

For mainnet deployment, follow the same steps but use the mainnet network configuration:

```bash
soroban network add --global mainnet \
  --rpc-url https://soroban-mainnet.stellar.org:443 \
  --network-passphrase "Public Global Stellar Network ; September 2015"
```

Then replace `--network testnet` with `--network mainnet` in all commands.

## Security Considerations

1. Store private keys securely - never commit them to version control
2. Use hardware wallets for mainnet admin keys
3. Test thoroughly on testnet before mainnet deployment
4. Verify all contract addresses before transactions
5. Monitor contract events for suspicious activity
6. Implement multi-signature for admin operations in production
7. Regular security audits recommended before mainnet launch

## Troubleshooting

### Build Errors

If you encounter build errors, ensure:
- Rust toolchain is up to date: `rustup update`
- Correct target is installed: `rustup target add wasm32-unknown-unknown`
- Dependencies are current: `cargo update`

### Transaction Failures

Common issues:
- Insufficient XLM balance for fees
- Incorrect authorization (wrong source account)
- Contract not initialized
- Agent not registered
- Invalid remittance status

Check transaction details in Stellar Laboratory: https://laboratory.stellar.org/

### Network Issues

If RPC calls fail:
- Verify network configuration: `soroban network ls`
- Check Stellar testnet status
- Try alternative RPC endpoints

## Additional Resources

- Soroban Documentation: https://soroban.stellar.org/docs
- Stellar Laboratory: https://laboratory.stellar.org/
- Soroban CLI Reference: https://soroban.stellar.org/docs/reference/soroban-cli
- Stellar Discord: https://discord.gg/stellar
