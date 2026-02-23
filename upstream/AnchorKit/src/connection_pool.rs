use soroban_sdk::{contracttype, Env, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConnectionPoolConfig {
    pub max_connections: u32,
    pub idle_timeout_seconds: u64,
    pub connection_timeout_seconds: u64,
    pub reuse_connections: bool,
}

impl ConnectionPoolConfig {
    pub fn default(env: &Env) -> Self {
        Self {
            max_connections: 10,
            idle_timeout_seconds: 300,      // 5 minutes
            connection_timeout_seconds: 30, // 30 seconds
            reuse_connections: true,
        }
    }
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConnectionStats {
    pub total_requests: u64,
    pub pooled_requests: u64,
    pub new_connections: u64,
    pub reused_connections: u64,
    pub avg_response_time_ms: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
struct PooledConnection {
    pub endpoint: String,
    pub created_at: u64,
    pub last_used: u64,
    pub request_count: u32,
}

pub struct ConnectionPool;

impl ConnectionPool {
    pub fn set_config(env: &Env, config: &ConnectionPoolConfig) {
        let key = soroban_sdk::symbol_short!("POOLCFG");
        env.storage().persistent().set(&key, config);
        env.storage()
            .persistent()
            .extend_ttl(&key, 7776000, 7776000); // 90 days
    }

    pub fn get_config(env: &Env) -> ConnectionPoolConfig {
        let key = soroban_sdk::symbol_short!("POOLCFG");
        env.storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| ConnectionPoolConfig::default(env))
    }

    pub fn get_connection(env: &Env, endpoint: &String) {
        let config = Self::get_config(env);
        let key = (soroban_sdk::symbol_short!("POOLCONN"), endpoint.clone());

        if config.reuse_connections {
            if let Some(mut conn) = env.storage().temporary().get::<_, PooledConnection>(&key) {
                let now = env.ledger().timestamp();

                // Check if connection is still valid
                if now - conn.last_used < config.idle_timeout_seconds {
                    conn.last_used = now;
                    conn.request_count += 1;
                    env.storage().temporary().set(&key, &conn);

                    // Update stats
                    Self::increment_reused(env);

                    return;
                }
            }
        }

        // Create new connection
        let now = env.ledger().timestamp();
        let conn = PooledConnection {
            endpoint: endpoint.clone(),
            created_at: now,
            last_used: now,
            request_count: 1,
        };

        env.storage().temporary().set(&key, &conn);
        env.storage()
            .temporary()
            .extend_ttl(&key, config.idle_timeout_seconds as u32, config.idle_timeout_seconds as u32);

        // Update stats
        Self::increment_new(env);
    }

    pub fn release_connection(env: &Env, endpoint: &String) {
        let key = (soroban_sdk::symbol_short!("POOLCONN"), endpoint.clone());
        if let Some(conn) = env.storage().temporary().get::<_, PooledConnection>(&key) {
            env.storage().temporary().set(&key, &conn);
        }
    }

    fn increment_new(env: &Env) {
        let key = soroban_sdk::symbol_short!("POOLNEW");
        let count: u64 = env.storage().temporary().get(&key).unwrap_or(0);
        env.storage().temporary().set(&key, &(count + 1));
        env.storage().temporary().extend_ttl(&key, 17280, 17280);
    }

    fn increment_reused(env: &Env) {
        let key = soroban_sdk::symbol_short!("POOLREUSE");
        let count: u64 = env.storage().temporary().get(&key).unwrap_or(0);
        env.storage().temporary().set(&key, &(count + 1));
        env.storage().temporary().extend_ttl(&key, 17280, 17280);
    }

    pub fn get_stats(env: &Env) -> ConnectionStats {
        let new_key = soroban_sdk::symbol_short!("POOLNEW");
        let reuse_key = soroban_sdk::symbol_short!("POOLREUSE");

        let new_connections: u64 = env.storage().temporary().get(&new_key).unwrap_or(0);
        let reused_connections: u64 = env.storage().temporary().get(&reuse_key).unwrap_or(0);
        let total_requests = new_connections + reused_connections;

        ConnectionStats {
            total_requests,
            pooled_requests: reused_connections,
            new_connections,
            reused_connections,
            avg_response_time_ms: 0, // Calculated separately
        }
    }

    pub fn reset_stats(env: &Env) {
        let new_key = soroban_sdk::symbol_short!("POOLNEW");
        let reuse_key = soroban_sdk::symbol_short!("POOLREUSE");
        env.storage().temporary().remove(&new_key);
        env.storage().temporary().remove(&reuse_key);
    }
}
