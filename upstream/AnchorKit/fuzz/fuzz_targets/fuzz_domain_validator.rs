#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(s) = std::str::from_utf8(data) else {
        return;
    };

    // Must not panic regardless of input
    let _ = anchorkit::domain_validator::validate_anchor_domain(s);

    // Batch variant must also be panic-free and return one result per input
    let inputs: Vec<&str> = s.lines().collect();
    let results = anchorkit::domain_validator::validate_anchor_domain_batch(&inputs);
    assert_eq!(results.len(), inputs.len());
});
