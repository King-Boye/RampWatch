# Connection Pooling

## Overview

Optimize repeated HTTP requests by reusing connections with configurable pool size.

## Features

- ✅ Configurable pool size
- ✅ Connection reuse
- ✅ Idle timeout management
- ✅ Per-endpoint pooling
- ✅ Performance statistics
- ✅ Benchmark improvements

## Usage

### Configure Pool

```rust
client.configure_connection_pool(
    &20,    // max_connections
    &600,   // idle_timeout_seconds (10 min)
    &60,    // connection_timeout_seconds
    &true   // reuse_connections
);
```

### Get Pooled Connection

```rust
let endpoint = String::from_str(&env, "https://anchor.example.com");
client.get_pooled_connection(&endpoint);
```

### Check Statistics

```rust
let stats = client.get_pool_stats();
println!("Total requests: {}", stats.total_requests);
println!("Reused connections: {}", stats.reused_connections);
println!("New connections: {}", stats.new_connections);
```

### Reset Statistics

```rust
client.reset_pool_stats();
```

## Configuration

```rust
pub struct ConnectionPoolConfig {
    pub max_connections: u32,           // Max pool size
    pub idle_timeout_seconds: u64,      // Idle before expiry
    pub connection_timeout_seconds: u64, // Connection timeout
    pub reuse_connections: bool,        // Enable reuse
}
```

## Statistics

```rust
pub struct ConnectionStats {
    pub total_requests: u64,
    pub pooled_requests: u64,      // Requests using pool
    pub new_connections: u64,       // New connections created
    pub reused_connections: u64,    // Connections reused
    pub avg_response_time_ms: u64,
}
```

## Benchmark Results

### Without Pooling
- 10 requests = 10 new connections
- Connection overhead: 100%

### With Pooling
- 10 requests = 1 new connection + 9 reused
- Connection overhead: 10%
- **90% improvement**

## Example

```rust
// Configure pool
client.configure_connection_pool(&10, &300, &30, &true);

let endpoint = String::from_str(&env, "https://anchor.example.com");

// First request - creates new connection
client.get_pooled_connection(&endpoint);

// Subsequent requests - reuse connection
for _ in 0..9 {
    client.get_pooled_connection(&endpoint);
}

// Check stats
let stats = client.get_pool_stats();
assert_eq!(stats.new_connections, 1);
assert_eq!(stats.reused_connections, 9);
```

## How It Works

1. **First Request**: Creates new connection, stores in pool
2. **Subsequent Requests**: Reuses existing connection if:
   - Connection exists
   - Not expired (within idle timeout)
   - Same endpoint
3. **Expiry**: Connections expire after idle timeout
4. **Per-Endpoint**: Each endpoint has separate pool

## Benefits

### Performance
- Reduces connection overhead
- Faster request processing
- Lower latency

### Resource Efficiency
- Fewer TCP connections
- Reduced memory usage
- Better scalability

### Cost Savings
- Lower network costs
- Reduced server load
- Better throughput

## Configuration Recommendations

### High-Traffic Anchors
```rust
max_connections: 50
idle_timeout_seconds: 600  // 10 minutes
reuse_connections: true
```

### Low-Traffic Anchors
```rust
max_connections: 5
idle_timeout_seconds: 60   // 1 minute
reuse_connections: true
```

### Testing/Development
```rust
max_connections: 10
idle_timeout_seconds: 300  // 5 minutes
reuse_connections: true
```

### Disable Pooling
```rust
reuse_connections: false
```

## Storage

- **Config**: Persistent storage (90-day TTL)
- **Connections**: Temporary storage (idle timeout TTL)
- **Stats**: Temporary storage (1-day TTL)

## Best Practices

1. **Enable reuse** - Always enable for production
2. **Set appropriate timeout** - Balance between reuse and resource usage
3. **Monitor stats** - Track reuse rate
4. **Adjust pool size** - Based on traffic patterns
5. **Reset stats periodically** - For accurate metrics

## API Methods

```rust
// Configure
pub fn configure_connection_pool(
    max_connections: u32,
    idle_timeout_seconds: u64,
    connection_timeout_seconds: u64,
    reuse_connections: bool,
) -> Result<(), Error>

// Query
pub fn get_pool_config() -> ConnectionPoolConfig
pub fn get_pool_stats() -> ConnectionStats

// Use
pub fn get_pooled_connection(endpoint: String) -> Result<(), Error>

// Manage
pub fn reset_pool_stats() -> Result<(), Error>
```
