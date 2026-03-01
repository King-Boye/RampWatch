<<<<<<< HEAD
# SwiftRemit Frontend Components

React components for asset verification in SwiftRemit.

## Components

### VerificationBadge

Visual indicator for asset verification status with detailed information modal.

```tsx
import { VerificationBadge } from './components/VerificationBadge';

<VerificationBadge
  assetCode="USDC"
  issuer="GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN"
  apiUrl="http://localhost:3000"
  onWarning={(verification) => console.warn(verification)}
  showDetails={true}
/>
```

## Installation

=======
# SwiftRemit Frontend

Simple React frontend for the SwiftRemit USDC remittance platform.

## Features

- Connect Freighter wallet
- Create remittances
- Agent payout confirmation
- View remittance history
- Real-time status updates

## Setup

1. Install dependencies:
>>>>>>> 927b448 (chore: local changes before syncing with main)
```bash
npm install
```

<<<<<<< HEAD
## Testing

```bash
npm test
```

## Documentation

See [ASSET_VERIFICATION.md](../ASSET_VERIFICATION.md) for complete documentation.
=======
2. Create `.env` file:
```bash
cp .env.example .env
```

3. Add your contract ID to `.env`:
```
VITE_CONTRACT_ID=your_contract_id_here
```

4. Start development server:
```bash
npm run dev
```

5. Open http://localhost:5173

## Prerequisites

- [Freighter Wallet](https://www.freighter.app/) browser extension
- Stellar testnet account with XLM for fees
- Deployed SwiftRemit contract

## Usage

### For Senders

1. Connect your Freighter wallet
2. Enter the contract ID
3. Fill in agent address and amount
4. Click "Create Remittance"
5. Approve transaction in Freighter

### For Agents

1. Connect your Freighter wallet
2. Enter the remittance ID
3. Click "Confirm Payout" after completing off-chain transfer
4. Approve transaction in Freighter

## Development

The frontend uses:
- React + Vite
- Stellar SDK
- Freighter API

To add full contract integration:
1. Generate contract bindings using `soroban contract bindings typescript`
2. Import generated types
3. Replace placeholder contract calls with actual SDK calls

## Build for Production

```bash
npm run build
```

Deploy the `dist` folder to your hosting service.

## Notes

- This is a demo frontend with mock data
- Full contract integration requires contract bindings
- Ensure contract is deployed and initialized before use
- Use testnet for development
>>>>>>> 927b448 (chore: local changes before syncing with main)
