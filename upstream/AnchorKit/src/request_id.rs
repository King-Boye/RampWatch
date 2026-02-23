use soroban_sdk::{contracttype, Bytes, BytesN, Env};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequestId {
    pub id: BytesN<16>, // 128-bit UUID
    pub created_at: u64,
}

impl RequestId {
    pub fn generate(env: &Env) -> Self {
        let timestamp = env.ledger().timestamp();
        let sequence = env.ledger().sequence();
        
        // Generate pseudo-UUID from timestamp + sequence
        let mut bytes = Bytes::new(env);
        bytes.append(&Bytes::from_array(env, &timestamp.to_be_bytes()));
        bytes.append(&Bytes::from_array(env, &sequence.to_be_bytes()));
        
        // Hash to get 32 bytes, take first 16
        let hash = env.crypto().sha256(&bytes);
        let hash_bytes = hash.to_array();
        let id = BytesN::from_array(env, &[
            hash_bytes[0], hash_bytes[1], hash_bytes[2], hash_bytes[3],
            hash_bytes[4], hash_bytes[5], hash_bytes[6], hash_bytes[7],
            hash_bytes[8], hash_bytes[9], hash_bytes[10], hash_bytes[11],
            hash_bytes[12], hash_bytes[13], hash_bytes[14], hash_bytes[15],
        ]);
        
        Self {
            id,
            created_at: timestamp,
        }
    }
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TracingSpan {
    pub request_id: RequestId,
    pub operation: soroban_sdk::String,
    pub actor: soroban_sdk::Address,
    pub started_at: u64,
    pub completed_at: u64,
    pub status: soroban_sdk::String,
}

pub struct RequestTracker;

impl RequestTracker {
    pub fn store_span(env: &Env, span: &TracingSpan) {
        let key = (
            soroban_sdk::symbol_short!("SPAN"),
            span.request_id.id.clone(),
        );
        env.storage().temporary().set(&key, span);
        env.storage().temporary().extend_ttl(&key, 17280, 17280); // 1 day
    }

    pub fn get_span(env: &Env, request_id: &BytesN<16>) -> Option<TracingSpan> {
        let key = (soroban_sdk::symbol_short!("SPAN"), request_id.clone());
        env.storage().temporary().get(&key)
    }
}
