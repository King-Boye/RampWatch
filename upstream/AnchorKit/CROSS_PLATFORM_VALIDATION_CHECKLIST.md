# Cross-Platform Validation Checklist

Use this checklist to verify cross-platform compatibility before releasing changes.

## Pre-Commit Checklist

### Code Review

- [ ] All path operations use `Path`/`PathBuf` (Rust) or `pathlib.Path` (Python)
- [ ] No hardcoded path separators (`/` or `\`) in code
- [ ] No platform-specific commands without alternatives
- [ ] File operations use standard library functions
- [ ] No assumptions about case sensitivity

### Path Handling

- [ ] All file paths constructed using `Path::join()` or equivalent
- [ ] No string concatenation for paths
- [ ] No hardcoded absolute paths
- [ ] Relative paths used where appropriate
- [ ] Path separators not hardcoded in strings

### Scripts

- [ ] Bash scripts provided for Linux/macOS (`.sh`)
- [ ] PowerShell scripts provided for Windows (`.ps1`)
- [ ] Both script versions have equivalent functionality
- [ ] Scripts use relative paths
- [ ] Scripts check for required dependencies

### Documentation

- [ ] README updated with platform-specific instructions
- [ ] Code comments mention platform compatibility
- [ ] Examples provided for all platforms
- [ ] Troubleshooting section includes platform-specific issues
- [ ] Setup instructions clear for each platform

## Testing Checklist

### Local Testing

#### Linux/macOS

- [ ] `cargo build` succeeds
- [ ] `cargo test` passes all tests
- [ ] `cargo test cross_platform` passes
- [ ] `./validate_all.sh` succeeds
- [ ] `./pre_deploy_validate.sh` succeeds
- [ ] Python scripts run without errors
- [ ] Configuration files load correctly

#### Windows

- [ ] `cargo build` succeeds
- [ ] `cargo test` passes all tests
- [ ] `cargo test cross_platform` passes
- [ ] `.\validate_all.ps1` succeeds
- [ ] `.\pre_deploy_validate.ps1` succeeds
- [ ] Python scripts run without errors
- [ ] Configuration files load correctly

### Cross-Platform Tests

- [ ] Path construction tests pass on all platforms
- [ ] File I/O tests pass on all platforms
- [ ] Directory operations work on all platforms
- [ ] Configuration loading works on all platforms
- [ ] Temporary file handling works correctly

### CI/CD Validation

- [ ] GitHub Actions workflow configured
- [ ] Tests run on Ubuntu, Windows, and macOS
- [ ] All matrix builds succeed
- [ ] WASM target builds on all platforms
- [ ] Configuration validation passes on all platforms

## File System Checklist

### File Naming

- [ ] No special characters in filenames
- [ ] Consistent case usage (prefer lowercase)
- [ ] No spaces in critical filenames
- [ ] Extensions are standard (.rs, .toml, .json, .md)
- [ ] No platform-specific naming conventions

### Directory Structure

- [ ] Relative paths used throughout
- [ ] No deep nesting (keep under 5 levels)
- [ ] Directory names are lowercase with hyphens
- [ ] No platform-specific directories
- [ ] Clear, descriptive directory names

### Line Endings

- [ ] `.gitattributes` file present
- [ ] Text files use LF (Unix) line endings
- [ ] PowerShell scripts use CRLF (Windows) line endings
- [ ] Binary files marked as binary
- [ ] Git configured correctly for line endings

## Script Checklist

### Bash Scripts (Linux/macOS)

- [ ] Shebang present (`#!/bin/bash`)
- [ ] `set -e` for error handling
- [ ] Relative paths used
- [ ] Dependencies checked before use
- [ ] Error messages are clear
- [ ] Exit codes are meaningful
- [ ] Executable permission set (`chmod +x`)

### PowerShell Scripts (Windows)

- [ ] `$ErrorActionPreference = "Stop"` set
- [ ] Relative paths used
- [ ] Dependencies checked before use
- [ ] Error messages are clear
- [ ] Exit codes are meaningful
- [ ] Color coding for output (optional)
- [ ] Execution policy documented

### Python Scripts

- [ ] Shebang present (`#!/usr/bin/env python3`)
- [ ] `pathlib.Path` used for all paths
- [ ] No `os.path` usage (deprecated)
- [ ] Works with Python 3.7+
- [ ] Dependencies listed in requirements
- [ ] Error handling present
- [ ] Cross-platform compatible

## Build Checklist

### Rust Build

- [ ] `cargo build` succeeds on all platforms
- [ ] `cargo build --release` succeeds
- [ ] `cargo build --target wasm32-unknown-unknown` succeeds
- [ ] No platform-specific dependencies
- [ ] All features compile on all platforms
- [ ] No warnings on any platform

### Dependencies

- [ ] All dependencies are cross-platform
- [ ] No platform-specific crates without alternatives
- [ ] Dependency versions are compatible
- [ ] `Cargo.lock` is committed
- [ ] No conflicting dependencies

## Documentation Checklist

### README.md

- [ ] Platform support section present
- [ ] Build instructions for all platforms
- [ ] Test instructions for all platforms
- [ ] Links to platform-specific guides
- [ ] Troubleshooting section includes all platforms

### Platform-Specific Docs

- [ ] `WINDOWS_SETUP.md` exists and is complete
- [ ] `CROSS_PLATFORM_MIGRATION.md` exists
- [ ] `PLATFORM_QUICK_REFERENCE.md` exists
- [ ] All guides are up-to-date
- [ ] Examples work on target platforms

### Code Documentation

- [ ] Functions document platform compatibility
- [ ] Platform-specific behavior noted
- [ ] Examples are cross-platform
- [ ] Warnings for platform-specific features
- [ ] Links to relevant documentation

## Release Checklist

### Pre-Release

- [ ] All tests pass on all platforms
- [ ] Documentation is complete
- [ ] CHANGELOG updated
- [ ] Version numbers updated
- [ ] Release notes prepared

### Release Artifacts

- [ ] Linux binary built and tested
- [ ] macOS binary built and tested (Intel)
- [ ] macOS binary built and tested (ARM)
- [ ] Windows binary built and tested
- [ ] WASM binary built and tested
- [ ] All binaries signed (if applicable)

### Post-Release

- [ ] Release notes published
- [ ] Documentation deployed
- [ ] Binaries uploaded
- [ ] Platform-specific installation tested
- [ ] User feedback monitored

## Common Issues Checklist

### Path Issues

- [ ] No "file not found" errors on any platform
- [ ] No "invalid path" errors
- [ ] Paths work with spaces in directory names
- [ ] Paths work with special characters
- [ ] Absolute paths resolve correctly

### Script Issues

- [ ] Scripts don't fail due to line endings
- [ ] Scripts handle missing dependencies gracefully
- [ ] Scripts work in different shells
- [ ] Scripts provide helpful error messages
- [ ] Scripts exit with appropriate codes

### Build Issues

- [ ] No platform-specific compilation errors
- [ ] No linking errors on any platform
- [ ] No missing dependencies
- [ ] Build times are reasonable
- [ ] Artifacts are correct size

## Validation Commands

### Quick Validation

```bash
# Linux/macOS
cargo test cross_platform && ./validate_all.sh

# Windows
cargo test cross_platform; .\validate_all.ps1
```

### Full Validation

```bash
# Linux/macOS
cargo clean
cargo build --release
cargo test --verbose
cargo test cross_platform --verbose
./validate_all.sh
./pre_deploy_validate.sh
python3 test_config_validation.py --test

# Windows
cargo clean
cargo build --release
cargo test --verbose
cargo test cross_platform --verbose
.\validate_all.ps1
.\pre_deploy_validate.ps1
python test_config_validation.py --test
```

### CI/CD Validation

```bash
# Trigger CI/CD pipeline
git push origin feature-branch

# Check results
# - Ubuntu tests pass
# - Windows tests pass
# - macOS tests pass
# - All matrix builds succeed
```

## Sign-Off

Before merging:

- [ ] All checklist items completed
- [ ] Tests pass on all platforms
- [ ] Documentation reviewed
- [ ] Code reviewed by another developer
- [ ] CI/CD pipeline passes
- [ ] No breaking changes (or documented)

## Notes

- This checklist should be reviewed before every release
- Update checklist as new platforms are added
- Document any platform-specific workarounds
- Keep checklist in sync with actual requirements

## Resources

- [Rust std::path](https://doc.rust-lang.org/std/path/)
- [Python pathlib](https://docs.python.org/3/library/pathlib.html)
- [GitHub Actions](https://docs.github.com/en/actions)
- [Git Attributes](https://git-scm.com/docs/gitattributes)

## Questions?

If unsure about any item:

1. Check existing code for patterns
2. Review `CROSS_PLATFORM_MIGRATION.md`
3. Run tests on multiple platforms
4. Ask in code review
5. Consult platform-specific documentation
