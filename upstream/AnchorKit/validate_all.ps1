# PowerShell equivalent of validate_all.sh
# AnchorKit Pre-Deployment Validation for Windows

$ErrorActionPreference = "Stop"

Write-Host "🔍 AnchorKit Pre-Deployment Validation" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Check if Python is available
try {
    $pythonVersion = python --version 2>&1
    Write-Host "✅ Python found: $pythonVersion" -ForegroundColor Green
} catch {
    Write-Host "❌ Python3 is required but not installed" -ForegroundColor Red
    Write-Host "   Download from: https://www.python.org/downloads/" -ForegroundColor Yellow
    exit 1
}

# Check if required Python packages are installed
Write-Host "📦 Checking Python dependencies..." -ForegroundColor Cyan
try {
    python -c "import jsonschema, toml" 2>$null
    Write-Host "✅ Python dependencies OK" -ForegroundColor Green
} catch {
    Write-Host "❌ Missing Python dependencies. Installing..." -ForegroundColor Yellow
    pip install jsonschema toml --quiet
    Write-Host "✅ Dependencies installed" -ForegroundColor Green
}
Write-Host ""

# Validate all configuration files
Write-Host "📋 Validating configuration files..." -ForegroundColor Cyan
$ConfigDir = "configs"
$SchemaFile = "config_schema.json"
$Failed = 0

if (-not (Test-Path $SchemaFile)) {
    Write-Host "❌ Schema file not found: $SchemaFile" -ForegroundColor Red
    exit 1
}

$configFiles = Get-ChildItem -Path $ConfigDir -Include *.json,*.toml -File

foreach ($configFile in $configFiles) {
    Write-Host "  Validating $($configFile.Name)... " -NoNewline
    
    $result = python validate_config_strict.py $configFile.FullName $SchemaFile 2>&1
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅" -ForegroundColor Green
    } else {
        Write-Host "❌" -ForegroundColor Red
        Write-Host $result -ForegroundColor Red
        $Failed = 1
    }
}

if ($Failed -eq 1) {
    Write-Host ""
    Write-Host "❌ Configuration validation failed" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "✅ All configurations valid" -ForegroundColor Green
Write-Host ""

# Run Rust tests
Write-Host "🧪 Running Rust validation tests..." -ForegroundColor Cyan
$testOutput = cargo test --quiet config 2>&1 | Out-String

if ($testOutput -match "test result: ok") {
    Write-Host "✅ Rust tests passed" -ForegroundColor Green
} else {
    Write-Host "❌ Rust tests failed" -ForegroundColor Red
    cargo test config
    exit 1
}

Write-Host ""
Write-Host "🎉 All validations passed! Ready for deployment." -ForegroundColor Green
