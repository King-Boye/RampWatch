# Cross-Platform Migration Guide

This guide helps developers ensure their code contributions maintain cross-platform compatibility.

## Overview

AnchorKit is designed to work on Linux, macOS, and Windows. This guide outlines best practices for maintaining this compatibility.

## Path Handling Best Practices

### ✅ DO: Use Platform-Agnostic APIs

#### Rust
```rust
use std::path::{Path, PathBuf};

// ✅ CORRECT: Platform-agnostic
let config_path = Path::new("configs").join("config.json");
let mut path = PathBuf::from("test_snapshots");
path.push("capability_detection_tests");
path.push("test_file.json");

// ✅ CORRECT: Reading files
let content = std::fs::read_to_string(&config_path)?;

// ✅ CORRECT: Directory iteration
for entry in std::fs::read_dir("configs")? {
    let entry = entry?;
    let path = entry.path();
    // Process path...
}
```

#### Python
```python
from pathlib import Path

# ✅ CORRECT: Platform-agnostic
config_dir = Path(__file__).resolve().parent.joinpath("configs")
config_file = config_dir / "config.json"

# ✅ CORRECT: Reading files
content = config_file.read_text()

# ✅ CORRECT: Directory iteration
for config in config_dir.glob("*.json"):
    # Process config...
```

### ❌ DON'T: Use Hardcoded Path Separators

#### Rust
```rust
// ❌ WRONG: Unix-only
let config_path = "configs/config.json";

// ❌ WRONG: Windows-only
let config_path = "configs\\config.json";

// ❌ WRONG: String concatenation
let config_path = format!("configs/{}", filename);
```

#### Python
```python
# ❌ WRONG: Unix-only
config_path = "configs/config.json"

# ❌ WRONG: String concatenation
config_path = f"configs/{filename}"

# ❌ WRONG: os.path.join (use pathlib instead)
import os
config_path = os.path.join("configs", filename)  # Deprecated, use pathlib
```

## Script Compatibility

### Providing Both Bash and PowerShell

When creating automation scripts, provide both versions:

```
validate_all.sh       # For Linux/macOS
validate_all.ps1      # For Windows
```

### Bash Script Template

```bash
#!/bin/bash
set -e

echo "Running validation..."

# Use relative paths
CONFIG_DIR="configs"
SCHEMA_FILE="config_schema.json"

# Check if files exist
if [ ! -f "$SCHEMA_FILE" ]; then
    echo "❌ Schema file not found: $SCHEMA_FILE"
    exit 1
fi

# Process files
for config_file in "$CONFIG_DIR"/*.json "$CONFIG_DIR"/*.toml; do
    if [ -f "$config_file" ]; then
        echo "Processing $(basename "$config_file")..."
        # Process file...
    fi
done
```

### PowerShell Script Template

```powershell
$ErrorActionPreference = "Stop"

Write-Host "Running validation..." -ForegroundColor Cyan

# Use relative paths
$ConfigDir = "configs"
$SchemaFile = "config_schema.json"

# Check if files exist
if (-not (Test-Path $SchemaFile)) {
    Write-Host "❌ Schema file not found: $SchemaFile" -ForegroundColor Red
    exit 1
}

# Process files
$configFiles = Get-ChildItem -Path $ConfigDir -Include *.json,*.toml -File

foreach ($configFile in $configFiles) {
    Write-Host "Processing $($configFile.Name)..." -ForegroundColor Cyan
    # Process file...
}
```

## Testing Cross-Platform Code

### Writing Cross-Platform Tests

```rust
#[cfg(test)]
mod tests {
    use std::path::Path;

    #[test]
    fn test_path_construction() {
        let path = Path::new("configs").join("test.json");
        
        // Test that path contains expected components
        assert!(path.to_string_lossy().contains("test.json"));
        
        // Platform-specific assertions
        #[cfg(target_os = "windows")]
        assert!(path.to_string_lossy().contains("\\"));
        
        #[cfg(not(target_os = "windows"))]
        assert!(path.to_string_lossy().contains("/"));
    }

    #[test]
    fn test_file_operations() {
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("test.txt");
        
        // Write
        std::fs::write(&test_file, b"test").unwrap();
        
        // Read
        let content = std::fs::read_to_string(&test_file).unwrap();
        assert_eq!(content, "test");
        
        // Cleanup
        std::fs::remove_file(&test_file).unwrap();
    }
}
```

### Running Tests on All Platforms

Use GitHub Actions or similar CI/CD:

```yaml
jobs:
  test:
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v3
      - name: Run tests
        run: cargo test
```

## Common Pitfalls

### 1. Line Endings

**Problem**: Git may convert line endings, causing issues.

**Solution**: Configure Git properly:

