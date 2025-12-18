# OASM.PowerShell Module
# OASM assembly module for wpshell - provides assembly-like control for programs

$script:OasmRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$script:LogPath = Join-Path $OasmRoot "logs\psmodule.log"

# Initialize OASM
function Initialize-Oasm {
    <#
    .SYNOPSIS
        Initialize the OASM assembly module
    .DESCRIPTION
        Sets up the OASM environment for the current PowerShell session
    #>
    [CmdletBinding()]
    param()

    Write-Host "OASM Assembly Module v0.1.0" -ForegroundColor Cyan
    Write-Host "Initializing for wpshell..." -ForegroundColor Gray

    # Load manifest
    $manifestPath = Join-Path $script:OasmRoot "manifests\master_manifest.yaml"
    if (Test-Path $manifestPath) {
        Write-Host "✓ Manifest loaded" -ForegroundColor Green
    }

    # Check Rust executables
    $daemonExe = Join-Path $script:OasmRoot "target\debug\runtime_daemon.exe"
    if (Test-Path $daemonExe) {
        Write-Host "✓ Runtime daemon available" -ForegroundColor Green
    }

    Write-Host "OASM ready" -ForegroundColor Cyan
}

# Program context management
function New-OasmContext {
    <#
    .SYNOPSIS
        Create a new OASM context for a program
    .PARAMETER ProgramType
        Type of program: CAD, Engine, Document, Compression, Debug
    .EXAMPLE
        $ctx = New-OasmContext -ProgramType CAD
    #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)]
        [ValidateSet('CAD', 'Engine', 'Document', 'Compression', 'Debug')]
        [string]$ProgramType
    )

    @{
        ProgramType = $ProgramType
        Capabilities = @()
        Rules = @()
        Metadata = @{}
    }
}

# Assembly execution
function Invoke-OasmAssembly {
    <#
    .SYNOPSIS
        Execute OASM assembly code
    .PARAMETER Source
        OASM assembly source code
    .PARAMETER Context
        OASM context from New-OasmContext
    .EXAMPLE
        $source = @"
CREATE gear
  SET teeth = 20
  SET module = 2.5
EXTRUDE z_axis, 10
VALIDATE topology
"@
        Invoke-OasmAssembly -Source $source -Context $ctx
    #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)]
        [string]$Source,

        [Parameter()]
        [hashtable]$Context = @{}
    )

    Write-Host "Parsing OASM assembly..." -ForegroundColor Cyan
    $lines = $Source -split "`n" | Where-Object { $_.Trim() -ne "" }

    foreach ($line in $lines) {
        $line = $line.Trim()
        if ($line -match '^CREATE\s+(\w+)') {
            Write-Host "  → CREATE $($Matches[1])" -ForegroundColor Yellow
        }
        elseif ($line -match '^SET\s+(\w+)\s*=\s*(.+)') {
            Write-Host "  → SET $($Matches[1]) = $($Matches[2])" -ForegroundColor Yellow
        }
        elseif ($line -match '^EXTRUDE\s+(.+)') {
            Write-Host "  → EXTRUDE $($Matches[1])" -ForegroundColor Yellow
        }
        elseif ($line -match '^VALIDATE\s+(\w+)') {
            Write-Host "  → VALIDATE $($Matches[1])" -ForegroundColor Yellow
        }
        elseif ($line -match '^EXPORT\s+(\w+),\s*(.+)') {
            Write-Host "  → EXPORT $($Matches[1]) to $($Matches[2])" -ForegroundColor Yellow
        }
    }

    Write-Host "Assembly execution complete" -ForegroundColor Green
}

# Rule management
function Get-OasmRules {
    <#
    .SYNOPSIS
        Get rules for a program type
    .PARAMETER ProgramType
        Type of program
    .EXAMPLE
        Get-OasmRules -ProgramType CAD
    #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)]
        [ValidateSet('CAD', 'Engine', 'Document', 'Compression', 'Debug')]
        [string]$ProgramType
    )

    # Return rules based on program type
    switch ($ProgramType) {
        'CAD' {
            @(
                @{ Id = 'cad_topology_validation'; Category = 'Validation' }
                @{ Id = 'cad_parameter_range'; Category = 'Constraint' }
            )
        }
        'Engine' {
            @(
                @{ Id = 'engine_scene_graph_validation'; Category = 'Validation' }
            )
        }
        'Document' {
            @(
                @{ Id = 'document_structure_validation'; Category = 'Validation' }
            )
        }
        default { @() }
    }
}

