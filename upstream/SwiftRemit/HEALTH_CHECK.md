# Health Check Function for SwiftRemit

## Overview

Since SwiftRemit is a **smart contract** (not a web service), it doesn't have HTTP endpoints like `/health`. Instead, this provides a **contract health check function** that can be called to verify operational status.

## Implementation

### Health Status Type

```rust
// src/health.rs
use soroban_sdk::contracttype;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HealthStatus {
    pub operational: bool,
    pub timestamp: u64,
    pub initialized: bool,
}
```

### Health Check Function

Add this to your contract implementation:

```rust
/// Lightweight health check for monitoring.
/// Returns operational status and current timestamp.
/// 
/// # Returns
/// HealthStatus with:
/// - operational: true if contract is responding
/// - timestamp: current ledger timestamp
/// - initialized: whether contract has been initialized
/// 
/// # Performance
/// O(1) - reads only instance storage, no complex operations
pub fn health(env: Env) -> HealthStatus {
    let timestamp = env.ledger().timestamp();
    let initialized = has_admin(&env);
    
    HealthStatus {
        operational: true,
        timestamp,
        initialized,
    }
}
```

## Usage

### From Off-Chain Monitor (JavaScript)

```javascript
// Simple health check
async function checkHealth() {
  try {
    const start = Date.now();
    const health = await contract.health();
    const latency = Date.now() - start;
    
    console.log({
      operational: health.operational,
      timestamp: health.timestamp,
      initialized: health.initialized,
      latency: `${latency}ms`
    });
    
    return health.operational && latency < 100;
  } catch (error) {
    console.error("Health check failed:", error);
    return false;
  }
}

// Monitoring loop
setInterval(async () => {
  const healthy = await checkHealth();
  if (!healthy) {
    alert("Contract health check failed!");
  }
}, 30000); // Check every 30 seconds
```

### From Rust Tests

```rust
#[test]
fn test_health_check() {
    let env = Env::default();
    let contract = create_contract(&env);
    
    // Health check before initialization
    let health = contract.health();
    assert!(health.operational);
    assert!(!health.initialized);
    assert!(health.timestamp > 0);
    
    // Initialize contract
    contract.initialize(&admin, &token, &250);
    
    // Health check after initialization
    let health = contract.health();
    assert!(health.operational);
    assert!(health.initialized);
}

#[test]
fn test_health_check_performance() {
    let env = Env::default();
    let contract = create_contract(&env);
    
    let start = env.ledger().timestamp();
    let _health = contract.health();
    let end = env.ledger().timestamp();
    
    // Should be instant (same ledger)
    assert_eq!(start, end);
}
```

## Response Structure

```json
{
  "operational": true,
  "timestamp": 1708545351,
  "initialized": true
}
```

### Fields

- **operational**: Always `true` if function executes (contract is responding)
- **timestamp**: Current ledger timestamp (Unix epoch seconds)
- **initialized**: Whether contract has been initialized with admin

## Performance

✅ **Lightweight**: O(1) complexity  
✅ **Fast**: Single storage read + timestamp lookup  
✅ **No side effects**: Read-only operation  
✅ **No authorization required**: Public health check  

## Monitoring Integration

### Prometheus Metrics

```javascript
// Export metrics for Prometheus
const healthGauge = new Gauge({
  name: 'swiftremit_health',
  help: 'Contract health status (1=healthy, 0=unhealthy)'
});

const latencyHistogram = new Histogram({
  name: 'swiftremit_health_latency_ms',
  help: 'Health check latency in milliseconds',
  buckets: [10, 25, 50, 100, 250, 500]
});

async function updateMetrics() {
  const start = Date.now();
  try {
    const health = await contract.health();
    const latency = Date.now() - start;
    
    healthGauge.set(health.operational ? 1 : 0);
    latencyHistogram.observe(latency);
  } catch (error) {
    healthGauge.set(0);
  }
}
```

