<#
.SYNOPSIS
OASM Phase 1 - One-Time Initializer Command Block

.DESCRIPTION
Drop this into any project root and run:
  cd C:\path\to\project
  .\Invoke-Phase1.ps1

Creates directory structure, performs recursive scan, generates:
- CLI dashboard (JSONL + TXT)
- Longform structure log (JSONL + TXT)
- Folder blueprint (JSON + TXT)
- Schemas and templates
- Baby wrapper scripts

.PARAMETER Root
Project root directory (defaults to current location)

.PARAMETER OasmPath
Path to OASM binaries (defaults to auto-detect)

.EXAMPLE
.\Invoke-Phase1.ps1
.\Invoke-Phase1.ps1 -Root . -OasmPath "C:\path\to\oasm\target\release"
#>

param(
    [string]$Root = (Get-Location).Path,
    [string]$OasmPath = $null
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

# Banner
Write-Host ""
Write-Host "🚀 OASM Phase 1 - One-Time Initializer" -ForegroundColor Cyan
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
Write-Host "📂 Root: $Root" -ForegroundColor White
Write-Host ""

# Resolve root
$Root = (Resolve-Path -LiteralPath $Root).Path

# Auto-detect OASM path if not provided
if (-not $OasmPath) {
    # Check if we're in OASM project itself
    $oasmBin = Join-Path $Root "target\release\oasm-phase1.exe"
    if (Test-Path $oasmBin) {
        $OasmPath = Join-Path $Root "target\release"
        Write-Host "✓ Found OASM binaries in project: $OasmPath" -ForegroundColor Green
    } else {
        # Look for OASM in common locations
        $searchPaths = @(
            "$env:USERPROFILE\Desktop\Projects\oasm\target\release",
            "C:\oasm\target\release",
            "C:\Program Files\oasm\bin"
        )
        foreach ($path in $searchPaths) {
            if (Test-Path (Join-Path $path "oasm-phase1.exe")) {
                $OasmPath = $path
                Write-Host "✓ Found OASM binaries at: $OasmPath" -ForegroundColor Green
                break
            }
        }
    }
}

# Verify OASM binary exists
$oasmPhase1 = Join-Path $OasmPath "oasm-phase1.exe"
if (-not (Test-Path $oasmPhase1)) {
    Write-Host "❌ ERROR: oasm-phase1.exe not found!" -ForegroundColor Red
    Write-Host "   Searched: $oasmPhase1" -ForegroundColor Yellow
    Write-Host "   Please build OASM or specify -OasmPath" -ForegroundColor Yellow
    exit 1
}

Write-Host "🔧 Using OASM Phase 1 binary: $oasmPhase1" -ForegroundColor Cyan
Write-Host ""

# Run Phase 1 binary
Write-Host "▶ Running Phase 1 scan..." -ForegroundColor Cyan
& $oasmPhase1 $Root --verbose

if ($LASTEXITCODE -ne 0) {
    Write-Host ""
    Write-Host "❌ Phase 1 failed with exit code: $LASTEXITCODE" -ForegroundColor Red
    exit $LASTEXITCODE
}

Write-Host ""
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
Write-Host "✅ Phase 1 Complete!" -ForegroundColor Green
Write-Host ""

# Generate baby wrappers that call OASM binaries
Write-Host "📝 Generating baby wrapper scripts..." -ForegroundColor Cyan

$scriptsDir = Join-Path $Root "scripts\PS"
if (-not (Test-Path $scriptsDir)) {
    New-Item -ItemType Directory -Path $scriptsDir -Force | Out-Null
}

# Baby-Full: Full rescan
$babyFull = @"
# Baby-Full: Full rescan (manifest-driven)
# Usage: pwsh baby-full.ps1
param([string]`$Root = (Get-Location).Path)
Set-StrictMode -Version Latest
`$ErrorActionPreference = "Stop"

Write-Host "🔄 Running full rescan..." -ForegroundColor Cyan
& "$oasmPhase1" "`$Root" --verbose

if (`$LASTEXITCODE -eq 0) {
    Write-Host "✅ Full scan complete!" -ForegroundColor Green
} else {
    Write-Host "❌ Scan failed!" -ForegroundColor Red
    exit `$LASTEXITCODE
}
"@

# Baby-Rerun: Rerun Phase 2 tests
$babyRerun = @"
# Baby-Rerun: Rerun Phase 2 tests (manifest-driven)
# Usage: pwsh baby-rerun.ps1
param([string]`$Root = (Get-Location).Path)
Set-StrictMode -Version Latest
`$ErrorActionPreference = "Stop"

Write-Host "🔁 Rerunning Phase 2 tests..." -ForegroundColor Cyan
Write-Host "⚠️  Phase 2 not yet implemented - placeholder" -ForegroundColor Yellow

# TODO: Call oasm-phase2 when implemented
# & "$OasmPath\oasm-phase2.exe" "`$Root" --rerun
"@

# Baby-Arm: Arm-specific scan
$babyArm = @"
# Baby-Arm: Arm-specific scan (manifest-driven)
# Usage: pwsh baby-arm.ps1 -Arm "compiler/src"
param(
    [Parameter(Mandatory=`$true)]
    [string]`$Arm,
    [string]`$Root = (Get-Location).Path
)
Set-StrictMode -Version Latest
`$ErrorActionPreference = "Stop"

Write-Host "🎯 Scanning arm: `$Arm" -ForegroundColor Cyan

`$armPath = Join-Path `$Root `$Arm
if (-not (Test-Path `$armPath)) {
    Write-Host "❌ Arm not found: `$armPath" -ForegroundColor Red
    exit 1
}

Write-Host "⚠️  Arm-specific scanning not yet implemented - placeholder" -ForegroundColor Yellow

# TODO: Call oasm-phase1 with arm filter
# & "$oasmPhase1" "`$armPath" --verbose
"@

# Write baby scripts
$babyFull | Out-File -FilePath (Join-Path $scriptsDir "baby-full.ps1") -Encoding UTF8
$babyRerun | Out-File -FilePath (Join-Path $scriptsDir "baby-rerun.ps1") -Encoding UTF8
$babyArm | Out-File -FilePath (Join-Path $scriptsDir "baby-arm.ps1") -Encoding UTF8

Write-Host "   ✓ baby-full.ps1" -ForegroundColor Green
Write-Host "   ✓ baby-rerun.ps1" -ForegroundColor Green
Write-Host "   ✓ baby-arm.ps1" -ForegroundColor Green
Write-Host ""

# Show next steps
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
Write-Host "📋 Next Steps:" -ForegroundColor Cyan
Write-Host ""
Write-Host "1. Review outputs in: $Root\logs\logs\" -ForegroundColor White
Write-Host "2. Run full rescan:   pwsh scripts\PS\baby-full.ps1" -ForegroundColor White
Write-Host "3. Run Phase 2 tests: pwsh scripts\PS\baby-rerun.ps1" -ForegroundColor White
Write-Host "4. Scan specific arm: pwsh scripts\PS\baby-arm.ps1 -Arm 'compiler/src'" -ForegroundColor White
Write-Host ""
Write-Host "🎯 Ready for Phase 2!" -ForegroundColor Green
Write-Host ""
