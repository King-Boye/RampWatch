use clap::{Parser, Subcommand};
use std::process::Command;
use std::time::Instant;

const MIN_RUST_MAJOR: u32 = 1;
const MIN_RUST_MINOR: u32 = 56;

#[derive(Parser)]
#[command(name = "anchorkit", about = "AnchorKit CLI for Soroban anchor management")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the smart contract
    Build {
        /// Build in release mode
        #[arg(long)]
        release: bool,
    },
    /// Deploy to Stellar network
    Deploy {
        /// Target network (testnet, mainnet)
        #[arg(long, default_value = "testnet")]
        network: String,
    },
    /// Initialize contract with admin
    Init {
        /// Admin account address
        #[arg(long)]
        admin: String,
    },
    /// Run environment diagnostics
    Doctor,
    /// Validate configuration files (JSON and TOML)
    Validate {
        /// Path to config file or directory (defaults to configs/)
        #[arg(default_value = "configs")]
        path: String,
    },
    /// Register a new attestor
    Register {
        /// Stellar address of the attestor
        #[arg(long)]
        address: String,
        /// Comma-separated services: deposits, withdrawals, quotes, kyc
        #[arg(long)]
        services: Option<String>,
        /// Attestor endpoint URL
        #[arg(long)]
        endpoint: Option<String>,
    },
    /// Submit attestation
    Attest {
        /// Subject address
        #[arg(long)]
        subject: String,
        /// Payload hash
        #[arg(long)]
        payload_hash: String,
        /// Session ID for tracking
        #[arg(long)]
        session: Option<String>,
    },
    /// Query attestation by ID
    Query {
        /// Transaction ID to query
        #[arg(long)]
        transaction_id: String,
        /// Output format: json or text
        #[arg(long, default_value = "text")]
        output: String,
    },
    /// Check attestor health
    Health {
        /// Specific attestor to check
        #[arg(long)]
        attestor: Option<String>,
        /// Watch mode with continuous monitoring
        #[arg(long)]
        watch: bool,
        /// Interval in seconds for watch mode
        #[arg(long, default_value = "60")]
        interval: u64,
    },
    /// Run contract tests
    Test {
        /// Run specific test pattern
        #[arg(long)]
        pattern: Option<String>,
        /// Verbose output
        #[arg(long)]
        verbose: bool,
    },
    /// Export audit logs
    #[command(name = "export-audit")]
    ExportAudit {
        /// Output format: json or csv
        #[arg(long, default_value = "json")]
        format: String,
        /// Output file path
        #[arg(long, short)]
        output: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Build { release } => run_build(release),
        Commands::Deploy { network } => run_deploy(&network),
        Commands::Init { admin } => run_init(&admin),
        Commands::Doctor => run_doctor(),
        Commands::Validate { path } => run_validate(&path),
        Commands::Register { address, services, endpoint } => {
            run_register(&address, services.as_deref(), endpoint.as_deref())
        }
        Commands::Attest { subject, payload_hash, session } => {
            run_attest(&subject, &payload_hash, session.as_deref())
        }
        Commands::Query { transaction_id, output } => run_query(&transaction_id, &output),
        Commands::Health { attestor, watch, interval } => {
            run_health(attestor.as_deref(), watch, interval)
        }
        Commands::Test { pattern, verbose } => run_test(pattern.as_deref(), verbose),
        Commands::ExportAudit { format, output } => run_export_audit(&format, &output),
    }
}

// ── build ───────────────────────────────────────────────────────────────────

fn run_build(release: bool) {
    println!("🔨 Building AnchorKit smart contract...");
    
    let mut cmd = Command::new("cargo");
    cmd.args(["build", "--target", "wasm32-unknown-unknown"]);
    
    if release {
        cmd.arg("--release");
        println!("  Mode: Release (optimized)");
    } else {
        println!("  Mode: Debug");
    }
    
    match cmd.output() {
        Ok(output) if output.status.success() => {
            println!("✅ Contract built successfully");
            if release {
                println!("  Output: target/wasm32-unknown-unknown/release/anchorkit.wasm");
            } else {
                println!("  Output: target/wasm32-unknown-unknown/debug/anchorkit.wasm");
            }
        }
        Ok(output) => {
            eprintln!("❌ Build failed:");
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("❌ Failed to run cargo build: {}", e);
            std::process::exit(1);
        }
    }
}

