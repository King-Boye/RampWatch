# SwiftRemit API Reference

Complete API documentation for the SwiftRemit smart contract.

## Contract Functions

### Administrative Functions

#### `initialize`

Initialize the contract with admin, USDC token, and platform fee.

**Authorization:** None (can only be called once)

**Parameters:**
- `admin: Address` - Admin address with full control
- `usdc_token: Address` - USDC token contract address
- `fee_bps: u32` - Platform fee in basis points (0-10000)

**Returns:** `Result<(), ContractError>`

**Errors:**
- `AlreadyInitialized` (1) - Contract already initialized
- `InvalidFeeBps` (4) - Fee exceeds 10000 bps (100%)

**Example:**
```bash
soroban contract invoke \
  --id $CONTRACT_ID \
  --source deployer \
  --network testnet \
  -- \
  initialize \
  --admin GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX \
  --usdc_token CXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX \
  --fee_bps 250
```

---

#### `register_agent`

Register an agent to handle remittances.

**Authorization:** Admin only

**Parameters:**
- `agent: Address` - Agent address to register

**Returns:** `Result<(), ContractError>`

**Errors:**
- `NotInitialized` (2) - Contract not initialized

**Events:** `agent_reg(agent)`

**Example:**
```bash
soroban contract invoke \
  --id $CONTRACT_ID \
  --source admin \
  --network testnet \
  -- \
  register_agent \
  --agent GXXXXXXXXXXXXXXXXXX