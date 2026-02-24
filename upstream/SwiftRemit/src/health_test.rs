#![cfg(test)]

use soroban_sdk::{Env, contracttype};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HealthStatus {
    pub operational: bool,
    pub timestamp: u64,
    pub initialized: bool,
}

fn mock_health_check(env: &Env, initialized: bool) -> HealthStatus {
    HealthStatus {
        operational: true,
        timestamp: env.ledger().timestamp(),
        initialized,
    }
}

#[test]
fn test_health_status_structure() {
    let env = Env::default();
    
    let health = mock_health_check(&env, false);
    
    assert!(health.operational);
    assert_eq!(health.initialized, false);
    assert!(health.timestamp > 0);
}

#[test]
fn test_health_status_initialized() {
    let env = Env::default();
    
    let health = mock_health_check(&env, true);
    
    assert!(health.operational);
    assert_eq!(health.initialized, true);
}

#[test]
fn test_health_status_timestamp() {
    let env = Env::default();
    
    let health1 = mock_health_check(&env, true);
    let health2 = mock_health_check(&env, true);
    
    // Same ledger, same timestamp
    assert_eq!(health1.timestamp, health2.timestamp);
}

#[test]
fn test_health_status_clone() {
    let env = Env::default();
    
    let health1 = mock_health_check(&env, true);
    let health2 = health1.clone();
    
    assert_eq!(health1, health2);
}
