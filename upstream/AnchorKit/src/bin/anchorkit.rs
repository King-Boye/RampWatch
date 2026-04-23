use clap::{Parser, Subcommand, ValueEnum};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{self, Read};

#[derive(Parser)]
#[command(name = "anchorkit", version, about = "AnchorKit command-line utility")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Query a transaction or anchor state
    Query(QueryArgs),
    /// Create an attestation payload hash from a file or stdin
    Attest(AttestArgs),
}

#[derive(Parser)]
struct QueryArgs {
    /// Output format for query results
    #[arg(long, value_enum, default_value_t = OutputFormat::Table)]
    output: OutputFormat,

    /// Transaction ID or anchor id to query
    #[arg(long)]
    transaction_id: Option<String>,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum OutputFormat {
    Table,
    Json,
}

#[derive(Parser)]
struct AttestArgs {
    /// Attestation subject identifier
    #[arg(long)]
    subject: String,

    /// Payload hash in hex, or '-' to compute from stdin
    #[arg(long)]
    payload_hash: Option<String>,

    /// Compute payload hash from a file
    #[arg(long, value_name = "FILE")]
    payload_file: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Query(args) => run_query(args),
        Commands::Attest(args) => run_attest(args),
    }
}

fn run_query(args: QueryArgs) -> Result<(), Box<dyn std::error::Error>> {
    let transaction_id = args.transaction_id.unwrap_or_else(|| "TX123456789".to_string());
    let kind = "deposit";
    let status = "pending";
    let amount_in = 1000;
    let amount_out = 990;
    let fee = 10;
    let message = "Awaiting anchor confirmation";

    match args.output {
        OutputFormat::Table => {
            println!("Transaction ID | Kind    | Status  | Amount In | Amount Out | Fee | Message");
            println!("---------------+---------+---------+-----------+------------+-----+---------------------------");
            println!("{:<14} | {:<7} | {:<7} | {:>9} | {:>10} | {:>3} | {}",
                transaction_id, kind, status, amount_in, amount_out, fee, message);
        }
        OutputFormat::Json => {
            println!(
                "{{\"transaction_id\":\"{}\",\"kind\":\"{}\",\"status\":\"{}\",\"amount_in\":{},\"amount_out\":{},\"fee\":{},\"message\":\"{}\"}}",
                transaction_id, kind, status, amount_in, amount_out, fee, message
            );
        }
    }

    Ok(())
}

fn run_attest(args: AttestArgs) -> Result<(), Box<dyn std::error::Error>> {
    if args.payload_hash.is_some() && args.payload_file.is_some() {
        return Err("Only one of --payload-hash or --payload-file may be supplied".into());
    }

    let (hash, source) = if let Some(file_path) = args.payload_file {
        let content = read_file_bytes(&file_path)?;
        (sha256_hex(&content), format!("file: {}", file_path))
    } else if let Some(payload_hash) = args.payload_hash {
        if payload_hash == "-" {
            let mut stdin_bytes = Vec::new();
            io::stdin().read_to_end(&mut stdin_bytes)?;
            (sha256_hex(&stdin_bytes), "stdin".to_string())
        } else {
            validate_hex_hash(&payload_hash)?;
            (payload_hash, "literal".to_string())
        }
    } else {
        return Err("Either --payload-hash or --payload-file must be provided".into());
    };

    println!("Attestation request:\n  subject: {}\n  payload hash: {}\n  source: {}", args.subject, hash, source);
    Ok(())
}

fn read_file_bytes(path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    Ok(data)
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hex::encode(hasher.finalize())
}

fn validate_hex_hash(hash: &str) -> Result<(), Box<dyn std::error::Error>> {
    if hash.len() != 64 || !hash.chars().all(|c| c.is_ascii_hexdigit()) {
        Err("payload hash must be 64 hex characters or '-'".into())
    } else {
        Ok(())
    }
}
