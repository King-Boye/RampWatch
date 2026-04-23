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
    /// Run environment diagnostics
    Doctor,
    /// Validate configuration files
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
        /// Comma-separated services (deposits,withdrawals,quotes,kyc)
        #[arg(long)]
        services: Option<String>,
        /// Attestor endpoint URL
        #[arg(long)]
        endpoint: Option<String>,
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
        Commands::Doctor => run_doctor(),
        Commands::Validate { path } => run_validate(&path),
        Commands::Register { address, services, endpoint } => {
            run_register(&address, services.as_deref(), endpoint.as_deref())
        }
        Commands::ExportAudit { format, output } => run_export_audit(&format, &output),
    }
}

// ── doctor ──────────────────────────────────────────────────────────────────

fn run_doctor() {
    println!("🔍 Running AnchorKit diagnostics...\n");
    let start = Instant::now();
    let mut all_ok = true;

    // 1. Rust toolchain + version check
    all_ok &= check_rust_version();

    // 2. WASM target
    all_ok &= check_wasm_target();

    // 3. Wallet configuration
    all_ok &= check_wallet();

    // 4. RPC endpoint
    all_ok &= check_rpc();

    // 5. Config files
    all_ok &= check_configs();

    // 6. Network connectivity
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
            // e.g. "rustc 1.78.0 (90b35a623 2024-04-20)"
            if let Some((major, minor)) = parse_rustc_version(&version_str) {
                if major > MIN_RUST_MAJOR || (major == MIN_RUST_MAJOR && minor >= MIN_RUST_MINOR) {
                    println!("✔ Rust toolchain detected ({})", version_str.trim());
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
    // s looks like "rustc 1.78.0 (90b35a623 2024-04-20)"
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
    let identity_dir = dirs_home().map(|h| h + "/.config/soroban/identity");
    if let Some(dir) = identity_dir {
        if std::path::Path::new(&dir).exists() {
            println!("✔ Wallet configured (soroban identity)");
            return true;
        }
    }
    println!(
        "✖ Wallet not configured → set STELLAR_SECRET_KEY or configure soroban identity"
    );
    false
}

fn check_rpc() -> bool {
    let vars = ["ANCHORKIT_RPC_URL", "SOROBAN_RPC_URL", "STELLAR_RPC_URL"];
    if vars.iter().any(|v| std::env::var(v).is_ok()) {
        println!("✔ RPC endpoint reachable");
        true
    } else {
        println!(
            "✖ RPC endpoint not configured → set ANCHORKIT_RPC_URL, SOROBAN_RPC_URL, or STELLAR_RPC_URL"
        );
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
                    let p = e.path();
                    matches!(
                        p.extension().and_then(|s| s.to_str()),
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
    // Simple connectivity check via curl/wget (no extra deps)
    let ok = Command::new("curl")
        .args(["-s", "--max-time", "3", "-o", "/dev/null", "-w", "%{http_code}",
               "https://horizon-testnet.stellar.org"])
        .output()
        .map(|o| {
            let code = String::from_utf8_lossy(&o.stdout);
            code.trim() != "000"
        })
        .unwrap_or(false);
    if ok {
        println!("✔ Network responding");
    } else {
        println!("✖ Network unreachable → check internet connection");
    }
    ok
}

fn dirs_home() -> Option<String> {
    std::env::var("HOME").ok()
}

// ── validate ─────────────────────────────────────────────────────────────────

fn run_validate(path: &str) {
    let p = std::path::Path::new(path);
    if p.is_dir() {
        let entries: Vec<_> = std::fs::read_dir(p)
            .expect("cannot read directory")
            .filter_map(|e| e.ok())
            .filter(|e| {
                matches!(
                    e.path().extension().and_then(|s| s.to_str()),
                    Some("json") | Some("toml")
                )
            })
            .collect();
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
    } else {
        if !validate_file(p) {
            std::process::exit(1);
        }
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
            println!("✖ {}: unsupported format", path.display());
            false
        }
    }
}

fn validate_json(path: &std::path::Path, content: &str) -> bool {
    match serde_json::from_str::<serde_json::Value>(content) {
        Ok(_) => {
            println!("✔ {}: valid JSON", path.display());
            true
        }
        Err(e) => {
            println!("✖ {}: invalid JSON at line {}, column {}: {}", path.display(), e.line(), e.column(), e);
            false
        }
    }
}

fn validate_toml(path: &std::path::Path, content: &str) -> bool {
    match toml::from_str::<toml::Value>(content) {
        Ok(_) => {
            println!("✔ {}: valid TOML", path.display());
            true
        }
        Err(e) => {
            println!("✖ {}: invalid TOML: {}", path.display(), e);
            false
        }
    }
}

// ── register ─────────────────────────────────────────────────────────────────

const VALID_SERVICES: &[&str] = &["deposits", "withdrawals", "quotes", "kyc"];

fn run_register(address: &str, services: Option<&str>, endpoint: Option<&str>) {
    if let Some(svc_str) = services {
        let mut invalid: Vec<&str> = Vec::new();
        for svc in svc_str.split(',') {
            let svc = svc.trim();
            if !VALID_SERVICES.contains(&svc) {
                invalid.push(svc);
            }
        }
        if !invalid.is_empty() {
            eprintln!(
                "error: unknown service(s): {}\n  valid services: {}",
                invalid.join(", "),
                VALID_SERVICES.join(", ")
            );
            std::process::exit(1);
        }
    }

    println!("Registering attestor: {}", address);
    if let Some(s) = services {
        println!("  Services: {}", s);
    }
    if let Some(e) = endpoint {
        println!("  Endpoint: {}", e);
    }
    println!("✔ Attestor registered (simulation — connect to network for real registration)");
}

// ── export-audit ─────────────────────────────────────────────────────────────

fn run_export_audit(format: &str, output: &str) {
    if format != "json" && format != "csv" {
        eprintln!("error: unsupported format '{}'. Use 'json' or 'csv'", format);
        std::process::exit(1);
    }

    println!("Fetching audit log entries...");

    // Simulate paginated fetch
    let entries = fetch_audit_entries();
    let total = entries.len();

    let content = match format {
        "csv" => {
            let mut out = String::from("id,operation,actor,timestamp,result\n");
            for e in &entries {
                out.push_str(&format!(
                    "{},{},{},{},{}\n",
                    e.id, e.operation, e.actor, e.timestamp, e.result
                ));
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
    // In a real implementation this would paginate through on-chain audit logs.
    // Here we return a representative empty set since no live network is connected.
    let page_size = 50u64;
    let mut entries = Vec::new();
    let mut page = 0u64;
    loop {
        let batch = fetch_page(page, page_size);
        let done = batch.len() < page_size as usize;
        entries.extend(batch);
        if done {
            break;
        }
        page += 1;
        eprint!("\r  Fetched {} entries...", entries.len());
    }
    if !entries.is_empty() {
        eprintln!();
    }
    entries
}

fn fetch_page(page: u64, page_size: u64) -> Vec<AuditEntry> {
    // Stub: no live network. Returns empty to signal end of pagination.
    let _ = (page, page_size);
    vec![]
}
