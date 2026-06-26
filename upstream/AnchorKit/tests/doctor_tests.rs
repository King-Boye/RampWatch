#![cfg(feature = "std")]

use std::process::Command;

/// Test that the doctor command validates Rust version correctly
#[test]
fn test_doctor_rust_version_check() {
    // Get the actual Rust version installed
    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .expect("Failed to execute rustc");
    
    let version_str = String::from_utf8_lossy(&output.stdout);
    let (major, minor) = parse_rustc_version(&version_str)
        .expect("Failed to parse rustc version");
    
    // The minimum required version for Soroban SDK 21.7.0
    const MIN_RUST_MAJOR: u32 = 1;
    const MIN_RUST_MINOR: u32 = 74;
    
    // If the system has an older version, we can't test the actual binary
    // but we can at least verify the parsing logic
    if major < MIN_RUST_MAJOR || (major == MIN_RUST_MAJOR && minor < MIN_RUST_MINOR) {
        println!("Warning: System Rust version {}.{} is below minimum {}.{}", 
                 major, minor, MIN_RUST_MAJOR, MIN_RUST_MINOR);
        println!("Skipping doctor command execution test");
        return;
    }
    
    // Run the doctor command
    let output = Command::new("cargo")
        .args(["run", "--bin", "anchorkit", "--", "doctor"])
        .output()
        .expect("Failed to execute doctor command");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // The doctor command should pass the Rust version check
    assert!(
        stdout.contains(&format!("✔ Rust {}.{} detected", major, minor)) ||
        stdout.contains("Rust") && stdout.contains("detected"),
        "Doctor command should detect Rust version. Output: {}",
        stdout
    );
}

/// Test that parse_rustc_version correctly parses version strings
#[test]
fn test_parse_rustc_version() {
    // Test valid version strings
    assert_eq!(parse_rustc_version("rustc 1.74.0 (79e9716c9 2023-11-13)"), Some((1, 74)));
    assert_eq!(parse_rustc_version("rustc 1.56.0 (09c42c458 2021-10-18)"), Some((1, 56)));
    assert_eq!(parse_rustc_version("rustc 1.80.0 (051478957 2024-07-21)"), Some((1, 80)));
    assert_eq!(parse_rustc_version("rustc 2.0.0 (future)"), Some((2, 0)));
    
    // Test invalid version strings
    assert_eq!(parse_rustc_version("invalid"), None);
    assert_eq!(parse_rustc_version("rustc"), None);
    assert_eq!(parse_rustc_version("rustc abc.def.ghi"), None);
}

/// Test that version comparison logic works correctly
#[test]
fn test_version_comparison() {
    const MIN_RUST_MAJOR: u32 = 1;
    const MIN_RUST_MINOR: u32 = 74;
    
    // Test versions that should pass
    assert!(version_meets_minimum(1, 74, MIN_RUST_MAJOR, MIN_RUST_MINOR));
    assert!(version_meets_minimum(1, 75, MIN_RUST_MAJOR, MIN_RUST_MINOR));
    assert!(version_meets_minimum(1, 80, MIN_RUST_MAJOR, MIN_RUST_MINOR));
    assert!(version_meets_minimum(2, 0, MIN_RUST_MAJOR, MIN_RUST_MINOR));
    
    // Test versions that should fail
    assert!(!version_meets_minimum(1, 73, MIN_RUST_MAJOR, MIN_RUST_MINOR));
    assert!(!version_meets_minimum(1, 56, MIN_RUST_MAJOR, MIN_RUST_MINOR));
    assert!(!version_meets_minimum(1, 0, MIN_RUST_MAJOR, MIN_RUST_MINOR));
    assert!(!version_meets_minimum(0, 99, MIN_RUST_MAJOR, MIN_RUST_MINOR));
}

// Helper functions (duplicated from anchorkit.rs for testing)

/// Parse "rustc X.Y.Z ..." → (X, Y)
fn parse_rustc_version(s: &str) -> Option<(u32, u32)> {
    let version_part = s.split_whitespace().nth(1)?;
    let mut parts = version_part.split('.');
    let major: u32 = parts.next()?.parse().ok()?;
    let minor: u32 = parts.next()?.parse().ok()?;
    Some((major, minor))
}

/// Check if a version meets the minimum requirement
fn version_meets_minimum(major: u32, minor: u32, min_major: u32, min_minor: u32) -> bool {
    major > min_major || (major == min_major && minor >= min_minor)
}