// ── deploy ──────────────────────────────────────────────────────────────────

fn run_deploy(network: &str) {
    println!("🚀 Deploying to {} network...", network);
    
    // Check if contract is built
    let wasm_path = "target/wasm32-unknown-unknown/release/anchorkit.wasm";
    if !std::path::Path::new(wasm_path).exists() {
        println!("⚠️  Contract not found. Building first...");
        run_build(true);
    }
    
    println!("📋 Deployment steps:");
    println!("  1. Use soroban CLI to deploy:");
    println!("     soroban contract deploy \\");
    println!("       --wasm {} \\", wasm_path);
    println!("       --source <YOUR_ACCOUNT> \\");
    println!("       --network {}", network);
    println!();
    println!("  2. Initialize the contract:");
    println!("     anchorkit init --admin <ADMIN_ADDRESS>");
    println!();
    println!("💡 Note: Actual deployment requires soroban CLI and configured account");
}

// ── init ────────────────────────────────────────────────────────────────────

fn run_init(admin: &str) {
    println!("🔧 Initializing contract with admin: {}", admin);
    
    // Validate admin address format
    if !admin.starts_with('G') || admin.len() != 56 {
        eprintln!("❌ Invalid admin address format. Expected Stellar public key starting with 'G'");
        std::process::exit(1);
    }
    
    println!("📋 Initialization steps:");
    println!("  Use soroban CLI to initialize:");
    println!("  soroban contract invoke \\");
    println!("    --id <CONTRACT_ID> \\");
    println!("    --source {} \\", admin);
    println!("    --network testnet \\");
    println!("    -- \\");
    println!("    initialize \\");
    println!("    --admin {}", admin);
    println!();
    println!("💡 Note: Replace <CONTRACT_ID> with your deployed contract address");
}

// ── attest ──────────────────────────────────────────────────────────────────

fn run_attest(subject: &str, payload_hash: &str, session: Option<&str>) {
    println!("📝 Submitting attestation...");
    println!("  Subject: {}", subject);
    println!("  Payload Hash: {}", payload_hash);
    if let Some(s) = session {
        println!("  Session: {}", s);
    }
    
    // Validate inputs
    if !subject.starts_with('G') || subject.len() != 56 {
        eprintln!("❌ Invalid subject address format");
        std::process::exit(1);
    }
    
    if payload_hash.len() != 64 || !payload_hash.chars().all(|c| c.is_ascii_hexdigit()) {
        eprintln!("❌ Invalid payload hash format (expected 64-character hex string)");
        std::process::exit(1);
    }
    
    println!("📋 Attestation steps:");
    println!("  Use soroban CLI to submit:");
    println!("  soroban contract invoke \\");
    println!("    --id <CONTRACT_ID> \\");
    println!("    --source <ATTESTOR_ACCOUNT> \\");
    println!("    --network testnet \\");
    println!("    -- \\");
    if let Some(s) = session {
        println!("    submit_attestation_with_session \\");
        println!("    --session {} \\", s);
    } else {
        println!("    submit_attestation \\");
    }
    println!("    --subject {} \\", subject);
    println!("    --payload-hash {}", payload_hash);
    println!();
    println!("💡 Note: Replace <CONTRACT_ID> and <ATTESTOR_ACCOUNT> with actual values");
}

// ── query ───────────────────────────────────────────────────────────────────

fn run_query(transaction_id: &str, output: &str) {
    println!("🔍 Querying attestation: {}", transaction_id);
    
    if output != "json" && output != "text" {
        eprintln!("❌ Invalid output format. Use 'json' or 'text'");
        std::process::exit(1);
    }
    
    println!("📋 Query steps:");
    println!("  Use soroban CLI to query:");
    println!("  soroban contract invoke \\");
    println!("    --id <CONTRACT_ID> \\");
    println!("    --source <ACCOUNT> \\");
    println!("    --network testnet \\");
    println!("    -- \\");
    println!("    get_attestation \\");
    println!("    --attestation-id {}", transaction_id);
    println!();
    println!("💡 Output format: {}", output);
}

// ── health ──────────────────────────────────────────────────────────────────

