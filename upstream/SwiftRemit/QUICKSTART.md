# SwiftRemit Quick Start Guide

Get up and running with SwiftRemit in 5 minutes.

## Prerequisites Check

Before starting, ensure you have:
- Rust and Cargo installed
- wasm32-unknown-unknown target
- Soroban CLI

## Automated Setup (Recommended)

### Linux/macOS
```bash
chmod +x setup.sh
./setup.sh
```

### Windows (PowerShell)
```powershell
.\setup.ps1
```

The setup script will:
1. Install Rust (if needed)
2. Add wasm32 target
3. Install Soroban CLI
4. Configure testnet
5. Build and optimize the contract

## Manual Setup

### 1. Install Prerequisites

Install Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Add wasm32 target:
```bash
rustup target add wasm32-unknown-unknown
```

Install Soroban CLI:
```bash
cargo install --locked soroban-cli --features opt
```

### 2. Configure Network

```bash
soroban network add --global testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015"
```

### 3. Build Contract

```bash
cargo build --target wasm32-unknown-unknown --release
soroban contract optimize --wasm target/wasm32-unknown-unknown/release/swiftremit.wasm
```

## Run Tests

```bash
cargo test
```

Expected output:
```
running 15 tests
test test_initialize ... ok
test test_register_agent ... ok
test test_create_remittance ... ok
test test_confirm_payout ... ok
test test_cancel_remittance ... ok
test test_withdraw_fees ... ok
test test_fee_calculation ... ok
...

test result: ok. 15 passed; 0 failed
```

## Deploy to Testnet

### 1. Create Identity

```bash
soroban keys generate --global deployer --network testnet
```

### 2. Fund Account

```bash
soroban keys fund deployer --network testnet
```

### 3. Deploy Contract

```bash
CONTRACT_ID=$(soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/swiftremit.optimized.wasm \
  --source deployer \
  --network testnet)

echo "Contract deployed at: $CONTRACT_ID"
```

### 4. Deploy Test USDC Token

```bash
USDC_ID=$(soroban contract asset deploy \
  --asset USDC:$(soroban keys address deployer) \
  --source deployer \
  --network testnet)

echo "USDC token deployed at: $USDC_ID"
```

### 5. Initialize Contract

```bash
ADMIN=$(soroban keys address deployer)

soroban contract invoke \
  --id $CONTRACT_ID \
  --source deployer \
  --network testnet \
  -- \
  initialize \
  --admin $ADMIN \
  --usdc_token $USDC_ID \
  --fee_bps 250
```

## Test the Contract

### Register an Agent

```bash
# Create agent identity
soroban keys generate --global agent --network testnet
soroban keys fund agent --network testnet

AGENT=$(soroban keys address agent)

# Register agent
soroban contract invoke \
  --id $CONTRACT_ID \
  --source deployer \
  --network testnet \
  -- \
  register_agent \
  --agent $AGENT
```

### Create a Remittance

```bash
# Create sender identity
soroban keys generate --global sender --network testnet
soroban keys fund sender --network testnet

SENDER=$(soroban keys address sender)

# Mint USDC to sender
soroban contract invoke \
  --id $USDC_ID \
  --source deployer \
  --network testnet \
  -- \
  mint \
  --to $SENDER \
  --amount 10000000000

# Create remittance (1000 USDC = 10000000000 stroops)
REMITTANCE_ID=$(soroban contract invoke \
  --id $CONTRACT_ID \
  --source sender \
  --network testnet \
  -- \
  create_remittance \
  --sender $SENDER \
  --agent $AGENT \
  --amount 10000000000)

echo "Remittance created with ID: $REMITTANCE_ID"
```

### Confirm Payout

```bash
soroban contract invoke \
  --id $CONTRACT_ID \
  --source agent \
  --network testnet \
  -- \
  confirm_payout \
  --remittance_id 1
```

### Check Results

```bash
# Get remittance details
soroban contract invoke \
  --id $CONTRACT_ID \
  --source deployer \
  --network testnet \
  -- \
  get_remittance \
  --remittance_id 1

# Check accumulated fees
soroban contract invoke \
  --id $CONTRACT_ID \
  --source deployer \
  --network testnet \
  -- \
  get_accumulated_fees
```

## Common Issues

### "wasm32-unknown-unknown target not installed"
```bash
rustup target add wasm32-unknown-unknown
```

### "soroban: command not found"
```bash
cargo install --locked soroban-cli --features opt
```

### "insufficient balance"
```bash
soroban keys fund <identity> --network testnet
```

### "contract not initialized"
Make sure you ran the `initialize` command with correct parameters.

## Next Steps

- Read [DEPLOYMENT.md](DEPLOYMENT.md) for detailed deployment guide
- Review [README.md](README.md) for complete documentation
- Explore the contract code in `src/`
- Customize fee percentages and add more agents
- Integrate with your frontend application

## Support

- Stellar Discord: https://discord.gg/stellar
- Soroban Docs: https://soroban.stellar.org/docs
- GitHub Issues: Report bugs and request features

## Production Checklist

Before deploying to mainnet:

- [ ] Complete security audit
- [ ] Test all edge cases thoroughly
- [ ] Set up monitoring and alerting
- [ ] Implement multi-sig for admin operations
- [ ] Document operational procedures
- [ ] Prepare incident response plan
- [ ] Configure proper fee percentages
- [ ] Verify all agent registrations
- [ ] Test with real USDC on testnet
- [ ] Review and test cancellation flows