```bash
# Linux/macOS
git config --global core.autocrlf input

# Windows
git config --global core.autocrlf true
```

Add `.gitattributes`:
```
* text=auto
*.rs text eol=lf
*.toml text eol=lf
*.json text eol=lf
*.md text eol=lf
*.sh text eol=lf
*.ps1 text eol=crlf
```

### 2. Case Sensitivity

**Problem**: Linux/macOS filesystems are case-sensitive, Windows is not.

**Solution**: Always use consistent casing:

```rust
// ✅ CORRECT: Consistent casing
let path = Path::new("configs").join("Config.json");

// ❌ WRONG: Inconsistent casing
let path1 = Path::new("Configs").join("config.json");  // May fail on Linux
let path2 = Path::new("configs").join("Config.json");  // Different file on Linux
```

### 3. Executable Permissions

**Problem**: Scripts need execute permissions on Unix but not Windows.

**Solution**: Document in README and use Git:

```bash
# Make scripts executable
chmod +x validate_all.sh
git add --chmod=+x validate_all.sh
```

### 4. Environment Variables

**Problem**: Different syntax on different platforms.

**Solution**: Use platform-specific syntax:

```bash
# Bash
export RUST_LOG=debug
cargo test

# PowerShell
$env:RUST_LOG="debug"
cargo test

# CMD
set RUST_LOG=debug
cargo test
```

### 5. Command Availability

**Problem**: Some commands are platform-specific.

**Solution**: Check availability or provide alternatives:

```rust
// Check if command exists
let output = std::process::Command::new("python3")
    .arg("--version")
    .output();

if output.is_err() {
    // Try alternative
    let output = std::process::Command::new("python")
        .arg("--version")
        .output();
}
```

## Documentation Requirements

When adding new features, ensure documentation covers all platforms:

### README Updates

```markdown
## Building

### Linux/macOS
\`\`\`bash
cargo build --release
\`\`\`

### Windows
\`\`\`powershell
cargo build --release
\`\`\`
```

### Code Comments

```rust
/// Loads configuration from the configs directory.
/// 
/// # Platform Notes
/// 
/// This function uses platform-agnostic path handling and works on:
/// - Linux
/// - macOS  
/// - Windows
/// 
/// # Examples
/// 
/// ```rust
/// let config = load_config("config.json")?;
/// ```
pub fn load_config(filename: &str) -> Result<Config, Error> {
    let config_path = Path::new("configs").join(filename);
    // ...
}
```

## Checklist for Pull Requests

Before submitting a PR, verify:

- [ ] All path operations use `Path`/`PathBuf` (Rust) or `pathlib.Path` (Python)
- [ ] No hardcoded path separators (`/` or `\`)
- [ ] Scripts provided in both bash and PowerShell versions
- [ ] Tests pass on Linux, macOS, and Windows
- [ ] Documentation updated for all platforms
- [ ] Line endings configured correctly in `.gitattributes`
- [ ] Case-sensitive file naming used consistently
- [ ] CI/CD tests on all platforms

## Testing Locally

### On Linux/macOS

```bash
# Run all tests
cargo test

# Run cross-platform tests specifically
cargo test cross_platform

# Validate configurations
./validate_all.sh
```

### On Windows

```powershell
# Run all tests
cargo test

# Run cross-platform tests specifically
cargo test cross_platform

# Validate configurations
.\validate_all.ps1
```

### Using Docker (for Linux testing on Windows/macOS)

```bash
# Build Docker image
docker build -t anchorkit-test .

# Run tests in container
docker run --rm anchorkit-test cargo test
```

## Resources

- [Rust std::path documentation](https://doc.rust-lang.org/std/path/)
- [Python pathlib documentation](https://docs.python.org/3/library/pathlib.html)
- [GitHub Actions matrix builds](https://docs.github.com/en/actions/using-jobs/using-a-matrix-for-your-jobs)
- [Git attributes documentation](https://git-scm.com/docs/gitattributes)

## Getting Help

If you're unsure about cross-platform compatibility:

1. Check existing code for patterns
2. Run tests on multiple platforms
3. Ask in PR review
4. Consult this guide

## Examples from Codebase

### Good Examples

1. **build.rs** - Uses `Path::new()` throughout
2. **src/bin/clean.rs** - Platform-agnostic file operations
3. **validate_config.py** - Uses `pathlib.Path`

### Files to Reference

- `src/cross_platform_tests.rs` - Comprehensive test examples
- `build.rs` - Build script with proper path handling
- `validate_config.py` - Python script with pathlib

## Conclusion

Following these guidelines ensures AnchorKit remains accessible to developers on all platforms. When in doubt, use the platform-agnostic APIs provided by Rust and Python standard libraries.