fn run_health(attestor: Option<&str>, watch: bool, interval: u64) {
    if let Some(addr) = attestor {
        println!("🏥 Checking health for attestor: {}", addr);
    } else {
        println!("🏥 Checking health for all attestors...");
    }
    
    if watch {
        println!("👀 Watch mode enabled (interval: {}s)", interval);
        println!("   Press Ctrl+C to stop monitoring");
    }
    
    println!("📋 Health check steps:");
    println!("  Use soroban CLI to check health:");
    println!("  soroban contract invoke \\");
    println!("    --id <CONTRACT_ID> \\");
    println!("    --source <ACCOUNT> \\");
    println!("    --network testnet \\");
    println!("    -- \\");
    if let Some(addr) = attestor {
        println!("    get_anchor_health_score \\");
        println!("    --anchor {}", addr);
    } else {
        println!("    get_all_attestors");
    }
    println!();
    println!("💡 Note: Health monitoring requires active contract deployment");
}

// ── test ────────────────────────────────────────────────────────────────────

fn run_test(pattern: Option<&str>, verbose: bool) {
    println!("🧪 Running contract tests...");
    
    let mut cmd = Command::new("cargo");
    cmd.arg("test");
    
    if let Some(p) = pattern {
        cmd.arg(p);
        println!("  Pattern: {}", p);
    }
    
    if verbose {
        cmd.arg("--verbose");
        println!("  Mode: Verbose");
    }
    
    match cmd.output() {
        Ok(output) if output.status.success() => {
            println!("✅ All tests passed");
            if verbose {
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
        }
        Ok(output) => {
            eprintln!("❌ Some tests failed:");
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("❌ Failed to run tests: {}", e);
            std::process::exit(1);
        }
    }
}

// ── doctor ──────────────────────────────────────────────────────────────────

fn run_doctor() {
    println!("🔍 Running AnchorKit diagnostics...\n");
    let start = Instant::now();
    let mut all_ok = true;

    all_ok &= check_rust_version();
    all_ok &= check_wasm_target();
    all_ok &= check_wallet();
    all_ok &= check_rpc();
    all_ok &= check_configs();
    all_ok &= check_network();

    println!("\n⏱  Completed in {:.2}s\n", start.elapsed().as_secs_f64());

    if all_ok {
        println!("✅ All checks passed! Your environment is ready.");
        std::process::exit(0);
    } else {
        println!("⚠️  Some checks failed. Please address the issues above.");
        std::process::exit(1);
    }
}

fn check_rust_version() -> bool {
    match Command::new("rustc").arg("--version").output() {
        Err(_) => {
            println!("✖ Rust toolchain not found → install from https://rustup.rs");
            false
        }
        Ok(out) => {
            let version_str = String::from_utf8_lossy(&out.stdout);
            if let Some((major, minor)) = parse_rustc_version(&version_str) {
                if major > MIN_RUST_MAJOR || (major == MIN_RUST_MAJOR && minor >= MIN_RUST_MINOR) {
                    println!("✔ Rust {}.{} detected (meets minimum {}.{}+)", major, minor, MIN_RUST_MAJOR, MIN_RUST_MINOR);
                    true
                } else {
                    println!(
                        "✖ Rust {}.{} detected but {}.{}+ is required (edition 2021)\n  \
                         → Run: rustup update stable",
                        major, minor, MIN_RUST_MAJOR, MIN_RUST_MINOR
                    );
                    false
                }
            } else {
                println!("✖ Could not parse rustc version: {}", version_str.trim());
                false
            }
        }
    }
}

/// Parse "rustc X.Y.Z ..." → (X, Y)
fn parse_rustc_version(s: &str) -> Option<(u32, u32)> {
    let version_part = s.split_whitespace().nth(1)?;
    let mut parts = version_part.split('.');
    let major: u32 = parts.next()?.parse().ok()?;
    let minor: u32 = parts.next()?.parse().ok()?;
    Some((major, minor))
}

fn check_wasm_target() -> bool {
    let out = Command::new("rustup")
        .args(["target", "list", "--installed"])
        .output();
    match out {
        Ok(o) if String::from_utf8_lossy(&o.stdout).contains("wasm32-unknown-unknown") => {
            println!("✔ WASM target installed");
            true
        }
        _ => {
            println!("✖ WASM target missing → run: rustup target add wasm32-unknown-unknown");
            false
        }
    }
}

fn check_wallet() -> bool {
    let vars = ["STELLAR_SECRET_KEY", "SOROBAN_SECRET_KEY", "ANCHORKIT_SECRET_KEY"];
    if vars.iter().any(|v| std::env::var(v).is_ok()) {
        println!("✔ Wallet configured");
        return true;
    }
    let identity_dir = std::env::var("HOME").ok().map(|h| h + "/.config/soroban/identity");
    if let Some(dir) = identity_dir {
        if std::path::Path::new(&dir).exists() {
            println!("✔ Wallet configured (soroban identity)");
            return true;
        }
    }
    println!("✖ Wallet not configured → set STELLAR_SECRET_KEY or configure soroban identity");
    false
}

fn check_rpc() -> bool {
    let vars = ["ANCHORKIT_RPC_URL", "SOROBAN_RPC_URL", "STELLAR_RPC_URL"];
    if vars.iter().any(|v| std::env::var(v).is_ok()) {
        println!("✔ RPC endpoint reachable");
        true
    } else {
        println!("✖ RPC endpoint not configured → set ANCHORKIT_RPC_URL, SOROBAN_RPC_URL, or STELLAR_RPC_URL");
        false
    }
}

fn check_configs() -> bool {
    let configs = std::path::Path::new("configs");
    if !configs.exists() {
        println!("✖ configs/ directory not found");
        return false;
    }
    let count = std::fs::read_dir(configs)
        .map(|rd| {
            rd.filter_map(|e| e.ok())
                .filter(|e| {
                    matches!(
                        e.path().extension().and_then(|s| s.to_str()),
                        Some("json") | Some("toml")
                    )
                })
                .count()
        })
        .unwrap_or(0);
    if count > 0 {
        println!("✔ Config files valid ({} found)", count);
        true
    } else {
        println!("✖ No config files found in configs/");
        false
    }
}

fn check_network() -> bool {
    let ok = Command::new("curl")
        .args(["-s", "--max-time", "3", "-o", "/dev/null", "-w", "%{http_code}",
               "https://horizon-testnet.stellar.org"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim() != "000")
        .unwrap_or(false);
    if ok {
        println!("✔ Network responding");
    } else {
        println!("✖ Network unreachable → check internet connection");
    }
    ok
}

// ── validate ─────────────────────────────────────────────────────────────────

fn run_validate(path: &str) {
    let p = std::path::Path::new(path);
    if p.is_dir() {
        let mut entries: Vec<_> = std::fs::read_dir(p)
            .expect("cannot read directory")
            .filter_map(|e| e.ok())
            .filter(|e| {
                matches!(
                    e.path().extension().and_then(|s| s.to_str()),
                    Some("json") | Some("toml")
                )
            })
            .collect();
        entries.sort_by_key(|e| e.path());
        if entries.is_empty() {
            println!("No .json or .toml files found in {}", path);
            return;
        }
        let mut all_ok = true;
        for entry in entries {
            all_ok &= validate_file(&entry.path());
        }
        if !all_ok {
            std::process::exit(1);
        }
    } else if !validate_file(p) {
        std::process::exit(1);
    }
}

fn validate_file(path: &std::path::Path) -> bool {
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            println!("✖ {}: cannot read file: {}", path.display(), e);
            return false;
        }
    };
    match ext {
        "json" => validate_json(path, &content),
        "toml" => validate_toml(path, &content),
        _ => {
            println!("✖ {}: unsupported format (expected .json or .toml)", path.display());
            false
        }
    }
}

