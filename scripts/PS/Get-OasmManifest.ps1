# Get-OasmManifest.ps1
# Query the master manifest for module locations, configs, schemas, etc.

<#
.SYNOPSIS
    Query OASM master manifest for easy location of modules, files, and settings

.DESCRIPTION
    Provides convenient access to the OASM manifest system for locating:
    - Modules (daemon, shell, UI, compiler, etc.)
    - Configuration files
    - Schemas and templates
    - Output directories
    - Integration points

.PARAMETER Type
    Type of item to query: module, config, schema, template, output, integration

.PARAMETER Name
    Name or ID of the item

.PARAMETER List
    List all items of the specified type

.EXAMPLE
    Get-OasmManifest -Type module -Name runtime_daemon

.EXAMPLE
    Get-OasmManifest -Type module -List

.EXAMPLE
    Get-OasmManifest -Type config -Name runtime
#>

[CmdletBinding()]
param(
    [Parameter(Mandatory)]
    [ValidateSet("module", "config", "schema", "template", "output", "integration", "all")]
    [string]$Type,

    [Parameter()]
    [string]$Name,

    [Parameter()]
    [switch]$List
)

$ManifestPath = "manifests\master_manifest.yaml"

if (-not (Test-Path $ManifestPath)) {
    Write-Error "Master manifest not found: $ManifestPath"
    exit 1
}

# Load manifest
$Manifest = Get-Content $ManifestPath -Raw | ConvertFrom-Yaml

function Show-Module {
    param([string]$Id)

    if ($Id) {
        $module = $Manifest.modules | Where-Object { $_.id -eq $Id }
        if ($module) {
            Write-Host "`nModule: $($module.name)" -ForegroundColor Cyan
            Write-Host "  ID:           $($module.id)"
            Write-Host "  Type:         $($module.type)"
            Write-Host "  Location:     $($module.location)"
            if ($module.entry) {
                Write-Host "  Entry:        $($module.entry)"
            }
            if ($module.config) {
                Write-Host "  Config:       $($module.config)"
            }
            Write-Host "  Auto-start:   $($module.auto_start)"
            Write-Host "  Capabilities: $($module.capabilities -join ', ')"
            if ($module.dependencies.Count -gt 0) {
                Write-Host "  Dependencies: $($module.dependencies -join ', ')"
            }
        } else {
            Write-Warning "Module '$Id' not found"
        }
    } else {
        Write-Host "`nAvailable Modules:" -ForegroundColor Cyan
        $Manifest.modules | ForEach-Object {
            $status = if ($_.auto_start) { "[AUTO]" } else { "      " }
            Write-Host "  $status $($_.id.PadRight(20)) - $($_.name)"
        }
    }
}

function Show-Config {
    param([string]$ConfigType)

    if ($ConfigType) {
        $config = $Manifest.configs.$ConfigType
        if ($config) {
            Write-Host "`nConfig: $ConfigType" -ForegroundColor Cyan
            Write-Host "  Primary:  $($config.primary)"
            if ($config.schema) {
                Write-Host "  Schema:   $($config.schema)"
            }
            if ($config.fallback) {
                Write-Host "  Fallback: $($config.fallback)"
            }
        } else {
            Write-Warning "Config type '$ConfigType' not found"
        }
    } else {
        Write-Host "`nAvailable Configs:" -ForegroundColor Cyan
        $Manifest.configs.PSObject.Properties | ForEach-Object {
            Write-Host "  $($_.Name.PadRight(15)) - $($_.Value.primary)"
        }
    }
}

function Show-Schema {
    param([string]$SchemaId)

    if ($SchemaId) {
        $schema = $Manifest.schemas | Where-Object { $_.id -eq $SchemaId }
        if ($schema) {
            Write-Host "`nSchema: $SchemaId" -ForegroundColor Cyan
            Write-Host "  Format:    $($schema.format)"
            Write-Host "  Location:  $($schema.location)"
            Write-Host "  Validates: $($schema.validates)"
        } else {
            Write-Warning "Schema '$SchemaId' not found"
        }
    } else {
        Write-Host "`nAvailable Schemas:" -ForegroundColor Cyan
        $Manifest.schemas | ForEach-Object {
            Write-Host "  $($_.id.PadRight(25)) - $($_.validates)"
        }
    }
}

function Show-Template {
    param([string]$Category)

    if ($Category) {
        $templateCat = $Manifest.templates.$Category
        if ($templateCat) {
            Write-Host "`nTemplate Category: $Category" -ForegroundColor Cyan
            Write-Host "  Location: $($templateCat.location)"
            Write-Host "  Templates:"
            $templateCat.index | ForEach-Object {
                Write-Host "    - $_"
            }
        } else {
            Write-Warning "Template category '$Category' not found"
        }
    } else {
        Write-Host "`nTemplate Categories:" -ForegroundColor Cyan
        $Manifest.templates.PSObject.Properties | ForEach-Object {
            Write-Host "  $($_.Name.PadRight(15)) - $($_.Value.index.Count) templates"
        }
    }
}

function Show-Output {
    param([string]$OutputType)

    Write-Host "`nOutput Locations:" -ForegroundColor Cyan
    Write-Host "  Logs:"
    $Manifest.outputs.logs.PSObject.Properties | ForEach-Object {
        Write-Host "    $($_.Name.PadRight(20)) → $($_.Value)"
    }
    Write-Host "  Exports:"
    $Manifest.outputs.exports.PSObject.Properties | ForEach-Object {
        Write-Host "    $($_.Name.PadRight(20)) → $($_.Value)"
    }
    Write-Host "  Cache:"
    $Manifest.outputs.cache.PSObject.Properties | ForEach-Object {
        Write-Host "    $($_.Name.PadRight(20)) → $($_.Value)"
    }
}

function Show-Integration {
    Write-Host "`nIntegrations:" -ForegroundColor Cyan
    Write-Host "  PowerShell:"
    Write-Host "    Module:  $($Manifest.integrations.powershell.module)"
    Write-Host "    Scripts: $($Manifest.integrations.powershell.scripts)"
    Write-Host "  Python:"
    Write-Host "    Plugins: $($Manifest.integrations.python.plugins)"
    Write-Host "    Venv:    $($Manifest.integrations.python.venv)"
    Write-Host "  wpshell:"
    Write-Host "    Enabled: $($Manifest.integrations.wpshell.enabled)"
    Write-Host "  Objex:"
    Write-Host "    Enabled: $($Manifest.integrations.objex.enabled)"
    if ($Manifest.integrations.objex.hdf5_archives) {
        Write-Host "    HDF5:    $($Manifest.integrations.objex.hdf5_archives)"
    }
}

function Show-All {
    Show-Module
    Write-Host ""
    Show-Config
    Write-Host ""
    Show-Schema
    Write-Host ""
    Show-Template
    Write-Host ""
    Show-Output
    Write-Host ""
    Show-Integration
}

# Main execution
switch ($Type) {
    "module"      { Show-Module -Id $Name }
    "config"      { Show-Config -ConfigType $Name }
    "schema"      { Show-Schema -SchemaId $Name }
    "template"    { Show-Template -Category $Name }
    "output"      { Show-Output }
    "integration" { Show-Integration }
    "all"         { Show-All }
}
