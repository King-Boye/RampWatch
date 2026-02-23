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

```bash
npm install
```

## Testing

```bash
npm test
```

## Documentation

See [ASSET_VERIFICATION.md](../ASSET_VERIFICATION.md) for complete documentation.
