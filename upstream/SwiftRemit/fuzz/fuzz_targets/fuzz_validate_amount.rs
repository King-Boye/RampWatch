#![no_main]

use libfuzzer_sys::fuzz_target;
use swiftremit::validation::validate_amount;

fuzz_target!(|data: &[u8]| {
    if data.len() < 16 {
        return;
    }
    // Interpret the first 16 bytes as a little-endian i128.
    let mut buf = [0u8; 16];
    buf.copy_from_slice(&data[..16]);
    let amount = i128::from_le_bytes(buf);

    // validate_amount must never panic — only return Ok or a ContractError.
    let _ = validate_amount(amount);
});
