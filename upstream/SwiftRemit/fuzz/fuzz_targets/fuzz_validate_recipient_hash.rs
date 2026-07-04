#![no_main]

use libfuzzer_sys::fuzz_target;
use soroban_sdk::{Bytes, Env};
use swiftremit::validation::validate_evidence_hash;

// Create the Soroban test environment once for the lifetime of the fuzzer
// process.  OnceLock is stable since Rust 1.70 and avoids the overhead of
// initialising a new Env on every fuzzer iteration.
static ENV: std::sync::OnceLock<Env> = std::sync::OnceLock::new();

fuzz_target!(|data: &[u8]| {
    let env = ENV.get_or_init(Env::default);

    // Build a Soroban Bytes value from the fuzzer-supplied bytes.
    let hash = Bytes::from_slice(env, data);

    // validate_evidence_hash must never panic — only return Ok or ContractError.
    let _ = validate_evidence_hash(&hash);
});
