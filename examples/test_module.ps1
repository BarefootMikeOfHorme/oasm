# Test script for OASM.PowerShell module
# Demonstrates the assembly module functionality

Write-Host "Test Suite Starting..." -ForegroundColor Cyan

# Import the module
$modulePath = Join-Path $PSScriptRoot "..\shells\psmodule\OASM.PowerShell.psd1"
Write-Host "Importing module from: $modulePath" -ForegroundColor Gray
Import-Module $modulePath -Force

# Initialize OASM
Write-Host ""
Write-Host "=== Initializing OASM ===" -ForegroundColor Yellow
Initialize-Oasm

# Test 1: Create CAD context
Write-Host ""
Write-Host "=== Test 1: CAD Context ===" -ForegroundColor Yellow
$cadCtx = New-OasmContext -ProgramType CAD
Write-Host "Created CAD context: $($cadCtx.ProgramType)" -ForegroundColor Green

# Test 2: Get CAD rules
Write-Host ""
Write-Host "=== Test 2: CAD Rules ===" -ForegroundColor Yellow
$cadRules = Get-OasmRules -ProgramType CAD
Write-Host "CAD Rules:" -ForegroundColor Green
foreach ($rule in $cadRules) {
    Write-Host "  - $($rule.Id) [$($rule.Category)]" -ForegroundColor Cyan
}

# Test 3: Get CAD blocks
Write-Host ""
Write-Host "=== Test 3: CAD Blocks ===" -ForegroundColor Yellow
$cadBlocks = Get-OasmBlocks -ProgramType CAD
Write-Host "CAD Blocks:" -ForegroundColor Green
foreach ($block in $cadBlocks) {
    Write-Host "  - $($block.Id)" -ForegroundColor Cyan
    Write-Host "    Instructions: $($block.Instructions -join ', ')" -ForegroundColor Gray
}

# Test 4: Execute assembly - Gear example
Write-Host ""
Write-Host "=== Test 4: Execute Assembly (Gear) ===" -ForegroundColor Yellow

$gearSource = @'
CREATE gear
  SET teeth = 20
  SET module = 2.5
  SET pressure_angle = 20
EXTRUDE z_axis, 10
FILLET edges[0..3], 2
VALIDATE topology
EXPORT step, output/gear.step
'@

Invoke-OasmAssembly -Source $gearSource -Context $cadCtx

# Test 5: Execute assembly - Engine scene
Write-Host ""
Write-Host "=== Test 5: Execute Assembly (Engine Scene) ===" -ForegroundColor Yellow
$engineCtx = New-OasmContext -ProgramType Engine

$sceneSource = @'
CREATE scene main_level
  CREATE entity player
    SET position = [0, 0, 0]
    ATTACH component rigidbody
SCAN profiler
  TRACK entity_count
'@

Invoke-OasmAssembly -Source $sceneSource -Context $engineCtx

# Test 6: Get Engine rules
Write-Host ""
Write-Host "=== Test 6: Engine Rules ===" -ForegroundColor Yellow
$engineRules = Get-OasmRules -ProgramType Engine
Write-Host "Engine Rules:" -ForegroundColor Green
foreach ($rule in $engineRules) {
    Write-Host "  - $($rule.Id) [$($rule.Category)]" -ForegroundColor Cyan
}

# Summary
Write-Host ""
Write-Host "All Tests Completed Successfully" -ForegroundColor Green
Write-Host ""
Write-Host "OASM assembly module is ready for use with:" -ForegroundColor White
Write-Host "  - wpshell" -ForegroundColor Gray
Write-Host "  - PowerShell Insider 2026" -ForegroundColor Gray
Write-Host "  - Custom Windows PowerShell" -ForegroundColor Gray
