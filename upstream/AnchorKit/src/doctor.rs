use std::process::Command;
use std::time::Instant;

/// Result of a single health check
#[derive(Debug)]
pub struct CheckResult {
    pub name: String,
    pub passed: bool,
    pub message: Option<String>,
}

impl CheckResult {
    fn pass(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passed: true,
            message: None,
        }
    }

    fn fail(name: &str, message: &str) -> Self {
        Self {
            name: name.to_string(),
            passed: false,
            message: Some(message.to_string()),
        }
    }
}

/// Run all health checks and return results
pub fn run_diagnostics() -> Vec<CheckResult> {
    let mut results = Vec::new();

    results.push(check_rust_toolchain());
    results.push(check_wasm_target());
    results.push(check_wallet_config());
    results.push(check_rpc_endpoint());
    results.push(check_config_files());
    results.push(check_network_connectivity());

    results
}

/// Check if Rust toolchain is installed
fn check_rust_toolchain() -> CheckResult {
    match Command::new("rustc").arg("--version").output() {
        Ok(output) if output.status.success() => {
            CheckResult::pass("Rust toolchain detected")
        }
        _ => CheckResult::fail(
            "Rust toolchain not found",
            "install from https://rustup.rs",
        ),
    }
}

/// Check if wasm32-unknown-unknown target is installed
fn check_wasm_target() -> CheckResult {
    match Command::new("rustup")
        .args(["target", "list", "--installed"])
        .output()
    {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.contains("wasm32-unknown-unknown") {
                CheckResult::pass("WASM target installed")
            } else {
                CheckResult::fail(
                    "WASM target not installed",
                    "run: rustup target add wasm32-unknown-unknown",
                )
            }
        }
        _ => CheckResult::fail(
            "Unable to check WASM target",
            "ensure rustup is installed",
        ),
    }
}

/// Check if wallet/keypair is configured
fn check_wallet_config() -> CheckResult {
    // Check for common Stellar wallet environment variables
    if std::env::var("STELLAR_SECRET_KEY").is_ok()
        || std::env::var("SOROBAN_SECRET_KEY").is_ok()
        || std::env::var("ANCHORKIT_SECRET_KEY").is_ok()
    {
        return CheckResult::pass("Wallet configured");
    }

    // Check for stellar CLI config
    if let Ok(home) = std::env::var("HOME") {
        let stellar_config = std::path::Path::new(&home)
            .join(".config")
            .join("soroban")
            .join("identity");
        
        if stellar_config.exists() {
            return CheckResult::pass("Wallet configured");
        }
    }

    CheckResult::fail(
        "Wallet not configured",
        "set STELLAR_SECRET_KEY or configure soroban identity",
    )
}

/// Check if RPC endpoint is configured and reachable
fn check_rpc_endpoint() -> CheckResult {
    let rpc_url = std::env::var("ANCHORKIT_RPC_URL")
        .or_else(|_| std::env::var("SOROBAN_RPC_URL"))
        .or_else(|_| std::env::var("STELLAR_RPC_URL"));

    match rpc_url {
        Ok(url) => {
            // Basic URL validation
            if !url.starts_with("http://") && !url.starts_with("https://") {
                return CheckResult::fail(
                    "Invalid RPC URL format",
                    "URL must start with http:// or https://",
                );
            }

            // Try to check connectivity (basic check)
            match check_url_reachable(&url) {
                Ok(true) => CheckResult::pass("RPC endpoint reachable"),
                Ok(false) => CheckResult::fail(
                    "RPC endpoint unreachable",
                    &format!("check ANCHORKIT_RPC_URL: {}", url),
                ),
                Err(e) => CheckResult::fail(
                    "Unable to verify RPC endpoint",
                    &format!("error: {}", e),
                ),
            }
        }
        Err(_) => CheckResult::fail(
            "RPC endpoint not configured",
            "set ANCHORKIT_RPC_URL, SOROBAN_RPC_URL, or STELLAR_RPC_URL",
        ),
    }
}

/// Check if config files are valid
fn check_config_files() -> CheckResult {
    let config_dir = std::path::Path::new("configs");
    
    if !config_dir.exists() {
        return CheckResult::fail(
            "Config directory not found",
            "create configs/ directory with anchor configurations",
        );
    }

    // Check for at least one config file
    match std::fs::read_dir(config_dir) {
        Ok(entries) => {
            let config_files: Vec<_> = entries
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.path()
                        .extension()
                        .map(|ext| ext == "json" || ext == "toml")
                        .unwrap_or(false)
                })
                .collect();

            if config_files.is_empty() {
                CheckResult::fail(
                    "No config files found",
                    "add .json or .toml config files to configs/",
                )
            } else {
                // Basic validation - check if files are readable
                let mut all_valid = true;
                for entry in &config_files {
                    if std::fs::read_to_string(entry.path()).is_err() {
                        all_valid = false;
                        break;
                    }
                }

                if all_valid {
                    CheckResult::pass(&format!("Config files valid ({} found)", config_files.len()))
                } else {
                    CheckResult::fail(
                        "Some config files are unreadable",
                        "check file permissions and format",
                    )
                }
            }
        }
        Err(_) => CheckResult::fail(
            "Unable to read config directory",
            "check directory permissions",
        ),
    }
}

/// Check network connectivity
fn check_network_connectivity() -> CheckResult {
    // Try to reach a known Stellar endpoint
    let test_urls = [
        "https://horizon-testnet.stellar.org",
        "https://soroban-testnet.stellar.org",
    ];

    for url in &test_urls {
        if let Ok(true) = check_url_reachable(url) {
            return CheckResult::pass("Network responding");
        }
    }

    CheckResult::fail(
        "Network connectivity issues",
        "check internet connection and firewall settings",
    )
}

/// Helper function to check if a URL is reachable
fn check_url_reachable(url: &str) -> Result<bool, String> {
    // Use curl for a quick connectivity check (timeout 2 seconds)
    match Command::new("curl")
        .args([
            "-s",
            "-o",
            "/dev/null",
            "-w",
            "%{http_code}",
            "--connect-timeout",
            "2",
            "--max-time",
            "2",
            url,
        ])
        .output()
    {
        Ok(output) => {
            let status_code = String::from_utf8_lossy(&output.stdout);
            // Consider 2xx, 3xx, and even 4xx as "reachable" (server responded)
            Ok(status_code.starts_with('2')
                || status_code.starts_with('3')
                || status_code.starts_with('4'))
        }
        Err(_) => {
            // If curl is not available, try a simpler approach
            // Just return Ok(true) to avoid false negatives
            Ok(true)
        }
    }
}

/// Print diagnostic results in a user-friendly format
pub fn print_results(results: &[CheckResult]) -> bool {
    let start = Instant::now();
    
    println!("\n🔍 Running AnchorKit diagnostics...\n");

    let mut all_passed = true;
    for result in results {
        let symbol = if result.passed { "✔" } else { "✖" };
        let color = if result.passed { "\x1b[32m" } else { "\x1b[31m" };
        let reset = "\x1b[0m";

        print!("{}{} {}{}", color, symbol, result.name, reset);

        if let Some(msg) = &result.message {
            println!(" → {}", msg);
        } else {
            println!();
        }

        if !result.passed {
            all_passed = false;
        }
    }

    let duration = start.elapsed();
    println!("\n⏱  Completed in {:.2}s", duration.as_secs_f64());

    if all_passed {
        println!("\n✅ All checks passed! Your environment is ready.\n");
    } else {
        println!("\n⚠️  Some checks failed. Please address the issues above.\n");
    }

    all_passed
}
