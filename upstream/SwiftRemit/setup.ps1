# SwiftRemit Setup Script for Windows
# This script installs all prerequisites and builds the contract

Write-Host "🚀 SwiftRemit Setup Script" -ForegroundColor Cyan
Write-Host "==========================" -ForegroundColor Cyan
Write-Host ""

# Check if Rust is installed
$rustInstalled = Get-Command rustc -ErrorAction SilentlyContinue
if (-not $rustInstalled) {
    Write-Host "❌ Rust is not installed" -ForegroundColor Red
    Write-Host "📦 Please install Rust from: https://rustup.rs/" -ForegroundColor Yellow
    Write-Host "After installation, restart PowerShell and run this script again." -ForegroundColor Yellow
    exit 1
} else {
    $rustVersion = rustc --version
    Write-Host "✅ Rust is already installed ($rustVersion)" -ForegroundColor Green
}

# Add wasm32 target
Write-Host ""
Write-Host "📦 Adding wasm32-unknown-unknown target..." -ForegroundColor Yellow
rustup target add wasm32-unknown-unknown
Write-Host "✅ wasm32 target added" -ForegroundColor Green

# Check if Soroban CLI is installed
Write-Host ""
$sorobanInstalled = Get-Command soroban -ErrorAction SilentlyContinue
if (-not $sorobanInstalled) {
    Write-Host "📦 Installing Soroban CLI..." -ForegroundColor Yellow
    cargo install --locked soroban-cli --features opt
    Write-Host "✅ Soroban CLI installed successfully" -ForegroundColor Green
} else {
    $sorobanVersion = soroban --version
    Write-Host "✅ Soroban CLI is already installed ($sorobanVersion)" -ForegroundColor Green
}

# Configure testnet
Write-Host ""
Write-Host "🌐 Configuring Stellar testnet..." -ForegroundColor Yellow
try {
    soroban network add --global testnet `
      --rpc-url https://soroban-testnet.stellar.org:443 `
      --network-passphrase "Test SDF Network ; September 2015" 2>$null
} catch {
    Write-Host "Testnet already configured" -ForegroundColor Gray
}
Write-Host "✅ Testnet configured" -ForegroundColor Green

# Build the contract
Write-Host ""
Write-Host "🔨 Building contract..." -ForegroundColor Yellow
cargo build --target wasm32-unknown-unknown --release

# Optimize the contract
Write-Host ""
Write-Host "⚡ Optimizing contract..." -ForegroundColor Yellow
soroban contract optimize --wasm target/wasm32-unknown-unknown/release/swiftremit.wasm

Write-Host ""
Write-Host "✅ Setup complete!" -ForegroundColor Green
Write-Host ""
Write-Host "📝 Next steps:" -ForegroundColor Cyan
Write-Host "1. Run tests: cargo test"
Write-Host "2. Deploy to testnet: See DEPLOYMENT.md for instructions"
Write-Host ""
