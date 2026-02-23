#!/usr/bin/env bash
# AnchorKit CLI Example - Deposit/Withdraw Workflow
# Demonstrates basic usage with mock transport

set -e

echo "🚀 AnchorKit CLI Example - Deposit/Withdraw Workflow"
echo "=================================================="
echo ""

# Mock addresses
ADMIN="GADMIN123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"
ANCHOR="GANCHOR123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"
USER="GUSER123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"

echo "📋 Configuration:"
echo "  Admin:  $ADMIN"
echo "  Anchor: $ANCHOR"
echo "  User:   $USER"
echo ""

# Step 1: Initialize Contract
echo "1️⃣  Initializing contract..."
echo "   → Setting admin: $ADMIN"
echo "   ✅ Contract initialized"
echo ""

# Step 2: Register Anchor
echo "2️⃣  Registering anchor..."
echo "   → Anchor: $ANCHOR"
echo "   ✅ Anchor registered"
echo ""

# Step 3: Configure Services
echo "3️⃣  Configuring anchor services..."
echo "   → Services: Deposits, Withdrawals"
echo "   ✅ Services configured"
echo ""

# Step 4: Configure Supported Assets
echo "4️⃣  Configuring supported assets..."
echo "   → Assets: USDC, BTC, ETH"
echo "   ✅ Assets configured"
echo ""

# Step 5: Deposit Flow
echo "5️⃣  Initiating deposit flow..."
echo "   → User: $USER"
echo "   → Asset: USDC"
echo "   → Amount: 1000"
echo "   → Validating asset compatibility..."
echo "   ✅ Asset validated"
echo "   → Generating request ID..."
REQUEST_ID="req_$(date +%s)_deposit"
echo "   → Request ID: $REQUEST_ID"
echo "   → Submitting attestation..."
echo "   ✅ Deposit attestation recorded (ID: 1)"
echo ""

# Step 6: Quote Request
echo "6️⃣  Requesting quote..."
echo "   → Pair: USDC/USD"
echo "   → Amount: 1000 USDC"
echo "   → Rate: 1.0000"
echo "   → Fee: 1%"
echo "   ✅ Quote received (ID: 1)"
echo ""

# Step 7: Withdraw Flow
echo "7️⃣  Initiating withdraw flow..."
echo "   → User: $USER"
echo "   → Asset: USDC"
echo "   → Amount: 500"
echo "   → Validating asset compatibility..."
echo "   ✅ Asset validated"
echo "   → Generating request ID..."
REQUEST_ID="req_$(date +%s)_withdraw"
echo "   → Request ID: $REQUEST_ID"
echo "   → Submitting attestation..."
echo "   ✅ Withdraw attestation recorded (ID: 2)"
echo ""

# Step 8: Check Health
echo "8️⃣  Checking anchor health..."
echo "   → Anchor: $ANCHOR"
echo "   → Latency: 45ms"
echo "   → Availability: 99.9%"
echo "   → Failure count: 0"
echo "   ✅ Anchor healthy"
echo ""

# Step 9: Audit Trail
echo "9️⃣  Retrieving audit trail..."
echo "   → Session operations: 2"
echo "   → Attestation 1: Deposit (Success)"
echo "   → Attestation 2: Withdraw (Success)"
echo "   ✅ Audit trail complete"
echo ""

echo "✅ Workflow completed successfully!"
echo ""
echo "📊 Summary:"
echo "  - Deposits: 1 (1000 USDC)"
echo "  - Withdrawals: 1 (500 USDC)"
echo "  - Net balance: 500 USDC"
echo "  - Total attestations: 2"
echo ""
echo "💡 This example uses mock transport for demonstration."
echo "   In production, connect to real Stellar network."
