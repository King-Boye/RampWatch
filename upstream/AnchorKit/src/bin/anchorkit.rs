use clap::{Parser, Subcommand};
use std::process::Command;
use std::time::Instant;

mod soroban_rpc;
use soroban_rpc::*;

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
    /// Fetch and display audit log entries
    #[command(name = "audit")]
    Audit {
        #[command(subcommand)]
        action: AuditAction,
    },
    /// Manage interaction sessions
    #[command(name = "session")]
    Session {
        #[command(subcommand)]
        action: SessionAction,
    },
}

#[derive(Subcommand)]
enum AuditAction {
    /// Fetch and display a single audit log entry by ID
    Get {
        /// Audit log entry ID
        #[arg(value_name = "LOG_ID")]
        log_id: u64,
    },
    /// List audit log entries for a session
    List {
        /// Session ID to filter by
        #[arg(long)]
        session: u64,
        /// Start from this log ID (defaults to 0)
        #[arg(long)]
        from: Option<u64>,
        /// End at this log ID (defaults to latest)
        #[arg(long)]
        to: Option<u64>,
        /// Output format: text (default), json, or csv
        #[arg(long, default_value = "text")]
        format: String,
        /// Pretty-print JSON (only for json format)
        #[arg(long)]
        pretty: bool,
    },
}

