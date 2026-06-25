#!/bin/bash
# Test script for enhanced validation

echo "=== Testing Enhanced Configuration Validation ==="
echo ""

echo "1. Testing valid configuration..."
./target/debug/anchorkit validate configs/testnet-example.json
echo ""

echo "2. Testing invalid configuration (missing fields)..."
./target/debug/anchorkit validate configs/test-invalid.json
echo ""

echo "3. Testing all configs in directory..."
./target/debug/anchorkit validate configs/
echo ""

echo "=== Test Complete ==="
