# SwiftRemit Frontend

React + TypeScript frontend for the SwiftRemit USDC remittance platform, built with Vite and the Stellar SDK.

See also: [Root README](../README.md) | [Deployment Guide](../DEPLOYMENT.md)

---

## Prerequisites

- Node.js >= 18
- [Freighter wallet extension](https://www.freighter.app/) installed in your browser
- A deployed SwiftRemit contract on Stellar testnet (see [DEPLOYMENT.md](../DEPLOYMENT.md))

---

## Environment Setup

Copy the example env file and fill in your values:

```bash
cp .env.example .env
```

Then edit `.env`:

```env
# Stellar Network
VITE_NETWORK=testnet
VITE_HORIZON_URL=https://horizon-testnet.stellar.org
VITE_SOROBAN_RPC_URL=https://soroban-testnet.stellar.org

# Your deployed contract address (C...)
VITE_CONTRACT_ID=

# Testnet USDC token contract address
VITE_USDC_TOKEN_ID=
```

`VITE_CONTRACT_ID` is the Soroban contract address you get after deploying the smart contract. `VITE_USDC_TOKEN_ID` is the testnet USDC token contract — the Circle testnet USDC address on Stellar is `CBIELTK6YBZJU5UP2WWQEUCYKLPU6AUNZ2BQ4WWFEIE3USCIHMXQDAMA`.

All `VITE_` prefixed variables are bundled into the client at build time by Vite.

---

## Running Locally

```bash
# Install dependencies
npm install

# Start dev server (http://localhost:5173)
npm run dev
```

---

## Building for Production

```bash
npm run build
# Output goes to dist/
```

To preview the production build locally:

```bash
npm run preview
```

---

## Pointing the App at a Deployed Contract

1. Deploy the SwiftRemit contract to testnet following [DEPLOYMENT.md](../DEPLOYMENT.md).
2. Copy the resulting contract ID (starts with `C`).
3. Set `VITE_CONTRACT_ID=<your-contract-id>` in your `.env`.
4. Restart the dev server (`npm run dev`) — Vite picks up env changes on restart.

The app reads `import.meta.env.VITE_CONTRACT_ID` at runtime to initialize the Soroban RPC client and sign transactions via Freighter.

---

## Connecting Your Wallet

1. Install the [Freighter extension](https://www.freighter.app/).
2. Switch Freighter to **Testnet** (Settings → Network → Testnet).
3. Fund your testnet account via [Stellar Friendbot](https://friendbot.stellar.org/?addr=<your-address>).
4. Click "Connect Wallet" in the app — Freighter will prompt for approval.