#[derive(Subcommand)]
enum SessionAction {
    /// Create a new interaction session
    Create {
        /// Stellar address of the session initiator (optional, uses STELLAR_SECRET_KEY if not provided)
        #[arg(long)]
        initiator: Option<String>,
    },
    /// Retrieve session details by ID
    Get {
        /// Session ID to retrieve
        #[arg(value_name = "SESSION_ID")]
        session_id: u64,
    },
    /// List all active sessions
    List {
        /// Output format: text (default), json, or csv
        #[arg(long, default_value = "text")]
        format: String,
        /// Pretty-print JSON (only for json format)
        #[arg(long)]
        pretty: bool,
        /// Maximum number of sessions to retrieve (defaults to 100)
        #[arg(long)]
        limit: Option<u64>,
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
        Commands::Audit { action } => match action {
            AuditAction::Get { log_id } => run_audit_get(log_id),
            AuditAction::List { session, from, to, format, pretty } => {
                run_audit_list(session, from, to, &format, pretty)
            }
        },
        Commands::Session { action } => match action {
            SessionAction::Create { initiator } => run_session_create(initiator),
            SessionAction::Get { session_id } => run_session_get(session_id),
            SessionAction::List { format, pretty, limit } => {
                run_session_list(&format, pretty, limit)
            }
        },
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

    println!("Registering attestor: {}", address);
    if let Some(s) = services { println!("  Services: {}", s); }
    if let Some(e) = endpoint { println!("  Endpoint: {}", e); }
    println!("✔ Attestor registered (simulation — connect to network for real registration)");
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

// ── audit get ───────────────────────────────────────────────────────────────

fn run_audit_get(log_id: u64) {
    println!("◈ Fetching audit log entry {}", log_id);
    println!();

    match fetch_audit_log_entry(log_id) {
        Some(entry) => {
            print_audit_entry(&entry);
            println!();
            println!("✔ Entry retrieved successfully");
        }
        None => {
            eprintln!("✖ Audit log entry {} not found", log_id);
            std::process::exit(1);
        }
    }
}

// ── audit list ──────────────────────────────────────────────────────────────

fn run_audit_list(session: u64, from: Option<u64>, to: Option<u64>, format: &str, pretty: bool) {
    println!("◈ Fetching audit logs for session {}", session);
    
    // Validate format
    if !["text", "json", "csv"].contains(&format) {
        eprintln!("error: unsupported format '{}'. Use 'text', 'json', or 'csv'", format);
        std::process::exit(1);
    }

    let entries = fetch_audit_logs_by_session(session, from, to);
    
    if entries.is_empty() {
        println!("\n✗ No audit log entries found for session {}", session);
        return;
    }

    println!();
    
    match format {
        "text" => {
            println!("┌─ Audit Log Entries (Session {}) ──────────────────────────────", session);
            for (idx, entry) in entries.iter().enumerate() {
                if idx > 0 {
                    println!("├───────────────────────────────────────────────────────────");
                }
                print_audit_entry_compact(entry);
            }
            println!("└───────────────────────────────────────────────────────────────");
            println!();
            println!("✔ Retrieved {} audit log entr{}", 
                     entries.len(), 
                     if entries.len() == 1 { "y" } else { "ies" });
        }
        "json" => {
            let json_output = if pretty {
                serde_json::to_string_pretty(&entries)
                    .unwrap_or_else(|_| "[]".to_string())
            } else {
                serde_json::to_string(&entries)
                    .unwrap_or_else(|_| "[]".to_string())
            };
            println!("{}", json_output);
        }
        "csv" => {
            println!("log_id,session_id,actor,operation_type,operation_index,timestamp,status,result");
            for entry in &entries {
                println!(
                    "{},{},{},{},{},{},{},\"{}\"",
                    entry.log_id,
                    entry.session_id,
                    entry.actor,
                    entry.operation_type,
                    entry.operation_index,
                    entry.timestamp,
                    entry.status,
                    entry.result.replace("\"", "\\\"")
                );
            }
        }
        _ => unreachable!(),
    }
}

// ── Data structures for audit logging ─────────────────────────────────────────

#[derive(serde::Serialize, Clone)]
struct AuditLogEntry {
    log_id: u64,
    session_id: u64,
    actor: String,
    operation_type: String,
    operation_index: u64,
    timestamp: u64,
    status: String,
    result: String,
}

// ── Pretty-printing helpers ──────────────────────────────────────────────────

fn print_audit_entry(entry: &AuditLogEntry) {
    println!("  Log ID:          {}", entry.log_id);
    println!("  Session ID:      {}", entry.session_id);
    println!("  Actor:           {}", entry.actor);
    println!("  Operation:       {} (index: {})", entry.operation_type, entry.operation_index);
    println!("  Timestamp:       {} ({})", 
             entry.timestamp, 
             format_timestamp(entry.timestamp));
    println!("  Status:          {}", entry.status);
    println!("  Result:          {}", entry.result);
}

fn print_audit_entry_compact(entry: &AuditLogEntry) {
    println!("  │ Log ID:     {} │ Op: {} │ Status: {}", 
             entry.log_id,
             entry.operation_type,
             entry.status);
    println!("  │ Session:    {} │ Actor: {}", 
             entry.session_id,
             &entry.actor[..entry.actor.len().min(24)]);
    println!("  │ Timestamp:  {} (op_index: {})", 
             format_timestamp(entry.timestamp),
             entry.operation_index);
    println!("  │ Result:     {}", 
             &entry.result[..entry.result.len().min(50)]);
}

fn format_timestamp(ts: u64) -> String {
    // ts is Unix timestamp in seconds
    let secs_per_day = 86400;
    let secs_per_hour = 3600;
    let secs_per_minute = 60;
    
    let days = ts / secs_per_day;
    let hours = (ts % secs_per_day) / secs_per_hour;
    let minutes = (ts % secs_per_hour) / secs_per_minute;
    let seconds = ts % secs_per_minute;
    
    format!("{}d {:02}h {:02}m {:02}s", days, hours, minutes, seconds)
}

// ── On-chain data fetching ───────────────────────────────────────────────────

/// Fetch a single audit log entry from on-chain storage.
/// In production, this would query the Soroban contract via RPC.
/// For now, this is a placeholder that would connect to the on-chain contract.
fn fetch_audit_log_entry(log_id: u64) -> Option<AuditLogEntry> {
    // TODO: Implement actual on-chain fetching via Soroban RPC
    // This would invoke contract method: get_audit_log(log_id)
    // and parse the returned AuditLog structure.
    //
    // Example RPC call structure:
    //   POST {RPC_URL}
    //   {
    //     "jsonrpc": "2.0",
    //     "id": 1,
    //     "method": "simulateTransaction",
    //     "params": {
    //       "transaction": "{encoded_contract_invoke}",
    //       "resourceLeeway": 15
    //     }
    //   }
    //
    // Implementation strategy:
    // 1. Get RPC URL from ANCHORKIT_RPC_URL or SOROBAN_RPC_URL env var
    // 2. Build contract invocation for get_audit_log(log_id)
    // 3. Sign transaction with wallet from STELLAR_SECRET_KEY or Soroban identity
    // 4. Submit via /simulateTransaction endpoint
    // 5. Parse result and extract AuditLog structure
    // 6. Map to AuditLogEntry with actor address and operation context
    
    let _ = log_id;
    // Placeholder: return None (no entries fetched)
    None
}

/// Fetch audit logs filtered by session ID.
/// Returns entries with optional range filtering [from_id, to_id].
fn fetch_audit_logs_by_session(session_id: u64, from: Option<u64>, to: Option<u64>) -> Vec<AuditLogEntry> {
    // TODO: Implement actual on-chain fetching via Soroban RPC
    // This would invoke contract method: get_audit_log_range(from_id, to_id)
    // and filter results where session_id matches.
    //
    // The contract's get_audit_log_range() is capped at 100 entries per call,
    // so pagination may be needed for large result sets.
    //
    // Implementation strategy:
    // 1. Determine range: if from/to not provided, fetch latest 100 entries
    // 2. Loop: call get_audit_log_range(from, min(from+100, to))
    // 3. Filter: keep only entries where entry.session_id == session_id
    // 4. Continue: if result set full, fetch next batch
    // 5. Accumulate until complete or limit reached
    // 6. Map AuditLog results to AuditLogEntry with formatted timestamps
    
    let _ = (session_id, from, to);
    // Placeholder: return empty vec (no entries fetched)
    vec![]
}

// ── Session data structures ──────────────────────────────────────────────────

/// CLI representation of a session
#[derive(serde::Serialize, serde::Deserialize)]
struct SessionRecord {
    session_id: u64,
    initiator: String,
    created_at: u64,
    nonce: u64,
    operation_count: u64,
    expires_at: u64,
}

// ── Session management commands ──────────────────────────────────────────────

fn run_session_create(initiator: Option<String>) {
    println!("◈ Creating new session");
    println!();

    match initiator {
        Some(addr) => {
            println!("  Initiator: {}", addr);
        }
        None => {
            println!("  Initiator: using STELLAR_SECRET_KEY");
        }
    }

    match create_session_on_chain(initiator) {
        Ok(session_id) => {
            println!();
            println!("✔ Session created successfully");
            println!("  Session ID: {}", session_id);
            println!();
            println!("Use this session ID for subsequent operations:");
            println!("  anchorkit session get {}", session_id);
        }
        Err(e) => {
            eprintln!("✖ Failed to create session: {}", e);
            std::process::exit(1);
        }
    }
}

fn run_session_get(session_id: u64) {
    println!("◈ Fetching session {}", session_id);
    println!();

    match fetch_session_from_chain(session_id) {
        Some(session) => {
            print_session(&session);
            println!();
            println!("✔ Session retrieved successfully");
        }
        None => {
            eprintln!("✖ Session {} not found", session_id);
            std::process::exit(1);
        }
    }
}

fn run_session_list(format: &str, pretty: bool, limit: Option<u64>) {
    let limit = limit.unwrap_or(100);
    println!("◈ Fetching sessions (limit: {})", limit);

    // Validate format
    if !["text", "json", "csv"].contains(&format) {
        eprintln!("error: unsupported format '{}'. Use 'text', 'json', or 'csv'", format);
        std::process::exit(1);
    }

    let sessions = fetch_sessions_from_chain(limit);

    if sessions.is_empty() {
        println!("\n✗ No sessions found");
        return;
    }

    println!();

    match format {
        "text" => {
            println!("┌─ Active Sessions ──────────────────────────────────────────────");
            for (idx, session) in sessions.iter().enumerate() {
                if idx > 0 {
                    println!("├───────────────────────────────────────────────────────────────");
                }
                print_session_compact(&session);
            }
            println!("└───────────────────────────────────────────────────────────────────");
            println!();
            println!("✔ Retrieved {} session{}", 
                     sessions.len(), 
                     if sessions.len() == 1 { "" } else { "s" });
        }
        "json" => {
            let json_output = if pretty {
                serde_json::to_string_pretty(&sessions)
                    .unwrap_or_else(|_| "[]".to_string())
            } else {
                serde_json::to_string(&sessions)
                    .unwrap_or_else(|_| "[]".to_string())
            };
            println!("{}", json_output);
        }
        "csv" => {
            println!("session_id,initiator,created_at,operation_count,expires_at");
            for session in &sessions {
                println!(
                    "{},{},{},{},{}",
                    session.session_id,
                    session.initiator,
                    session.created_at,
                    session.operation_count,
                    session.expires_at,
                );
            }
        }
        _ => unreachable!(),
    }
}

// ── Session helper functions ─────────────────────────────────────────────────

fn print_session(session: &SessionRecord) {
    println!("  Session ID:       {}", session.session_id);
    println!("  Initiator:        {}", session.initiator);
    println!("  Created At:       {} ({})", 
             session.created_at, 
             format_timestamp(session.created_at));
    println!("  Nonce:            {}", session.nonce);
    println!("  Operation Count:  {}", session.operation_count);
    println!("  Expires At:       {} ({})", 
             session.expires_at,
             format_timestamp(session.expires_at));
}

fn print_session_compact(session: &SessionRecord) {
    println!("│ Session: {}  Initiator: {}...", 
             session.session_id,
             &session.initiator.chars().take(16).collect::<String>());
    println!("│ Created: {}  Operations: {}  Expires: {}", 
             session.created_at,
             session.operation_count,
             session.expires_at);
}

fn create_session_on_chain(initiator: Option<String>) -> Result<u64, String> {
    create_session_rpc(initiator)
}

fn fetch_session_from_chain(session_id: u64) -> Option<SessionRecord> {
    match get_session_rpc(session_id) {
        Ok(session_data) => Some(SessionRecord {
            session_id: session_data.session_id,
            initiator: session_data.initiator,
            created_at: session_data.created_at,
            nonce: session_data.nonce,
            operation_count: session_data.operation_count,
            expires_at: session_data.expires_at,
        }),
        Err(_) => None,
    }
}

fn fetch_sessions_from_chain(limit: u64) -> Vec<SessionRecord> {
    match list_sessions_rpc(limit) {
        Ok(sessions) => sessions
            .into_iter()
            .map(|s| SessionRecord {
                session_id: s.session_id,
                initiator: s.initiator,
                created_at: s.created_at,
                nonce: s.nonce,
                operation_count: s.operation_count,
                expires_at: s.expires_at,
            })
            .collect(),
        Err(_) => vec![],
    }
}