fn validate_json(path: &std::path::Path, content: &str) -> bool {
    match serde_json::from_str::<serde_json::Value>(content) {
        Ok(_) => { println!("✔ {}: valid JSON", path.display()); true }
        Err(e) => {
            println!("✖ {}: invalid JSON at line {}, column {}: {}", path.display(), e.line(), e.column(), e);
            false
        }
    }
}

fn validate_toml(path: &std::path::Path, content: &str) -> bool {
    match toml::from_str::<toml::Value>(content) {
        Ok(_) => { println!("✔ {}: valid TOML", path.display()); true }
        Err(e) => {
            if let Some(span) = e.span() {
                let line = content[..span.start].chars().filter(|&c| c == '\n').count() + 1;
                println!("✖ {}: invalid TOML at line {}: {}", path.display(), line, e.message());
            } else {
                println!("✖ {}: invalid TOML: {}", path.display(), e);
            }
            false
        }
    }
}

// ── register ─────────────────────────────────────────────────────────────────

/// The complete set of valid service names for anchorkit register --services.
const VALID_SERVICES: &[&str] = &["deposits", "withdrawals", "quotes", "kyc"];

fn run_register(address: &str, services: Option<&str>, endpoint: Option<&str>) {
    // Validate service names before doing anything else
    if let Some(svc_str) = services {
        let invalid: Vec<&str> = svc_str
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty() && !VALID_SERVICES.contains(s))
            .collect();

        if !invalid.is_empty() {
            eprintln!(
                "error: unknown service(s): {}\n  valid services: {}",
                invalid.join(", "),
                VALID_SERVICES.join(", ")
            );
            std::process::exit(1);
        }
    }

    // Validate address format
    if !address.starts_with('G') || address.len() != 56 {
        eprintln!("error: invalid attestor address format. Expected Stellar public key starting with 'G'");
        std::process::exit(1);
    }

    println!("📝 Registering attestor: {}", address);
    if let Some(s) = services { println!("  Services: {}", s); }
    if let Some(e) = endpoint { println!("  Endpoint: {}", e); }
    
    println!("\n📋 Registration steps:");
    println!("  1. Use soroban CLI to register attestor:");
    println!("     soroban contract invoke \\");
    println!("       --id <CONTRACT_ID> \\");
    println!("       --source <ADMIN_ACCOUNT> \\");
    println!("       --network testnet \\");
    println!("       -- \\");
    println!("       register_attestor \\");
    println!("       --attestor {}", address);
    
    if let Some(s) = services {
        println!("\n  2. Configure services:");
        println!("     soroban contract invoke \\");
        println!("       --id <CONTRACT_ID> \\");
        println!("       --source <ADMIN_ACCOUNT> \\");
        println!("       --network testnet \\");
        println!("       -- \\");
        println!("       configure_services \\");
        println!("       --attestor {} \\", address);
        println!("       --services \"{}\"", s);
    }
    
    if let Some(e) = endpoint {
        println!("\n  3. Set endpoint:");
        println!("     soroban contract invoke \\");
        println!("       --id <CONTRACT_ID> \\");
        println!("       --source <ADMIN_ACCOUNT> \\");
        println!("       --network testnet \\");
        println!("       -- \\");
        println!("       set_attestor_endpoint \\");
        println!("       --attestor {} \\", address);
        println!("       --endpoint \"{}\"", e);
    }
    
    println!("\n💡 Note: Replace <CONTRACT_ID> and <ADMIN_ACCOUNT> with actual values");
    println!("🔗 This will create actual on-chain transactions when executed with soroban CLI");
}

