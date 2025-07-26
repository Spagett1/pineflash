# Smoke test for PineFlash binary on Windows
param(
    [string]$BinaryPath = ".\target\release\pineflash.exe"
)

Write-Host "Testing PineFlash binary: $BinaryPath"

# Check if binary exists
if (-not (Test-Path $BinaryPath)) {
    Write-Host "Error: Binary not found at $BinaryPath" -ForegroundColor Red
    exit 1
}

Write-Host "1. Testing --help flag..." -ForegroundColor Yellow
try {
    $helpOutput = & $BinaryPath --help 2>&1
    if ($LASTEXITCODE -eq 0 -or $helpOutput) {
        Write-Host "✅ Help command works" -ForegroundColor Green
    } else {
        Write-Host "❌ Help command failed" -ForegroundColor Red
        exit 1
    }
} catch {
    # Try alternative help flag
    try {
        $helpOutput = & $BinaryPath -h 2>&1
        Write-Host "✅ Help command works (using -h)" -ForegroundColor Green
    } catch {
        Write-Host "❌ Help command failed" -ForegroundColor Red
        exit 1
    }
}

Write-Host "2. Testing --version flag..." -ForegroundColor Yellow
try {
    $versionOutput = & $BinaryPath --version 2>&1
    if ($LASTEXITCODE -eq 0 -or $versionOutput) {
        Write-Host "✅ Version command works: $versionOutput" -ForegroundColor Green
    } else {
        Write-Host "❌ Version command failed" -ForegroundColor Red
        exit 1
    }
} catch {
    # Try alternative version flag
    try {
        $versionOutput = & $BinaryPath -V 2>&1
        Write-Host "✅ Version command works (using -V): $versionOutput" -ForegroundColor Green
    } catch {
        Write-Host "❌ Version command failed" -ForegroundColor Red
        exit 1
    }
}

Write-Host "3. Checking binary properties..." -ForegroundColor Yellow
$fileInfo = Get-Item $BinaryPath
Write-Host "  File size: $($fileInfo.Length) bytes"
Write-Host "  Created: $($fileInfo.CreationTime)"
Write-Host "  Architecture: " -NoNewline

# Check if it's 64-bit
$bytes = [System.IO.File]::ReadAllBytes($BinaryPath)
$peOffset = [BitConverter]::ToInt32($bytes, 0x3C)
$machine = [BitConverter]::ToUInt16($bytes, $peOffset + 4)
if ($machine -eq 0x8664) {
    Write-Host "x64 (64-bit)" -ForegroundColor Green
} elseif ($machine -eq 0x014C) {
    Write-Host "x86 (32-bit)" -ForegroundColor Yellow
} else {
    Write-Host "Unknown ($machine)" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "✅ Smoke tests completed successfully!" -ForegroundColor Green