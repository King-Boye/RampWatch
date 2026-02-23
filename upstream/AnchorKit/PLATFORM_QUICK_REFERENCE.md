# Platform Quick Reference

Quick reference for running AnchorKit commands on different platforms.

## Build Commands

| Task | Linux/macOS | Windows |
|------|-------------|---------|
| Build | `cargo build` | `cargo build` |
| Build Release | `cargo build --release` | `cargo build --release` |
| Build WASM | `cargo build --target wasm32-unknown-unknown` | `cargo build --target wasm32-unknown-unknown` |
| Clean | `cargo clean` | `cargo clean` |

## Test Commands

| Task | Linux/macOS | Windows |
|------|-------------|---------|
| All Tests | `cargo test` | `cargo test` |
| Specific Test | `cargo test test_name` | `cargo test test_name` |
| Cross-Platform Tests | `cargo test cross_platform` | `cargo test cross_platform` |
| Verbose | `cargo test -- --nocapture` | `cargo test -- --nocapture` |

## Validation Scripts

| Task | Linux/macOS | Windows |
|------|-------------|---------|
| Validate All | `./validate_all.sh` | `.\validate_all.ps1` |
| Pre-Deploy | `./pre_deploy_validate.sh` | `.\pre_deploy_validate.ps1` |
| Python Validator | `python3 validate_config.py` | `python validate_config.py` |

## Python Commands

| Task | Linux/macOS | Windows |
|------|-------------|---------|
| Run Python | `python3 script.py` | `python script.py` |
| Install Packages | `pip3 install package` | `pip install package` |
| Virtual Env | `python3 -m venv venv` | `python -m venv venv` |
| Activate Venv | `source venv/bin/activate` | `.\venv\Scripts\Activate.ps1` |

## File Paths

| Task | Linux/macOS | Windows |
|------|-------------|---------|
| Config Dir | `configs/` | `configs\` |
| Separator | `/` | `\` |
| Home Dir | `~` or `$HOME` | `$env:USERPROFILE` |
| Temp Dir | `/tmp` | `$env:TEMP` |

## Environment Variables

| Task | Linux/macOS (Bash) | Windows (PowerShell) |
|------|-------------------|---------------------|
| Set Variable | `export VAR=value` | `$env:VAR="value"` |
| Use Variable | `$VAR` | `$env:VAR` |
| Rust Log | `export RUST_LOG=debug` | `$env:RUST_LOG="debug"` |
| Path Separator | `:` | `;` |

## Common Commands

| Task | Linux/macOS | Windows |
|------|-------------|---------|
| List Files | `ls -la` | `Get-ChildItem` or `dir` |
| Copy File | `cp src dst` | `Copy-Item src dst` |
| Move File | `mv src dst` | `Move-Item src dst` |
| Delete File | `rm file` | `Remove-Item file` |
| Make Dir | `mkdir dir` | `New-Item -ItemType Directory dir` |
| Current Dir | `pwd` | `Get-Location` or `pwd` |
| Change Dir | `cd dir` | `Set-Location dir` or `cd dir` |

## Git Commands (Same on All Platforms)

| Task | Command |
|------|---------|
| Clone | `git clone <url>` |
| Status | `git status` |
| Add | `git add .` |
| Commit | `git commit -m "message"` |
| Push | `git push` |
| Pull | `git pull` |

## Cargo Commands (Same on All Platforms)

| Task | Command |
|------|---------|
| New Project | `cargo new project` |
| Build | `cargo build` |
| Run | `cargo run` |
| Test | `cargo test` |
| Check | `cargo check` |
| Format | `cargo fmt` |
| Lint | `cargo clippy` |
| Update | `cargo update` |
| Tree | `cargo tree` |

## Path Handling in Code

### Rust (Platform-Agnostic)

```rust
use std::path::{Path, PathBuf};

// ✅ CORRECT - Works everywhere
let path = Path::new("configs").join("config.json");
let mut buf = PathBuf::from("test");
buf.push("file.txt");

// ❌ WRONG - Platform-specific
let path = "configs/config.json";  // Unix only
let path = "configs\\config.json"; // Windows only
```

### Python (Platform-Agnostic)

```python
from pathlib import Path

# ✅ CORRECT - Works everywhere
path = Path("configs") / "config.json"
path = Path("configs").joinpath("config.json")

# ❌ WRONG - Platform-specific
path = "configs/config.json"  # Unix only
path = "configs\\config.json" # Windows only
```

## Script Execution

| Task | Linux/macOS | Windows |
|------|-------------|---------|
| Bash Script | `./script.sh` | N/A (use Git Bash) |
| PowerShell | N/A | `.\script.ps1` |
| Python | `python3 script.py` | `python script.py` |
| Make Executable | `chmod +x script.sh` | N/A |

## Troubleshooting

### Python Not Found

| Platform | Solution |
|----------|----------|
| Linux/macOS | Install: `sudo apt install python3` or `brew install python3` |
| Windows | Download from python.org, ensure "Add to PATH" is checked |

### Cargo Not Found

| Platform | Solution |
|----------|----------|
| All | Install from https://rustup.rs/ and restart terminal |

### Permission Denied (Scripts)

| Platform | Solution |
|----------|----------|
| Linux/macOS | `chmod +x script.sh` |
| Windows | `Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser` |

### Line Ending Issues

| Platform | Solution |
|----------|----------|
| Linux/macOS | `git config --global core.autocrlf input` |
| Windows | `git config --global core.autocrlf true` |

## Quick Setup

### Linux/macOS

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Python dependencies
pip3 install jsonschema toml

# Clone and build
git clone <repo>
cd anchorkit
cargo build --release
```

### Windows

```powershell
# Install Rust (download from https://rustup.rs/)

# Install Python dependencies
pip install jsonschema toml

# Clone and build
git clone <repo>
cd anchorkit
cargo build --release
```

## Documentation Links

- **Windows Setup**: See `WINDOWS_SETUP.md`
- **Migration Guide**: See `CROSS_PLATFORM_MIGRATION.md`
- **Main README**: See `README.md`
- **Audit Report**: See `CROSS_PLATFORM_AUDIT.md`

## Support

- Linux/macOS: Standard bash/zsh terminal
- Windows: PowerShell 5.1+ or PowerShell Core 7+
- All: Git Bash (Windows), Terminal (macOS), GNOME Terminal (Linux)

## Notes

1. All `cargo` commands work identically on all platforms
2. Path separators are handled automatically by Rust and Python
3. Use platform-specific scripts (`.sh` vs `.ps1`) for automation
4. Git commands are identical across platforms
5. Line endings are managed by `.gitattributes`