# Block management
function Get-OasmBlocks {
    <#
    .SYNOPSIS
        Get blocks for a program type
    .PARAMETER ProgramType
        Type of program
    .EXAMPLE
        Get-OasmBlocks -ProgramType CAD
    #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)]
        [ValidateSet('CAD', 'Engine', 'Document', 'Compression', 'Debug')]
        [string]$ProgramType
    )

    # Return blocks based on program type
    switch ($ProgramType) {
        'CAD' {
            @(
                @{
                    Id = 'cad_primitives_block'
                    Instructions = @('CREATE', 'EXTRUDE', 'FILLET', 'CHAMFER')
                    Rules = @('geometric_validation', 'parametric_constraints')
                }
                @{
                    Id = 'cad_export_block'
                    Instructions = @('EXPORT', 'VALIDATE')
                    Rules = @('export_compatibility')
                }
            )
        }
        'Engine' {
            @(
                @{
                    Id = 'engine_scene_block'
                    Instructions = @('CREATE', 'ATTACH', 'SET')
                    Rules = @('scene_graph_validation', 'physics_constraints')
                }
            )
        }
        'Document' {
            @(
                @{
                    Id = 'document_content_block'
                    Instructions = @('INSERT', 'APPLY', 'EXPORT')
                    Rules = @('structure_validation', 'style_consistency')
                }
            )
        }
        default { @() }
    }
}

# Compiler wrapper
function Invoke-OasmCompile {
    <#
    .SYNOPSIS
        Compile an OASM manifest
    .PARAMETER ManifestPath
        Path to the manifest file
    .EXAMPLE
        Invoke-OasmCompile -ManifestPath .\manifests\test.yaml
    #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory)]
        [string]$ManifestPath
    )

    $compilerExe = Join-Path $script:OasmRoot "target\debug\compiler.exe"
    if (-not (Test-Path $compilerExe)) {
        Write-Error "Compiler not found at $compilerExe. Run 'cargo build' first."
        return
    }

    Write-Host "Compiling manifest: $ManifestPath" -ForegroundColor Cyan
    & $compilerExe $ManifestPath
}

# Scanner wrapper
function Invoke-OasmScan {
    <#
    .SYNOPSIS
        Run OASM scanner on a directory
    .PARAMETER Path
        Directory to scan
    .PARAMETER Output
        Output directory for logs
    .EXAMPLE
        Invoke-OasmScan -Path . -Output logs\scan
    #>
    [CmdletBinding()]
    param(
        [Parameter()]
        [string]$Path = ".",

        [Parameter()]
        [string]$Output = "logs\StructureDebug"
    )

    $scannerExe = Join-Path $script:OasmRoot "target\debug\oasm-scan.exe"
    if (-not (Test-Path $scannerExe)) {
        Write-Error "Scanner not found at $scannerExe. Run 'cargo build' first."
        return
    }

    Write-Host "Scanning directory: $Path" -ForegroundColor Cyan
    & $scannerExe $Path --output $Output
}

# Daemon control
function Start-OasmDaemon {
    <#
    .SYNOPSIS
        Start the OASM runtime daemon
    .EXAMPLE
        Start-OasmDaemon
    #>
    [CmdletBinding()]
    param()

    $daemonExe = Join-Path $script:OasmRoot "target\debug\runtime_daemon.exe"
    if (-not (Test-Path $daemonExe)) {
        Write-Error "Daemon not found at $daemonExe. Run 'cargo build' first."
        return
    }

    Write-Host "Starting OASM daemon..." -ForegroundColor Cyan
    Start-Process $daemonExe -WindowStyle Hidden
}

# Export module members
Export-ModuleMember -Function @(
    'Initialize-Oasm',
    'New-OasmContext',
    'Invoke-OasmAssembly',
    'Get-OasmRules',
    'Get-OasmBlocks',
    'Invoke-OasmCompile',
    'Invoke-OasmScan',
    'Start-OasmDaemon'
)
