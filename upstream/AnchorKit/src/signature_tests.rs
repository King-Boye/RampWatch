#![cfg(test)]

use soroban_sdk::{Bytes, BytesN, Env, Vec};

fn generate_signature(env: &Env, data: &BytesN<32>) -> BytesN<32> {
    env.crypto()
        .sha256(&Bytes::from_array(env, &data.to_array()))
        .into()
}

fn generate_signature_with_key(env: &Env, data: &BytesN<32>, key: &Bytes) -> BytesN<32> {
    let mut combined = Bytes::new(env);
    combined.append(&Bytes::from_array(env, &data.to_array()));
    combined.append(key);
    env.crypto().sha256(&combined).into()
}

#[test]
fn test_same_input_produces_identical_signature() {
    let env = Env::default();
    let data = BytesN::from_array(&env, &[1; 32]);

    let sig1 = generate_signature(&env, &data);
    let sig2 = generate_signature(&env, &data);

    assert_eq!(sig1, sig2);
}

#[test]
fn test_signature_reproducible_across_executions() {
    let env1 = Env::default();
    let env2 = Env::default();

    let data1 = BytesN::from_array(&env1, &[42; 32]);
    let data2 = BytesN::from_array(&env2, &[42; 32]);

    let sig1 = generate_signature(&env1, &data1);
    let sig2 = generate_signature(&env2, &data2);

    assert_eq!(sig1.to_array(), sig2.to_array());
}

#[test]
fn test_different_input_produces_different_signature() {
    let env = Env::default();
    let data1 = BytesN::from_array(&env, &[1; 32]);
    let data2 = BytesN::from_array(&env, &[2; 32]);

    let sig1 = generate_signature(&env, &data1);
    let sig2 = generate_signature(&env, &data2);

    assert_ne!(sig1, sig2);
}

#[test]
fn test_signature_with_key_is_deterministic() {
    let env = Env::default();
    let data = BytesN::from_array(&env, &[5; 32]);
    let key = Bytes::from_array(&env, &[10, 20, 30, 40]);

    let sig1 = generate_signature_with_key(&env, &data, &key);
    let sig2 = generate_signature_with_key(&env, &data, &key);

    assert_eq!(sig1, sig2);
}

#[test]
fn test_different_keys_produce_different_signatures() {
    let env = Env::default();
    let data = BytesN::from_array(&env, &[7; 32]);
    let key1 = Bytes::from_array(&env, &[1, 2, 3]);
    let key2 = Bytes::from_array(&env, &[4, 5, 6]);

    let sig1 = generate_signature_with_key(&env, &data, &key1);
    let sig2 = generate_signature_with_key(&env, &data, &key2);

    assert_ne!(sig1, sig2);
}

#[test]
fn test_empty_key_produces_signature() {
    let env = Env::default();
    let data = BytesN::from_array(&env, &[9; 32]);
    let empty_key = Bytes::new(&env);

    let sig1 = generate_signature_with_key(&env, &data, &empty_key);
    let sig2 = generate_signature_with_key(&env, &data, &empty_key);

    // Empty key should still produce consistent signature
    assert_eq!(sig1, sig2);
}

#[test]
fn test_signature_consistency_multiple_runs() {
    let env = Env::default();
    let data = BytesN::from_array(&env, &[100; 32]);

    let mut signatures = Vec::new(&env);
    for _ in 0..5 {
        signatures.push_back(generate_signature(&env, &data));
    }

    // All signatures should be identical
    for i in 1..signatures.len() {
        assert_eq!(signatures.get(0).unwrap(), signatures.get(i).unwrap());
    }
}

#[test]
fn test_signature_with_zero_data() {
    let env = Env::default();
    let data = BytesN::from_array(&env, &[0; 32]);

    let sig1 = generate_signature(&env, &data);
    let sig2 = generate_signature(&env, &data);

    assert_eq!(sig1, sig2);
}

#[test]
fn test_signature_with_max_data() {
    let env = Env::default();
    let data = BytesN::from_array(&env, &[255; 32]);

    let sig1 = generate_signature(&env, &data);
    let sig2 = generate_signature(&env, &data);

    assert_eq!(sig1, sig2);
}

#[test]
fn test_key_order_matters() {
    let env = Env::default();
    let data = BytesN::from_array(&env, &[50; 32]);
    let key1 = Bytes::from_array(&env, &[1, 2, 3]);
    let key2 = Bytes::from_array(&env, &[3, 2, 1]);

    let sig1 = generate_signature_with_key(&env, &data, &key1);
    let sig2 = generate_signature_with_key(&env, &data, &key2);

    assert_ne!(sig1, sig2);
}

#[test]
fn test_signature_length_consistency() {
    let env = Env::default();
    let data1 = BytesN::from_array(&env, &[1; 32]);
    let data2 = BytesN::from_array(&env, &[255; 32]);

    let sig1 = generate_signature(&env, &data1);
    let sig2 = generate_signature(&env, &data2);

    assert_eq!(sig1.to_array().len(), 32);
    assert_eq!(sig2.to_array().len(), 32);
}