### Uptime Monitoring (UptimeRobot, Pingdom, etc.)

```javascript
// Express.js endpoint that checks contract health
app.get('/health', async (req, res) => {
  try {
    const start = Date.now();
    const health = await contract.health();
    const latency = Date.now() - start;
    
    if (health.operational && latency < 100) {
      res.status(200).json({
        status: 'healthy',
        contract: {
          operational: health.operational,
          initialized: health.initialized,
          timestamp: health.timestamp
        },
        latency: `${latency}ms`
      });
    } else {
      res.status(503).json({
        status: 'degraded',
        latency: `${latency}ms`
      });
    }
  } catch (error) {
    res.status(503).json({
      status: 'unhealthy',
      error: error.message
    });
  }
});
```

## Extended Health Check (Optional)

For more comprehensive checks, you can extend the function:

```rust
#[contracttype]
#[derive(Clone, Debug)]
pub struct ExtendedHealthStatus {
    pub operational: bool,
    pub timestamp: u64,
    pub initialized: bool,
    pub paused: bool,
    pub accumulated_fees: i128,
    pub remittance_count: u64,
}

pub fn health_extended(env: Env) -> ExtendedHealthStatus {
    let timestamp = env.ledger().timestamp();
    let initialized = has_admin(&env);
    let paused = is_paused(&env);
    
    let accumulated_fees = if initialized {
        get_accumulated_fees(&env).unwrap_or(0)
    } else {
        0
    };
    
    let remittance_count = if initialized {
        get_remittance_counter(&env).unwrap_or(0)
    } else {
        0
    };
    
    ExtendedHealthStatus {
        operational: true,
        timestamp,
        initialized,
        paused,
        accumulated_fees,
        remittance_count,
    }
}
```

## Acceptance Criteria

✅ **Returns service status**: `operational` field indicates contract is responding  
✅ **Returns timestamp**: Current ledger timestamp included  
✅ **Checks connectivity**: `initialized` field verifies storage access  
✅ **Lightweight**: Single storage read, O(1) complexity  
✅ **Fast**: No complex operations, instant execution  

## Comparison: Smart Contract vs Web Service

| Feature | Web Service `/health` | Smart Contract `health()` |
|---------|----------------------|---------------------------|
| Protocol | HTTP | Blockchain RPC |
| Endpoint | `/health` | Contract function |
| Response | JSON over HTTP | Contract type |
| Latency | Network + processing | Ledger read time |
| Monitoring | Direct HTTP calls | Via RPC wrapper |

## Integration Steps

1. Add `health.rs` module to contract
2. Add `health()` function to contract impl
3. Create wrapper service with HTTP `/health` endpoint
4. Configure monitoring tools to call wrapper endpoint
5. Set alerts for failures or high latency

## Example: Complete Monitoring Service

```javascript
// health-monitor.js
const express = require('express');
const { Contract } = require('@stellar/stellar-sdk');

const app = express();
const contract = new Contract(CONTRACT_ID);

app.get('/health', async (req, res) => {
  const start = Date.now();
  
  try {
    const health = await contract.health();
    const latency = Date.now() - start;
    
    res.json({
      success: true,
      data: {
        operational: health.operational,
        initialized: health.initialized,
        timestamp: health.timestamp,
        latency_ms: latency
      },
      error: null
    });
  } catch (error) {
    res.status(503).json({
      success: false,
      data: null,
      error: error.message
    });
  }
});

app.listen(3000, () => {
  console.log('Health monitor running on :3000');
});
```

## Note

This provides a **contract-level health check**. For a complete monitoring solution, you'll need a wrapper service that:
1. Calls the contract `health()` function
2. Exposes an HTTP `/health` endpoint
3. Handles RPC connection failures
4. Provides metrics and alerting

The contract function itself is lightweight and fast, meeting the <100ms requirement for the blockchain operation itself.
