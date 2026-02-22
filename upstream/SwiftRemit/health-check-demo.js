#!/usr/bin/env node

/**
 * Health Check Demo for SwiftRemit Smart Contract
 * 
 * This demonstrates how a health check would work in practice.
 * Since the contract has compilation issues, this shows the expected behavior.
 */

const { createLogger } = require('./examples/logger');
const logger = createLogger('health-check-demo');

// Mock contract health check response
function mockContractHealth() {
  return {
    operational: true,
    timestamp: Math.floor(Date.now() / 1000),
    initialized: true
  };
}

// Simulate health check with latency
async function checkHealth() {
  const start = Date.now();
  
  try {
    // Simulate network call
    await new Promise(resolve => setTimeout(resolve, Math.random() * 50));
    
    const health = mockContractHealth();
    const latency = Date.now() - start;
    
    return {
      success: true,
      data: health,
      latency_ms: latency
    };
  } catch (error) {
    return {
      success: false,
      error: error.message,
      latency_ms: Date.now() - start
    };
  }
}

// Main demo
async function main() {
  logger.info('SwiftRemit Health Check Demo');
  
  // Run 5 health checks
  for (let i = 1; i <= 5; i++) {
    const result = await checkHealth();
    
    logger.info({
      check_num: i,
      status: result.success ? '✅ HEALTHY' : '❌ UNHEALTHY',
      operational: result.data?.operational,
      initialized: result.data?.initialized,
      timestamp: result.data?.timestamp,
      latency_ms: result.latency_ms,
      performance: result.latency_ms < 100 ? '✅ PASS' : '⚠️  SLOW'
    }, 'Health check result');
  }
  
  logger.info('Health check demo complete!');
  
  logger.info({
    expected_response_structure: {
      success: true,
      data: {
        operational: true,
        timestamp: 1708545351,
        initialized: true
      },
      error: null
    }
  }, 'Expected Response Structure');
}

main().catch(err => logger.error({ error: err.message }, 'Demo failed'));
