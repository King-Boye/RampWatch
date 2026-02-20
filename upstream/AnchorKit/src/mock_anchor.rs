use soroban_sdk::{Address, Bytes, BytesN, Env};

/// Mock anchor for testing without external APIs
pub struct MockAnchor;

impl MockAnchor {
    /// Generate a mock signature for testing
    pub fn sign(_env: &Env, _issuer: &Address, _subject: &Address, _timestamp: u64, payload_hash: &BytesN<32>) -> Bytes {
        // Create deterministic signature from payload_hash
        // In real scenarios, this would be a cryptographic signature
        Bytes::from(payload_hash.clone())
    }
    
    /// Create a mock payload hash
    pub fn hash_payload(env: &Env, data: &Bytes) -> BytesN<32> {
        env.crypto().sha256(data).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::Address as _;

    #[test]
    fn test_mock_signature() {
        let env = Env::default();
        let issuer = Address::generate(&env);
        let subject = Address::generate(&env);
        let timestamp = 1000u64;
        let payload = Bytes::from_slice(&env, b"test payload");
        let payload_hash = MockAnchor::hash_payload(&env, &payload);
        
        let sig = MockAnchor::sign(&env, &issuer, &subject, timestamp, &payload_hash);
        assert!(sig.len() > 0);
    }
    
    #[test]
    fn test_deterministic_signature() {
        let env = Env::default();
        let issuer = Address::generate(&env);
        let subject = Address::generate(&env);
        let timestamp = 1000u64;
        let payload = Bytes::from_slice(&env, b"test payload");
        let payload_hash = MockAnchor::hash_payload(&env, &payload);
        
        let sig1 = MockAnchor::sign(&env, &issuer, &subject, timestamp, &payload_hash);
        let sig2 = MockAnchor::sign(&env, &issuer, &subject, timestamp, &payload_hash);
        
        assert_eq!(sig1, sig2);
    }
    
    #[test]
    fn test_hash_payload() {
        let env = Env::default();
        let payload = Bytes::from_slice(&env, b"test data");
        let hash = MockAnchor::hash_payload(&env, &payload);
        
        assert_eq!(hash.len(), 32);
    }
}
