# Invoke-OasmScan.ps1
# PowerShell 2026 wrapper for OASM Scanner
# Executive Function: Pre-compile diagnostic with AI assistance

<#
.SYNOPSIS
    Universal codebase scanner for pre-compile diagnostics

.DESCRIPTION
    Scans any project root and generates comprehensive metrics including:
    - Lines of code
    - Function counts
    - Logging analysis
    - Compilation status    - Structural health

.PARAMETER Root
    Project root directory to scan (defaults to current directory)

.PARAMETER OutputDir
    Output directory for logs (defaults to logs/StructureDebug)

.PARAMETER Format
    Output format: json, yaml, or both (default: both)

.PARAMETER Verbose
    Show detailed progress and top files

.EXAMPLE
    Invoke-OasmScan -Root "C:\Projects\MyApp"

.EXAMPLE
    Invoke-OasmScan -Verbose -Format json
#>

[CmdletBinding()]
param(
    [Parameter(Position = 0)]
    [string]$Root = ".",

    [Parameter()]
    [string]$OutputDir = "logs/StructureDebug",

    [Parameter()]
    [ValidateSet("json", "yaml", "both")]
    [string]$Format = "both",

    [Parameter()]
    [switch]$Verbose
)

# Modern CLI output with colors
$Script:Colors = @{
    Header = 'Cyan'
    Success = 'Green'
    Warning = 'Yellow'
    Error = 'Red'
    Info = 'White'
    Progress = 'Magenta'
    Metric = 'DarkCyan'
}

function Write-OasmHeader {
    Write-Host ""
    Write-Host "╔═══════════════════════════════════════════════════════╗" -ForegroundColor $Colors.Header
    Write-Host "║          OASM SCANNER - PRE-COMPILE DIAGNOSTICS       ║" -ForegroundColor $Colors.Header
    Write-Host "║          Executive Function AI Powerhouse             ║" -ForegroundColor $Colors.Header
    Write-Host "╚═══════════════════════════════════════════════════════╝" -ForegroundColor $Colors.Header
    Write-Host ""
}

function Write-OasmProgress {
    param([string]$Status, [int]$Percent)
    $barLength = 40
    $completed = [Math]::Floor($barLength * $Percent / 100)
    $remaining = $barLength - $completed

    $bar = "[$('█' * $completed)$('░' * $remaining)]"
    Write-Host "`r$bar $Percent% - $Status" -NoNewline -ForegroundColor $Colors.Progress
}

function Invoke-OasmScanNative {
    param([string]$RootPath, [string]$Output, [string]$FormatType, [bool]$VerboseOutput)

    $oasmScanExe = "target/debug/oasm-scan.exe"

    # Check if built
    if (-not (Test-Path $oasmScanExe)) {
        Write-Host "⚙️  Building oasm-scan..." -ForegroundColor $Colors.Info
        cargo build --bin oasm-scan
        if ($LASTEXITCODE -ne 0) {
            Write-Host "❌ Build failed" -ForegroundColor $Colors.Error
            exit 1
        }
    }

    # Build arguments
    $args = @($RootPath, "--output", $Output, "--format", $FormatType)
    if ($VerboseOutput) {
        $args += "--verbose"
    }

    # Execute scanner
    Write-OasmProgress -Status "Scanning project structure" -Percent 10

    & $oasmScanExe @args

    if ($LASTEXITCODE -eq 0) {
        Write-Host "`n"
        Write-Host "✅ Scan completed successfully!" -ForegroundColor $Colors.Success
    } else {
        Write-Host "`n"
        Write-Host "❌ Scan failed with exit code $LASTEXITCODE" -ForegroundColor $Colors.Error
        exit $LASTEXITCODE
    }
}

