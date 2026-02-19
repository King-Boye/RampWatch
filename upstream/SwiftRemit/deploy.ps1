# SwiftRemit Deployment Script for Windows (PowerShell)

param (
    [string]$Network = "testnet"
)

$ErrorActionPreference = "Stop"

# Configuration
$Deployer = "deployer"
$WasmPath = "target/wasm32-unknown-unknown/release/swiftremit.optimized.wasm"

Write-Host "🚀 SwiftRemit Deployment Script" -ForegroundColor Cyan
Write-Host "Network: $Network" -ForegroundColor Gray
Write-Host "Deployer Identity: $Deployer" -ForegroundColor Gray

# Check prerequisites
if (-not (Get-Command soroban -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Soroban CLI not found. Please install it first." -ForegroundColor Red
    exit 1
}

# Setup Identity
Write-Host "Checking identity..." -ForegroundColor Yellow
try {
    $Address = soroban keys address $Deployer 2>$null
    if (-not $Address) {
        throw "Identity not found"
    }
    Write-Host "Identity '$Deployer' found: $Address" -ForegroundColor Green
} catch {
    Write-Host "Creating new identity '$Deployer'..." -ForegroundColor Yellow
    soroban keys generate --global $Deployer --network $Network
    $Address = soroban keys address $Deployer
}

# Fund Identity (attempt on testnet/standalone, skip on mainnet)
if ($Network -ne "mainnet") {
    Write-Host "Funding identity (this may take a moment)..." -ForegroundColor Yellow
    try {
        soroban keys fund $Deployer --network $Network
    } catch {
        Write-Host "Funding warning: Request may have failed or account already funded (or network doesn't support funding)." -ForegroundColor DarkYellow
    }
}

# Build and Optimize
Write-Host "🔨 Building and Optimizing Contract..." -ForegroundColor Yellow
cargo build --target wasm32-unknown-unknown --release
soroban contract optimize --wasm target/wasm32-unknown-unknown/release/swiftremit.wasm

if (-not (Test-Path $WasmPath)) {
    Write-Host "❌ Build failed. $WasmPath not found." -ForegroundColor Red
    exit 1
}

# Deploy Contract
Write-Host "📤 Deploying Contract..." -ForegroundColor Yellow
$ContractId = soroban contract deploy `
  --wasm $WasmPath `
  --source $Deployer `
  --network $Network

Write-Host "✅ Contract Deployed: $ContractId" -ForegroundColor Green

# Deploy Mock USDC Token
Write-Host "💰 Deploying Mock USDC Token..." -ForegroundColor Yellow
$UsdcId = soroban contract asset deploy `
  --asset "USDC:$Address" `
  --source $Deployer `
  --network $Network

Write-Host "✅ USDC Token Deployed: $UsdcId" -ForegroundColor Green

# Initialize Contract
Write-Host "⚙️ Initializing Contract..." -ForegroundColor Yellow
soroban contract invoke `
  --id $ContractId `
  --source $Deployer `
  --network $Network `
  -- `
  initialize `
  --admin $Address `
  --usdc_token $UsdcId `
  --fee_bps 250

Write-Host ""
Write-Host "🎉 Deployment Complete!" -ForegroundColor Cyan
Write-Host "----------------------------------------" -ForegroundColor Gray
Write-Host "Contract ID: $ContractId"
Write-Host "USDC Token ID: $UsdcId"
Write-Host "Admin Address: $Address"
Write-Host "----------------------------------------" -ForegroundColor Gray

# Save to .env.local file for frontend use
$EnvContent = "NEXT_PUBLIC_CONTRACT_ID=$ContractId`nNEXT_PUBLIC_USDC_TOKEN_ADDRESS=$UsdcId"
Set-Content -Path ".env.local" -Value $EnvContent
Write-Host "IDs saved to .env.local" -ForegroundColor Green
