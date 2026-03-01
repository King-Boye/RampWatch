# SwiftRemit Deployment Guide

## Frontend Deployment

### Quick Deploy to Vercel (Recommended)

#### Option 1: Deploy via Vercel Dashboard (Easiest)

1. **Go to Vercel**: https://vercel.com
2. **Sign in** with your GitHub account
3. **Import Project**:
   - Click "Add New..." → "Project"
   - Select your GitHub repository: `Haroldwonder/SwiftRemit`
   - Select branch: `refactor/production-readiness-soroban`
4. **Configure Project**:
   - Framework Preset: Vite
   - Root Directory: `frontend`
   - Build Command: `npm install --legacy-peer-deps && npm run build`
   - Output Directory: `dist`
   - Install Command: `npm install --legacy-peer-deps`
5. **Environment Variables** (Add these in Vercel dashboard):
   ```
   VITE_NETWORK=testnet
   VITE_HORIZON_URL=https://horizon-testnet.stellar.org
   VITE_SOROBAN_RPC_URL=https://soroban-testnet.stellar.org
   VITE_CONTRACT_ID=your_contract_id_here
   VITE_USDC_TOKEN_ID=your_usdc_token_id_here
   ```
6. **Deploy**: Click "Deploy"

Your site will be live at: `https://swiftremit-[random].vercel.app`

#### Option 2: Deploy via Vercel CLI

```bash
# Install Vercel CLI globally
npm install -g vercel

# Navigate to frontend directory
cd SwiftRemit/frontend

# Login to Vercel
vercel login

# Deploy
vercel

# Follow the prompts:
# - Set up and deploy? Yes
# - Which scope? Your account
# - Link to existing project? No
# - Project name? swiftremit-frontend
# - Directory? ./
# - Override settings? No

# Your deployment URL will be shown!
```

#### Option 3: Deploy via GitHub Integration

1. Push your code to GitHub (already done ✅)
2. Go to https://vercel.com/new
3. Import your repository
4. Vercel will auto-detect Vite and deploy

---

### Alternative: Deploy to Netlify

#### Via Netlify Dashboard

1. **Go to Netlify**: https://app.netlify.com
2. **Sign in** with GitHub
3. **Add new site** → "Import an existing project"
4. **Connect to Git provider**: GitHub
5. **Select repository**: `Haroldwonder/SwiftRemit`
6. **Configure**:
   - Branch: `refactor/production-readiness-soroban`
   - Base directory: `frontend`
   - Build command: `npm install --legacy-peer-deps && npm run build`
   - Publish directory: `frontend/dist`
7. **Environment variables**: Add the same as Vercel
8. **Deploy**

Your site will be live at: `https://swiftremit-[random].netlify.app`

#### Via Netlify CLI

```bash
# Install Netlify CLI
npm install -g netlify-cli

# Navigate to frontend
cd SwiftRemit/frontend

# Login
netlify login

# Deploy
netlify deploy --prod

# Follow prompts and your site will be live!
```

---

### Alternative: GitHub Pages

1. **Enable GitHub Pages**:
   - Go to: https://github.com/Haroldwonder/SwiftRemit/settings/pages
   - Source: Deploy from a branch
   - Branch: `refactor/production-readiness-soroban`
   - Folder: `/frontend` (if available) or `/` (root)
   - Save

2. **Add GitHub Actions workflow** (if needed):
   Create `.github/workflows/deploy.yml`:
   ```yaml
   name: Deploy to GitHub Pages
   
   on:
     push:
       branches: [refactor/production-readiness-soroban]
   
   jobs:
     build-and-deploy:
       runs-on: ubuntu-latest
       steps:
         - uses: actions/checkout@v3
         - uses: actions/setup-node@v3
           with:
             node-version: '18'
         - name: Install and Build
           run: |
             cd frontend
             npm install --legacy-peer-deps
             npm run build
         - name: Deploy
           uses: peaceiris/actions-gh-pages@v3
           with:
             github_token: ${{ secrets.GITHUB_TOKEN }}
             publish_dir: ./frontend/dist
   ```

Your site will be at: `https://haroldwonder.github.io/SwiftRemit/`

---

## Smart Contract Deployment

### Prerequisites

1. **Install Stellar CLI**:
   ```bash
   cargo install --locked stellar-cli
   ```

2. **Create Testnet Identity**:
   ```bash
   stellar keys generate --global admin --network testnet
   stellar keys address admin
   ```

3. **Fund Account**:
   - Go to: https://laboratory.stellar.org/#account-creator?network=test
   - Paste your address and click "Get test network lumens"

### Deploy Contract

```bash
# Navigate to contract directory
cd SwiftRemit

# Build the contract
stellar contract build

# Deploy to testnet
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/swiftremit.wasm \
  --source admin \
  --network testnet

# Save the contract ID that's returned!
```

### Initialize Contract

```bash
# Set variables
CONTRACT_ID="your_contract_id_here"
ADMIN_ADDRESS="your_admin_address_here"
USDC_TOKEN="USDC_testnet_token_id"

# Initialize
stellar contract invoke \
  --id $CONTRACT_ID \
  --source admin \
  --network testnet \
  -- \
  initialize \
  --admin $ADMIN_ADDRESS \
  --usdc_token $USDC_TOKEN \
  --fee_bps 250 \
  --rate_limit_cooldown 5 \
  --protocol_fee_bps 50 \
  --treasury $ADMIN_ADDRESS
```

### Update Frontend Environment Variables

After deploying the contract, update your frontend `.env` or Vercel environment variables:

```env
VITE_CONTRACT_ID=your_actual_contract_id
VITE_USDC_TOKEN_ID=your_actual_usdc_token_id
```

Then redeploy the frontend.

---

## Post-Deployment Checklist

### Frontend
- [ ] Site is accessible via public URL
- [ ] Wallet connection works (Freighter)
- [ ] Can view remittances
- [ ] Forms render correctly
- [ ] No console errors

### Smart Contract
- [ ] Contract deployed to testnet
- [ ] Contract initialized successfully
- [ ] Admin can register agents
- [ ] Can create remittances
- [ ] Can confirm payouts
- [ ] Events are emitted correctly

---

## Monitoring & Maintenance

### Frontend Monitoring
- **Vercel**: Built-in analytics at https://vercel.com/dashboard
- **Netlify**: Analytics at https://app.netlify.com

### Contract Monitoring
- **Stellar Expert**: https://stellar.expert/explorer/testnet
- **Horizon API**: https://horizon-testnet.stellar.org

### Logs
- **Frontend**: Check Vercel/Netlify deployment logs
- **Contract**: Use Stellar CLI to query contract state

---

## Troubleshooting

### Frontend Build Fails
```bash
# Clear cache and reinstall
cd frontend
rm -rf node_modules package-lock.json
npm install --legacy-peer-deps
npm run build
```

### Contract Deployment Fails
```bash
# Check account balance
stellar account --id admin

# Rebuild contract
cargo clean
stellar contract build
```

### Environment Variables Not Working
- Ensure variables start with `VITE_` prefix
- Redeploy after changing variables
- Check browser console for actual values

---

## Support

- **Documentation**: See README.md files in each directory
- **Issues**: https://github.com/Haroldwonder/SwiftRemit/issues
- **Stellar Discord**: https://discord.gg/stellar

---

## Quick Links

- **Repository**: https://github.com/Haroldwonder/SwiftRemit
- **Branch**: refactor/production-readiness-soroban
- **Stellar Testnet**: https://horizon-testnet.stellar.org
- **Freighter Wallet**: https://www.freighter.app/
