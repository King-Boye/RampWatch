#![no_main]

use libfuzzer_sys::fuzz_target;
use swiftremit::validation::{validate_amount, validate_fee_bps, validate_escrow_ttl};

/// Fuzz target for corridor-level validation: fee basis points, amounts, and
/// escrow TTLs all flow through corridor-gated logic.  None of these functions
/// should ever panic regardless of the input value.
fuzz_target!(|data: &[u8]| {
    if data.len() < 13 {
        return;
    }

    // Derive a u32 fee_bps from the first 4 bytes.
    let mut fee_buf = [0u8; 4];
    fee_buf.copy_from_slice(&data[..4]);
    let fee_bps = u32::from_le_bytes(fee_buf);

    // Derive an i128 amount from the next 16 bytes (if available).
    let amount: i128 = if data.len() >= 20 {
        let mut amt_buf = [0u8; 16];
        amt_buf.copy_from_slice(&data[4..20]);
        i128::from_le_bytes(amt_buf)
    } else {
        0
    };

    // Derive a u64 TTL from the remaining bytes (if available).
    let ttl: u64 = if data.len() >= 28 {
        let mut ttl_buf = [0u8; 8];
        ttl_buf.copy_from_slice(&data[20..28]);
        u64::from_le_bytes(ttl_buf)
    } else {
        0
    };

    let _ = validate_fee_bps(fee_bps);
    let _ = validate_amount(amount);
    let _ = validate_escrow_ttl(ttl);
});
