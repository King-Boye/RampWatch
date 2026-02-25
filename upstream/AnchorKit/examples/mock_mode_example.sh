#!/bin/bash
# Mock mode example - test without live anchors

cargo build --features mock-only

echo "=== Mock Deposit Test ==="
# Simulates deposit with 2s delay
echo "Deposit initiated..."

echo "=== Mock Withdrawal Test ==="
# Simulates withdrawal with 1s delay
echo "Withdrawal initiated..."

echo "=== Mock Webhook Test ==="
# Trigger fake webhook
echo "Webhook triggered: transaction.completed"

echo "Done!"
