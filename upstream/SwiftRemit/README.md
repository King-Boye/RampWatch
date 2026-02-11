# SwiftRemit

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

