# PowerShell equivalent of pre_deploy_validate.sh
# Pre-deployment validation script for Windows
# Validates all configurations before contract deployment

$ErrorActionPreference = "Stop"

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$SchemaFile = Join-Path $ScriptDir "config_schema.json"
$Validator = Join-Path $ScriptDir "validate_config_strict.py"
$ConfigsDir = Join-Path $ScriptDir "configs"

Write-Host "🔍 AnchorKit Pre-Deployment Validation" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Check dependencies
try {
    $null = python --version 2>&1
} catch {
    Write-Host "❌ Python3 not found. Please install Python 3.7+" -ForegroundColor Red
    Write-Host "   Download from: https://www.python.org/downloads/" -ForegroundColor Yellow
    exit 1
}

# Install required Python packages
Write-Host "📦 Checking Python dependencies..." -ForegroundColor Cyan
try {
    pip install -q jsonschema toml 2>$null
} catch {
    Write-Host "⚠️  Installing jsonschema and toml..." -ForegroundColor Yellow
    pip install jsonschema toml
}

# Validate schema file exists
if (-not (Test-Path $SchemaFile)) {
    Write-Host "❌ Schema file not found: $SchemaFile" -ForegroundColor Red
    exit 1
}

# Validate all config files
Write-Host ""
Write-Host "🔎 Validating configuration files..." -ForegroundColor Cyan
Write-Host ""

$Failed = 0
$Passed = 0

$configFiles = Get-ChildItem -Path $ConfigsDir -Include *.toml,*.json -File

foreach ($config in $configFiles) {
    Write-Host "  Validating $($config.Name)... " -NoNewline
    
    $output = python $Validator $config.FullName $SchemaFile 2>&1
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✓" -ForegroundColor Green
        $Passed++
    } else {
        Write-Host "✗" -ForegroundColor Red
        $output | ForEach-Object { Write-Host "    $_" -ForegroundColor Red }
        $Failed++
    }
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Results: $Passed passed, $Failed failed" -ForegroundColor Cyan
Write-Host ""

if ($Failed -gt 0) {
    Write-Host "❌ Validation failed. Fix errors before deployment." -ForegroundColor Red
    exit 1
} else {
    Write-Host "✅ All configurations valid. Ready for deployment." -ForegroundColor Green
    exit 0
}
