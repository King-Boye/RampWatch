# SwiftRemit

Production-ready Soroban smart contract for USDC remittance platform on Stellar blockchain.

## Overview

SwiftRemit is an escrow-based remittance system that enables secure cross-border money transfers using USDC stablecoin. The platform connects senders with registered agents who handle fiat payouts, with the smart contract managing escrow, fee collection, and settlement.

## Features

- **Escrow-Based Transfers**: Secure USDC deposits held in contract until payout confirmation
- **Agent Network**: Registered agents handle fiat distribution off-chain
- **Automated Fee Collection**: Platform fees calculated and accumulated automatically
- **Multi-Status Tracking**: Remittances tracked through Pending, Completed, and Cancelled states
- **Authorization Security**: Role-based access control for all operations
- **Event Emission**: Comprehensive event logging for off-chain monitoring
- **Cancellation Support**: Senders can cancel pending remittances with full refund
- **Admin Controls**: Platform fee management and fee withdrawal capabilities

## Architecture

### Core Components

- **lib.rs**: Main contract implementation with all public functions
- **types.rs**: Data structures (Remittance, RemittanceStatus)
- **storage.rs**: Persistent and instance storage management
- **errors.rs**: Custom error types for contract operations
- **events.rs**: Event emission functions for monitoring
- **test.rs**: Comprehensive test suite with 15+ test cases

### Storage Model

- **Instance Storage**: Admin, USDC token, fee configuration, counters, accumulated fees
- **Persistent Storage**: Individual remittances, agent registrations

### Fee Calculation

Fees are calculated in basis points (bps):
- 250 bps = 2.5%
- 500 bps = 5.0%
- Formula: `fee = amount * fee_bps / 10000`

## Contract Functions

### Administrative Functions

- `initialize(admin, usdc_token, fee_bps)` - One-time contract initialization
- `register_agent(agent)` - Add agent to approved list (admin only)
- `remove_agent(agent)` - Remove agent from approved list (admin only)
- `update_fee(fee_bps)` - Update platform fee percentage (admin only)
- `withdraw_fees(to)` - Withdraw accumulated fees (admin only)

### User Functions

- `create_remittance(sender, agent, amount)` - Create new remittance (sender auth required)
- `confirm_payout(remittance_id)` - Confirm fiat payout (agent auth required)
- `cancel_remittance(remittance_id)` - Cancel pending remittance (sender auth required)

### Query Functions

- `get_remittance(remittance_id)` - Retrieve remittance details
- `get_accumulated_fees()` - Check total platform fees collected
- `is_agent_registered(agent)` - Verify agent registration status
- `get_platform_fee_bps()` - Get current fee percentage

## Security Features

1. **Authorization Checks**: All state-changing operations require proper authorization
2. **Status Validation**: Prevents double confirmation and invalid state transitions
3. **Overflow Protection**: Safe math operations with overflow checks
4. **Agent Verification**: Only registered agents can receive payouts
5. **Ownership Validation**: Senders can only cancel their own remittances

## Testing

The contract includes comprehensive tests covering:

- ✅ Initialization and configuration
- ✅ Agent registration and removal
- ✅ Fee updates and validation
- ✅ Remittance creation with proper token transfers
- ✅ Payout confirmation and fee accumulation
- ✅ Cancellation logic and refunds
- ✅ Fee withdrawal by admin
- ✅ Authorization enforcement
- ✅ Error conditions (invalid amounts, unauthorized access, double confirmation)
- ✅ Event emission verification
- ✅ Multiple remittances handling
- ✅ Fee calculation accuracy

Run tests with:
```bash
cargo test
```

## Quick Start

### Automated Deployment (Recommended)

Run the deployment script to build, deploy, and initialize everything automatically:

**Linux/macOS:**
```bash
chmod +x deploy.sh
./deploy.sh
# To deploy to a specific network (default: testnet):
./deploy.sh mainnet
```

**Windows (PowerShell):**
```powershell
.\deploy.ps1
# To deploy to a specific network (default: testnet):
.\deploy.ps1 -Network mainnet
```

This will:
- Build and optimize the contract
- Create/fund a `deployer` identity
- Deploy the contract and a mock USDC token
- Initialize the contract
- Save contract IDs to `.env.local`

### Manual Setup

If you prefer to run steps manually:

### 1. Build the Contract

