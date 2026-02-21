#!/usr/bin/env node

/**
 * Health Check Demo for SwiftRemit Smart Contract
 * 
 * This demonstrates how a health check would work in practice.
 * Since the contract has compilation issues, this shows the expected behavior.
 */

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
  console.log('SwiftRemit Health Check Demo\n');
  console.log('=' .repeat(50));
  
  // Run 5 health checks
  for (let i = 1; i <= 5; i++) {
    const result = await checkHealth();
    
    console.log(`\nCheck #${i}:`);
    console.log(`  Status: ${result.success ? '✅ HEALTHY' : '❌ UNHEALTHY'}`);
    console.log(`  Operational: ${result.data?.operational}`);
    console.log(`  Initialized: ${result.data?.initialized}`);
    console.log(`  Timestamp: ${result.data?.timestamp}`);
    console.log(`  Latency: ${result.latency_ms}ms`);
    console.log(`  Performance: ${result.latency_ms < 100 ? '✅ PASS' : '⚠️  SLOW'} (<100ms)`);
  }
  
  console.log('\n' + '='.repeat(50));
  console.log('\n✅ Health check demo complete!');
  console.log('\nExpected Response Structure:');
  console.log(JSON.stringify({
    success: true,
    data: {
      operational: true,
      timestamp: 1708545351,
      initialized: true
    },
    error: null
  }, null, 2));
}

main().catch(console.error);