function Show-OasmSummary {
    param([string]$OutputPath)

    $timestamp = Get-Date -Format "yyyyMMdd_HHmmss"
    $jsonFile = Get-ChildItem -Path $OutputPath -Filter "baseline_index_*.json" |
                Sort-Object LastWriteTime -Descending |
                Select-Object -First 1

    if ($jsonFile) {
        $data = Get-Content $jsonFile.FullName | ConvertFrom-Json

        Write-Host ""
        Write-Host "📊 METRICS SUMMARY" -ForegroundColor $Colors.Header
        Write-Host "═══════════════════════════════════════" -ForegroundColor $Colors.Header

        $totalFiles = $data.Count
        $totalLOC = ($data | Measure-Object -Property loc -Sum).Sum
        $totalFunctions = ($data | Measure-Object -Property fn_count -Sum).Sum
        $totalTests = ($data | Measure-Object -Property tests -Sum).Sum

        Write-Host "  📁 Total Files:     $totalFiles" -ForegroundColor $Colors.Metric
        Write-Host "  📝 Total LOC:       $totalLOC" -ForegroundColor $Colors.Metric
        Write-Host "  🔧 Total Functions: $totalFunctions" -ForegroundColor $Colors.Metric
        Write-Host "  ✓  Total Tests:     $totalTests" -ForegroundColor $Colors.Metric

        if ($totalFiles -gt 0) {
            $avgLOC = [Math]::Round($totalLOC / $totalFiles, 2)
            Write-Host "  📈 Avg LOC/File:    $avgLOC" -ForegroundColor $Colors.Metric
        }

        # Health indicators
        Write-Host ""
        Write-Host "🏥 HEALTH INDICATORS" -ForegroundColor $Colors.Header
        Write-Host "═══════════════════════════════════════" -ForegroundColor $Colors.Header

        $unsafeCount = ($data | Measure-Object -Property unsafe_fn_count -Sum).Sum
        $warningsCount = ($data | Where-Object { $_.warn -gt 0 }).Count
        $blocksCount = ($data | Where-Object { $_.block -gt 0 }).Count

        $healthColor = if ($blocksCount -eq 0 -and $unsafeCount -lt 5) { $Colors.Success } elseif ($blocksCount -lt 3) { $Colors.Warning } else { $Colors.Error }

        Write-Host "  ⚠️  Blocking Issues: $blocksCount" -ForegroundColor $healthColor
        Write-Host "  ⚡ Warnings:        $warningsCount" -ForegroundColor $(if ($warningsCount -lt 10) { $Colors.Success } else { $Colors.Warning })
        Write-Host "  🔒 Unsafe Code:     $unsafeCount" -ForegroundColor $(if ($unsafeCount -eq 0) { $Colors.Success } else { $Colors.Warning })

        # Test coverage estimate
        if ($totalFunctions -gt 0) {
            $testCoverage = [Math]::Round(($totalTests / $totalFunctions) * 100, 1)
            $coverageColor = if ($testCoverage -gt 80) { $Colors.Success } elseif ($testCoverage -gt 50) { $Colors.Warning } else { $Colors.Error }
            Write-Host "  🧪 Test Coverage:   $testCoverage%" -ForegroundColor $coverageColor
        }

        Write-Host ""
    }
}

# Main execution
try {
    Write-OasmHeader

    $resolvedRoot = Resolve-Path $Root -ErrorAction Stop
    Write-Host "📂 Root: $resolvedRoot" -ForegroundColor $Colors.Info
    Write-Host "📊 Format: $Format" -ForegroundColor $Colors.Info
    Write-Host ""

    # Run scan
    Invoke-OasmScanNative -RootPath $resolvedRoot -Output $OutputDir -FormatType $Format -VerboseOutput:$Verbose

    # Show enhanced summary
    Show-OasmSummary -OutputPath $OutputDir

    # AI assistance hint (PS 2026 integration point)
    Write-Host "💡 TIP: Run 'Get-OasmInsights' for AI-powered code analysis" -ForegroundColor $Colors.Info
    Write-Host ""

} catch {
    Write-Host "❌ Error: $_" -ForegroundColor $Colors.Error
    Write-Host $_.ScriptStackTrace -ForegroundColor $Colors.Error
    exit 1
}
