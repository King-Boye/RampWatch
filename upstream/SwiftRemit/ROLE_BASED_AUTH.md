# Role-Based Authorization Implementation (#99)

## Overview

Implemented role-based permissions using Soroban's native `require_auth()` with two roles: **Admin** and **Settler**.

## Implementation Details

### 1. Role Types (`src/types.rs`)

```rust
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Role {
    Admin,
    Settler,
}
```

### 2. Storage (`src/storage.rs`)

**DataKey Addition:**
- `RoleAssignment(Address, Role)` - Persistent storage for role assignments

**Functions:**
- `assign_role(env, address, role)` - Assigns a role to an address
- `remove_role(env, address, role)` - Removes a role from an address
- `has_role(env, address, role)` - Checks if address has specific role
- `require_role_admin(env, address)` - Enforces Admin role (returns `Unauthorized` error)
- `require_role_settler(env, address)` - Enforces Settler role (returns `Unauthorized` error)

### 3. Contract Functions (`src/lib.rs`)

**Public API:**
```rust
pub fn assign_role(env, caller, address, role) -> Result<(), ContractError>
pub fn remove_role(env, caller, address, role) -> Result<(), ContractError>
pub fn has_role(env, address, role) -> bool
```

**Authorization:**
- `assign_role` and `remove_role` require `caller.require_auth()` + Admin role
- Unauthorized calls return `ContractError::Unauthorized`

### 4. Integration

**Initialization:**
- Initial admin automatically receives Admin role during `initialize()`

**Settlement:**
- `confirm_payout()` now requires Settler role via `require_role_settler()`
- Agent must have both agent registration AND Settler role to finalize transfers

## Acceptance Criteria

✅ **Only Admin can assign/remove roles**
- `assign_role()` and `remove_role()` enforce Admin role via `require_role_admin()`

✅ **Only Settler can finalize transfers**
- `confirm_payout()` enforces Settler role via `require_role_settler()`

✅ **Unauthorized calls must panic**
- Returns `ContractError::Unauthorized` which causes transaction to fail

✅ **Role assignments persist across invocations**
- Stored in persistent storage via `DataKey::RoleAssignment(Address, Role)`

## Usage Example

```rust
// Initialize contract (admin gets Admin role automatically)
contract.initialize(admin, usdc_token, 250, 3600);

// Admin assigns Settler role to agent
contract.assign_role(admin, agent, Role::Settler);

// Agent can now finalize transfers
contract.confirm_payout(remittance_id); // Requires Settler role

// Admin can remove role
contract.remove_role(admin, agent, Role::Settler);

// Check role
let has_role = contract.has_role(agent, Role::Settler); // false
```

## Security

- Uses Soroban's native `require_auth()` for caller verification
- Role checks happen before business logic execution
- Persistent storage ensures roles survive contract upgrades
- Explicit error handling with `ContractError::Unauthorized`

## Testing

Tests located in `src/test_roles_simple.rs`:
- `test_role_storage_and_retrieval` - Basic role assignment/removal
- `test_role_authorization_checks` - Admin authorization enforcement
- `test_settler_authorization` - Settler authorization enforcement
- `test_role_persistence` - Role persistence across calls
