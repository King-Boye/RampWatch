use std::fs;
use std::io;
use std::path::Path;

fn remove_path(path: &str, summary: &mut Vec<String>) -> io::Result<()> {
    let p = Path::new(path);
    if p.exists() {
        if p.is_dir() {
            fs::remove_dir_all(p)?;
            summary.push(format!("Removed directory: {}", path));
        } else {
            fs::remove_file(p)?;
            summary.push(format!("Removed file: {}", path));
        }
    }
    Ok(())
}

fn main() {
    let mut summary = Vec::new();
    let args: Vec<String> = std::env::args().collect();
    let full = args.iter().any(|a| a == "--full");

    // Standard clean
    let _ = remove_path(".anchorkit/cache", &mut summary);
    let _ = remove_path("target", &mut summary);
    let _ = remove_path("build", &mut summary);
    let _ = remove_path("idl", &mut summary); // Example artifact dir
    let _ = remove_path("temp", &mut summary); // Example temp dir

    // Full clean (add more aggressive removals here)
    if full {
        // Example: remove deployment history, only if --full
        let _ = remove_path("deployment_history", &mut summary);
    }

    if summary.is_empty() {
        println!("Nothing to clean.");
    } else {
        println!("Cleanup summary:");
        for line in summary {
            println!("  {}", line);
        }
    }
}
