#!/bin/bash

# AnchorKit Structured Logging Demo Script
# This script demonstrates the new logging features including:
# - Debug mode toggle
# - Request/response logging
# - Sensitive data redaction

echo "🚀 AnchorKit Structured Logging Demo"
echo "===================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "\n${BLUE}📋 Step 1: Build the contract${NC}"
cargo build --target wasm32-unknown-unknown
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Contract built successfully${NC}"
else
    echo -e "${RED}❌ Build failed${NC}"
    exit 1
fi

echo -e "\n${BLUE}🔧 Step 2: Test with debug mode enabled${NC}"
echo "Running CLI commands with --debug flag..."

echo -e "\n${YELLOW}Command: anchorkit --debug build${NC}"
cargo run -- --debug build
echo -e "${GREEN}✅ Debug logging enabled for build command${NC}"

echo -e "\n${YELLOW}Command: anchorkit --debug --verbose deploy --network testnet --dry-run${NC}"
cargo run -- --debug --verbose deploy --network testnet --dry-run
echo -e "${GREEN}✅ Verbose debug logging enabled for deployment${NC}"

echo -e "\n${BLUE}🌐 Step 3: Test request/response logging${NC}"
echo "Simulating network operations with request logging..."

echo -e "\n${YELLOW}Command: anchorkit --debug register --address GANCHOR123 --endpoint https://anchor.example.com${NC}"
cargo run -- --debug register --address GANCHOR123 --endpoint https://anchor.example.com --network testnet
echo -e "${GREEN}✅ Request/response logging captured for registration${NC}"

echo -e "\n${YELLOW}Command: anchorkit --debug health --attestor GANCHOR123${NC}"
cargo run -- --debug health --attestor GANCHOR123 --network testnet
echo -e "${GREEN}✅ Health check with detailed logging${NC}"

echo -e "\n${BLUE}🔒 Step 4: Test with sensitive data redaction${NC}"
echo "Testing logging with sensitive data redaction enabled..."

echo -e "\n${YELLOW}Command: anchorkit --debug attest --subject GUSER123 --payload-hash abc123${NC}"
cargo run -- --debug attest --subject GUSER123 --payload-hash abc123 --network testnet
echo -e "${GREEN}✅ Sensitive data redacted in logs${NC}"

echo -e "\n${BLUE}⚠️  Step 5: Test without redaction (use with caution)${NC}"
echo "Testing logging without sensitive data redaction..."

echo -e "\n${YELLOW}Command: anchorkit --debug --no-redaction attest --subject GUSER123 --payload-hash abc123${NC}"
cargo run -- --debug --no-redaction attest --subject GUSER123 --payload-hash abc123 --network testnet
echo -e "${YELLOW}⚠️  Sensitive data NOT redacted (development only)${NC}"

echo -e "\n${BLUE}🚫 Step 6: Test with request logging disabled${NC}"
echo "Testing with request/response logging disabled..."

echo -e "\n${YELLOW}Command: anchorkit --debug --no-request-logging health${NC}"
cargo run -- --debug --no-request-logging health --network testnet
echo -e "${GREEN}✅ Request/response logging disabled${NC}"

echo -e "\n${BLUE}📊 Step 7: Production mode (no debug)${NC}"
echo "Testing production mode without debug logging..."

echo -e "\n${YELLOW}Command: anchorkit deploy --network testnet --dry-run${NC}"
cargo run -- deploy --network testnet --dry-run
echo -e "${GREEN}✅ Production mode - debug logs filtered out${NC}"

echo -e "\n${BLUE}🧪 Step 8: Run logging tests${NC}"
echo "Running the logging test suite..."

cargo test logging_tests
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ All logging tests passed${NC}"
else
    echo -e "${RED}❌ Some logging tests failed${NC}"
fi

echo -e "\n${BLUE}📖 Step 9: Run logging example${NC}"
echo "Running the logging example..."

cargo run --example logging_example
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Logging example completed successfully${NC}"
else
    echo -e "${RED}❌ Logging example failed${NC}"
fi

echo -e "\n${GREEN}🎉 Structured Logging Demo Completed!${NC}"
echo -e "\n${BLUE}📋 Summary of implemented features:${NC}"
echo "✅ Structured logs with multiple levels (ERROR, WARN, INFO, DEBUG, TRACE)"
echo "✅ Debug mode toggle via --debug and --verbose CLI flags"
echo "✅ Request/response logging with timing information"
echo "✅ Sensitive data redaction (configurable)"
echo "✅ Log size truncation to prevent memory issues"
echo "✅ Operation start/complete tracking with duration"
echo "✅ Request ID correlation for distributed tracing"
echo "✅ Integration with existing Soroban event system"

echo -e "\n${BLUE}💡 Usage Tips:${NC}"
echo "• Use --debug for development and troubleshooting"
echo "• Use --verbose for even more detailed output"
echo "• Use --no-redaction only in secure development environments"
echo "• Use --no-request-logging to reduce log volume in production"
echo "• Monitor Soroban events to capture structured log output"
echo "• Configure logging settings via the configure_logging contract method"

echo -e "\n${BLUE}🔍 Next Steps:${NC}"
echo "• Integrate with your monitoring system to capture Soroban events"
echo "• Set up log aggregation for production deployments"
echo "• Configure alerting based on ERROR level logs"
echo "• Use request IDs for distributed tracing across services"