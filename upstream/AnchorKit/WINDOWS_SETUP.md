# Windows Setup Guide for AnchorKit

This guide provides Windows-specific instructions for setting up and using AnchorKit.

## Prerequisites

### Required Software

1. **Rust** (1.70 or later)
   - Download from: https://rustup.rs/
   - Run the installer and follow the prompts
   - Restart your terminal after installation
   - Verify: `rustc --version`

2. **Python 3.7+**
   - Download from: https://www.python.org/downloads/
   - ✅ Check "Add Python to PATH" during installation
   - Verify: `python --version`

3. **Git for Windows** (optional but recommended)
   - Download from: https://git-scm.com/download/win
   - Includes Git Bash for running Unix scripts

### Python Dependencies

```powershell
pip install jsonschema toml
```

## Building the Project

### Using PowerShell (Recommended)

```powershell
# Clone the repository
git clone <repository-url>
cd anchorkit

# Build the project
cargo build --release

# Run tests
cargo test

# Build for WASM target
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
```

### Using Command Prompt

```cmd
REM Clone the repository
git clone <repository-url>
cd anchorkit

REM Build the project
cargo build --release

REM Run tests
cargo test
```

## Configuration Validation

### Using PowerShell Scripts

AnchorKit provides PowerShell equivalents for all bash scripts:

```powershell
# Validate all configurations
.\validate_all.ps1

# Pre-deployment validation
.\pre_deploy_validate.ps1
```

### Using Python Scripts Directly

```powershell
# Validate a specific config file
python validate_config_strict.py configs\stablecoin-issuer.json config_schema.json

# Validate all configs
python validate_config.py
```

## Running Tests

### All Tests

```powershell
cargo test
```

### Specific Test Suites

```powershell
# Configuration tests
cargo test config

# Cross-platform path tests
cargo test cross_platform

# Validation tests
cargo test validation
```

### Python Validation Tests

```powershell
python test_config_validation.py --test
```

## Common Issues and Solutions

### Issue: Python not found

**Solution:**
- Ensure Python is installed and added to PATH
- Restart your terminal after installation
- Try using `py` instead of `python`:
  ```powershell
  py --version
  py -m pip install jsonschema toml
  ```

### Issue: Cargo not found

**Solution:**
- Install Rust from https://rustup.rs/
- Restart your terminal
- Verify installation: `cargo --version`

### Issue: WASM target not found

**Solution:**
```powershell
rustup target add wasm32-unknown-unknown
```

### Issue: Permission denied when running PowerShell scripts

**Solution:**
```powershell
# Check execution policy
Get-ExecutionPolicy

# Set execution policy (run as Administrator)
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser

# Or run scripts with bypass
PowerShell -ExecutionPolicy Bypass -File .\validate_all.ps1
```

### Issue: Line ending issues with Git

**Solution:**
```powershell
# Configure Git to handle line endings correctly
git config --global core.autocrlf true
```

## Path Handling Notes

AnchorKit uses platform-agnostic path handling:

- ✅ **Rust code**: Uses `std::path::Path` and `PathBuf` (works on all platforms)
- ✅ **Python scripts**: Uses `pathlib.Path` (works on all platforms)
- ⚠️ **Bash scripts**: Unix-only (use PowerShell equivalents on Windows)

### Example: Cross-Platform Path Construction

**Correct (works everywhere):**
```rust
use std::path::Path;

let config_path = Path::new("configs").join("config.json");
```

**Incorrect (Unix-only):**
```rust
let config_path = "configs/config.json";  // ❌ Won't work on Windows
```

## Development Workflow

### 1. Make Changes

Edit source files using your preferred editor:
- Visual Studio Code (recommended)
- IntelliJ IDEA with Rust plugin
- Notepad++
- Any text editor

### 2. Validate Changes

```powershell
# Run tests
cargo test

# Validate configurations
.\validate_all.ps1

# Check formatting
cargo fmt --check

# Run linter
cargo clippy
```

### 3. Build

```powershell
# Debug build
cargo build

# Release build
cargo build --release

# WASM build
cargo build --release --target wasm32-unknown-unknown
```

## IDE Setup

### Visual Studio Code

1. Install extensions:
   - rust-analyzer
   - CodeLLDB (for debugging)
   - Even Better TOML

2. Configure settings (`.vscode/settings.json`):
```json
{
    "rust-analyzer.checkOnSave.command": "clippy",
    "files.eol": "\n",
    "files.insertFinalNewline": true
}
```

### IntelliJ IDEA / CLion

1. Install Rust plugin
2. Configure Rust toolchain in settings
3. Enable Cargo integration

## Deployment

### Pre-Deployment Checklist

```powershell
# 1. Validate all configurations
.\pre_deploy_validate.ps1

# 2. Run full test suite
cargo test --release

# 3. Build WASM contract
cargo build --release --target wasm32-unknown-unknown

# 4. Verify build artifacts
dir target\wasm32-unknown-unknown\release\*.wasm
```

### Using the CLI

```powershell
# Build contract
cargo run -- build --release

# Deploy to testnet
cargo run -- deploy --network testnet

# Initialize contract
cargo run -- init --admin GADMIN123... --network testnet

# Register attestor
cargo run -- register --address GANCHOR123... --network testnet
```

## Troubleshooting

### Enable Verbose Logging

```powershell
$env:RUST_LOG="debug"
cargo test
```

### Clean Build

```powershell
cargo clean
cargo build --release
```

### Check Dependencies

```powershell
cargo tree
```

### Update Dependencies

```powershell
cargo update
```

## Additional Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Documentation](https://doc.rust-lang.org/cargo/)
- [Soroban Documentation](https://soroban.stellar.org/docs)
- [PowerShell Documentation](https://docs.microsoft.com/en-us/powershell/)

## Getting Help

If you encounter issues:

1. Check this guide for common solutions
2. Review the main README.md
3. Check existing GitHub issues
4. Open a new issue with:
   - Windows version
   - PowerShell version (`$PSVersionTable`)
   - Rust version (`rustc --version`)
   - Error messages and logs
