# Protocol Fee Implementation (#101)

## Overview

Configurable protocol fee automatically deducted during remittance execution and routed to treasury address.

## Implementation

### 1. Storage (`src/storage.rs`)

**DataKeys:**
- `ProtocolFeeBps` - Protocol fee in basis points (instance storage)
- `Treasury` - Treasury address for protocol fees (instance storage)

**Constants:**
- `MAX_PROTOCOL_FEE_BPS = 200` (2% maximum)

**Functions:**
```rust
get_protocol_fee_bps(env) -> u32
set_protocol_fee_bps(env, fee_bps) -> Result<(), ContractError>
get_treasury(env) -> Result<Address, ContractError>
set_treasury(env, treasury)
```

### 2. Fee Calculation

**Basis Points Model:**
```
protocol_fee = amount * protocol_fee_bps / 10000
```

**Examples:**
- 100 bps = 1%: `10000 * 100 / 10000 = 100`
- 200 bps = 2%: `10000 * 200 / 10000 = 200`

### 3. Integration (`src/lib.rs`)

**Initialization:**
```rust
initialize(
    admin,
    usdc_token,
    fee_bps,
    rate_limit_cooldown,
    protocol_fee_bps,  // New parameter
    treasury           // New parameter
)
```

**Payout Flow:**
1. Calculate protocol fee from remittance amount
2. Deduct platform fee + protocol fee from amount
3. Transfer payout to agent
4. Transfer protocol fee to treasury (if > 0)
5. Accumulate platform fee

**Admin Functions:**
```rust
update_protocol_fee(caller, fee_bps) -> Result<(), ContractError>
update_treasury(caller, treasury) -> Result<(), ContractError>
get_protocol_fee_bps() -> u32
get_treasury() -> Result<Address, ContractError>
```

## Acceptance Criteria

✅ **Fee automatically routed to treasury address**
- Protocol fee calculated during `confirm_payout()`
- Automatically transferred to treasury via token client
- Separate from platform fee accumulation

✅ **Fee capped (max 200 bps)**
- `MAX_PROTOCOL_FEE_BPS = 200` constant
- `set_protocol_fee_bps()` validates against cap
- Returns `InvalidFeeBps` error if exceeded

✅ **Admin can update fee and treasury**
- `update_protocol_fee()` - Admin only
- `update_treasury()` - Admin only
- Both require `require_auth()` + admin check

✅ **Use token client for transfers**
- Uses `token::Client::new()` for USDC transfers
- Separate transfers for agent payout and treasury fee
- Atomic execution within transaction

## Usage Example

```rust
// Initialize with protocol fee
contract.initialize(
    admin,
    usdc_token,
    250,      // 2.5% platform fee
    3600,     // rate limit
    100,      // 1% protocol fee
    treasury  // treasury address
);

// Create remittance: 10000 USDC
let id = contract.create_remittance(sender, agent, 10000, None);

// Confirm payout:
// - Platform fee: 10000 * 250 / 10000 = 250
// - Protocol fee: 10000 * 100 / 10000 = 100
// - Agent receives: 10000 - 250 - 100 = 9650
// - Treasury receives: 100
contract.confirm_payout(id);

// Admin updates
contract.update_protocol_fee(admin, 150); // Change to 1.5%
contract.update_treasury(admin, new_treasury);

// Query
let fee = contract.get_protocol_fee_bps(); // 150
let treasury = contract.get_treasury(); // new_treasury
```

## Fee Distribution

**Example: 10000 USDC remittance**
- Platform fee (2.5%): 250 USDC → Accumulated fees
- Protocol fee (1%): 100 USDC → Treasury (immediate)
- Agent payout: 9650 USDC → Agent (immediate)

## Security

- Fee capped at 200 bps (2%) to prevent excessive fees
- Admin-only updates with authentication
- Overflow protection in calculations
- Treasury address validated during initialization
- Zero protocol fee allowed (optional feature)

## Testing

Tests in `src/test_protocol_fee.rs`:
- `test_protocol_fee_storage` - Storage operations
- `test_protocol_fee_cap` - Maximum fee enforcement
- `test_treasury_storage` - Treasury address management
- `test_protocol_fee_calculation` - Fee calculation accuracy
- `test_zero_protocol_fee` - Zero fee handling
- `test_default_protocol_fee` - Default value
