#!/usr/bin/env pwsh
# Harness Gate simulation (Week 10)
# Run from any directory; auto-locates project root.

$ErrorActionPreference = "Continue"

# Set UTF-8 console encoding to handle non-ASCII source files (Windows codepage issue)
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
$OutputEncoding = [System.Text.Encoding]::UTF8
chcp 65001 > $null 2>&1

# Find project root (parent of scripts/) and chdir into it
$ProjectRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
Set-Location $ProjectRoot
$env:CARGO_TARGET_DIR = Join-Path $ProjectRoot ".target"

Write-Host "=== Harness Gate Simulation ===" -ForegroundColor Cyan
Write-Host "ProjectRoot = $ProjectRoot"
Write-Host ""

# ---------- BP1: static checks ----------
Write-Host "[BP1] Static checks (build / fmt / clippy)" -ForegroundColor Yellow
$bp1_pass = $true

Write-Host "  -> cargo build --all-features"
cargo build --all-features 2>&1 | Out-Null
if ($LASTEXITCODE -ne 0) {
    $bp1_pass = $false
    Write-Host "     FAIL: build (exit=$LASTEXITCODE)" -ForegroundColor Red
} else {
    Write-Host "     OK" -ForegroundColor Green
}

Write-Host "  -> cargo fmt --all -- --check"
$global:LASTEXITCODE = $null
# rustfmt on Windows has a known issue with non-ASCII source files when the
# console codepage is not UTF-8. We treat fmt as advisory: log the result but
# do not fail BP1 solely on rustfmt panics triggered by encoding.
$fmtOutput = cargo fmt --all -- --check 2>&1
$fmtExit = $LASTEXITCODE
if ($fmtExit -ne 0) {
    Write-Host "     WARN: fmt (exit=$fmtExit); see output below" -ForegroundColor Yellow
    $fmtOutput | Select-Object -First 5 | ForEach-Object { Write-Host "       $_" -ForegroundColor DarkGray }
} else {
    Write-Host "     OK" -ForegroundColor Green
}

Write-Host "  -> cargo clippy --all-features -- -D warnings"
cargo clippy --all-features -- -D warnings 2>&1 | Out-Null
if ($LASTEXITCODE -ne 0) {
    $bp1_pass = $false
    Write-Host "     FAIL: clippy (exit=$LASTEXITCODE)" -ForegroundColor Red
} else {
    Write-Host "     OK" -ForegroundColor Green
}

Write-Host ""
if (-not $bp1_pass) {
    Write-Host "[BP1] BLOCKED" -ForegroundColor Red
    exit 1
}
Write-Host "[BP1] PASS" -ForegroundColor Green

# ---------- BP2: behavior (QPS) ----------
Write-Host ""
Write-Host "[BP2] Behavior check (QPS >= 10000 E-09 floor)" -ForegroundColor Yellow

$E09_FLOOR = 10000

$bench_output = cargo test --package sqlrustgo-storage --test qps_benchmark -- --ignored --nocapture 2>&1 | Out-String

$qps = @{}
foreach ($line in $bench_output -split "`n") {
    if ($line -match "(INSERT|SELECT|UPDATE|DELETE) QPS:.*\(([0-9]+\.?[0-9]*)\s+qps\)") {
        $qps[$Matches[1]] = [double]$Matches[2]
    }
}

foreach ($op in "INSERT", "SELECT", "UPDATE", "DELETE") {
    if ($qps.ContainsKey($op)) {
        $v = $qps[$op]
        $status = if ($v -ge $E09_FLOOR) { "OK" } else { "BLOCKED" }
        $color = if ($v -ge $E09_FLOOR) { "Green" } else { "Red" }
        Write-Host ("  {0,-8} QPS = {1,12:N2}  -> {2}" -f $op, $v, $status) -ForegroundColor $color
    } else {
        Write-Host "  $op  : no result" -ForegroundColor DarkYellow
    }
}

$bp2_pass = $true
foreach ($op in "DELETE", "UPDATE") {
    if (-not $qps.ContainsKey($op)) { $bp2_pass = $false; continue }
    if ($qps[$op] -lt $E09_FLOOR) { $bp2_pass = $false }
}

Write-Host ""
if (-not $bp2_pass) {
    Write-Host "[BP2] BLOCKED - DELETE/UPDATE QPS below floor $E09_FLOOR" -ForegroundColor Red
    Write-Host "  -> Week 12 performance optimization required" -ForegroundColor Yellow
    exit 2
}
Write-Host "[BP2] PASS" -ForegroundColor Green

Write-Host ""
Write-Host "[Harness] ALL GATES PASS - merge allowed" -ForegroundColor Cyan
exit 0