```bash
cd SwiftRemit
cargo build --target wasm32-unknown-unknown --release
soroban contract optimize --wasm target/wasm32-unknown-unknown/release/swiftremit.wasm
```

### 2. Deploy to Testnet

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/swiftremit.optimized.wasm \
  --source deployer \
  --network testnet
```

### 3. Initialize

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

See [DEPLOYMENT.md](DEPLOYMENT.md) for complete deployment instructions.

## Usage Flow

1. **Admin Setup**
   - Deploy contract
   - Initialize with admin address, USDC token, and fee percentage
   - Register trusted agents

2. **Create Remittance**
   - Sender approves USDC transfer to contract
   - Sender calls `create_remittance` with agent and amount
   - Contract transfers USDC from sender to escrow
   - Remittance ID returned for tracking

3. **Agent Payout**
   - Agent pays out fiat to recipient off-chain
   - Agent calls `confirm_payout` with remittance ID
   - Contract transfers USDC minus fee to agent
   - Fee added to accumulated platform fees

4. **Fee Management**
   - Admin monitors accumulated fees
   - Admin calls `withdraw_fees` to collect platform revenue

## Error Codes

| Code | Error | Description |
|------|-------|-------------|
| 1 | AlreadyInitialized | Contract already initialized |
| 2 | NotInitialized | Contract not initialized |
| 3 | InvalidAmount | Amount must be greater than 0 |
| 4 | InvalidFeeBps | Fee must be between 0-10000 bps |
| 5 | AgentNotRegistered | Agent not in approved list |
| 6 | RemittanceNotFound | Remittance ID does not exist |
| 7 | InvalidStatus | Operation not allowed in current status |
| 8 | Overflow | Arithmetic overflow detected |
| 9 | NoFeesToWithdraw | No accumulated fees available |

## Events

The contract emits events for monitoring:

- `created` - New remittance created
- `completed` - Payout confirmed and settled
- `cancelled` - Remittance cancelled by sender
- `agent_reg` - Agent registered
- `agent_rem` - Agent removed
- `fee_upd` - Platform fee updated
- `fees_with` - Fees withdrawn by admin

## Dependencies

- `soroban-sdk = "21.7.0"` - Latest Soroban SDK

## License

MIT

## Support

For issues and questions:
- GitHub Issues: [Create an issue](https://github.com/yourusername/swiftremit/issues)
- Stellar Discord: https://discord.gg/stellar
- Documentation: See [DEPLOYMENT.md](DEPLOYMENT.md)

## Contributing

Contributions welcome! Please ensure:
- All tests pass: `cargo test`
- Code follows Rust best practices
- New features include tests
- Documentation is updated

## Roadmap

- [ ] Multi-currency support
- [ ] Batch remittance processing
- [ ] Agent reputation system
- [ ] Dispute resolution mechanism
- [ ] Time-locked escrow options
- [ ] Integration with fiat on/off ramps

SwiftRemit is a Soroban smart contract built in Rust that enables secure, escrow-based USDC remittances on the Stellar network.

The contract allows users to send USDC into escrow, assigns registered payout agents, and releases funds once off-chain fiat payment is confirmed. A configurable platform fee is automatically deducted and retained by the protocol.

This project is designed for emerging markets where stablecoin remittance rails can significantly reduce cross-border payment costs.

---

## Overview

SwiftRemit implements a simple escrow flow:

1. A sender creates a remittance by depositing USDC.
2. A registered agent pays the recipient in local fiat off-chain.
3. The agent confirms payout on-chain.
4. The contract releases USDC to the agent minus a platform fee.
5. The platform accumulates fees for withdrawal by the admin.

The system is designed to be secure, transparent, and modular.

---

## Key Features

- Escrow-based remittance logic
- Agent registration system
- Configurable platform fee (basis points model)
- Secure authorization using Soroban Address auth
- Protection against double confirmation
- Cancellation mechanism for pending remittances
- Accumulated fee withdrawal by admin
- Full unit test coverage

---

## Contract Architecture

The contract stores:

- Remittance records
- Registered agents
- Admin address
- Platform fee configuration
- Accumulated platform fees
- USDC token address

Each remittance includes:

- Unique ID
- Sender address
- Agent address
- Amount
- Fee
- Status (Pending, Completed, Cancelled)

---

## Fee Model

Platform fees are calculated using basis points:

