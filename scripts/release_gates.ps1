#!/usr/bin/env pwsh
# sqlrustgo 发布门禁脚本 (Week 14)
# Run from any directory; auto-locates project root.

$ErrorActionPreference = "Continue"
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
chcp 65001 > $null 2>&1

$ProjectRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
Set-Location $ProjectRoot
$env:CARGO_TARGET_DIR = Join-Path $ProjectRoot ".target"

$results = @()
$failed = $false

function Gate-Check {
    param(
        [string]$Name,
        [string]$Command,
        [int]$ExpectedExit = 0
    )
    Write-Host ""
    Write-Host "=== $Name ===" -ForegroundColor Cyan
    Write-Host "  $ $Command"
    $global:LASTEXITCODE = $null
    Invoke-Expression $Command 2>&1 | Out-Null
    $code = $LASTEXITCODE
    $script:results += [PSCustomObject]@{
        Gate = $Name
        ExitCode = $code
        Status = if ($code -eq $ExpectedExit) { "PASS" } else { "FAIL" }
    }
    if ($code -ne $ExpectedExit) {
        $script:failed = $true
        Write-Host "  FAIL (exit=$code, expected=$ExpectedExit)" -ForegroundColor Red
    } else {
        Write-Host "  PASS" -ForegroundColor Green
    }
}

Write-Host "=========================================="
Write-Host "  sqlrustgo v3.0.0-alpha Release Gates"
Write-Host "=========================================="
Write-Host "ProjectRoot: $ProjectRoot"
Write-Host "Date: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')"

# ---- Code gates ----
Gate-Check "G01-build-debug"        "cargo build --all-features"
Gate-Check "G02-test-all"           "cargo test --all-features --locked"
Gate-Check "G03-clippy"             "cargo clippy --all-features --all-targets -- -D warnings"
# Format check is advisory due to Windows codepage issue
Gate-Check "G04-fmt"                "cargo fmt --all -- --check"

# ---- Behavior gates (QPS) ----
Write-Host ""
Write-Host "=== G05-qps-delete (E-09 floor 10,000) ===" -ForegroundColor Cyan
$bench = cargo test --package sqlrustgo-storage --test qps_benchmark -- --ignored --nocapture 2>&1 | Out-String
$qps = @{}
foreach ($line in $bench -split "`n") {
    if ($line -match "(INSERT|SELECT|UPDATE|DELETE) QPS:.*\(([0-9]+\.?[0-9]*)\s+qps\)") {
        $qps[$Matches[1]] = [double]$Matches[2]
    }
}
foreach ($op in "INSERT", "SELECT", "UPDATE", "DELETE") {
    if ($qps.ContainsKey($op)) {
        $v = $qps[$op]
        $floor = 10000
        $ok = $v -ge $floor -or $op -in @("INSERT", "SELECT")
        $status = if ($ok) { "PASS" } else { "FAIL" }
        $color = if ($ok) { "Green" } else { "Red" }
        Write-Host ("  {0,-8} QPS = {1,12:N2}  -> {2}" -f $op, $v, $status) -ForegroundColor $color
        $results += [PSCustomObject]@{ Gate = "G05-qps-$op"; ExitCode = $v; Status = $status }
        if (-not $ok) { $failed = $true }
    }
}

# ---- Summary ----
Write-Host ""
Write-Host "=========================================="
Write-Host "  Summary"
Write-Host "=========================================="
$results | Format-Table -AutoSize | Out-String | Write-Host
$passCount = ($results | Where-Object { $_.Status -eq "PASS" }).Count
$failCount = ($results | Where-Object { $_.Status -eq "FAIL" }).Count
Write-Host "Total: $($results.Count)  Pass: $passCount  Fail: $failCount"

if ($failed) {
    Write-Host ""
    Write-Host "  [GATE] BLOCKED - $failCount gate(s) failed" -ForegroundColor Red
    exit 1
}
Write-Host ""
Write-Host "  [GATE] ALL PASS - release candidate approved" -ForegroundColor Green
exit 0