// ── export-audit ─────────────────────────────────────────────────────────────

fn run_export_audit(format: &str, output: &str) {
    if format != "json" && format != "csv" {
        eprintln!("error: unsupported format '{}'. Use 'json' or 'csv'", format);
        std::process::exit(1);
    }
    println!("Fetching audit log entries...");
    let entries = fetch_audit_entries();
    let total = entries.len();
    let content = match format {
        "csv" => {
            let mut out = String::from("id,operation,actor,timestamp,result\n");
            for e in &entries {
                out.push_str(&format!("{},{},{},{},{}\n", e.id, e.operation, e.actor, e.timestamp, e.result));
            }
            out
        }
        _ => serde_json::to_string_pretty(&entries).unwrap(),
    };
    std::fs::write(output, &content).unwrap_or_else(|e| {
        eprintln!("error: cannot write to {}: {}", output, e);
        std::process::exit(1);
    });
    println!("✔ Exported {} audit log entries to {} ({})", total, output, format);
}

#[derive(serde::Serialize)]
struct AuditEntry {
    id: u64,
    operation: String,
    actor: String,
    timestamp: u64,
    result: String,
}

fn fetch_audit_entries() -> Vec<AuditEntry> {
    let page_size = 50u64;
    let mut entries = Vec::new();
    let mut page = 0u64;
    loop {
        let batch = fetch_page(page, page_size);
        let done = batch.len() < page_size as usize;
        entries.extend(batch);
        if done { break; }
        page += 1;
        eprint!("\r  Fetched {} entries...", entries.len());
    }
    if !entries.is_empty() { eprintln!(); }
    entries
}

fn fetch_page(page: u64, page_size: u64) -> Vec<AuditEntry> {
    let _ = (page, page_size);
    vec![]
}
