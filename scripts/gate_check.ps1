#!/usr/bin/env pwsh
# Harness Gate simulation (Week 10)
$ErrorActionPreference = "Stop"
$env:CARGO_TARGET_DIR = "C:\sqlrustgo-build"

Write-Host "=== Harness Gate Simulation ===" -ForegroundColor Cyan
Write-Host ""

# ---------- BP1: static checks ----------
Write-Host "[BP1] Static checks (build / fmt / clippy)" -ForegroundColor Yellow
$bp1_pass = $true

Write-Host "  -> cargo build --all-features"
$out = cargo build --all-features 2>&1 | Out-String
if ($LASTEXITCODE -ne 0) { $bp1_pass = $false; Write-Host "     FAIL: build" -ForegroundColor Red } else { Write-Host "     OK" -ForegroundColor Green }

Write-Host "  -> cargo fmt --all -- --check"
$out = cargo fmt --all -- --check 2>&1 | Out-String
if ($LASTEXITCODE -ne 0) { $bp1_pass = $false; Write-Host "     FAIL: fmt" -ForegroundColor Red } else { Write-Host "     OK" -ForegroundColor Green }

Write-Host "  -> cargo clippy --all-features -- -D warnings"
$out = cargo clippy --all-features -- -D warnings 2>&1 | Out-String
if ($LASTEXITCODE -ne 0) { $bp1_pass = $false; Write-Host "     FAIL: clippy" -ForegroundColor Red } else { Write-Host "     OK" -ForegroundColor Green }

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
    if ($line -match "^(INSERT|SELECT|UPDATE|DELETE) QPS:.*\(([0-9]+\.?[0-9]*)\s+qps\)") {
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
